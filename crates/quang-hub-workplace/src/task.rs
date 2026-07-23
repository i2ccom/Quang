//! Task — a unit of assignable work with a full state machine.
//!
//! Tasks are the core work unit in the workplace. They can be assigned to
//! humans or agents, have dependencies, and flow through a defined lifecycle.
//! Tasks are compatible with the QuangTask system from QuangHubRepo.

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

pub type TaskId = NodeId;

/// The standard task lifecycle state machine.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is defined but not yet ready for work
    Backlog,
    /// Task is ready to be picked up
    Ready,
    /// Task is currently being worked on
    InProgress,
    /// Work is complete, pending review
    InReview,
    /// Task has been reviewed and needs rework
    ChangesRequested,
    /// Task is complete
    Done,
    /// Task was cancelled
    Cancelled,
    /// Task is blocked by another task or external dependency
    Blocked,
    /// Task is archived
    Archived,
}

impl TaskStatus {
    /// Returns all valid transitions from this state.
    pub fn valid_transitions(&self) -> Vec<TaskStatus> {
        match self {
            TaskStatus::Backlog => vec![TaskStatus::Ready, TaskStatus::Cancelled],
            TaskStatus::Ready => vec![TaskStatus::InProgress, TaskStatus::Backlog, TaskStatus::Cancelled],
            TaskStatus::InProgress => vec![TaskStatus::InReview, TaskStatus::Blocked, TaskStatus::Backlog],
            TaskStatus::InReview => vec![TaskStatus::Done, TaskStatus::ChangesRequested],
            TaskStatus::ChangesRequested => vec![TaskStatus::InProgress, TaskStatus::InReview],
            TaskStatus::Done => vec![TaskStatus::Archived],
            TaskStatus::Cancelled => vec![TaskStatus::Backlog, TaskStatus::Archived],
            TaskStatus::Blocked => vec![TaskStatus::InProgress, TaskStatus::Backlog, TaskStatus::Cancelled],
            TaskStatus::Archived => vec![],
        }
    }

    /// Can this task be transitioned to the given status?
    pub fn can_transition_to(&self, target: &TaskStatus) -> bool {
        self.valid_transitions().contains(target)
    }

    /// Is this task considered "active" (being worked on or awaiting review)?
    pub fn is_active(&self) -> bool {
        matches!(self, TaskStatus::InProgress | TaskStatus::InReview | TaskStatus::ChangesRequested)
    }

    /// Is this task considered "closed" (no further work expected)?
    pub fn is_closed(&self) -> bool {
        matches!(self, TaskStatus::Done | TaskStatus::Cancelled | TaskStatus::Archived)
    }

    /// Is this task available to be picked up?
    pub fn is_available(&self) -> bool {
        matches!(self, TaskStatus::Backlog | TaskStatus::Ready)
    }
}

impl std::fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskStatus::Backlog => write!(f, "backlog"),
            TaskStatus::Ready => write!(f, "ready"),
            TaskStatus::InProgress => write!(f, "in_progress"),
            TaskStatus::InReview => write!(f, "in_review"),
            TaskStatus::ChangesRequested => write!(f, "changes_requested"),
            TaskStatus::Done => write!(f, "done"),
            TaskStatus::Cancelled => write!(f, "cancelled"),
            TaskStatus::Blocked => write!(f, "blocked"),
            TaskStatus::Archived => write!(f, "archived"),
        }
    }
}

/// Priority level for a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPriority {
    Critical,
    High,
    Medium,
    Low,
}

impl std::fmt::Display for TaskPriority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPriority::Critical => write!(f, "critical"),
            TaskPriority::High => write!(f, "high"),
            TaskPriority::Medium => write!(f, "medium"),
            TaskPriority::Low => write!(f, "low"),
        }
    }
}

/// The type/size of a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskSize {
    Tiny,
    Small,
    Medium,
    Large,
    Epic,
}

impl std::fmt::Display for TaskSize {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskSize::Tiny => write!(f, "tiny"),
            TaskSize::Small => write!(f, "small"),
            TaskSize::Medium => write!(f, "medium"),
            TaskSize::Large => write!(f, "large"),
            TaskSize::Epic => write!(f, "epic"),
        }
    }
}

