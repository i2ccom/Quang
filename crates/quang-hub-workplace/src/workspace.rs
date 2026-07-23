//! WorkSpace — the top-level organizational unit in the collaboration graph.
//!
//! A WorkSpace represents a company, organization, or open-source community.
//! It contains Teams, Projects, Channels, and all collaboration state.

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

use crate::graph::{now, ActorId, NodeId, Timestamp};

/// Unique identifier for a WorkSpace.
pub type WorkSpaceId = NodeId;

/// The top-level organizational unit.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSpace {
    pub id: WorkSpaceId,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub owner: ActorId,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    /// Arbitrary key-value metadata for extensibility
    pub metadata: IndexMap<String, String>,
    /// Current workspace settings
    pub settings: WorkSpaceSettings,
}

/// Configurable settings for a WorkSpace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkSpaceSettings {
    pub default_team_id: Option<NodeId>,
    pub allow_agent_teams: bool,
    pub max_projects: i32,
    pub default_view: String, // "table" | "kanban" | "chart" | "gantt"
    pub features: Vec<String>,
}

impl Default for WorkSpaceSettings {
    fn default() -> Self {
        Self {
            default_team_id: None,
            allow_agent_teams: true,
            max_projects: 100,
            default_view: "kanban".to_string(),
            features: vec![
                "chat".into(),
                "tasks".into(),
                "goals".into(),
                "reviews".into(),
            ],
        }
    }
}

impl WorkSpace {
    pub fn new(name: &str, description: &str, slug: &str, owner: ActorId) -> Self {
        let now = now();
        Self {
            id: NodeId::new("ws"),
            name: name.to_string(),
            description: description.to_string(),
            slug: slug.to_string(),
            owner,
            created_at: now,
            updated_at: now,
            metadata: IndexMap::new(),
            settings: WorkSpaceSettings::default(),
        }
    }

    pub fn update_name(&mut self, name: &str) {
        self.name = name.to_string();
        self.updated_at = now();
    }

    pub fn update_description(&mut self, description: &str) {
        self.description = description.to_string();
        self.updated_at = now();
    }

    pub fn set_metadata(&mut self, key: &str, value: &str) {
        self.metadata.insert(key.to_string(), value.to_string());
        self.updated_at = now();
    }

    pub fn kind() -> &'static str {
        "workspace"
    }
}
