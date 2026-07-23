//! CollabEvent — the typed event bus for all workplace mutations.
//!
//! Every state change in the workplace emits an event. Events drive:
//! - WebSocket pushes to connected clients
//! - AI agent triggers (e.g., "task assigned to agent")
//! - Summary generation hooks
//! - View refresh signals
//! - Audit logging

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId};

pub type EventId = String;

/// All collaborative events in the workplace.
/// This is the single source of truth for "what happened".
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum CollabEvent {
    // ── WorkSpace events ──
    WorkSpaceCreated {
        workspace_id: NodeId,
        actor: ActorId,
    },
    WorkSpaceUpdated {
        workspace_id: NodeId,
        actor: ActorId,
    },
    WorkSpaceDeleted {
        workspace_id: NodeId,
        actor: ActorId,
    },

    // ── Team events ──
    TeamCreated {
        team_id: NodeId,
        workspace_id: NodeId,
        actor: ActorId,
    },
    TeamMemberAdded {
        team_id: NodeId,
        member: ActorId,
        actor: ActorId,
    },
    TeamMemberRemoved {
        team_id: NodeId,
        member: ActorId,
        actor: ActorId,
    },

    // ── Project events ──
    ProjectCreated {
        project_id: NodeId,
        workspace_id: NodeId,
        actor: ActorId,
    },
    ProjectUpdated {
        project_id: NodeId,
        actor: ActorId,
    },
    ProjectStatusChanged {
        project_id: NodeId,
        old_status: String,
        new_status: String,
        actor: ActorId,
    },

    // ── Channel events ──
    ChannelCreated {
        channel_id: NodeId,
        parent_id: NodeId,
        actor: ActorId,
    },
    ChannelUpdated {
        channel_id: NodeId,
        actor: ActorId,
    },

    // ── Chat events ──
    MessageSent {
        message_id: NodeId,
        channel_id: NodeId,
        author: ActorId,
    },
    MessageEdited {
        message_id: NodeId,
        channel_id: NodeId,
        author: ActorId,
    },
    ReactionAdded {
        message_id: NodeId,
        emoji: String,
        actor: ActorId,
    },

    // ── Task events ──
    TaskCreated {
        task_id: NodeId,
        parent_id: NodeId, // Project or WorkSpace
        actor: ActorId,
    },
    TaskAssigned {
        task_id: NodeId,
        assignee: ActorId,
        actor: ActorId,
    },
    TaskUnassigned {
        task_id: NodeId,
        actor: ActorId,
    },
    TaskStatusChanged {
        task_id: NodeId,
        old_status: String,
        new_status: String,
        actor: ActorId,
    },
    TaskUpdated {
        task_id: NodeId,
        actor: ActorId,
    },
    TaskCompleted {
        task_id: NodeId,
        actor: ActorId,
        evidence_count: usize,
    },

    // ── Goal events ──
    GoalCreated {
        goal_id: NodeId,
        parent_id: NodeId,
        actor: ActorId,
    },
    GoalProgressUpdated {
        goal_id: NodeId,
        progress: f64,
        actor: ActorId,
    },
    GoalStatusChanged {
        goal_id: NodeId,
        old_status: String,
        new_status: String,
        actor: ActorId,
    },

    // ── Review events ──
    ReviewCreated {
        review_id: NodeId,
        target_id: NodeId,
        actor: ActorId,
    },
    ReviewApproved {
        review_id: NodeId,
        reviewer: ActorId,
    },
    ReviewChangesRequested {
        review_id: NodeId,
        reviewer: ActorId,
    },
    ReviewRejected {
        review_id: NodeId,
        reviewer: ActorId,
    },
    ReviewCommentAdded {
        review_id: NodeId,
        comment_id: String,
        actor: ActorId,
    },

    // ── Summary events ──
    SummaryGenerated {
        summary_id: NodeId,
        source_id: NodeId,
        actor: ActorId,
    },

    // ── Generic event for extensibility ──
    Custom {
        kind: String,
        payload: serde_json::Value,
        actor: ActorId,
    },
}

