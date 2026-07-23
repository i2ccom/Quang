//! Chat — message threading and real-time communication.
//!
//! Chat messages are HyperGraph nodes connected to Channels via BelongsTo edges.
//! They support threads, reactions, and rich content (markdown, code blocks, file refs).

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

pub type ChatMessageId = NodeId;
pub type ThreadId = NodeId;

/// The content of a chat message — supports plain text, markdown, and structured blocks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageContent {
    Text(String),
    Markdown(String),
    /// Code block with language hint
    Code {
        language: String,
        code: String,
    },
    /// Reference to a graph node (task, project, goal, etc.)
    NodeReference(NodeId),
    /// File attachment reference
    Attachment {
        filename: String,
        url: String,
        mime_type: String,
        size_bytes: u64,
    },
    /// Structured data (JSON blob)
    Structured(serde_json::Value),
}

/// A reaction to a message (emoji or custom).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reaction {
    pub emoji: String,
    pub actors: Vec<ActorId>,
}

impl Reaction {
    pub fn new(emoji: &str, actor: ActorId) -> Self {
        Self {
            emoji: emoji.to_string(),
            actors: vec![actor],
        }
    }
}

/// A single message in a chat channel.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub id: ChatMessageId,
    pub channel_id: NodeId,
    pub author: ActorId,
    pub content: MessageContent,
    pub thread_id: Option<ThreadId>,
    pub reply_to: Option<ChatMessageId>,
    pub reactions: Vec<Reaction>,
    pub created_at: Timestamp,
    pub edited_at: Option<Timestamp>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl ChatMessage {
    pub fn new(channel_id: NodeId, author: ActorId, content: MessageContent) -> Self {
        Self {
            id: NodeId::new("msg"),
            channel_id,
            author,
            content,
            thread_id: None,
            reply_to: None,
            reactions: Vec::new(),
            created_at: now(),
            edited_at: None,
            metadata: serde_json::Map::new(),
        }
    }

    /// Start or continue a thread by replying to a parent message.
    pub fn reply_to(mut self, parent_id: ChatMessageId, thread_id: ThreadId) -> Self {
        self.reply_to = Some(parent_id);
        self.thread_id = Some(thread_id);
        self
    }

    pub fn add_reaction(&mut self, emoji: &str, actor: ActorId) {
        if let Some(existing) = self.reactions.iter_mut().find(|r| r.emoji == emoji) {
            if !existing.actors.contains(&actor) {
                existing.actors.push(actor);
            }
        } else {
            self.reactions.push(Reaction::new(emoji, actor));
        }
    }

    pub fn edit(&mut self, content: MessageContent) {
        self.content = content;
        self.edited_at = Some(now());
    }

    pub fn kind() -> &'static str {
        "chat_message"
    }
}

/// A thread is a collection of replies grouped under a root message.
/// It is represented as a lightweight struct for easy API responses.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Thread {
    pub id: ThreadId,
    pub root_message_id: ChatMessageId,
    pub channel_id: NodeId,
    pub reply_count: u32,
    pub last_activity: Timestamp,
    pub participant_ids: Vec<ActorId>,
}
