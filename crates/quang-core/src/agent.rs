//! Agent — unified agent identity and descriptor.
//!
//! This is Quang's own agent model — it composes concepts from:
//! - `jigsaw-core::SpikeClass` (Semantic Spikes for agent attestations)
//! - `minh_agent::AgentConfig` (name, role, system_prompt)
//! - `minh_agent::security::{PermissionLevel, SecurityContext}` (tool sandboxing)
//! - workplace `AgentCapability` (model, tools, resources)
//!
//! It does NOT copy any lower-level code. It defines the Quang-level
//! abstraction: a complete agent identity that can be deployed to any runtime.

use serde::{Deserialize, Serialize};

use crate::types::{ActorId, Timestamp, now};

// ---------------------------------------------------------------------------
// AgentRole
// ---------------------------------------------------------------------------

/// The role an agent plays in a workflow.
/// Maps to minh-agent's Orchestrator/Planner/Researcher/Coder/... pattern.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentRole {
    /// Coordinates sub-agents, summarizes results
    Orchestrator,
    /// Analyzes requirements, creates task plans
    Planner,
    /// Explores context, reads existing files/docs
    Researcher,
    /// Writes code, implements features
    Coder,
    /// Reviews code, audits quality
    Reviewer,
    /// Runs commands, executes builds
    Executor,
    /// Designs UI, styles, layouts
    Stylist,
    /// Writes documentation, READMEs
    Documenter,
    /// General-purpose assistant
    Assistant,
    /// Custom role
    Custom(String),
}

impl std::fmt::Display for AgentRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentRole::Orchestrator => write!(f, "orchestrator"),
            AgentRole::Planner => write!(f, "planner"),
            AgentRole::Researcher => write!(f, "researcher"),
            AgentRole::Coder => write!(f, "coder"),
            AgentRole::Reviewer => write!(f, "reviewer"),
            AgentRole::Executor => write!(f, "executor"),
            AgentRole::Stylist => write!(f, "stylist"),
            AgentRole::Documenter => write!(f, "documenter"),
            AgentRole::Assistant => write!(f, "assistant"),
            AgentRole::Custom(s) => write!(f, "custom:{}", s),
        }
    }
}

// ---------------------------------------------------------------------------
// PermissionLevel — tool sandboxing
// ---------------------------------------------------------------------------

/// Sandbox permission tier for agent tool execution.
/// Composes the concept from minh-agent's security model.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PermissionLevel {
    /// Full filesystem + network access (admin agents)
    AllowAll,
    /// Can read files, cannot write or execute
    ReadOnly,
    /// All commands run in Docker/Podman/Wasm sandbox
    SandboxOnly,
    /// Prompts human for approval before each write/exec
    Interactive,
}

// ---------------------------------------------------------------------------
// ModelDescriptor
// ---------------------------------------------------------------------------

/// The AI model an agent runs on.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelDescriptor {
    /// Provider: "openai", "anthropic", "google", "minh", etc.
    pub provider: String,
    /// Model name: "gpt-4o", "claude-sonnet-4-20250514", etc.
    pub model_name: String,
    /// Model version or date tag
    pub model_version: String,
    /// Maximum output tokens per request
    pub max_tokens: u64,
    /// Maximum context window length
    pub context_length: u64,
}

impl ModelDescriptor {
    pub fn new(provider: &str, model_name: &str, model_version: &str) -> Self {
        Self {
            provider: provider.to_string(),
            model_name: model_name.to_string(),
            model_version: model_version.to_string(),
            max_tokens: 4096,
            context_length: 128_000,
        }
    }
}

// ---------------------------------------------------------------------------
// ToolDescriptor
// ---------------------------------------------------------------------------

/// Declares a tool an agent can call — the schema layer.
/// The `Tool` trait implementation stays in minh-agent (or a runtime crate).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescriptor {
    /// Tool name (e.g., "read_file", "run_command")
    pub name: String,
    /// Human-readable description for the agent
    pub description: String,
    /// JSON Schema for the tool's input arguments
    pub input_schema: serde_json::Value,
    /// Does this tool require human approval before each call?
    pub requires_approval: bool,
    /// Rate limit (calls per minute)
    pub rate_limit_per_minute: Option<u32>,
    /// Estimated cost per invocation (micro-dollars)
    pub estimated_cost_per_call: Option<f64>,
}

// ---------------------------------------------------------------------------
// MemoryConfig
// ---------------------------------------------------------------------------

/// Configuration for agent memory — session and long-term.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryConfig {
    /// Maximum conversation turns before summarization
    pub session_max_turns: u32,
    /// Whether long-term fact extraction is enabled
    pub long_term_enabled: bool,
    /// Whether to auto-extract facts after each session
    pub fact_extraction_enabled: bool,
    /// Maximum stored facts in long-term memory
    pub max_facts: usize,
}

