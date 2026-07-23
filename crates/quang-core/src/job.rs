//! Job — a policy-governed DAG of Tasks.
//!
//! A Job composes Tasks into a directed acyclic execution graph,
//! enforces a global Policy, and tracks aggregate Cost.

use serde::{Deserialize, Serialize};

use crate::cost::Cost;
use crate::policy::Policy;
use crate::task::{Task, TaskPhase};
use crate::types::{ActorId, JobId, NodeId, Timestamp, now};

// ---------------------------------------------------------------------------
// ExecutionRing
// ---------------------------------------------------------------------------

/// The execution scope for a job — where and by whom it runs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutionRing {
    /// Scoped to a single workspace
    Workspace(NodeId),
    /// Scoped to a repository
    Repository(NodeId),
    /// Scoped to an agent circle (group of agents)
    AgentCircle {
        circle_id: NodeId,
        members: Vec<ActorId>,
    },
    /// Any available executor
    AdHoc,
}

// ---------------------------------------------------------------------------
// DepKind
// ---------------------------------------------------------------------------

/// How a task dependency edge behaves.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DepKind {
    /// `to` cannot start until `from` succeeds
    Blocks,
    /// `to` starts automatically when `from` succeeds
    Triggers,
    /// `to` runs independently but receives `from`'s output
    Informs,
}

// ---------------------------------------------------------------------------
// JobEdge
// ---------------------------------------------------------------------------

/// A dependency edge between two tasks in a job.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JobEdge {
    /// Source task ID (must complete first)
    pub from: String,
    /// Target task ID (depends on `from`)
    pub to: String,
    /// How this dependency behaves
    pub kind: DepKind,
}

// ---------------------------------------------------------------------------
// JobPhase
// ---------------------------------------------------------------------------

/// Job-level lifecycle.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JobPhase {
    /// Job template, not yet instantiated
    Draft,
    /// Instantiated, waiting for trigger
    Pending,
    /// Tasks are executing
    Running,
    /// All tasks done, verifying evidence
    Verifying,
    /// Job completed successfully
    Completed,
    /// Job failed (one or more tasks failed)
    Failed,
    /// Job was cancelled
    Cancelled,
}

impl JobPhase {
    pub fn is_terminal(self) -> bool {
        matches!(self, JobPhase::Completed | JobPhase::Failed | JobPhase::Cancelled)
    }
}

impl std::fmt::Display for JobPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JobPhase::Draft => write!(f, "draft"),
            JobPhase::Pending => write!(f, "pending"),
            JobPhase::Running => write!(f, "running"),
            JobPhase::Verifying => write!(f, "verifying"),
            JobPhase::Completed => write!(f, "completed"),
            JobPhase::Failed => write!(f, "failed"),
            JobPhase::Cancelled => write!(f, "cancelled"),
        }
    }
}

// ---------------------------------------------------------------------------
// Job
// ---------------------------------------------------------------------------

/// A Job is a directed acyclic graph of Tasks that execute toward a goal.
///
/// Jobs compose individual Tasks, govern execution order, enforce a global
/// Policy, and track aggregate Cost across all dimensions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Job {
    pub id: JobId,
    pub title: String,
    /// The goal this job aims to achieve
    pub goal: String,

    // ── Task Graph ──
    /// Tasks in this job
    pub tasks: Vec<Task>,
    /// Dependency edges between tasks
    pub edges: Vec<JobEdge>,

    // ── Execution ──
    /// Where this job runs
    pub execution_ring: ExecutionRing,
    /// Current phase
    pub phase: JobPhase,

    // ── Policy ──
    /// Governance policy for the entire job
    pub policy: Policy,

    // ── Cost ──
    /// Budget cap
    pub budget: Cost,
    /// Accumulated spend across all tasks
    pub spent: Cost,

    // ── Provenance ──
    pub owner: ActorId,
    pub created_at: Timestamp,
    pub completed_at: Option<Timestamp>,
}

impl Job {
    pub fn new(title: &str, goal: &str, owner: ActorId, policy: Policy) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            goal: goal.to_string(),
            tasks: Vec::new(),
            edges: Vec::new(),
            execution_ring: ExecutionRing::AdHoc,
            phase: JobPhase::Draft,
            policy,
            budget: Cost::default(),
            spent: Cost::default(),
            owner,
            created_at: now(),
            completed_at: None,
        }
    }

    /// Add a task to the job.
    pub fn add_task(&mut self, task: Task) -> &mut Self {
        self.tasks.push(task);
        self
    }

    /// Add a dependency edge between two tasks.
    pub fn depends_on(&mut self, task_id: &str, depends_on_id: &str, kind: DepKind) -> &mut Self {
        self.edges.push(JobEdge {
            from: depends_on_id.to_string(),
            to: task_id.to_string(),
            kind,
        });
        self
    }

    /// Set the execution ring.
    pub fn in_ring(mut self, ring: ExecutionRing) -> Self {
        self.execution_ring = ring;
        self
    }

    /// Set a budget cap.
    pub fn with_budget(mut self, budget: Cost) -> Self {
        self.budget = budget;
        self
    }

    /// Start the job.
    pub fn start(&mut self) {
        self.phase = JobPhase::Running;
        for task in &mut self.tasks {
            if task.phase == TaskPhase::Defined {
                task.transition_to(TaskPhase::Ready);
            }
        }
    }

    /// Accumulate cost from completed tasks and check against budget.
    pub fn accumulate_cost(&mut self) {
        self.spent = Cost::default();
        for _task in &self.tasks {
            // Cost is tracked per-task — in practice this would be accumulated
            // from tool calls, inference metrics, human hours logged, etc.
        }
    }

    /// Check if the job is over budget.
    pub fn is_over_budget(&self) -> bool {
        self.spent.total() > self.budget.total() && !self.budget.is_zero()
    }

    /// Mark job as verifying (all tasks done, now check evidence).
    pub fn verify(&mut self) {
        self.phase = JobPhase::Verifying;
    }

    /// Complete the job.
    pub fn complete(&mut self) {
        self.phase = JobPhase::Completed;
        self.completed_at = Some(now());
    }

    /// Fail the job.
    pub fn fail(&mut self) {
        self.phase = JobPhase::Failed;
        self.completed_at = Some(now());
    }

    /// Cancel the job.
    pub fn cancel(&mut self) {
        self.phase = JobPhase::Cancelled;
        self.completed_at = Some(now());
    }

    /// Count tasks by phase.
    pub fn count_by_phase(&self, phase: TaskPhase) -> usize {
        self.tasks.iter().filter(|t| t.phase == phase).count()
    }

    /// Are all tasks in a terminal phase?
    pub fn all_tasks_terminal(&self) -> bool {
        self.tasks.iter().all(|t| t.phase.is_terminal())
    }

    /// Number of tasks that succeeded.
    pub fn tasks_done(&self) -> usize {
        self.count_by_phase(TaskPhase::Done)
    }

    /// Number of tasks that failed.
    pub fn tasks_failed(&self) -> usize {
        self.count_by_phase(TaskPhase::Failed)
    }
}