impl CollabEvent {
    /// A human-readable description of the event.
    pub fn description(&self) -> String {
        match self {
            CollabEvent::WorkSpaceCreated { workspace_id, .. } => {
                format!("WorkSpace {} created", workspace_id)
            }
            CollabEvent::WorkSpaceUpdated { workspace_id, .. } => {
                format!("WorkSpace {} updated", workspace_id)
            }
            CollabEvent::WorkSpaceDeleted { workspace_id, .. } => {
                format!("WorkSpace {} deleted", workspace_id)
            }
            CollabEvent::TeamCreated { team_id, .. } => format!("Team {} created", team_id),
            CollabEvent::TeamMemberAdded {
                team_id, member, ..
            } => {
                format!("Member {} added to team {}", member, team_id)
            }
            CollabEvent::TeamMemberRemoved {
                team_id, member, ..
            } => {
                format!("Member {} removed from team {}", member, team_id)
            }
            CollabEvent::ProjectCreated { project_id, .. } => {
                format!("Project {} created", project_id)
            }
            CollabEvent::ProjectUpdated { project_id, .. } => {
                format!("Project {} updated", project_id)
            }
            CollabEvent::ProjectStatusChanged {
                project_id,
                new_status,
                ..
            } => {
                format!("Project {} status changed to {}", project_id, new_status)
            }
            CollabEvent::ChannelCreated { channel_id, .. } => {
                format!("Channel {} created", channel_id)
            }
            CollabEvent::ChannelUpdated { channel_id, .. } => {
                format!("Channel {} updated", channel_id)
            }
            CollabEvent::MessageSent { message_id, .. } => {
                format!("Message {} sent", message_id)
            }
            CollabEvent::MessageEdited { message_id, .. } => {
                format!("Message {} edited", message_id)
            }
            CollabEvent::ReactionAdded {
                message_id, emoji, ..
            } => {
                format!("Reaction {} on message {}", emoji, message_id)
            }
            CollabEvent::TaskCreated { task_id, .. } => format!("Task {} created", task_id),
            CollabEvent::TaskAssigned {
                task_id, assignee, ..
            } => {
                format!("Task {} assigned to {}", task_id, assignee)
            }
            CollabEvent::TaskUnassigned { task_id, .. } => format!("Task {} unassigned", task_id),
            CollabEvent::TaskStatusChanged {
                task_id,
                new_status,
                ..
            } => {
                format!("Task {} status changed to {}", task_id, new_status)
            }
            CollabEvent::TaskUpdated { task_id, .. } => format!("Task {} updated", task_id),
            CollabEvent::TaskCompleted { task_id, .. } => format!("Task {} completed", task_id),
            CollabEvent::GoalCreated { goal_id, .. } => format!("Goal {} created", goal_id),
            CollabEvent::GoalProgressUpdated {
                goal_id, progress, ..
            } => {
                format!("Goal {} progress: {:.1}%", goal_id, progress * 100.0)
            }
            CollabEvent::GoalStatusChanged {
                goal_id,
                new_status,
                ..
            } => {
                format!("Goal {} status changed to {}", goal_id, new_status)
            }
            CollabEvent::ReviewCreated { review_id, .. } => {
                format!("Review {} created", review_id)
            }
            CollabEvent::ReviewApproved { review_id, .. } => {
                format!("Review {} approved", review_id)
            }
            CollabEvent::ReviewChangesRequested { review_id, .. } => {
                format!("Review {} changes requested", review_id)
            }
            CollabEvent::ReviewRejected { review_id, .. } => {
                format!("Review {} rejected", review_id)
            }
            CollabEvent::ReviewCommentAdded { review_id, .. } => {
                format!("Comment on review {}", review_id)
            }
            CollabEvent::SummaryGenerated { summary_id, .. } => {
                format!("Summary {} generated", summary_id)
            }
            CollabEvent::Custom { kind, .. } => format!("Custom event: {}", kind),
        }
    }

