# Meet Architecture

This document describes the real-time meeting architecture for QuangHub Meet: WebRTC topology, DurableObject signaling, recording flow, and AI transcription.

---

## 1. WebRTC Topology

QuangHub Meet supports **two topologies**, selected automatically based on room size:

### 1.1 Peer-to-Peer (Mesh) — ≤4 participants

Each participant creates an `RTCPeerConnection` to every other participant.

```
  ┌──────┐  SDP/ICE   ┌──────┐
  │ Alice │◄─────────►│  Bob  │
  └──┬───┘            └──┬───┘
     │   ┌──────┐        │
     └──►│ Carol│◄───────┘
         └──────┘
```

**Pros**: No server-side media processing, lowest latency.
**Cons**: Upload bandwidth scales O(n²). Not viable beyond ~4 participants.

### 1.2 SFU (Selective Forwarding Unit) — ≥5 participants

A lightweight SFU runs on Cloudflare Workers (or a dedicated Edge SFU). Participants send one uplink; the SFU fans out to all others.

```
         ┌─────────┐
    ┌───►│  SFU    │◄───┐
    │    │(Worker) │    │
    │    └──┬──┬───┘    │
    │       │  │        │
  ┌─┴─┐   ┌─┴──┴─┐   ┌─┴─┐
  │A  │   │  B   │   │ C │
  └───┘   └──────┘   └───┘
```

**Pros**: Upload bandwidth O(n), scales to 50+ participants.
**Cons**: Adds 1-hop latency, requires server-side media relay.

### 1.3 Simulcast (SFU only)

High-end clients send 3 spatial layers (low/med/high resolution). The SFU forwards only the appropriate layer to each receiver based on their viewport and bandwidth.

---

## 2. DurableObject Signaling

Each meeting room maps to exactly one **Durable Object (DO)** instance.

### 2.1 Registration

```
wrangler.toml:
[[durable_objects.bindings]]
name = "MEET_ROOM"
class_name = "MeetRoomDO"
```

DO IDs are derived from the room's `NodeId`:

```rust
let room_id = NodeId::new("room").to_string();
let do_id = env.durable_object("MEET_ROOM")?.id_from_name(&room_id)?;
```

### 2.2 Signaling Protocol

```
  Client A                     DO MeetRoomDO                 Client B
     │                            │                             │
     │  POST /api/meet/rooms      │                             │
     │  ─────────────────────────►│                             │
     │   { title, host_actor_id } │                             │
     │◄───────────────────────────│                             │
     │   { room_id, status }      │                             │
     │                            │                             │
     │  POST /api/meet/rooms/:id/join                           │
     │  ─────────────────────────►│                             │
     │   { actor_id, name }       │                             │
     │◄───────────────────────────│                             │
     │  { participant_id,         │                             │
     │    signaling_url,          │                             │
     │    turn_credentials }      │                             │
     │                            │                             │
     │  ─── WebSocket Upgrade ────│──── WebSocket Upgrade ─────►│
     │  /signal?participant_id=.. │    /signal?participant_id=..│
     │                            │                             │
     │  ── SDP Offer ────────────►│── SDP Offer ──────────────►│
     │  ◄── SDP Answer ──────────│◄── SDP Answer ─────────────│
     │  ◄─ ICE Candidate ────────│◄── ICE Candidate ──────────│
     │  ── ICE Candidate ───────►│── ICE Candidate ──────────►│
     │                            │                             │
     │  ◄── ParticipantJoined ◄──│── ParticipantJoined ──────►│
     │  ◄── ParticipantList ◄─── │                             │
```

### 2.3 Message Relay (DO as Transparent Proxy)

The DO **never inspects or modifies** SDP or ICE payloads. It only:

1. Receives a `SignalingMessage` over WebSocket
2. Looks up the `to` participant's WebSocket in `self.connections`
3. Forwards the raw message

---

## 3. Recording Flow

Recording is initiated by the host and streamed to **Cloudflare R2** via multipart upload.

