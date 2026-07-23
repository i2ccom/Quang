# quang-hub-meet — Implementation Plan

## Overview

`quang-hub-meet` provides real-time video/audio meetings for QuangHub. It spans three compilation targets:

| Target | Feature | Crate Root | Description |
|--------|---------|------------|-------------|
| Wasm (browser) | `web` | `src-web/` | Dioxus UI components + WebRTC |
| Cloudflare Workers | `server` | `src-server/` | REST API + DurableObject signaling |
| Native/shared | default | `src/` | Data models + `MeetHub` orchestrator |

---

## Phase 0: Foundation ✅ (Complete)

- [x] Core data models: `room`, `participant`, `media`, `recording`, `chat`, `event`, `hub`
- [x] `MeetHub` orchestrator with full lifecycle management
- [x] Cargo workspace integration (`quang-hub-workplace` graph types)
- [x] Feature flags: `web`, `server`, `full`
- [x] Dioxus 0.6 dependency setup (optional via `web` feature)
- [x] `worker` (cloudflare-workers) 0.4 dependency setup (optional via `server` feature)

## Phase 1: Web UI — Pages ✅ (Complete)

- [x] `src-web/lib.rs` — Feature-gated web module entry point
- [x] `src-web/pages/meet_home.rs` — Room listing, create room form, join-by-link
- [x] `src-web/pages/meet_room.rs` — Active meeting with video grid, controls, chat sidebar
- [x] `src-web/pages/meet_schedule.rs` — Schedule future meeting form with date/time/duration

## Phase 1.5: Web UI — Components ✅ (Complete)

- [x] `video_tile.rs` — Single participant tile: live `<video>` or avatar fallback, name overlay, mute indicator, handraise badge
- [x] `video_grid.rs` — Responsive CSS grid (1–4 columns), automatic screen-share spotlight layout
- [x] `participant_bar.rs` — Participant list sidebar with mic/camera/handraise indicators
- [x] `media_controls.rs` — Mic, camera, screen share, record, leave buttons with `EventHandler` callbacks
- [x] `in_call_chat.rs` — Message list + text input with Enter-to-send
- [x] `screen_share_overlay.rs` — Full-screen share view with mini participant bar
- [x] `meeting_info_panel.rs` — Invite link copy, duration, recording status, room ID

## Phase 2: WebRTC Integration 🟡 (In Progress)

- [x] `src-web/webrtc/mod.rs` — `PeerConnectionManager` abstraction (API scaffold)
- [ ] **Connect Browser WebRTC API** — Use `web-sys::RTCPeerConnection` for real ICE/STUN/TURN
- [ ] **Local media acquisition** — `navigator.mediaDevices.getUserMedia()` for mic/camera
- [ ] **Screen capture** — `navigator.mediaDevices.getDisplayMedia()` for screen sharing
- [ ] **Stream-to-video binding** — Attach `MediaStream` to `<video>` elements by track ID
- [ ] **ICE candidate handling** — Process and forward ICE candidates via signaling
- [ ] **Connection state monitoring** — Track `connectionstatechange` for reconnection
- [ ] **Simulcast support** — 3-layer SVC for scalable video (SFU mode)

### Milestone: P2P call works between 2 browser tabs

| Task | Est. | Owner |
|------|------|-------|
| getUserMedia + local preview | 2d | - |
| RTCPeerConnection + SDP exchange | 3d | - |
| ICE + STUN/TURN | 2d | - |
| 2-tab test | 1d | - |

## Phase 3: DurableObject Signaling 🟡 (In Progress)

- [x] `src-server/durable_object.rs` — `MeetRoomDO` with REST commands + WebSocket upgrade
- [x] `src-server/handlers/signaling_handlers.rs` — `SignalingMessage` enum + WebSocket handler
- [x] `src-server/handlers/room_handlers.rs` — Room CRUD proxied to DO
- [ ] **Worker router** — `wrangler.toml` + `lib.rs` entry point with route macros
- [ ] **DO persistence** — `state.storage().put()` / `.get()` for room state across restarts
- [ ] **Participant list sync** — Full sync on connect, incremental on join/leave
- [ ] **WebSocket reconnection** — Handle disconnect/reconnect with participant state recovery
- [ ] **Rate limiting** — Throttle signaling messages per participant
- [ ] **Room expiry** — Auto-delete DO after `status == ended` + TTL

