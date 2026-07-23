//! Task — the unified work unit in Quang.
//!
//! This is the **core abstraction** that both `quang-hub-workplace::Task`
//! (human-collaboration task) and `quang-hub-repo::RepoTask` (agent-repo task)
//! extend. It defines the shared lifecycle, priority, tool execution trace,
//! and assignment model.
//!
//! ## Lifecycle
//!
//! ```text
//! Defined ──▶ Ready ──▶ Executing ──▶ InReview ──▶ Done
//!   │                     │    │                      │
//!   │                     │    └──▶ AwaitingInput ────┘
//!   │                     │
//!   └──▶ Cancelled        └──▶ Failed
//!                                  Blocked
//!                                  Archived
//! ```

use serde::{Deserialize, Serialize};

use crate::types::{ActorId, TaskId, Timestamp, now};

// ---------------------------------------------------------------------------
// Priority
// ---------------------------------------------------------------------------

/// Shared priority level — maps to both workplace `TaskPriority` enum
/// and repo's numeric 1–5 scale.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Priority {
    Critical = 1,
    High = 2,
    Medium = 3,
    Low = 4,
    Trivial = 5,
}

impl std::fmt::Display for Priority {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Priority::Critical => write!(f, "critical"),
            Priority::High => write!(f, "high"),
            Priority::Medium => write!(f, "medium"),
            Priority::Low => write!(f, "low"),
            Priority::Trivial => write!(f, "trivial"),
        }
    }
}

impl From<u8> for Priority {
    fn from(n: u8) -> Self {
        match n {
            1 => Priority::Critical,
            2 => Priority::High,
            3 => Priority::Medium,
            4 => Priority::Low,
            _ => Priority::Trivial,
        }
    }
}

// ---------------------------------------------------------------------------
// TaskPhase — unified lifecycle
// ---------------------------------------------------------------------------

/// Unified task phase. Superset of both:
/// - workplace `TaskStatus` (9 variants: Backlog, Ready, InProgress, InReview,
///   ChangesRequested, Done, Cancelled, Blocked, Archived)
/// - repo `QTaskExecutionState` (6 variants: Pending, Running, AwaitingInput,
///   Completed, Failed, Cancelled)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskPhase {
    /// Task exists but isn't ready for work (workplace.backlog | repo.pending)
    Defined,
    /// Task is ready to be picked up (workplace.ready)
    Ready,
    /// Work is in progress (workplace.in_progress | repo.running)
    Executing,
    /// Agent is waiting for human feedback (repo.awaiting_input)
    AwaitingInput,
    /// Work done, pending review (workplace.in_review + changes_requested)
    InReview,
    /// Task completed successfully (workplace.done | repo.completed)
    Done,
    /// Task failed (repo.failed)
    Failed,
    /// Task was cancelled (both)
    Cancelled,
    /// Blocked by a dependency (workplace.blocked)
    Blocked,
    /// Archived, read-only (workplace.archived)
    Archived,
}

impl TaskPhase {
    /// Has this phase reached a terminal state?
    pub fn is_terminal(self) -> bool {
        matches!(
            self,
            TaskPhase::Done | TaskPhase::Failed | TaskPhase::Cancelled | TaskPhase::Archived
        )
    }

    /// Is work actively happening?
    pub fn is_active(self) -> bool {
        matches!(self, TaskPhase::Executing | TaskPhase::InReview | TaskPhase::AwaitingInput)
    }

    /// Is the task waiting to start?
    pub fn is_pending(self) -> bool {
        matches!(self, TaskPhase::Defined | TaskPhase::Ready | TaskPhase::Blocked)
    }
}

impl std::fmt::Display for TaskPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TaskPhase::Defined => write!(f, "defined"),
            TaskPhase::Ready => write!(f, "ready"),
            TaskPhase::Executing => write!(f, "executing"),
            TaskPhase::AwaitingInput => write!(f, "awaiting_input"),
            TaskPhase::InReview => write!(f, "in_review"),
            TaskPhase::Done => write!(f, "done"),
            TaskPhase::Failed => write!(f, "failed"),
            TaskPhase::Cancelled => write!(f, "cancelled"),
            TaskPhase::Blocked => write!(f, "blocked"),
            TaskPhase::Archived => write!(f, "archived"),
        }
    }
}

// ---------------------------------------------------------------------------
// ToolCall — record of an agent tool invocation
// ---------------------------------------------------------------------------

/// A single tool invocation made during task execution.
/// Shared by both agent-executed workplace tasks and repo tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCall {
    /// Tool name (e.g. "read_file", "run_command")
    pub tool: String,
    /// Tool input (JSON arguments)
    pub input: serde_json::Value,
    /// Tool output / result
    pub output: Option<serde_json::Value>,
    /// Whether the call succeeded
    pub success: bool,
    /// When it happened
    pub timestamp: Timestamp,
}

// ---------------------------------------------------------------------------
// Core Task
// ---------------------------------------------------------------------------