```
  Host clicks "Record"
       │
       ▼
  DO receives POST /api/meet/rooms/:id/recording/start
       │
       ▼
  RecordingManager::new(room_id)
       │
       ├──→ R2 Bucket: create_multipart_upload(key)
       │       key = "meetings/{room_id}/recording_{timestamp}.webm"
       │
       ▼
  Media chunks arrive from SFU (every 5s)
       │
       ▼
  rec.write_chunk(data)  →  bucket.upload_part(upload_id, part_n, data)
       │
       ▼  (repeated every 5s until stop)
  Host clicks "Stop" / Meeting ends
       │
       ▼
  rec.stop()  →  bucket.complete_multipart_upload(upload_id, parts)
       │
       ▼
  DO emits MeetingEvent::RecordingReady { room_id, recording_id }
       │
       ▼
  R2 object available → presigned URL (24h expiry) for download
```

### 3.1 Storage Layout in R2

```
meetings/
  ├── {room_id}/
  │   ├── recording_20250601_143000.webm
  │   ├── recording_20250601_143000.json    (metadata)
  │   └── transcript_20250601_143000.vtt    (captions)
  └── ...
```

### 3.2 Recording Metadata

```json
{
  "room_id": "room_abc123",
  "started_at": 1748275200000,
  "duration_secs": 3600,
  "mime_type": "video/webm",
  "size_bytes": 524288000,
  "participant_count": 4
}
```

---

## 4. AI Transcription Pipeline

After a recording is finalized, the DO triggers an AI transcription job:

```
Recording Ready
       │
       ▼
DO → Queue: transcription/{room_id}/{recording_id}
       │
       ▼
Worker picks up from Queue
       │
       ├── Download audio from R2 (presigned URL)
       ├── Send to Whisper (or Workers AI)
       │     POST /accounts/{id}/ai/run/@cf/openai/whisper
       │     multipart: audio file → { text, segments[] }
       │
       ▼
  Store transcript in R2 as VTT + JSON
       │
       ▼
  Emit MeetingEvent::TranscriptionReady
```

### 4.1 Real-Time Captions (optional)

With Workers AI streaming, captions can be generated in near-real-time:

```
  SFU audio stream (Opus packets)
       │
       ▼
  Audio chunk → Whisper streaming endpoint
       │
       ▼
  Caption segment → DO → broadcast to all clients
       │
       ▼
  InCallChat shows live captions
```

---

## 5. TURN Server Integration

TURN credentials are generated per-participant using the **Coturn REST API**:

```
  username = <expiry_timestamp>:<participant_id>
  password = base64(hmac-sha256(turn_secret, username))
```

**TURN servers** (fallback for restrictive NATs):

| Protocol | URL |
|----------|-----|
| TURN/UDP | `turn:turn.quanghub.app:3478?transport=udp` |
| TURN/TCP | `turn:turn.quanghub.app:3478?transport=tcp` |
| STUN     | `stun:stun.quanghub.app:3478` |

---

## 6. Component Data Flow

```
                    ┌─────────────────────┐
                    │    MeetRoom (page)   │
                    │  ┌───────────────┐  │
                    │  │  VideoGrid    │  │
                    │  │  ┌─────────┐  │  │
                    │  │  │VideoTile│  │  │
                    │  │  └─────────┘  │  │
                    │  └───────────────┘  │
                    │  ┌───────────────┐  │
                    │  │ MediaControls │  │
                    │  └───────────────┘  │
                    │  ┌───────────────┐  │
                    │  │ InCallChat    │  │
                    │  └───────────────┘  │
                    │  ┌───────────────┐  │
                    │  │ParticipantBar │  │
                    │  └───────────────┘  │
                    │  ┌───────────────┐  │
                    │  │ScreenShareOvr │  │
                    │  └───────────────┘  │
                    │  ┌───────────────┐  │
                    │  │MeetingInfoPnl │  │
                    │  └───────────────┘  │
                    └─────────┬───────────┘
                              │ Dioxus Signals
                    ┌─────────▼───────────┐
                    │ PeerConnectionMgr   │  (webrtc/mod.rs)
                    │  ┌───────────────┐  │
                    │  │RTCConnection  │  │
                    │  │SDP Negotiation│  │
                    │  │ICE Management │  │
                    │  │Stream Binding │  │
                    │  └───────────────┘  │
                    └─────────┬───────────┘
                              │ WebSocket
                    ┌─────────▼───────────┐
                    │  DO / MeetRoomDO    │  (Cloudflare Worker)
                    │  ┌───────────────┐  │
                    │  │ Room State    │  │
                    │  │ Signaling REL │  │
                    │  │ Broadcasting  │  │
                    │  │ Recording Mgr │  │
                    │  └───────────────┘  │
                    └─────────────────────┘
```
