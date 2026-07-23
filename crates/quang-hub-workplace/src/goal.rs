//! Goal — high-level objectives aligned to OKR (Objectives and Key Results).
//!
//! Goals belong to Projects or WorkSpaces. They decompose into Key Results
//! which are measurable milestones. Progress is tracked and visible in views.

use serde::{Deserialize, Serialize};

use crate::graph::{now, ActorId, NodeId, Timestamp};

pub type GoalId = NodeId;

/// The status of a goal.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GoalStatus {
    Draft,
    Active,
    OnTrack,
    AtRisk,
    Behind,
    Completed,
    Cancelled,
}

impl std::fmt::Display for GoalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GoalStatus::Draft => write!(f, "draft"),
            GoalStatus::Active => write!(f, "active"),
            GoalStatus::OnTrack => write!(f, "on_track"),
            GoalStatus::AtRisk => write!(f, "at_risk"),
            GoalStatus::Behind => write!(f, "behind"),
            GoalStatus::Completed => write!(f, "completed"),
            GoalStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// A measurable Key Result that tracks progress toward a Goal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyResult {
    pub id: String,
    pub title: String,
    pub description: String,
    pub target_value: f64,
    pub current_value: f64,
    pub unit: String, // "percent", "count", "dollars", "hours", etc.
    pub owner: ActorId,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl KeyResult {
    pub fn new(title: &str, target_value: f64, unit: &str, owner: ActorId) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            title: title.to_string(),
            description: String::new(),
            target_value,
            current_value: 0.0,
            unit: unit.to_string(),
            owner,
            created_at: now(),
            updated_at: now(),
        }
    }

    pub fn progress(&self) -> f64 {
        if self.target_value == 0.0 {
            return 0.0;
        }
        (self.current_value / self.target_value).clamp(0.0, 1.0)
    }

    pub fn update_value(&mut self, value: f64) {
        self.current_value = value;
        self.updated_at = now();
    }
}

/// A high-level objective with measurable Key Results.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Goal {
    pub id: GoalId,
    pub title: String,
    pub description: String,
    pub status: GoalStatus,
    /// The quarter or period this goal targets (e.g., "2026-Q2")
    pub period: String,
    pub owner: ActorId,
    pub key_results: Vec<KeyResult>,
    pub start_date: Option<Timestamp>,
    pub end_date: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub weight: f64, // importance weight relative to sibling goals
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Goal {
    pub fn new(title: &str, description: &str, period: &str, owner: ActorId) -> Self {
        let now = now();
        Self {
            id: NodeId::new("goal"),
            title: title.to_string(),
            description: description.to_string(),
            status: GoalStatus::Draft,
            period: period.to_string(),
            owner,
            key_results: Vec::new(),
            start_date: None,
            end_date: None,
            created_at: now,
            updated_at: now,
            weight: 1.0,
            metadata: serde_json::Map::new(),
        }
    }

    /// Overall progress is the weighted average of Key Result progress.
    pub fn progress(&self) -> f64 {
        if self.key_results.is_empty() {
            return 0.0;
        }
        let sum: f64 = self.key_results.iter().map(|kr| kr.progress()).sum();
        sum / self.key_results.len() as f64
    }

    pub fn add_key_result(&mut self, kr: KeyResult) {
        self.key_results.push(kr);
        self.updated_at = now();
    }

    pub fn set_status(&mut self, status: GoalStatus) {
        self.status = status;
        self.updated_at = now();
    }

    /// Automatically compute status based on progress.
    pub fn recalculate_status(&mut self) {
        let p = self.progress();
        self.status = if p >= 1.0 {
            GoalStatus::Completed
        } else if p >= 0.75 {
            GoalStatus::OnTrack
        } else if p >= 0.5 {
            GoalStatus::AtRisk
        } else if self.status != GoalStatus::Draft {
            GoalStatus::Active
        } else {
            GoalStatus::Draft
        };
        self.updated_at = now();
    }

    pub fn kind() -> &'static str {
        "goal"
    }
}
