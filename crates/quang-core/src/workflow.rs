//! Workflow — a reusable, parameterized Job template.
//!
//! A Workflow defines the shape of work, not specific instances.
//! Think: GitHub Actions workflow YAML, but generalized across human
//! and agent work. Workflows are instantiated into Jobs.

use serde::{Deserialize, Serialize};

use crate::cost::Cost;
use crate::job::JobEdge;
use crate::policy::Policy;
use crate::types::{ActorId, ExecutorKind, ParamType, RetryPolicy, Timestamp, WorkflowId, now};

// ---------------------------------------------------------------------------
// WorkflowParam
// ---------------------------------------------------------------------------

/// A parameter that a workflow accepts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowParam {
    pub name: String,
    pub description: String,
    pub param_type: ParamType,
    pub required: bool,
    pub default: Option<serde_json::Value>,
}

// ---------------------------------------------------------------------------
// TaskTemplate
// ---------------------------------------------------------------------------

/// A parameterized task within a workflow.
/// Variables like `{repo_url}` or `{branch}` are substituted at instantiation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplate {
    /// Title template with `{param}` placeholders
    pub title_template: String,
    /// Description template with `{param}` placeholders
    pub description_template: String,

    /// Who executes this task
    pub executor: ExecutorKind,

    /// Estimated cost (for budgeting)
    pub estimated_cost: Cost,

    /// Timeout in seconds
    pub timeout_seconds: Option<u64>,

    /// Retry policy on failure
    pub retry_policy: Option<RetryPolicy>,

    /// Tags applied to instantiated tasks
    pub tags: Vec<String>,
}

// ---------------------------------------------------------------------------
// WorkflowTrigger
// ---------------------------------------------------------------------------

/// What causes a workflow to be instantiated into a Job.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum WorkflowTrigger {
    /// A human or agent explicitly starts it
    Manual,
    /// Triggered by a specific event kind
    OnEvent { event_kind: String },
    /// Cron schedule (e.g. "0 9 * * 1-5")
    OnSchedule { cron: String },
    /// Triggered by an incoming webhook
    OnWebhook { source: String, event: String },
    /// Triggered when a specific task completes
    OnTaskComplete { task_id: String },
    /// Triggered by a push to a repository branch
    OnPush { repo_id: String, branch_pattern: String },
    /// Triggered by a pull request event
    OnPullRequest { repo_id: String, action: String },
}

// ---------------------------------------------------------------------------
// Workflow
// ---------------------------------------------------------------------------

/// A reusable, parameterized Job template.
///
/// Workflows are the "recipe" — Jobs are the "meal." A single workflow
/// can be instantiated many times with different parameters.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub id: WorkflowId,
    pub name: String,
    pub description: String,

    // ── Parameters ──
    /// Parameters the workflow accepts
    pub parameters: Vec<WorkflowParam>,

    // ── Template Tasks ──
    /// Task templates — instantiated with parameters at runtime
    pub task_templates: Vec<TaskTemplate>,

    // ── Template Edges ──
    /// Dependency edges between tasks (referenced by template index)
    pub edges: Vec<JobEdge>,

    // ── Policy ──
    /// Governance policy for every job instantiated from this workflow
    pub policy: Policy,

    // ── Triggers ──
    /// What events instantiate this workflow
    pub triggers: Vec<WorkflowTrigger>,

    // ── Cost estimate ──
    /// Estimated total cost per run
    pub estimated_cost: Cost,

    // ── Metadata ──
    pub created_by: ActorId,
    pub created_at: Timestamp,
    pub version: u32,
}

