//! Actor — unified identity and profile for both humans and agents.
//!
//! The core insight: Humans and Agents are both Actors in the workplace.
//! They share identity concepts like profile, organization membership, and rank.
//! Human-specific extensions go in `human.rs`, agent-specific in `agent.rs`.

use serde::{Deserialize, Serialize};

use crate::graph::{now, ActorId, NodeId, Timestamp};

/// A profile shared by both human and agent actors.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorProfile {
    pub actor_id: ActorId,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: String,
    pub timezone: Option<String>,
    pub locale: Option<String>,
    /// The organization/department this actor belongs to
    pub organization_id: Option<NodeId>,
    /// The rank/grade this actor holds
    pub rank_id: Option<NodeId>,
    pub is_active: bool,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl ActorProfile {
    pub fn new(actor_id: ActorId, display_name: &str) -> Self {
        let now = now();
        Self {
            actor_id,
            display_name: display_name.to_string(),
            email: None,
            avatar_url: None,
            bio: String::new(),
            timezone: None,
            locale: None,
            organization_id: None,
            rank_id: None,
            is_active: true,
            created_at: now,
            updated_at: now,
            metadata: serde_json::Map::new(),
        }
    }
}

/// An organizational unit (department, division, team).
/// Shared by humans and agents — agents can belong to "agent teams" or "capability groups".
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
    pub id: NodeId,
    pub name: String,
    pub description: String,
    pub parent_id: Option<NodeId>,
    pub org_type: OrgType,
    pub created_at: Timestamp,
}

/// Type of organizational unit.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum OrgType {
    Company,
    Division,
    Department,
    Team,
    CapabilityGroup,
    AgentPool,
    Custom(String),
}

impl Organization {
    pub fn new(name: &str, description: &str, org_type: OrgType) -> Self {
        Self {
            id: NodeId::new("org"),
            name: name.to_string(),
            description: description.to_string(),
            parent_id: None,
            org_type,
            created_at: now(),
        }
    }

    pub fn with_parent(mut self, parent_id: NodeId) -> Self {
        self.parent_id = Some(parent_id);
        self
    }

    pub fn kind() -> &'static str {
        "organization"
    }
}

/// Rank or grade level — shared between humans (job title, seniority) and agents (capability tier).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rank {
    pub id: NodeId,
    pub name: String, // "Senior Engineer", "T3 Agent", "Director"
    pub level: u32,   // Numeric level for ordering (1 = lowest)
    pub category: RankCategory,
    pub description: String,
    pub created_at: Timestamp,
}

/// What kind of rank this is.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RankCategory {
    /// Human job title / seniority level
    JobTitle,
    /// Agent capability tier
    AgentTier,
    /// Trust/safety clearance
    Clearance,
    /// Custom
    Custom(String),
}

impl Rank {
    pub fn new(name: &str, level: u32, category: RankCategory) -> Self {
        Self {
            id: NodeId::new("rank"),
            name: name.to_string(),
            level,
            category,
            description: String::new(),
            created_at: now(),
        }
    }

    pub fn kind() -> &'static str {
        "rank"
    }
}
