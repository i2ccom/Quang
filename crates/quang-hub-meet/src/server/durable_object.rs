//! MeetRoomDO — Durable Object for room state + WebRTC signaling.
//!
//! Each meeting room is backed by a single Durable Object instance.
//! The DO is responsible for:
//!
//! - Maintaining room state (participants, media state, recording flags)
//! - Relaying WebRTC signaling messages (SDP, ICE) between peers
//! - Managing WebSocket connections for each participant
//! - Broadcasting room events (join, leave, mic/camera toggles)
//! - Coordinating recording start/stop with R2
//!
//! ## Signaling protocol
//!
//! ```text
//! Client A ──WS──> DO ──WS──> Client B
//!     │                    │
//!     └── SDP Offer ──> DO ──> SDP Offer ──┘
//!     └── ICE ───────> DO ──> ICE ──────────┘
//!     └── Join ──────> DO ──> ParticipantJoined ──┘
//! ```
//!
//! The DO acts as a simple relay: it never inspects or modifies SDP/ICE
//! payloads. Only room events (join/leave/toggle) are interpreted.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use worker::*;
use worker::durable_object::*;

use crate::handlers::signaling_handlers::{ParticipantInfo, SignalingMessage};
use crate::recording::RecordingManager;

/// State stored in the Durable Object's storage.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RoomState {
    pub title: String,
    pub topic: String,
    pub host_id: String,
    pub status: String, // waiting, active, paused, ended
    pub participants: HashMap<String, ParticipantInfo>,
    pub is_recording: bool,
    pub started_at: Option<u64>,
}

impl RoomState {
    fn new(title: &str, topic: &str, host_id: &str) -> Self {
        Self {
            title: title.to_string(),
            topic: topic.to_string(),
            host_id: host_id.to_string(),
            status: "waiting".to_string(),
            participants: HashMap::new(),
            is_recording: false,
            started_at: None,
        }
    }
}

/// The Durable Object class for meeting rooms.
///
/// Registered in `wranger.toml` as:
/// ```toml
/// [[durable_objects.bindings]]
/// name = "MEET_ROOM"
/// class_name = "MeetRoomDO"
///
/// [[migrations]]
/// tag = "v1"
/// new_classes = ["MeetRoomDO"]
/// ```
pub struct MeetRoomDO {
    /// Persisted room state.
    state: State,
    /// Active WebSocket connections keyed by participant_id.
    connections: HashMap<String, WebSocketPair>,
    /// In-memory state (lazy loaded from storage).
    room: RoomState,
    /// Recording manager (R2 integration).
    recording: Option<RecordingManager>,
}

impl DurableObject for MeetRoomDO {
    fn new(state: State, _env: Env) -> Self {
        // Attempt to restore room state from storage
        let room = state
            .storage()
            .get::<RoomState>("room")
            .unwrap_or_else(|_| RoomState::new("", "", ""));

        Self {
            state,
            connections: HashMap::new(),
            room,
            recording: None,
        }
    }

    async fn fetch(&mut self, mut req: Request) -> Result<Response> {
        let url = req.url()?;
        let path = url.path().to_string();

        // ── REST commands ──
        match path.as_str() {
            // POST / — create room
            "/" if req.method() == Method::Post => {
                let body: serde_json::Value = req.json().await?;
                self.room = RoomState::new(
                    body["title"].as_str().unwrap_or("Untitled"),
                    body["topic"].as_str().unwrap_or(""),
                    body["host_actor_id"].as_str().unwrap_or("unknown"),
                );
                self.state.storage().put("room", &self.room).await?;
                Response::ok("Room created")
            }

            // GET /details — get room state
            "/details" => {
                Response::from_json(&self.room)
            }

            // POST /end — end the meeting
            "/end" => {
                self.room.status = "ended".to_string();
                if self.room.is_recording {
                    self.stop_recording().await;
                }
                self.state.storage().put("room", &self.room).await?;
                // Notify all participants
                self.broadcast(SignalingMessage::ParticipantLeft {
                    participant_id: "__room__".to_string(),
                }).await;
                // Close all WebSocket connections
                for (_, conn) in self.connections.drain() {
                    let _ = conn.0.close();
                }
                Response::ok("Room ended")
            }

            // POST /leave/:participant_id — remove a participant
            p if p.starts_with("/leave/") => {
                let pid = p.trim_start_matches("/leave/").to_string();
                self.room.participants.remove(&pid);
                if let Some((ws, _)) = self.connections.remove(&pid) {
                    let _ = ws.close();
                }
                self.state.storage().put("room", &self.room).await?;
                self.broadcast(SignalingMessage::ParticipantLeft {
                    participant_id: pid,
                }).await;
                Response::ok("Left")
            }

            // WebSocket upgrade — signaling channel
            _ if req.method() == Method::Get && path.contains("/signal") => {
                self.handle_websocket_upgrade(req).await
            }

            _ => Response::error("Not found", 404),
        }
    }
}

