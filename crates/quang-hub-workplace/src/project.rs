//! Project — a time-bounded container for work with goals.
//!
//! Projects belong to a WorkSpace and optionally to a Team.
//! They contain Tasks, Goals, Channels, and Reviews.

use serde::{Deserialize, Serialize};

use crate::graph::{now, ActorId, NodeId, Timestamp};

pub type ProjectId = NodeId;

/// Lifecycle state of a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ProjectStatus {
    Planning,
    Active,
    Paused,
    Completed,
    Cancelled,
    Archived,
}

impl std::fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "planning"),
            ProjectStatus::Active => write!(f, "active"),
            ProjectStatus::Paused => write!(f, "paused"),
            ProjectStatus::Completed => write!(f, "completed"),
            ProjectStatus::Cancelled => write!(f, "cancelled"),
            ProjectStatus::Archived => write!(f, "archived"),
        }
    }
}

/// Priority level for a project.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Priority {
    Critical,
    High,
    Medium,
    Low,
    None,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Critical => write!(f, "critical"),
            Priority::High => write!(f, "high"),
            Priority::Medium => write!(f, "medium"),
            Priority::Low => write!(f, "low"),
            Priority::None => write!(f, "none"),
        }
    }
}

/// A time-bounded collaborative effort.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: ProjectId,
    pub name: String,
    pub description: String,
    pub status: ProjectStatus,
    pub priority: Priority,
    pub owner: ActorId,
    pub start_date: Option<Timestamp>,
    pub end_date: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Project {
    pub fn new(name: &str, description: &str, owner: ActorId) -> Self {
        let now = now();
        Self {
            id: NodeId::new("proj"),
            name: name.to_string(),
            description: description.to_string(),
            status: ProjectStatus::Planning,
            priority: Priority::Medium,
            owner,
            start_date: None,
            end_date: None,
            created_at: now,
            updated_at: now,
            metadata: serde_json::Map::new(),
        }
    }

    pub fn set_status(&mut self, status: ProjectStatus) {
        self.status = status;
        self.updated_at = now();
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.updated_at = now();
    }

    pub fn schedule(&mut self, start: Timestamp, end: Timestamp) {
        self.start_date = Some(start);
        self.end_date = Some(end);
        self.updated_at = now();
    }

    /// Progress percentage (0.0 - 1.0) based on completed tasks vs total.
    /// Actual calculation requires graph traversal — this is a stub for
    /// when the graph context is available.
    pub fn progress_estimate(&self, completed_tasks: usize, total_tasks: usize) -> f64 {
        if total_tasks == 0 {
            return 0.0;
        }
        completed_tasks as f64 / total_tasks as f64
    }

    pub fn kind() -> &'static str {
        "project"
    }
}
