//! Human — human-specific identity, HR, and compliance extensions.
//!
//! While the Actor model provides a shared base, humans have additional
//! identity and compliance requirements: legal name, tax ID, government IDs,
//! emergency contacts, and compliance documents.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

/// Human-specific identity and legal information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HumanIdentity {
    pub actor_id: ActorId,
    pub legal_first_name: String,
    pub legal_last_name: String,
    pub legal_middle_name: Option<String>,
    pub government_id: Option<String>,   // SSN, SIN, etc. — encrypted at rest
    pub tax_id: Option<String>,          // Tax identifier
    pub nationality: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub emergency_contact_name: Option<String>,
    pub emergency_contact_phone: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl HumanIdentity {
    pub fn new(actor_id: ActorId, first_name: &str, last_name: &str) -> Self {
        Self {
            actor_id,
            legal_first_name: first_name.to_string(),
            legal_last_name: last_name.to_string(),
            legal_middle_name: None,
            government_id: None,
            tax_id: None,
            nationality: None,
            date_of_birth: None,
            gender: None,
            emergency_contact_name: None,
            emergency_contact_phone: None,
            address: None,
            phone: None,
            created_at: now(),
            updated_at: now(),
            metadata: serde_json::Map::new(),
        }
    }

    pub fn full_legal_name(&self) -> String {
        match &self.legal_middle_name {
            Some(middle) => format!("{} {} {}", self.legal_first_name, middle, self.legal_last_name),
            None => format!("{} {}", self.legal_first_name, self.legal_last_name),
        }
    }
}

/// A compliance document (contract, NDA, policy acknowledgment).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComplianceDocument {
    pub id: NodeId,
    pub actor_id: ActorId,
    pub document_type: String,       // "nda", "employment_contract", "policy", "code_of_conduct"
    pub title: String,
    pub file_url: Option<String>,
    pub signed_at: Option<Timestamp>,
    pub expires_at: Option<Timestamp>,
    pub is_acknowledged: bool,
    pub created_at: Timestamp,
}

impl ComplianceDocument {
    pub fn new(actor_id: ActorId, document_type: &str, title: &str) -> Self {
        Self {
            id: NodeId::new("doc"),
            actor_id,
            document_type: document_type.to_string(),
            title: title.to_string(),
            file_url: None,
            signed_at: None,
            expires_at: None,
            is_acknowledged: false,
            created_at: now(),
        }
    }

    pub fn sign(&mut self) {
        self.signed_at = Some(now());
        self.is_acknowledged = true;
    }
}

/// Leave/absence record for humans.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaveRecord {
    pub id: NodeId,
    pub actor_id: ActorId,
    pub leave_type: LeaveType,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub reason: String,
    pub status: LeaveStatus,
    pub approved_by: Option<ActorId>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaveType {
    Annual,
    Sick,
    Parental,
    Bereavement,
    Unpaid,
    Custom(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum LeaveStatus {
    Pending,
    Approved,
    Rejected,
    Cancelled,
    Taken,
}

impl LeaveRecord {
    pub fn new(actor_id: ActorId, leave_type: LeaveType, start_date: NaiveDate, end_date: NaiveDate, reason: &str) -> Self {
        Self {
            id: NodeId::new("leave"),
            actor_id,
            leave_type,
            start_date,
            end_date,
            reason: reason.to_string(),
            status: LeaveStatus::Pending,
            approved_by: None,
            created_at: now(),
            updated_at: now(),
        }
    }

    pub fn approve(&mut self, by: ActorId) {
        self.status = LeaveStatus::Approved;
        self.approved_by = Some(by);
        self.updated_at = now();
    }

    pub fn reject(&mut self) {
        self.status = LeaveStatus::Rejected;
        self.updated_at = now();
    }

    pub fn duration_days(&self) -> i64 {
        (self.end_date - self.start_date).num_days().max(0)
    }
}