impl Workflow {
    pub fn new(name: &str, created_by: ActorId, policy: Policy) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: String::new(),
            parameters: Vec::new(),
            task_templates: Vec::new(),
            edges: Vec::new(),
            policy,
            triggers: Vec::new(),
            estimated_cost: Cost::default(),
            created_by,
            created_at: now(),
            version: 1,
        }
    }

    /// Add a parameter.
    pub fn param(mut self, name: &str, param_type: ParamType, required: bool) -> Self {
        self.parameters.push(WorkflowParam {
            name: name.to_string(),
            description: String::new(),
            param_type,
            required,
            default: None,
        });
        self
    }

    /// Add a parameter with a default value.
    pub fn param_with_default(
        mut self,
        name: &str,
        param_type: ParamType,
        default: serde_json::Value,
    ) -> Self {
        self.parameters.push(WorkflowParam {
            name: name.to_string(),
            description: String::new(),
            param_type,
            required: false,
            default: Some(default),
        });
        self
    }

    /// Add a task template.
    pub fn task(mut self, title: &str, description: &str, executor: ExecutorKind) -> Self {
        self.task_templates.push(TaskTemplate {
            title_template: title.to_string(),
            description_template: description.to_string(),
            executor,
            estimated_cost: Cost::default(),
            timeout_seconds: None,
            retry_policy: None,
            tags: Vec::new(),
        });
        self
    }

    /// Add a task template with full config.
    pub fn task_with(
        mut self,
        template: TaskTemplate,
    ) -> Self {
        self.task_templates.push(template);
        self
    }

    /// Add a dependency edge between two task templates (by index).
    pub fn depends_on(mut self, task_idx: usize, depends_on_idx: usize) -> Self {
        // Store by index; resolved to actual task IDs at instantiation
        self.edges.push(JobEdge {
            from: format!("{}", depends_on_idx),
            to: format!("{}", task_idx),
            kind: crate::job::DepKind::Blocks,
        });
        self
    }

    /// Add a trigger.
    pub fn trigger(mut self, trigger: WorkflowTrigger) -> Self {
        self.triggers.push(trigger);
        self
    }

    /// Set the estimated cost.
    pub fn estimated_cost(mut self, cost: Cost) -> Self {
        self.estimated_cost = cost;
        self
    }

    /// Bump the version.
    pub fn bump_version(&mut self) {
        self.version += 1;
    }

    /// Count task templates.
    pub fn task_count(&self) -> usize {
        self.task_templates.len()
    }
}

// ---------------------------------------------------------------------------
// Common workflow presets
// ---------------------------------------------------------------------------

impl Workflow {
    /// A standard CI workflow: checkout → build → test → deploy.
    pub fn ci_pipeline(created_by: ActorId) -> Self {
        Self::new(
            "CI Pipeline",
            created_by,
            Policy::standard_commit(),
        )
        .param("repo_url", ParamType::RepoRef, true)
        .param("branch", ParamType::BranchRef, true)
        .param_with_default("deploy", ParamType::Boolean, serde_json::Value::Bool(false))
        .task("Checkout", "Clone repository {repo_url} at {branch}", ExecutorKind::Pipeline)
        .task("Build", "Build the project", ExecutorKind::Pipeline)
        .task("Test", "Run the test suite", ExecutorKind::Pipeline)
        .task("Deploy", "Deploy to production", ExecutorKind::Pipeline)
        .depends_on(1, 0) // Build depends on Checkout
        .depends_on(2, 1) // Test depends on Build
        .depends_on(3, 2) // Deploy depends on Test
        .trigger(WorkflowTrigger::OnPush {
            repo_id: "{repo_id}".to_string(),
            branch_pattern: "main".to_string(),
        })
    }

    /// A standard code review workflow: plan → research → code → review → test.
    pub fn agent_code_review(created_by: ActorId, required_reviewers: usize) -> Self {
        Self::new(
            "Agent Code Review",
            created_by,
            Policy::pr_merge(required_reviewers),
        )
        .param("repo_url", ParamType::RepoRef, true)
        .param("pr_number", ParamType::Number, true)
        .task("Plan", "Analyze PR #{pr_number} and plan review", ExecutorKind::Agent)
        .task("Research", "Read relevant files and context", ExecutorKind::Agent)
        .task("Review", "Review code changes for bugs, style, security", ExecutorKind::Agent)
        .task("Summarize", "Generate review summary", ExecutorKind::Agent)
        .depends_on(1, 0)
        .depends_on(2, 1)
        .depends_on(3, 2)
        .trigger(WorkflowTrigger::OnPullRequest {
            repo_id: "{repo_id}".to_string(),
            action: "opened".to_string(),
        })
    }
}