impl Default for MemoryConfig {
    fn default() -> Self {
        Self {
            session_max_turns: 32,
            long_term_enabled: true,
            fact_extraction_enabled: true,
            max_facts: 50,
        }
    }
}

// ---------------------------------------------------------------------------
// AgentCostProfile
// ---------------------------------------------------------------------------

/// Cost profile for an agent — used to estimate and track spending.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCostProfile {
    /// Cost per 1000 input tokens
    pub cost_per_1k_input_tokens: f64,
    /// Cost per 1000 output tokens
    pub cost_per_1k_output_tokens: f64,
    /// Cost per second of inference compute time
    pub cost_per_inference_second: f64,
    /// Currency (USD, VND, etc.)
    pub currency: String,
}

impl Default for AgentCostProfile {
    fn default() -> Self {
        Self {
            cost_per_1k_input_tokens: 0.003,
            cost_per_1k_output_tokens: 0.015,
            cost_per_inference_second: 0.0,
            currency: "USD".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// AgentDescriptor — the full agent identity
// ---------------------------------------------------------------------------

/// Complete agent identity — the Quang-level abstraction.
///
/// A single descriptor that fully defines an agent: who it is, what it can do,
/// how it behaves, what tools it has, and what it costs.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDescriptor {
    /// Actor identity
    pub actor_id: ActorId,

    // ── Behavior ──
    /// What role this agent plays
    pub role: AgentRole,
    /// System prompt that shapes the agent's behavior
    pub system_prompt: String,

    // ── Model ──
    /// The AI model this agent runs on
    pub model: ModelDescriptor,

    // ── Tools ──
    /// Tools the agent can call
    pub tools: Vec<ToolDescriptor>,

    // ── Security ──
    /// Sandbox permission level
    pub permission_level: PermissionLevel,
    /// Workspace root path (path containment boundary)
    pub workspace_root: Option<String>,

    // ── Memory ──
    /// Session and long-term memory configuration
    pub memory: MemoryConfig,

    // ── Cost ──
    /// Cost profile for budgeting and tracking
    pub cost_profile: AgentCostProfile,

    // ── Lifecycle ──
    pub is_active: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl AgentDescriptor {
    pub fn new(actor_id: ActorId, role: AgentRole, model: ModelDescriptor) -> Self {
        Self {
            actor_id,
            role,
            system_prompt: String::new(),
            model,
            tools: Vec::new(),
            permission_level: PermissionLevel::ReadOnly,
            workspace_root: None,
            memory: MemoryConfig::default(),
            cost_profile: AgentCostProfile::default(),
            is_active: true,
            created_at: now(),
            updated_at: now(),
        }
    }

    /// Builder: set the system prompt.
    pub fn with_system_prompt(mut self, prompt: &str) -> Self {
        self.system_prompt = prompt.to_string();
        self
    }

    /// Builder: add a tool.
    pub fn with_tool(mut self, tool: ToolDescriptor) -> Self {
        self.tools.push(tool);
        self
    }

    /// Builder: set permission level.
    pub fn with_permission(mut self, level: PermissionLevel) -> Self {
        self.permission_level = level;
        self
    }

    /// Builder: set workspace root.
    pub fn with_workspace(mut self, root: &str) -> Self {
        self.workspace_root = Some(root.to_string());
        self
    }

    /// Builder: set cost profile.
    pub fn with_cost(mut self, profile: AgentCostProfile) -> Self {
        self.cost_profile = profile;
        self
    }

    /// Check if this agent has a tool by name.
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.iter().any(|t| t.name == name)
    }

    /// Deactivate the agent.
    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.updated_at = now();
    }
}

// ---------------------------------------------------------------------------
// AgentExecutionEvent — cross-crate observability
// ---------------------------------------------------------------------------

/// Events emitted during agent execution.
/// Consumed by workplace (task assignment), repo (QTask execution),
/// and any hub watching agent activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AgentExecutionEvent {
    /// Agent started working on a task
    Started {
        agent_id: ActorId,
        task_id: String,
    },
    /// Agent entered a new phase
    PhaseChanged {
        agent_id: ActorId,
        task_id: String,
        old_phase: String,
        new_phase: String,
    },
    /// Agent called a tool
    ToolCall {
        agent_id: ActorId,
        task_id: String,
        tool_name: String,
        args: serde_json::Value,
    },
    /// Tool returned an observation
    ToolObservation {
        agent_id: ActorId,
        task_id: String,
        tool_name: String,
        observation: String,
    },
    /// Agent produced a log message
    Log {
        agent_id: ActorId,
        text: String,
    },
    /// Agent finished (success or failure)
    Finished {
        agent_id: ActorId,
        task_id: String,
        success: bool,
        summary: Option<String>,
    },
    /// Agent encountered an error
    Error {
        agent_id: ActorId,
        task_id: Option<String>,
        message: String,
    },
}
