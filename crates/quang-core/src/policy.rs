//! Policy — governance rules for Tasks and Jobs.
//!
//! Quang's own governance model. Composes jigsaw-core's evidence model
//! (SpikeClass, Weight) into business-level rules without duplicating them.

use serde::{Deserialize, Serialize};
use jigsaw_core::Weight;

use crate::types::{ActorId, PolicyId, Timestamp, now};

// ---------------------------------------------------------------------------
// RequirementLevel
// ---------------------------------------------------------------------------

/// How critical an evidence requirement is.
/// Maps to jigsaw-core's `SpikeClass` but is Quang's own business-level
/// classification (not a re-export).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RequirementLevel {
    /// Must be satisfied — task/job fails without it
    Mandatory,
    /// Should be satisfied — contributes strongly to confidence
    Expected,
    /// Nice to have — adds marginal confidence
    Optional,
    /// Informational only — AI-generated, human annotation
    Annotative,
}

// ---------------------------------------------------------------------------
// EvidenceRequirement
// ---------------------------------------------------------------------------

/// A single evidence requirement for a Policy.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRequirement {
    /// The kind of evidence: "content_hash", "actor_signature", "review_approval",
    /// "test_pass", "deploy_success", etc.
    pub kind: String,
    /// How critical this requirement is
    pub level: RequirementLevel,
    /// How much it contributes to the acceptance score.
    /// Serializes as i64 (milli-units) via the weight_serde adapter.
    #[serde(with = "crate::weight_serde")]
    pub weight: Weight,
}

// ---------------------------------------------------------------------------
// PolicyOutcome
// ---------------------------------------------------------------------------

/// What happens at each acceptance level.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PolicyOutcome {
    /// Continue without restriction
    Proceed,
    /// Continue but surface a warning
    ProceedWithWarning,
    /// Escalate to specific approvers
    EscalateTo { approvers: Vec<ActorId> },
    /// Block — do not proceed
    Block,
    /// Fork — create a parallel branch instead of blocking
    Fork,
}

// ---------------------------------------------------------------------------
// PolicyOutcomes
// ---------------------------------------------------------------------------

/// Actions to take for each policy decision level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PolicyOutcomes {
    /// Evidence fully corroborated
    pub on_accept: PolicyOutcome,
    /// Partial corroboration, no conflicts
    pub on_softpass: PolicyOutcome,
    /// Evidence contradicts
    pub on_conflict: PolicyOutcome,
    /// Hard failure — mandatory requirement unmet
    pub on_reject: PolicyOutcome,
}

impl Default for PolicyOutcomes {
    fn default() -> Self {
        Self {
            on_accept: PolicyOutcome::Proceed,
            on_softpass: PolicyOutcome::ProceedWithWarning,
            on_conflict: PolicyOutcome::EscalateTo {
                approvers: Vec::new(),
            },
            on_reject: PolicyOutcome::Block,
        }
    }
}

// ---------------------------------------------------------------------------
// Policy
// ---------------------------------------------------------------------------

/// A Policy governs how a Task or Job is evaluated.
///
/// It defines what evidence must be present, at what weight, and what
/// happens at each acceptance level. Policies compose Jigsaw's evidence
/// model into business rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub id: PolicyId,
    pub name: String,
    pub description: String,

    /// The acceptance score threshold.
    /// Serializes as i64 (milli-units) via the weight_serde adapter.
    #[serde(with = "crate::weight_serde")]
    pub threshold: Weight,

    /// Required evidence categories with weights
    pub requirements: Vec<EvidenceRequirement>,

    /// Who can approve policy deviations
    pub approvers: Vec<ActorId>,

    /// What happens at each acceptance level
    pub outcomes: PolicyOutcomes,

    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

impl Policy {
    /// Create a new policy with a given threshold.
    pub fn new(name: &str, threshold: Weight) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            description: String::new(),
            threshold,
            requirements: Vec::new(),
            approvers: Vec::new(),
            outcomes: PolicyOutcomes::default(),
            created_at: now(),
            updated_at: now(),
        }
    }

    /// Add an evidence requirement.
    pub fn require(mut self, kind: &str, level: RequirementLevel, weight: Weight) -> Self {
        self.requirements.push(EvidenceRequirement {
            kind: kind.to_string(),
            level,
            weight,
        });
        self
    }

    /// Add an approver.
    pub fn with_approver(mut self, approver: ActorId) -> Self {
        self.approvers.push(approver);
        self
    }

    /// Set what happens on conflict.
    pub fn on_conflict(mut self, outcome: PolicyOutcome) -> Self {
        self.outcomes.on_conflict = outcome;
        self
    }

    /// Set what happens on reject.
    pub fn on_reject(mut self, outcome: PolicyOutcome) -> Self {
        self.outcomes.on_reject = outcome;
        self
    }

    /// A policy that requires nothing — always accepts.
    pub fn permissive() -> Self {
        Self::new("permissive", Weight::ZERO)
    }

    /// A policy that requires content hash + actor signature — standard commit policy.
    pub fn standard_commit() -> Self {
        Self::new("standard_commit", Weight::units(1))
            .require("content_hash", RequirementLevel::Mandatory, Weight::ONE)
            .require("actor_signature", RequirementLevel::Mandatory, Weight::ONE)
    }

    /// A policy that requires content hash + review approval — PR merge policy.
    pub fn pr_merge(required_reviewers: usize) -> Self {
        let mut policy = Self::new("pr_merge", Weight::units(2))
            .require("content_hash", RequirementLevel::Mandatory, Weight::ONE)
            .require(
                "review_approval",
                RequirementLevel::Mandatory,
                Weight::units(required_reviewers as i64),
            )
            .require("ci_green", RequirementLevel::Expected, Weight::units(1));
        if required_reviewers == 0 {
            policy = policy.on_reject(PolicyOutcome::ProceedWithWarning);
        }
        policy
    }
}
