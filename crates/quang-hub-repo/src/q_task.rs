//! QTask — an agent task tied to a repository.
//!
//! QTask extends the workplace `Task` model with repo-specific fields:
//! file paths, branches, commit references, and agent tool execution state.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::linked_repo::RepoId;

/// Unique identifier for a QTask.
pub type QTaskId = String;

/// The scope of a QTask within the repository.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QTaskScope {
    /// Task applies to the entire repo
    Repository,
    /// Task targets a specific directory
    Directory(String),
    /// Task targets a specific file
    File(String),
    /// Task targets a specific branch
    Branch(String),
    /// Task is tied to a pull request
    PullRequest(u64),
}

impl std::fmt::Display for QTaskScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QTaskScope::Repository => write!(f, "repo"),
            QTaskScope::Directory(p) => write!(f, "dir:{}", p),
            QTaskScope::File(p) => write!(f, "file:{}", p),
            QTaskScope::Branch(b) => write!(f, "branch:{}", b),
            QTaskScope::PullRequest(n) => write!(f, "pr:{}", n),
        }
    }
}

/// The type of agent action for a QTask.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QTaskAction {
    /// Refactor a file or directory
    Refactor,
    /// Add a new feature
    Feature,
    /// Fix a bug
    BugFix,
    /// Write tests
    Test,
    /// Document code
    Documentation,
    /// Review a pull request
    Review,
    /// Analyze code (read-only)
    Analyze,
    /// Deploy to an environment
    Deploy,
    /// Generate a summary
    Summarize,
    /// Custom action
    Custom(String),
}

impl std::fmt::Display for QTaskAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QTaskAction::Refactor => write!(f, "refactor"),
            QTaskAction::Feature => write!(f, "feature"),
            QTaskAction::BugFix => write!(f, "bugfix"),
            QTaskAction::Test => write!(f, "test"),
            QTaskAction::Documentation => write!(f, "docs"),
            QTaskAction::Review => write!(f, "review"),
            QTaskAction::Analyze => write!(f, "analyze"),
            QTaskAction::Deploy => write!(f, "deploy"),
            QTaskAction::Summarize => write!(f, "summarize"),
            QTaskAction::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

/// Execution state of a QTask.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QTaskExecutionState {
    /// Task is created but not started
    Pending,
    /// Agent is working on the task
    Running,
    /// Agent is waiting for human feedback
    AwaitingInput,
    /// Task completed successfully
    Completed,
    /// Task failed
    Failed,
    /// Task was cancelled
    Cancelled,
}

impl std::fmt::Display for QTaskExecutionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            QTaskExecutionState::Pending => write!(f, "pending"),
            QTaskExecutionState::Running => write!(f, "running"),
            QTaskExecutionState::AwaitingInput => write!(f, "awaiting_input"),
            QTaskExecutionState::Completed => write!(f, "completed"),
            QTaskExecutionState::Failed => write!(f, "failed"),
            QTaskExecutionState::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// A tool call made by the agent during task execution.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QTaskToolCall {
    /// Tool name (e.g. "read_file", "edit_file", "run_command")
    pub tool: String,
    /// Tool input arguments
    pub input: serde_json::Value,
    /// Tool output / result
    pub output: Option<serde_json::Value>,
    /// Whether the tool call succeeded
    pub success: bool,
    /// Timestamp
    pub timestamp: DateTime<Utc>,
}

