//! Compensation & Budget — shared economic concepts for humans and agents.
//!
//! Both humans and agents have compensation plans and budgets:
//! - Humans: salary, bonus, expense budget
//! - Agents: compute allocation, token budget, API credits
//! The shared abstractions enable unified reporting and planning.

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

/// How compensation is measured.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CompensationPeriod {
    Hourly,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    PerTask,
    PerToken,
    PerComputeUnit,
}

impl std::fmt::Display for CompensationPeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompensationPeriod::Hourly => write!(f, "hourly"),
            CompensationPeriod::Daily => write!(f, "daily"),
            CompensationPeriod::Weekly => write!(f, "weekly"),
            CompensationPeriod::Monthly => write!(f, "monthly"),
            CompensationPeriod::Yearly => write!(f, "yearly"),
            CompensationPeriod::PerTask => write!(f, "per_task"),
            CompensationPeriod::PerToken => write!(f, "per_token"),
            CompensationPeriod::PerComputeUnit => write!(f, "per_compute_unit"),
        }
    }
}

/// A compensation plan for any actor (human salary or agent resource allocation).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompensationPlan {
    pub id: NodeId,
    pub actor: ActorId,
    pub base_amount: f64,
    pub currency: String,
    pub period: CompensationPeriod,
    pub is_active: bool,
    pub effective_from: Timestamp,
    pub effective_to: Option<Timestamp>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl CompensationPlan {
    pub fn new(actor: ActorId, base_amount: f64, currency: &str, period: CompensationPeriod) -> Self {
        Self {
            id: NodeId::new("comp"),
            actor,
            base_amount,
            currency: currency.to_string(),
            period,
            is_active: true,
            effective_from: now(),
            effective_to: None,
            metadata: serde_json::Map::new(),
        }
    }

    pub fn deactivate(&mut self) {
        self.is_active = false;
        self.effective_to = Some(now());
    }
}

/// A budget allocation for projects, departments, or compute resources.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Budget {
    pub id: NodeId,
    pub owner: ActorId,
    pub name: String,
    pub total_amount: f64,
    pub spent_amount: f64,
    pub currency: String,
    pub period_start: Timestamp,
    pub period_end: Timestamp,
    pub budget_type: BudgetType,
    pub status: BudgetStatus,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// What type of budget this is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetType {
    /// Human: project expense budget
    ProjectExpense,
    /// Human: departmental operating budget
    Department,
    /// Human: personal development budget
    PersonalDevelopment,
    /// Agent: compute resource allocation
    Compute,
    /// Agent: API token budget
    Token,
    /// Agent: API call budget
    ApiCalls,
    /// Shared: task incentive pool
    IncentivePool,
    Custom(String),
}

impl std::fmt::Display for BudgetType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BudgetType::ProjectExpense => write!(f, "project_expense"),
            BudgetType::Department => write!(f, "department"),
            BudgetType::PersonalDevelopment => write!(f, "personal_development"),
            BudgetType::Compute => write!(f, "compute"),
            BudgetType::Token => write!(f, "token"),
            BudgetType::ApiCalls => write!(f, "api_calls"),
            BudgetType::IncentivePool => write!(f, "incentive_pool"),
            BudgetType::Custom(s) => write!(f, "custom_{}", s),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BudgetStatus {
    Active,
    Frozen,
    Exhausted,
    Closed,
}

impl Budget {
    pub fn new(
        owner: ActorId,
        name: &str,
        total_amount: f64,
        currency: &str,
        period_start: Timestamp,
        period_end: Timestamp,
        budget_type: BudgetType,
    ) -> Self {
        Self {
            id: NodeId::new("budget"),
            owner,
            name: name.to_string(),
            total_amount,
            spent_amount: 0.0,
            currency: currency.to_string(),
            period_start,
            period_end,
            budget_type,
            status: BudgetStatus::Active,
            metadata: serde_json::Map::new(),
        }
    }

    /// Spend from the budget. Returns false if insufficient funds.
    pub fn spend(&mut self, amount: f64) -> bool {
        if self.spent_amount + amount > self.total_amount {
            return false;
        }
        self.spent_amount += amount;
        if self.spent_amount >= self.total_amount {
            self.status = BudgetStatus::Exhausted;
        }
        true
    }

    pub fn remaining(&self) -> f64 {
        (self.total_amount - self.spent_amount).max(0.0)
    }

    pub fn utilization(&self) -> f64 {
        if self.total_amount == 0.0 {
            return 0.0;
        }
        (self.spent_amount / self.total_amount).clamp(0.0, 1.0)
    }

    pub fn freeze(&mut self) {
        self.status = BudgetStatus::Frozen;
    }

    pub fn close(&mut self) {
        self.status = BudgetStatus::Closed;
    }
}

/// An incentive or reward for an actor.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Incentive {
    pub id: NodeId,
    pub actor: ActorId,
    pub name: String,
    pub description: String,
    pub incentive_type: IncentiveType,
    pub amount: Option<f64>,
    pub currency: Option<String>,
    /// Agent: reputation points, priority tokens, capability unlocks
    pub agent_reward: Option<String>,
    pub status: IncentiveStatus,
    pub granted_by: ActorId,
    pub granted_at: Timestamp,
    pub fulfilled_at: Option<Timestamp>,
}

/// Types of incentives available.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncentiveType {
    /// Human: monetary bonus
    Bonus,
    /// Human: promotion
    Promotion,
    /// Human: recognition award
    Recognition,
    /// Agent: reputation score increase
    Reputation,
    /// Agent: priority access to compute
    PriorityAccess,
    /// Agent: capability unlock (new tool/model access)
    CapabilityUnlock,
    /// Agent: token credit
    TokenCredit,
    /// Shared: commission
    Commission,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum IncentiveStatus {
    Promised,
    Granted,
    Fulfilled,
    Cancelled,
}

impl Incentive {
    pub fn new(
        actor: ActorId,
        name: &str,
        incentive_type: IncentiveType,
        granted_by: ActorId,
    ) -> Self {
        Self {
            id: NodeId::new("inc"),
            actor,
            name: name.to_string(),
            description: String::new(),
            incentive_type,
            amount: None,
            currency: None,
            agent_reward: None,
            status: IncentiveStatus::Promised,
            granted_by,
            granted_at: now(),
            fulfilled_at: None,
        }
    }

    pub fn fulfill(&mut self) {
        self.status = IncentiveStatus::Fulfilled;
        self.fulfilled_at = Some(now());
    }
}
