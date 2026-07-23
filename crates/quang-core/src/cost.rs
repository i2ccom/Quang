//! Cost — unified cost model for humans, agents, and infrastructure.
//!
//! Quang's own abstraction — higher than:
//! - minh-agent's raw metrics (tokens, duration)
//! - workplace's `Budget` (monetary only)
//! - jigsaw-core's `Weight` (fixed-point scoring, not cost)

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// HumanCost
// ---------------------------------------------------------------------------

/// Human labor cost.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HumanCost {
    /// Total hours worked
    pub hours: f64,
    /// Hourly rate
    pub hourly_rate: f64,
    /// Currency code
    pub currency: String,
}

impl HumanCost {
    pub fn total(&self) -> f64 {
        self.hours * self.hourly_rate
    }

    pub fn usd(hours: f64, rate: f64) -> Self {
        Self {
            hours,
            hourly_rate: rate,
            currency: "USD".to_string(),
        }
    }
}

// ---------------------------------------------------------------------------
// AgentCost
// ---------------------------------------------------------------------------

/// Agent compute cost — tokens + inference time.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AgentCost {
    /// Total tokens consumed
    pub tokens_used: u64,
    /// Cost per 1000 tokens
    pub cost_per_1k_tokens: f64,
    /// Total inference wall-clock seconds
    pub inference_seconds: f64,
    /// Cost per inference second
    pub cost_per_inference_second: f64,
    /// Currency code
    pub currency: String,
    /// Model name for reporting
    pub model_name: String,
}

impl AgentCost {
    pub fn total(&self) -> f64 {
        (self.tokens_used as f64 / 1000.0) * self.cost_per_1k_tokens
            + self.inference_seconds * self.cost_per_inference_second
    }
}

// ---------------------------------------------------------------------------
// InfraCost
// ---------------------------------------------------------------------------

/// Infrastructure cost — API calls, storage, bandwidth.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InfraCost {
    /// Number of API calls made
    pub api_calls: u64,
    /// Cost per API call
    pub cost_per_api_call: f64,
    /// Bytes stored
    pub storage_bytes: u64,
    /// Cost per GB stored
    pub cost_per_gb_stored: f64,
    /// Bytes transferred
    pub bandwidth_bytes: u64,
    /// Cost per GB transferred
    pub cost_per_gb_transfer: f64,
    /// Currency code
    pub currency: String,
}

impl InfraCost {
    pub fn total(&self) -> f64 {
        (self.api_calls as f64) * self.cost_per_api_call
            + (self.storage_bytes as f64 / 1_000_000_000.0) * self.cost_per_gb_stored
            + (self.bandwidth_bytes as f64 / 1_000_000_000.0) * self.cost_per_gb_transfer
    }
}

// ---------------------------------------------------------------------------
// Cost — unified container
// ---------------------------------------------------------------------------

/// Total cost across all dimensions: human + agent + infrastructure.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Cost {
    pub human: HumanCost,
    pub agent: AgentCost,
    pub infra: InfraCost,
}

impl Cost {
    /// Total cost across all dimensions.
    pub fn total(&self) -> f64 {
        self.human.total() + self.agent.total() + self.infra.total()
    }

    /// Create a cost estimate for a work estimate.
    pub fn estimate_human(hours: f64, hourly_rate: f64) -> Self {
        Self {
            human: HumanCost::usd(hours, hourly_rate),
            ..Default::default()
        }
    }

    /// Create a cost estimate for an agent task.
    pub fn estimate_agent(
        estimated_tokens: u64,
        cost_per_1k: f64,
        model_name: &str,
    ) -> Self {
        Self {
            agent: AgentCost {
                tokens_used: estimated_tokens,
                cost_per_1k_tokens: cost_per_1k,
                model_name: model_name.to_string(),
                currency: "USD".to_string(),
                ..Default::default()
            },
            ..Default::default()
        }
    }

    /// Add another cost into this one (accumulate).
    pub fn add(&mut self, other: &Cost) {
        self.human.hours += other.human.hours;
        self.agent.tokens_used += other.agent.tokens_used;
        self.agent.inference_seconds += other.agent.inference_seconds;
        self.infra.api_calls += other.infra.api_calls;
        self.infra.storage_bytes += other.infra.storage_bytes;
        self.infra.bandwidth_bytes += other.infra.bandwidth_bytes;
    }

    /// Is this cost zero across all dimensions?
    pub fn is_zero(&self) -> bool {
        self.total() == 0.0
    }
}