    /// The actor who triggered this event.
    pub fn actor(&self) -> &ActorId {
        match self {
            CollabEvent::WorkSpaceCreated { actor, .. }
            | CollabEvent::WorkSpaceUpdated { actor, .. }
            | CollabEvent::WorkSpaceDeleted { actor, .. }
            | CollabEvent::TeamCreated { actor, .. }
            | CollabEvent::TeamMemberAdded { actor, .. }
            | CollabEvent::TeamMemberRemoved { actor, .. }
            | CollabEvent::ProjectCreated { actor, .. }
            | CollabEvent::ProjectUpdated { actor, .. }
            | CollabEvent::ProjectStatusChanged { actor, .. }
            | CollabEvent::ChannelCreated { actor, .. }
            | CollabEvent::ChannelUpdated { actor, .. }
            | CollabEvent::MessageSent { author: actor, .. }
            | CollabEvent::MessageEdited { author: actor, .. }
            | CollabEvent::ReactionAdded { actor, .. }
            | CollabEvent::TaskCreated { actor, .. }
            | CollabEvent::TaskAssigned { actor, .. }
            | CollabEvent::TaskUnassigned { actor, .. }
            | CollabEvent::TaskStatusChanged { actor, .. }
            | CollabEvent::TaskUpdated { actor, .. }
            | CollabEvent::TaskCompleted { actor, .. }
            | CollabEvent::GoalCreated { actor, .. }
            | CollabEvent::GoalProgressUpdated { actor, .. }
            | CollabEvent::GoalStatusChanged { actor, .. }
            | CollabEvent::ReviewCreated { actor, .. }
            | CollabEvent::ReviewCommentAdded { actor, .. }
            | CollabEvent::SummaryGenerated { actor, .. } => actor,
            CollabEvent::ReviewApproved { reviewer, .. }
            | CollabEvent::ReviewChangesRequested { reviewer, .. }
            | CollabEvent::ReviewRejected { reviewer, .. } => reviewer,
            CollabEvent::Custom { actor, .. } => actor,
        }
    }
}

/// An event envelope with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventEnvelope {
    pub id: EventId,
    pub event: CollabEvent,
    pub timestamp: DateTime<Utc>,
    pub sequence: u64,
}

impl EventEnvelope {
    pub fn new(event: CollabEvent, sequence: u64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event,
            timestamp: Utc::now(),
            sequence,
        }
    }
}

/// A simple synchronous event bus for the workplace.
/// In production, this would bridge to tokio::sync::broadcast or a message queue.
#[derive(Debug, Clone)]
pub struct EventBus {
    sequence: u64,
    /// List of all events that have occurred (for audit/replay).
    pub history: Vec<EventEnvelope>,
}

impl Default for EventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            sequence: 0,
            history: Vec::new(),
        }
    }

    /// Emit an event. Returns the envelope.
    pub fn emit(&mut self, event: CollabEvent) -> EventEnvelope {
        self.sequence += 1;
        let envelope = EventEnvelope::new(event, self.sequence);
        self.history.push(envelope.clone());
        tracing::debug!("[EventBus] {}", envelope.event.description());
        envelope
    }

    /// Get all events of a certain kind (by matching variant name).
    pub fn events_of_kind(&self, kind: &str) -> Vec<&EventEnvelope> {
        self.history
            .iter()
            .filter(|e| {
                let desc = e.event.description();
                desc.to_lowercase().contains(kind)
            })
            .collect()
    }

    /// Get all events since a given sequence number.
    pub fn since(&self, sequence: u64) -> Vec<&EventEnvelope> {
        self.history
            .iter()
            .filter(|e| e.sequence > sequence)
            .collect()
    }

    /// Get the latest event, if any.
    pub fn latest(&self) -> Option<&EventEnvelope> {
        self.history.last()
    }

    pub fn event_count(&self) -> u64 {
        self.sequence
    }
}
