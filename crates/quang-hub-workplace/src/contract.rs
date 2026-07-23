//! Contract — agreements for both humans and agents.
//!
//! Shared contract concepts: terms, scope, duration, parties.
//! Human-specific: employment contracts with benefits, working hours.
//! Agent-specific: service agreements with SLA, tool permissions, sandbox rules.

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

/// The status of a contract.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContractStatus {
    Draft,
    Active,
    PendingRenewal,
    Expired,
    Terminated,
    Cancelled,
}

/// A shared contract base — any agreement between parties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: NodeId,
    pub title: String,
    pub description: String,
    pub parties: Vec<ActorId>,
    pub status: ContractStatus,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub terms: Vec<String>,
    pub signed_by: Vec<ActorId>,
    pub signed_at: Option<Timestamp>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Contract {
    pub fn new(title: &str, parties: Vec<ActorId>, start_date: NaiveDate) -> Self {
        Self {
            id: NodeId::new("contract"),
            title: title.to_string(),
            description: String::new(),
            parties,
            status: ContractStatus::Draft,
            start_date,
            end_date: None,
            terms: Vec::new(),
            signed_by: Vec::new(),
            signed_at: None,
            created_at: now(),
            updated_at: now(),
            metadata: serde_json::Map::new(),
        }
    }

    pub fn add_term(&mut self, term: &str) {
        self.terms.push(term.to_string());
        self.updated_at = now();
    }

    pub fn sign(&mut self, by: ActorId) {
        if !self.signed_by.contains(&by) {
            self.signed_by.push(by);
        }
        if self.signed_by.len() >= self.parties.len() {
            self.status = ContractStatus::Active;
            self.signed_at = Some(now());
        }
        self.updated_at = now();
    }

    pub fn terminate(&mut self) {
        self.status = ContractStatus::Terminated;
        self.updated_at = now();
    }

    pub fn kind() -> &'static str {
        "contract"
    }
}

/// Human-specific: employment contract with HR fields.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmploymentContract {
    pub id: NodeId,
    pub contract_id: NodeId, // Link to base Contract
    pub human_id: ActorId,
    pub employer_id: ActorId,
    pub contract_type: EmploymentType,
    pub department: String,
    pub title: String,
    pub manager: Option<ActorId>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub probation_end: Option<NaiveDate>,
    pub working_hours_per_week: f64,
    pub is_full_time: bool,
    pub benefits: Vec<String>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EmploymentType {
    FullTime,
    PartTime,
    Contractor,
    Intern,
    Freelancer,
    Consultant,
    Custom(String),
}

impl EmploymentContract {
    pub fn new(
        contract_id: NodeId,
        human_id: ActorId,
        employer_id: ActorId,
        contract_type: EmploymentType,
        department: &str,
        title: &str,
        start_date: NaiveDate,
    ) -> Self {
        Self {
            id: NodeId::new("employment"),
            contract_id,
            human_id,
            employer_id,
            contract_type,
            department: department.to_string(),
            title: title.to_string(),
            manager: None,
            start_date,
            end_date: None,
            probation_end: None,
            working_hours_per_week: 40.0,
            is_full_time: true,
            benefits: Vec::new(),
            created_at: now(),
            updated_at: now(),
        }
    }

    pub fn kind() -> &'static str {
        "employment_contract"
    }
}
