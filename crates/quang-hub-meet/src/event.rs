//! MeetingEvent — all events emitted during meetings for WebSocket, AI, and audit.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use quang_hub_workplace::graph::{ActorId, NodeId};

/// All events that can occur during a meeting.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum MeetingEvent {
    // ── Room lifecycle ──
    RoomCreated {
        room_id: NodeId,
        actor: ActorId,
    },
    RoomStarted {
        room_id: NodeId,
        actor: ActorId,
    },
    RoomEnded {
        room_id: NodeId,
        actor: ActorId,
    },
    RoomPaused {
        room_id: NodeId,
        actor: ActorId,
    },
    RoomResumed {
        room_id: NodeId,
        actor: ActorId,
    },

    // ── Participant events ──
    ParticipantJoined {
        room_id: NodeId,
        participant_id: NodeId,
        actor: ActorId,
    },
    ParticipantLeft {
        room_id: NodeId,
        participant_id: NodeId,
        actor: ActorId,
    },
    ParticipantRoleChanged {
        room_id: NodeId,
        participant_id: NodeId,
        new_role: String,
        actor: ActorId,
    },
    HandRaised {
        room_id: NodeId,
        participant_id: NodeId,
        actor: ActorId,
    },
    HandLowered {
        room_id: NodeId,
        participant_id: NodeId,
        actor: ActorId,
    },

    // ── Media events ──
    MicToggled {
        room_id: NodeId,
        participant_id: NodeId,
        enabled: bool,
    },
    CameraToggled {
        room_id: NodeId,
        participant_id: NodeId,
        enabled: bool,
    },
    ScreenShareStarted {
        room_id: NodeId,
        participant_id: NodeId,
    },
    ScreenShareEnded {
        room_id: NodeId,
        participant_id: NodeId,
    },

    // ── Recording events ──
    RecordingStarted {
        room_id: NodeId,
        actor: ActorId,
    },
    RecordingStopped {
        room_id: NodeId,
        actor: ActorId,
    },
    RecordingReady {
        room_id: NodeId,
        recording_id: NodeId,
    },

    // ── Chat events ──
    ChatMessageSent {
        room_id: NodeId,
        message_id: String,
        actor: ActorId,
    },
    PollCreated {
        room_id: NodeId,
        poll_id: String,
        actor: ActorId,
    },
    PollVoted {
        room_id: NodeId,
        poll_id: String,
        actor: ActorId,
    },
    PollClosed {
        room_id: NodeId,
        poll_id: String,
    },
}

impl MeetingEvent {
    pub fn description(&self) -> String {
        match self {
            MeetingEvent::RoomCreated { room_id, .. } => format!("Room {} created", room_id),
            MeetingEvent::RoomStarted { room_id, .. } => format!("Room {} started", room_id),
            MeetingEvent::RoomEnded { room_id, .. } => format!("Room {} ended", room_id),
            MeetingEvent::RoomPaused { room_id, .. } => format!("Room {} paused", room_id),
            MeetingEvent::RoomResumed { room_id, .. } => format!("Room {} resumed", room_id),
            MeetingEvent::ParticipantJoined { participant_id, .. } => {
                format!("Participant {} joined", participant_id)
            }
            MeetingEvent::ParticipantLeft { participant_id, .. } => {
                format!("Participant {} left", participant_id)
            }
            MeetingEvent::ParticipantRoleChanged {
                participant_id,
                new_role,
                ..
            } => {
                format!(
                    "Participant {} role changed to {}",
                    participant_id, new_role
                )
            }
            MeetingEvent::HandRaised { participant_id, .. } => {
                format!("Hand raised by {}", participant_id)
            }
            MeetingEvent::HandLowered { participant_id, .. } => {
                format!("Hand lowered by {}", participant_id)
            }
            MeetingEvent::MicToggled {
                participant_id,
                enabled,
                ..
            } => {
                format!(
                    "Mic {} for participant {}",
                    if *enabled { "on" } else { "off" },
                    participant_id
                )
            }
            MeetingEvent::CameraToggled {
                participant_id,
                enabled,
                ..
            } => {
                format!(
                    "Camera {} for participant {}",
                    if *enabled { "on" } else { "off" },
                    participant_id
                )
            }
            MeetingEvent::ScreenShareStarted { participant_id, .. } => {
                format!("Screen share started by {}", participant_id)
            }
            MeetingEvent::ScreenShareEnded { participant_id, .. } => {
                format!("Screen share ended by {}", participant_id)
            }
            MeetingEvent::RecordingStarted { .. } => "Recording started".to_string(),
            MeetingEvent::RecordingStopped { .. } => "Recording stopped".to_string(),
            MeetingEvent::RecordingReady { recording_id, .. } => {
                format!("Recording {} ready", recording_id)
            }
            MeetingEvent::ChatMessageSent { .. } => "Chat message sent".to_string(),
            MeetingEvent::PollCreated { poll_id, .. } => format!("Poll {} created", poll_id),
            MeetingEvent::PollVoted { poll_id, .. } => format!("Vote on poll {}", poll_id),
            MeetingEvent::PollClosed { poll_id, .. } => format!("Poll {} closed", poll_id),
        }
    }
}

/// Envelope for a meeting event.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingEventEnvelope {
    pub id: String,
    pub event: MeetingEvent,
    pub timestamp: DateTime<Utc>,
    pub sequence: u64,
}