/// A QTask — an agent task tied to a specific repository.
///
/// Extends the workplace `Task` model with repo-targeted fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QTask {
    /// Unique QTask ID
    pub id: QTaskId,
    /// Reference to the parent workplace Task ID (if any)
    pub parent_task_id: Option<String>,
    /// The repo this task operates on
    pub repo_id: RepoId,
    /// Human-readable title
    pub title: String,
    /// Detailed description / instructions for the agent
    pub description: String,
    /// The scope of the task within the repo
    pub scope: QTaskScope,
    /// The type of action
    pub action: QTaskAction,
    /// Current execution state
    pub state: QTaskExecutionState,
    /// The target branch for changes (if applicable)
    pub target_branch: Option<String>,
    /// The base branch to branch from (if applicable)
    pub base_branch: Option<String>,
    /// PR number if this task is linked to a PR
    pub pull_request_number: Option<u64>,
    /// Commit SHA where the task result is
    pub result_commit_sha: Option<String>,
    /// Agent ID assigned to this task
    pub assigned_agent: Option<String>,
    /// Tool calls made by the agent
    pub tool_calls: Vec<QTaskToolCall>,
    /// Agent output / result summary
    pub result_summary: Option<String>,
    /// Error message if failed
    pub error_message: Option<String>,
    /// Priority (1-5, 1 = highest)
    pub priority: u8,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// File paths this task touches
    pub affected_files: Vec<String>,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

impl QTask {
    /// Create a new QTask.
    pub fn new(
        repo_id: RepoId,
        title: &str,
        description: &str,
        scope: QTaskScope,
        action: QTaskAction,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            parent_task_id: None,
            repo_id,
            title: title.to_string(),
            description: description.to_string(),
            scope,
            action,
            state: QTaskExecutionState::Pending,
            target_branch: None,
            base_branch: None,
            pull_request_number: None,
            result_commit_sha: None,
            assigned_agent: None,
            tool_calls: Vec::new(),
            result_summary: None,
            error_message: None,
            priority: 3,
            tags: Vec::new(),
            affected_files: Vec::new(),
            created_at: now,
            updated_at: now,
            started_at: None,
            completed_at: None,
        }
    }

    /// Start executing this task (assign to an agent).
    pub fn start(&mut self, agent_id: &str) {
        self.state = QTaskExecutionState::Running;
        self.assigned_agent = Some(agent_id.to_string());
        self.started_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Record a tool call made by the agent.
    pub fn add_tool_call(&mut self, tool: &str, input: serde_json::Value, success: bool) {
        self.tool_calls.push(QTaskToolCall {
            tool: tool.to_string(),
            input,
            output: None,
            success,
            timestamp: Utc::now(),
        });
        self.updated_at = Utc::now();
    }

    /// Mark task as completed with a summary.
    pub fn complete(&mut self, summary: &str, commit_sha: Option<&str>) {
        self.state = QTaskExecutionState::Completed;
        self.result_summary = Some(summary.to_string());
        self.result_commit_sha = commit_sha.map(|s| s.to_string());
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Mark task as failed with an error.
    pub fn fail(&mut self, error: &str) {
        self.state = QTaskExecutionState::Failed;
        self.error_message = Some(error.to_string());
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Set the task as awaiting human input.
    pub fn await_input(&mut self) {
        self.state = QTaskExecutionState::AwaitingInput;
        self.updated_at = Utc::now();
    }

    /// Cancel the task.
    pub fn cancel(&mut self) {
        self.state = QTaskExecutionState::Cancelled;
        self.completed_at = Some(Utc::now());
        self.updated_at = Utc::now();
    }

    /// Add an affected file path.
    pub fn add_affected_file(&mut self, path: &str) {
        if !self.affected_files.contains(&path.to_string()) {
            self.affected_files.push(path.to_string());
        }
    }
}

/// A summary list view of a QTask.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QTaskSlim {
    pub id: QTaskId,
    pub repo_id: RepoId,
    pub title: String,
    pub action: QTaskAction,
    pub state: QTaskExecutionState,
    pub priority: u8,
    pub assigned_agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl From<QTask> for QTaskSlim {
    fn from(t: QTask) -> Self {
        Self {
            id: t.id,
            repo_id: t.repo_id,
            title: t.title,
            action: t.action,
            state: t.state,
            priority: t.priority,
            assigned_agent: t.assigned_agent,
            created_at: t.created_at,
        }
    }
}