/// The unified Task — a unit of assignable work.
///
/// This is the **base type** that domain crates extend:
/// - `quang-hub-workplace::Task` adds sizing, estimates, evidence, skill requirements
/// - `quang-hub-repo::RepoTask` adds repo scope, git refs, affected files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    // ── Identity ──
    pub id: TaskId,
    pub title: String,
    pub description: String,

    // ── Lifecycle ──
    pub phase: TaskPhase,
    pub priority: Priority,

    // ── Assignment ──
    pub assignee: Option<ActorId>,
    pub created_by: ActorId,

    // ── Classification ──
    pub tags: Vec<String>,

    // ── Tool execution trace ──
    pub tool_calls: Vec<ToolCall>,

    // ── Result ──
    pub result_summary: Option<String>,
    pub error_message: Option<String>,

    // ── Timestamps ──
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub started_at: Option<Timestamp>,
    pub completed_at: Option<Timestamp>,
}

impl Task {
    /// Create a new Task in the `Defined` phase.
    pub fn new(title: &str, description: &str, created_by: ActorId) -> Self {
        let now = now();
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: description.to_string(),
            phase: TaskPhase::Defined,
            priority: Priority::Medium,
            assignee: None,
            created_by,
            tags: Vec::new(),
            tool_calls: Vec::new(),
            result_summary: None,
            error_message: None,
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
        }
    }

    // ── Lifecycle transitions ──

    /// Transition to a new phase. Sets `started_at` on first execution,
    /// `completed_at` on terminal phases.
    pub fn transition_to(&mut self, phase: TaskPhase) {
        if phase == TaskPhase::Executing && self.started_at.is_none() {
            self.started_at = Some(now());
        }
        if phase.is_terminal() {
            self.completed_at = Some(now());
        }
        self.phase = phase;
        self.updated_at = now();
    }

    /// Assign to an actor. If the task is pending, begins execution.
    pub fn assign(&mut self, assignee: ActorId) {
        self.assignee = Some(assignee);
        if self.phase == TaskPhase::Defined || self.phase == TaskPhase::Ready {
            self.transition_to(TaskPhase::Executing);
        } else {
            self.updated_at = now();
        }
    }

    /// Unassign the current assignee.
    pub fn unassign(&mut self) {
        self.assignee = None;
        self.updated_at = now();
    }

    /// Record a tool call made during execution.
    pub fn record_tool_call(&mut self, tool: &str, input: serde_json::Value, success: bool) {
        self.tool_calls.push(ToolCall {
            tool: tool.to_string(),
            input,
            output: None,
            success,
            timestamp: now(),
        });
        self.updated_at = now();
    }

    /// Complete the task successfully.
    pub fn complete(&mut self, summary: &str) {
        self.result_summary = Some(summary.to_string());
        self.transition_to(TaskPhase::Done);
    }

    /// Mark the task as failed.
    pub fn fail(&mut self, error: &str) {
        self.error_message = Some(error.to_string());
        self.transition_to(TaskPhase::Failed);
    }

    /// Wait for human input before continuing.
    pub fn await_input(&mut self) {
        self.transition_to(TaskPhase::AwaitingInput);
    }

    /// Cancel the task.
    pub fn cancel(&mut self) {
        self.transition_to(TaskPhase::Cancelled);
    }

    /// Block the task on a dependency.
    pub fn block(&mut self) {
        self.transition_to(TaskPhase::Blocked);
    }

    /// Resume a blocked task.
    pub fn resume(&mut self) {
        if self.phase == TaskPhase::Blocked {
            self.transition_to(TaskPhase::Executing);
        } else {
            self.updated_at = now();
        }
    }

    /// Send the task for review.
    pub fn submit_for_review(&mut self) {
        self.transition_to(TaskPhase::InReview);
    }

    /// Mark the task as ready to be picked up.
    pub fn mark_ready(&mut self) {
        self.transition_to(TaskPhase::Ready);
    }

    /// Archive a completed/cancelled task.
    pub fn archive(&mut self) {
        self.transition_to(TaskPhase::Archived);
    }

    // ── Helpers ──

    pub fn add_tag(&mut self, tag: &str) {
        if !self.tags.contains(&tag.to_string()) {
            self.tags.push(tag.to_string());
        }
    }

    pub fn set_priority(&mut self, priority: Priority) {
        self.priority = priority;
        self.updated_at = now();
    }

    /// Estimated progress (0.0 → 1.0) based on phase.
    pub fn progress(&self) -> f64 {
        match self.phase {
            TaskPhase::Defined => 0.0,
            TaskPhase::Ready => 0.05,
            TaskPhase::Executing => 0.4,
            TaskPhase::AwaitingInput => 0.35,
            TaskPhase::InReview => 0.75,
            TaskPhase::Done => 1.0,
            TaskPhase::Failed => 1.0,
            TaskPhase::Cancelled => 1.0,
            TaskPhase::Blocked => 0.1,
            TaskPhase::Archived => 1.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Task errors
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, thiserror::Error)]
pub enum TaskError {
    #[error("Task not found: {0}")]
    NotFound(TaskId),
    #[error("Invalid phase transition: {from} -> {to}")]
    InvalidTransition { from: TaskPhase, to: TaskPhase },
    #[error("Task execution failed: {0}")]
    ExecutionFailed(String),
}
