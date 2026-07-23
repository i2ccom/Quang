//! Audit — immutable audit trail for all actor actions and decisions.
//!
//! Every significant action (approval, rejection, assignment, budget change, etc.)
//! is recorded as an AuditEntry. The audit trail is append-only and supports:
//! - Decision rationale capture (why was this decision made?)
//! - Evidence linking (what evidence supports this decision?)
//! - Compliance reporting
//! - Agent action traceability

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

/// A single audit entry recording an action or decision.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: String,
    pub actor: ActorId,
    /// What action was taken (e.g., "task.approve", "budget.allocate", "contract.sign")
    pub action: String,
    /// What type of thing was acted upon
    pub target_type: String,
    /// The specific node that was acted upon
    pub target_id: NodeId,
    /// The decision that was made
    pub decision: Option<String>,
    /// Why the decision was made (free text rationale)
    pub rationale: Option<String>,
    /// Links to evidence (file URLs, document IDs, node IDs)
    pub evidence: Vec<String>,
    /// The previous state (JSON snapshot)
    pub previous_state: Option<serde_json::Value>,
    /// The new state (JSON snapshot)
    pub new_state: Option<serde_json::Value>,
    pub timestamp: Timestamp,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl AuditEntry {
    pub fn new(actor: ActorId, action: &str, target_type: &str, target_id: NodeId) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            actor,
            action: action.to_string(),
            target_type: target_type.to_string(),
            target_id,
            decision: None,
            rationale: None,
            evidence: Vec::new(),
            previous_state: None,
            new_state: None,
            timestamp: now(),
            metadata: serde_json::Map::new(),
        }
    }

    pub fn with_decision(mut self, decision: &str) -> Self {
        self.decision = Some(decision.to_string());
        self
    }

    pub fn with_rationale(mut self, rationale: &str) -> Self {
        self.rationale = Some(rationale.to_string());
        self
    }

    pub fn with_evidence(mut self, evidence: Vec<String>) -> Self {
        self.evidence = evidence;
        self
    }

    pub fn with_state_snapshot<T: Serialize>(mut self, previous: Option<&T>, new: Option<&T>) -> Self {
        self.previous_state = previous.and_then(|v| serde_json::to_value(v).ok());
        self.new_state = new.and_then(|v| serde_json::to_value(v).ok());
        self
    }
}

/// The audit trail — an append-only log of all significant actions.
#[derive(Debug, Clone)]
pub struct AuditTrail {
    pub entries: Vec<AuditEntry>,
}

impl AuditTrail {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    /// Record an audit entry.
    pub fn record(&mut self, entry: AuditEntry) {
        tracing::info!(
            "[Audit] {} performed '{}' on {}",
            entry.actor, entry.action, entry.target_id
        );
        self.entries.push(entry);
    }

    /// Get all audit entries for a specific target.
    pub fn for_target(&self, target_id: &NodeId) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.target_id == *target_id)
            .collect()
    }

    /// Get all audit entries by a specific actor.
    pub fn by_actor(&self, actor: &ActorId) -> Vec<&AuditEntry> {
        self.entries.iter().filter(|e| e.actor == *actor).collect()
    }

    /// Get all audit entries for a specific action type.
    pub fn by_action(&self, action: &str) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.action == action)
            .collect()
    }

    /// Get all audit entries since a given timestamp.
    pub fn since(&self, timestamp: Timestamp) -> Vec<&AuditEntry> {
        self.entries
            .iter()
            .filter(|e| e.timestamp > timestamp)
            .collect()
    }

    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }
}

impl Default for AuditTrail {
    fn default() -> Self {
        Self::new()
    }
}

/// An approval record — a specific type of audit entry for approval workflows.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalRecord {
    pub id: String,
    pub approver: ActorId,
    pub target_type: String,
    pub target_id: NodeId,
    pub decision: ApprovalDecision,
    pub rationale: String,
    pub evidence: Vec<String>,
    pub timestamp: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ApprovalDecision {
    Approved,
    Rejected,
    ChangesRequested,
    Deferred,
}