### Milestone: Two clients can signal via DO

| Task | Est. | Owner |
|------|------|-------|
| Worker route config | 1d | - |
| WebSocket upgrade in DO | 2d | - |
| End-to-end SDP relay test | 2d | - |

## Phase 4: Recording 🟡 (In Progress)

- [x] `src-server/recording.rs` — `RecordingManager` with multipart upload lifecycle
- [ ] **R2 bucket binding** — Wire `RecordingManager` to actual R2 bucket
- [ ] **Multipart upload implementation** — `create_multipart_upload`, `upload_part`, `complete`
- [ ] **Recording toggle in DO** — Wire start/stop to DO REST endpoints
- [ ] **Recording UI** — Wire `is_recording` signal to the DO endpoint
- [ ] **Recording notification** — Emit `RecordingReady` event when finalized
- [ ] **Download UI** — Presigned URL button in meeting info panel

### Milestone: Record a call and download from R2

| Task | Est. | Owner |
|------|------|-------|
| R2 binding + env config | 1d | - |
| Multipart upload loop | 2d | - |
| UI record button wired | 1d | - |

## Phase 5: Screen Share ⬜ (Planned)

- [ ] **getDisplayMedia integration** — Capture screen stream in `PeerConnectionManager`
- [ ] **Screen share track substitution** — Replace/Add video track in peer connection
- [ ] **Spotlight layout** — `VideoGrid` switches to main + sidebar (scaffold done)
- [ ] **Screen share overlay** — Full-screen view (scaffold done, needs stream binding)
- [ ] **Presenter highlighting** — Border indicator on sharer's tile
- [ ] **Stop share on tab close** — Listen for `ended` event on screen track

## Phase 6: AI Transcription ⬜ (Planned)

- [ ] **Cloudflare Workers AI binding** — `@cf/openai/whisper` for audio-to-text
- [ ] **Audio extraction from recording** — FFmpeg WASM or Workers AI media processing
- [ ] **Transcription queue** — `POST /api/meet/rooms/:id/transcribe` → Queue → Worker
- [ ] **VTT generation** — Convert Whisper segments to WebVTT format
- [ ] **Live captions** — Real-time Whisper streaming during active meeting
- [ ] **Transcript in UI** — Sidebar toggle showing live captions
- [ ] **Summary generation** — LLM call to summarize transcript after meeting ends

## Phase 7: Polish & Production ⬜ (Planned)

- [ ] **Connection health overlay** — Show network quality, packet loss, jitter
- [ ] **Bandwidth adaptation** — Dynamically switch between simulcast layers
- [ ] **Noise suppression** — Integrate RNNoise or WebRTC noise suppression
- [ ] **Virtual backgrounds** — WASM-based background blur/replace
- [ ] **Meeting reactions** — Emoji reactions overlay (👍🎉😂)
- [ ] **Keyboard shortcuts** — M (mute), V (camera), Esc (stop share)
- [ ] **Responsive mobile layout** — Stacked layout on small screens
- [ ] **Accessibility** — ARIA labels, focus management, screenreader support

---

## Dependency Graph

```
Phase 0 (Models)
    │
    ▼
Phase 1 (Web UI) ─────► Phase 1.5 (Components)
    │                           │
    │                           ▼
    │                    Phase 2 (WebRTC)
    │                           │
    ▼                           ▼
Phase 3 (DO Signaling) ◄───────┘
    │
    ├──► Phase 4 (Recording)
    │         │
    │         ▼
    │    Phase 6 (AI Transcription)
    │
    └──► Phase 5 (Screen Share)
                  │
                  ▼
           Phase 7 (Polish)
```

## Key Design Decisions

1. **Mesh → SFU hybrid**: P2P for small rooms, SFU for larger ones. Simpler than full SFU from day one while scaling well.
2. **DO per room**: Each room is an isolated DO. No cross-room state, simple scaling.
3. **DO as transparent signaling proxy**: Never inspect SDP/ICE — just relay. Keeps the DO simple and avoids serialization issues.
4. **R2 multipart upload**: WebRTC recording produces chunks every few seconds. Multipart upload allows streaming rather than buffering the entire recording in memory.
5. **Dioxus Signals**: Reactive state management via `use_signal` — no external state library needed for the meeting UI.
6. **CSS variables**: All components use `--q-*` variables for consistent theming with quang-web.
