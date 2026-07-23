//! Channel — a topic-based communication stream within a workspace.
//!
//! Channels group conversations by topic, project, or team.
//! They contain ChatMessages and support real-time event streaming.

use serde::{Deserialize, Serialize};

use crate::graph::{now, ActorId, NodeId, Timestamp};

pub type ChannelId = NodeId;

/// The type of a channel.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ChannelKind {
    /// General discussion channel
    General,
    /// Project-specific channel
    Project,
    /// Team-specific channel
    Team,
    /// Agent-to-human communication channel
    Agent,
    /// AI-generated summary/digest channel
    Digest,
    /// Standalone direct message thread
    DirectMessage,
    /// Custom channel type
    Custom(String),
}

impl std::fmt::Display for ChannelKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ChannelKind::General => write!(f, "general"),
            ChannelKind::Project => write!(f, "project"),
            ChannelKind::Team => write!(f, "team"),
            ChannelKind::Agent => write!(f, "agent"),
            ChannelKind::Digest => write!(f, "digest"),
            ChannelKind::DirectMessage => write!(f, "direct_message"),
            ChannelKind::Custom(s) => write!(f, "custom_{}", s),
        }
    }
}

/// A topic-based communication channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Channel {
    pub id: ChannelId,
    pub name: String,
    pub topic: String,
    pub kind: ChannelKind,
    pub is_private: bool,
    pub created_by: ActorId,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub last_activity: Timestamp,
    pub message_count: u64,
    pub member_ids: Vec<ActorId>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Channel {
    pub fn new(name: &str, topic: &str, kind: ChannelKind, created_by: ActorId) -> Self {
        let now = now();
        Self {
            id: NodeId::new("ch"),
            name: name.to_string(),
            topic: topic.to_string(),
            kind,
            is_private: false,
            created_by,
            created_at: now,
            updated_at: now,
            last_activity: now,
            message_count: 0,
            member_ids: Vec::new(),
            metadata: serde_json::Map::new(),
        }
    }

    /// Mark the channel as private.
    pub fn set_private(mut self) -> Self {
        self.is_private = true;
        self
    }

    pub fn add_member(&mut self, actor: ActorId) {
        if !self.member_ids.contains(&actor) {
            self.member_ids.push(actor);
        }
        self.updated_at = now();
    }

    pub fn remove_member(&mut self, actor: &ActorId) {
        self.member_ids.retain(|m| m != actor);
        self.updated_at = now();
    }

    pub fn has_member(&self, actor: &ActorId) -> bool {
        self.member_ids.contains(actor)
    }

    /// Called when a new message is posted — updates activity tracking.
    pub fn touch(&mut self) {
        self.last_activity = now();
        self.message_count += 1;
    }

    pub fn kind() -> &'static str {
        "channel"
    }
}
