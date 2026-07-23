//! WorkLog — shared activity and time tracking for both humans and agents.
//!
//! For humans: timesheets, billable hours, attendance.
//! For agents: execution traces, token usage, operation logging.
//! WorkLog entries are immutable once created (append-only ledger).

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

/// A single work log entry recording an activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkLogEntry {
    pub id: String,
    pub actor: ActorId,
    /// The type of activity (e.g., "task_work", "meeting", "review", "tool_call", "research")
    pub activity_type: String,
    pub description: String,
    pub project_id: Option<NodeId>,
    pub task_id: Option<NodeId>,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    /// Computed duration in minutes
    pub duration_minutes: Option<f64>,
    pub billable: bool,
    /// Token usage for agent activities
    pub tokens_used: Option<u64>,
    /// Cost in micro-dollars or compute units
    pub cost: Option<f64>,
    /// Arbitrary metadata (e.g., tool calls, API endpoints, notes)
    pub metadata: serde_json::Value,
}

impl WorkLogEntry {
    pub fn new(actor: ActorId, activity_type: &str, description: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            actor,
            activity_type: activity_type.to_string(),
            description: description.to_string(),
            project_id: None,
            task_id: None,
            start_time: now(),
            end_time: None,
            duration_minutes: None,
            billable: false,
            tokens_used: None,
            cost: None,
            metadata: serde_json::Value::Null,
        }
    }

    /// Complete the work log entry, computing duration.
    pub fn complete(&mut self) {
        let end = now();
        self.end_time = Some(end);
        let dur = end - self.start_time;
        self.duration_minutes = Some(dur.num_minutes().max(0) as f64);
    }

    /// Complete with explicit end time.
    pub fn complete_at(&mut self, end_time: Timestamp) {
        self.end_time = Some(end_time);
        let dur = end_time - self.start_time;
        self.duration_minutes = Some(dur.num_minutes().max(0) as f64);
    }

    pub fn set_billable(mut self) -> Self {
        self.billable = true;
        self
    }

    pub fn for_task(mut self, task_id: NodeId) -> Self {
        self.task_id = Some(task_id);
        self
    }

    pub fn for_project(mut self, project_id: NodeId) -> Self {
        self.project_id = Some(project_id);
        self
    }

    pub fn with_tokens(mut self, tokens: u64) -> Self {
        self.tokens_used = Some(tokens);
        self
    }

    pub fn with_cost(mut self, cost: f64) -> Self {
        self.cost = Some(cost);
        self
    }
}

/// A timesheet aggregating work log entries for a period.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Timesheet {
    pub id: NodeId,
    pub actor: ActorId,
    pub period_start: Timestamp,
    pub period_end: Timestamp,
    pub entries: Vec<WorkLogEntry>,
    pub total_minutes: f64,
    pub billable_minutes: f64,
    pub status: TimesheetStatus,
    pub submitted_at: Option<Timestamp>,
    pub approved_by: Option<ActorId>,
    pub approved_at: Option<Timestamp>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TimesheetStatus {
    Draft,
    Submitted,
    Approved,
    Rejected,
}

impl Timesheet {
    pub fn new(actor: ActorId, period_start: Timestamp, period_end: Timestamp) -> Self {
        Self {
            id: NodeId::new("ts"),
            actor,
            period_start,
            period_end,
            entries: Vec::new(),
            total_minutes: 0.0,
            billable_minutes: 0.0,
            status: TimesheetStatus::Draft,
            submitted_at: None,
            approved_by: None,
            approved_at: None,
        }
    }

    pub fn add_entry(&mut self, entry: WorkLogEntry) {
        if let Some(mins) = entry.duration_minutes {
            self.total_minutes += mins;
            if entry.billable {
                self.billable_minutes += mins;
            }
        }
        self.entries.push(entry);
    }

    pub fn submit(&mut self) {
        self.status = TimesheetStatus::Submitted;
        self.submitted_at = Some(now());
    }

    pub fn approve(&mut self, by: ActorId) {
        self.status = TimesheetStatus::Approved;
        self.approved_by = Some(by);
        self.approved_at = Some(now());
    }

    pub fn reject(&mut self) {
        self.status = TimesheetStatus::Rejected;
    }
}