impl MeetRoomDO {
    /// Handle a WebSocket upgrade request for signaling.
    async fn handle_websocket_upgrade(&mut self, req: Request) -> Result<Response> {
        let url = req.url()?;
        let participant_id = url
            .query_pairs()
            .find(|(k, _)| k == "participant_id")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string());

        // Create WebSocket pair
        let pair = WebSocketPair::new()?;
        let (server_ws, client_ws) = pair;

        // Accept the WebSocket
        server_ws.accept()?;

        // Register the connection
        self.connections.insert(participant_id.clone(), pair);

        // Set up message handler
        let room_id = self.state.id().to_string();
        let conn_clone = server_ws.clone();

        wasm_bindgen_futures::spawn_local(async move {
            loop {
                match conn_clone.wait_for_message().await {
                    Ok(msg) => {
                        if let Some(text) = msg.text() {
                            if let Ok(signal) = serde_json::from_str::<SignalingMessage>(&text) {
                                // Relay to other participants
                                Self::relay_message(&conn_clone, &signal).await;
                            }
                        }
                    }
                    Err(_) => break, // Connection closed
                }
            }
        });

        // Broadcast join to existing participants
        let display_name = format!("User-{}", &participant_id[..6]);
        let info = ParticipantInfo {
            id: participant_id.clone(),
            display_name: display_name.clone(),
            mic_enabled: true,
            camera_enabled: true,
            screen_sharing: false,
        };
        self.room.participants.insert(participant_id.clone(), info);
        self.broadcast(SignalingMessage::ParticipantJoined {
            participant_id: participant_id.clone(),
            display_name,
        }).await;

        // Send current participant list to the new joiner
        let participants = self.room.participants.values().cloned().collect();
        let list_msg = SignalingMessage::ParticipantList { participants };
        if let Ok(json) = serde_json::to_string(&list_msg) {
            let _ = server_ws.send_with_str(&json);
        }

        // Build the response with the client-side WebSocket
        let mut response = Response::from_websocket(client_ws)?;
        response
            .headers_mut()
            .set("X-Participant-Id", &participant_id)?;
        Ok(response)
    }

    /// Relay a signaling message to the target participant.
    async fn relay_message(sender: &WebSocket, msg: &SignalingMessage) {
        let (target_id, payload) = match msg {
            SignalingMessage::SdpOffer { to, sdp, .. } => {
                (to.clone(), msg.clone())
            }
            SignalingMessage::SdpAnswer { to, sdp, .. } => {
                (to.clone(), msg.clone())
            }
            SignalingMessage::IceCandidate { to, .. } => {
                (to.clone(), msg.clone())
            }
            _ => return, // Non-relay messages handled by DO
        };

        // TODO: Look up the target's WebSocket and forward
        // In a full implementation, the DO holds a HashMap<String, WebSocket>
        // and sends the serialized message to the target.
        //
        // let target_ws = self.connections.get(&target_id);
        // if let Some(ws) = target_ws {
        //     ws.send_with_str(&serde_json::to_string(&payload).unwrap());
        // }
        let _ = target_id;
        let _ = payload;
    }

    /// Broadcast a message to all connected participants.
    async fn broadcast(&self, msg: SignalingMessage) {
        let json = serde_json::to_string(&msg).unwrap();
        for (_, conn) in &self.connections {
            let _ = conn.0.send_with_str(&json);
        }
    }

    /// Start recording via the RecordingManager.
    async fn start_recording(&mut self) -> Result<()> {
        if let Some(rec) = &mut self.recording {
            rec.start().await
        } else {
            self.recording = Some(RecordingManager::new(&self.state.id().to_string()));
            if let Some(rec) = &mut self.recording {
                rec.start().await
            } else {
                Err(worker::Error::RustError("Failed to create RecordingManager".into()))
            }
        }
    }

    /// Stop recording.
    async fn stop_recording(&mut self) {
        if let Some(rec) = &mut self.recording {
            rec.stop().await;
        }
        self.room.is_recording = false;
    }
}