/// Evidence that a task was completed (test results, docs, screenshots, etc.)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskEvidence {
    pub kind: String,
    pub description: String,
    pub url: Option<String>,
    pub created_at: Timestamp,
}

impl TaskEvidence {
    pub fn new(kind: &str, description: &str) -> Self {
        Self {
            kind: kind.to_string(),
            description: description.to_string(),
            url: None,
            created_at: now(),
        }
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = Some(url.to_string());
        self
    }
}

/// The core assignable work unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub id: TaskId,
    pub title: String,
    pub description: String,
    pub status: TaskStatus,
    pub priority: TaskPriority,
    pub size: TaskSize,
    pub assignee: Option<ActorId>,
    pub created_by: ActorId,
    pub tags: Vec<String>,
    pub estimated_hours: Option<f64>,
    pub spent_hours: Option<f64>,
    pub start_date: Option<Timestamp>,
    pub due_date: Option<Timestamp>,
    pub completed_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub evidence: Vec<TaskEvidence>,
    /// Required skills for agent assignment
    pub required_skills: Vec<String>,
    /// Budget for agent-takeable tasks (in micro-dollars or tokens)
    pub budget: Option<u64>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Task {
    pub fn new(title: &str, description: &str, created_by: ActorId) -> Self {
        let now = now();
        Self {
            id: NodeId::new("task"),
            title: title.to_string(),
            description: description.to_string(),
            status: TaskStatus::Backlog,
            priority: TaskPriority::Medium,
            size: TaskSize::Medium,
            assignee: None,
            created_by,
            tags: Vec::new(),
            estimated_hours: None,
            spent_hours: None,
            start_date: None,
            due_date: None,
            completed_at: None,
            created_at: now,
            updated_at: now,
            evidence: Vec::new(),
            required_skills: Vec::new(),
            budget: None,
            metadata: serde_json::Map::new(),
        }
    }

    /// Transition the task to a new status. Returns Err if transition is invalid.
    pub fn transition_to(&mut self, target: TaskStatus) -> Result<(), TaskError> {
        if !self.status.can_transition_to(&target) {
            return Err(TaskError::InvalidTransition {
                from: self.status.clone(),
                to: target,
            });
        }

        if target == TaskStatus::Done {
            self.completed_at = Some(now());
        }

        self.status = target;
        self.updated_at = now();
        Ok(())
    }

    pub fn assign(&mut self, assignee: ActorId) {
        self.assignee = Some(assignee);
        if self.status == TaskStatus::Backlog || self.status == TaskStatus::Ready {
            let _ = self.transition_to(TaskStatus::InProgress);
        } else {
            self.updated_at = now();
        }
    }

    pub fn unassign(&mut self) {
        self.assignee = None;
        self.updated_at = now();
    }

    pub fn add_evidence(&mut self, evidence: TaskEvidence) {
        self.evidence.push(evidence);
        self.updated_at = now();
    }

    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
        }
    }

    pub fn add_required_skill(&mut self, skill: &str) {
        if !self.required_skills.contains(&skill.to_string()) {
            self.required_skills.push(skill.to_string());
        }
    }

    pub fn log_hours(&mut self, hours: f64) {
        self.spent_hours = Some(self.spent_hours.unwrap_or(0.0) + hours);
        self.updated_at = now();
    }

    pub fn estimated_progress(&self) -> f64 {
        match self.status {
            TaskStatus::Backlog => 0.0,
            TaskStatus::Ready => 0.05,
            TaskStatus::InProgress => 0.4,
            TaskStatus::InReview => 0.7,
            TaskStatus::ChangesRequested => 0.5,
            TaskStatus::Done => 1.0,
            TaskStatus::Cancelled | TaskStatus::Archived => 1.0,
            TaskStatus::Blocked => 0.1,
        }
    }

    pub fn kind() -> &'static str {
        "task"
    }
}

/// Errors related to task operations.
#[derive(Debug, Clone, thiserror::Error)]
pub enum TaskError {
    #[error("Invalid transition: {from} -> {to}")]
    InvalidTransition {
        from: TaskStatus,
        to: TaskStatus,
    },
    #[error("Task not found: {0}")]
    NotFound(String),
}
