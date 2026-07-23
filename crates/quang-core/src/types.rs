//! Shared type aliases and foundational primitives for quang-core.
//!
//! These are Quang's own types — not re-exports from lower crates.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ---------------------------------------------------------------------------
// IDs
// ---------------------------------------------------------------------------

/// Unique identifier for a Task.
pub type TaskId = String;

/// Unique identifier for a Job.
pub type JobId = String;

/// Unique identifier for a Workflow.
pub type WorkflowId = String;

/// Unique identifier for a Policy.
pub type PolicyId = String;

/// Unique identifier for any graph node (workspace, repo, org, etc.).
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId(pub String);

impl NodeId {
    pub fn new(kind: &str) -> Self {
        Self(format!(
            "{}_{}",
            kind,
            Uuid::new_v4().to_string().replace('-', "")
        ))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for NodeId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ---------------------------------------------------------------------------
// ActorId — unified human / agent identity
// ---------------------------------------------------------------------------

/// Who performed an action. Shared across all Quang crates.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ActorId {
    Human(String),
    Agent(String),
}

impl ActorId {
    pub fn human(id: &str) -> Self {
        Self::Human(id.to_string())
    }

    pub fn agent(id: &str) -> Self {
        Self::Agent(id.to_string())
    }

    pub fn as_str(&self) -> &str {
        match self {
            ActorId::Human(s) | ActorId::Agent(s) => s.as_str(),
        }
    }

    pub fn is_human(&self) -> bool {
        matches!(self, ActorId::Human(_))
    }

    pub fn is_agent(&self) -> bool {
        matches!(self, ActorId::Agent(_))
    }
}

impl std::fmt::Display for ActorId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActorId::Human(id) => write!(f, "human:{}", id),
            ActorId::Agent(id) => write!(f, "agent:{}", id),
        }
    }
}

// ---------------------------------------------------------------------------
// Timestamp
// ---------------------------------------------------------------------------

pub type Timestamp = DateTime<Utc>;

pub fn now() -> Timestamp {
    Utc::now()
}

// ---------------------------------------------------------------------------
// Executor — who / what executes a task
// ---------------------------------------------------------------------------

/// What kind of executor runs a task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ExecutorKind {
    /// A human user
    Human,
    /// An AI agent
    Agent,
    /// An automated pipeline (CI/CD)
    Pipeline,
    /// Human + agent collaboration
    Hybrid { human: ActorId, agent: ActorId },
}

// ---------------------------------------------------------------------------
// Retry
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryPolicy {
    pub max_retries: u32,
    pub backoff_seconds: u64,
    pub backoff_multiplier: f64,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            backoff_seconds: 5,
            backoff_multiplier: 2.0,
        }
    }
}

// ---------------------------------------------------------------------------
// Parameter types (for Workflow templates)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParamType {
    String,
    Number,
    Boolean,
    Json,
    RepoRef,
    BranchRef,
    FilePath,
}
