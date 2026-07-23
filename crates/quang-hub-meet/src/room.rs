//! MeetingRoom — represents a real-time meeting with lifecycle management.

use serde::{Deserialize, Serialize};

use quang_hub_workplace::graph::{now, ActorId, NodeId, Timestamp};

pub type RoomId = NodeId;

/// The lifecycle state of a meeting room.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RoomStatus {
    Scheduled,
    Waiting,
    Active,
    Paused,
    Ended,
    Cancelled,
}

impl std::fmt::Display for RoomStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RoomStatus::Scheduled => write!(f, "scheduled"),
            RoomStatus::Waiting => write!(f, "waiting"),
            RoomStatus::Active => write!(f, "active"),
            RoomStatus::Paused => write!(f, "paused"),
            RoomStatus::Ended => write!(f, "ended"),
            RoomStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// Configuration for a meeting room.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomConfig {
    pub max_participants: u32,
    pub allow_recording: bool,
    pub allow_chat: bool,
    pub allow_handraise: bool,
    pub allow_screen_share: bool,
    pub ai_agent_allowed: bool,
    pub auto_generate_summary: bool,
    pub mute_on_join: bool,
    pub video_off_on_join: bool,
}

impl Default for RoomConfig {
    fn default() -> Self {
        Self {
            max_participants: 50,
            allow_recording: true,
            allow_chat: true,
            allow_handraise: true,
            allow_screen_share: true,
            ai_agent_allowed: true,
            auto_generate_summary: true,
            mute_on_join: true,
            video_off_on_join: false,
        }
    }
}

/// A meeting room that hosts audio/video calls.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeetingRoom {
    pub id: RoomId,
    pub title: String,
    pub topic: String,
    pub status: RoomStatus,
    pub host: ActorId,
    pub config: RoomConfig,
    pub scheduled_at: Option<Timestamp>,
    pub started_at: Option<Timestamp>,
    pub ended_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl MeetingRoom {
    pub fn new(title: &str, topic: &str, host: ActorId) -> Self {
        let now = now();
        Self {
            id: NodeId::new("room"),
            title: title.to_string(),
            topic: topic.to_string(),
            status: RoomStatus::Waiting,
            host,
            config: RoomConfig::default(),
            scheduled_at: None,
            started_at: None,
            ended_at: None,
            created_at: now,
            updated_at: now,
            metadata: serde_json::Map::new(),
        }
    }

    pub fn start(&mut self) {
        self.status = RoomStatus::Active;
        self.started_at = Some(now());
        self.updated_at = now();
    }

    pub fn pause(&mut self) {
        self.status = RoomStatus::Paused;
        self.updated_at = now();
    }

    pub fn resume(&mut self) {
        self.status = RoomStatus::Active;
        self.updated_at = now();
    }

    pub fn end(&mut self) {
        self.status = RoomStatus::Ended;
        self.ended_at = Some(now());
        self.updated_at = now();
    }

    pub fn schedule(&mut self, at: Timestamp) {
        self.scheduled_at = Some(at);
        self.status = RoomStatus::Scheduled;
        self.updated_at = now();
    }

    pub fn is_active(&self) -> bool {
        self.status == RoomStatus::Active
    }

    pub fn kind() -> &'static str {
        "meeting_room"
    }
}
