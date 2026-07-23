//! Team — a group of humans and/or agents collaborating together.
//!
//! Teams have typed members with roles and can own projects, channels, etc.

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

pub type TeamId = NodeId;

/// Roles a member can have within a team.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TeamRole {
    Owner,
    Admin,
    Member,
    Viewer,
    Agent,
    Custom(String),
}

impl std::fmt::Display for TeamRole {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TeamRole::Owner => write!(f, "owner"),
            TeamRole::Admin => write!(f, "admin"),
            TeamRole::Member => write!(f, "member"),
            TeamRole::Viewer => write!(f, "viewer"),
            TeamRole::Agent => write!(f, "agent"),
            TeamRole::Custom(s) => write!(f, "custom_{}", s),
        }
    }
}

/// A member of a team (human or agent).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TeamMember {
    pub actor: ActorId,
    pub role: TeamRole,
    pub joined_at: Timestamp,
    pub display_name: String,
    pub avatar_url: Option<String>,
}

impl TeamMember {
    pub fn new(actor: ActorId, role: TeamRole, display_name: &str) -> Self {
        Self {
            actor,
            role,
            joined_at: now(),
            display_name: display_name.to_string(),
            avatar_url: None,
        }
    }

    pub fn with_avatar(mut self, url: &str) -> Self {
        self.avatar_url = Some(url.to_string());
        self
    }
}

/// A group of collaborators.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Team {
    pub id: TeamId,
    pub name: String,
    pub description: String,
    pub members: Vec<TeamMember>,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Team {
    pub fn new(name: &str, description: &str) -> Self {
        let now = now();
        Self {
            id: NodeId::new("team"),
            name: name.to_string(),
            description: description.to_string(),
            members: Vec::new(),
            created_at: now,
            updated_at: now,
            metadata: serde_json::Map::new(),
        }
    }

    pub fn add_member(&mut self, member: TeamMember) {
        // Replace if same actor exists
        if let Some(pos) = self.members.iter().position(|m| m.actor == member.actor) {
            self.members[pos] = member;
        } else {
            self.members.push(member);
        }
        self.updated_at = now();
    }

    pub fn remove_member(&mut self, actor: &ActorId) {
        self.members.retain(|m| m.actor != *actor);
        self.updated_at = now();
    }

    pub fn has_member(&self, actor: &ActorId) -> bool {
        self.members.iter().any(|m| m.actor == *actor)
    }

    pub fn members_by_role(&self, role: &TeamRole) -> Vec<&TeamMember> {
        self.members.iter().filter(|m| m.role == *role).collect()
    }

    pub fn human_members(&self) -> Vec<&TeamMember> {
        self.members.iter().filter(|m| m.actor.is_human()).collect()
    }

    pub fn agent_members(&self) -> Vec<&TeamMember> {
        self.members.iter().filter(|m| m.actor.is_agent()).collect()
    }

    pub fn member_count(&self) -> usize {
        self.members.len()
    }

    pub fn kind() -> &'static str {
        "team"
    }
}
