//! Participant — a human or agent in a meeting room.

use serde::{Deserialize, Serialize};

use quang_hub_workplace::graph::{now, ActorId, NodeId, Timestamp};

pub type ParticipantId = NodeId;

/// Role a participant has in a meeting.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParticipantRole {
    Host,
    CoHost,
    Presenter,
    Participant,
    Viewer,
    AgentListener,
}

impl std::fmt::Display for ParticipantRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParticipantRole::Host => write!(f, "host"),
            ParticipantRole::CoHost => write!(f, "co_host"),
            ParticipantRole::Presenter => write!(f, "presenter"),
            ParticipantRole::Participant => write!(f, "participant"),
            ParticipantRole::Viewer => write!(f, "viewer"),
            ParticipantRole::AgentListener => write!(f, "agent_listener"),
        }
    }
}

/// Audio/video state for a participant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MediaState {
    pub mic_enabled: bool,
    pub camera_enabled: bool,
    pub screen_sharing: bool,
    pub hand_raised: bool,
}

impl Default for MediaState {
    fn default() -> Self {
        Self {
            mic_enabled: true,
            camera_enabled: true,
            screen_sharing: false,
            hand_raised: false,
        }
    }
}

/// A participant in a meeting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Participant {
    pub id: ParticipantId,
    pub room_id: NodeId,
    pub actor: ActorId,
    pub role: ParticipantRole,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub media: MediaState,
    pub joined_at: Timestamp,
    pub left_at: Option<Timestamp>,
    pub is_connected: bool,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Participant {
    pub fn new(room_id: NodeId, actor: ActorId, role: ParticipantRole, display_name: &str) -> Self {
        Self {
            id: NodeId::new("part"),
            room_id,
            actor,
            role,
            display_name: display_name.to_string(),
            avatar_url: None,
            media: MediaState::default(),
            joined_at: now(),
            left_at: None,
            is_connected: true,
            metadata: serde_json::Map::new(),
        }
    }

    pub fn toggle_mic(&mut self) {
        self.media.mic_enabled = !self.media.mic_enabled;
    }

    pub fn toggle_camera(&mut self) {
        self.media.camera_enabled = !self.media.camera_enabled;
    }

    pub fn toggle_screen_share(&mut self) {
        self.media.screen_sharing = !self.media.screen_sharing;
    }

    pub fn raise_hand(&mut self) {
        self.media.hand_raised = true;
    }

    pub fn lower_hand(&mut self) {
        self.media.hand_raised = false;
    }

    pub fn disconnect(&mut self) {
        self.is_connected = false;
        self.left_at = Some(now());
    }

    pub fn kind() -> &'static str {
        "participant"
    }
}
