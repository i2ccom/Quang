//! Agent — agent-specific extensions for capabilities, resources, and trust.
//!
//! While the Actor model provides a shared base, agents have additional
//! concepts: capability manifests, model metadata, resource allocations,
//! trust/reputation scores, and owner relationships.

use serde::{Deserialize, Serialize};

use crate::graph::{now, ActorId, NodeId, Timestamp};

/// Agent capability manifest — what this agent can do.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCapability {
    pub actor_id: ActorId,
    pub model_name: String,
    pub model_provider: String, // "openai", "anthropic", "google", "minh", etc.
    pub model_version: String,
    pub tools: Vec<AgentTool>,
    pub max_tokens_per_request: u64,
    pub max_context_length: u64,
    pub supported_languages: Vec<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

/// A tool/function that an agent can call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value, // JSON Schema
    pub requires_approval: bool,
    pub rate_limit: Option<u32>,
    pub cost_per_call: Option<f64>,
}

impl AgentCapability {
    pub fn new(
        actor_id: ActorId,
        model_name: &str,
        model_provider: &str,
        model_version: &str,
    ) -> Self {
        Self {
            actor_id,
            model_name: model_name.to_string(),
            model_provider: model_provider.to_string(),
            model_version: model_version.to_string(),
            tools: Vec::new(),
            max_tokens_per_request: 4096,
            max_context_length: 128_000,
            supported_languages: vec!["en".to_string()],
            created_at: now(),
            updated_at: now(),
        }
    }

    pub fn add_tool(&mut self, tool: AgentTool) {
        self.tools.push(tool);
        self.updated_at = now();
    }

    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.iter().any(|t| t.name == name)
    }
}

/// Agent resource allocation — compute, tokens, concurrency limits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentResourceAllocation {
    pub actor_id: ActorId,
    pub compute_priority: u32, // Higher = more priority
    pub api_rate_limit_per_minute: u32,
    pub token_budget_daily: u64,
    pub token_used_today: u64,
    pub concurrent_task_limit: u32,
    pub max_memory_mb: u64,
    pub max_storage_mb: u64,
    pub allowed_networks: Vec<String>, // "internal", "internet", "none"
    pub updated_at: Timestamp,
}

impl AgentResourceAllocation {
    pub fn new(actor_id: ActorId) -> Self {
        Self {
            actor_id,
            compute_priority: 1,
            api_rate_limit_per_minute: 60,
            token_budget_daily: 1_000_000,
            token_used_today: 0,
            concurrent_task_limit: 5,
            max_memory_mb: 512,
            max_storage_mb: 1024,
            allowed_networks: vec!["internal".to_string()],
            updated_at: now(),
        }
    }

    /// Record token usage. Returns false if over budget.
    pub fn record_tokens(&mut self, tokens: u64) -> bool {
        self.token_used_today += tokens;
        self.token_used_today <= self.token_budget_daily
    }

    pub fn reset_daily_usage(&mut self) {
        self.token_used_today = 0;
        self.updated_at = now();
    }

    pub fn remaining_tokens(&self) -> u64 {
        self.token_budget_daily
            .saturating_sub(self.token_used_today)
    }
}

/// Agent trust and reputation score.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReputation {
    pub actor_id: ActorId,
    pub trust_score: f64,       // 0.0 (untrusted) to 1.0 (fully trusted)
    pub accuracy_score: f64,    // Historical accuracy of outputs
    pub reliability_score: f64, // Uptime / availability
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub avg_response_time_ms: Option<f64>,
    pub reviews_received: Vec<AgentReview>,
    pub updated_at: Timestamp,
}

/// A review/evaluation of an agent's work.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentReview {
    pub reviewer: ActorId,
    pub score: u8, // 1-5
    pub feedback: String,
    pub task_id: Option<NodeId>,
    pub created_at: Timestamp,
}

impl AgentReputation {
    pub fn new(actor_id: ActorId) -> Self {
        Self {
            actor_id,
            trust_score: 0.5, // Start neutral
            accuracy_score: 0.5,
            reliability_score: 1.0,
            tasks_completed: 0,
            tasks_failed: 0,
            avg_response_time_ms: None,
            reviews_received: Vec::new(),
            updated_at: now(),
        }
    }

    pub fn record_completion(&mut self, success: bool) {
        if success {
            self.tasks_completed += 1;
        } else {
            self.tasks_failed += 1;
        }
        let total = self.tasks_completed + self.tasks_failed;
        self.accuracy_score = self.tasks_completed as f64 / total as f64;
        self.updated_at = now();
    }

    pub fn add_review(&mut self, review: AgentReview) {
        self.reviews_received.push(review);
        // Recompute trust score as average of recent reviews
        if !self.reviews_received.is_empty() {
            let avg: f64 = self
                .reviews_received
                .iter()
                .map(|r| r.score as f64)
                .sum::<f64>()
                / self.reviews_received.len() as f64;
            self.trust_score = (avg / 5.0).clamp(0.0, 1.0);
        }
        self.updated_at = now();
    }

    pub fn success_rate(&self) -> f64 {
        let total = self.tasks_completed + self.tasks_failed;
        if total == 0 {
            return 1.0;
        }
        self.tasks_completed as f64 / total as f64
    }
}

/// Agent owner — who created/governs this agent.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentOwnership {
    pub actor_id: ActorId,
    pub owner: ActorId,            // Human or organization that owns this agent
    pub creator: ActorId,          // Who originally created/deployed this agent
    pub governed_by: Vec<ActorId>, // Who can modify agent settings/policies
    pub created_at: Timestamp,
}
