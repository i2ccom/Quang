//! LinkedRepo — a GitHub repository connection in QuangHub.
//!
//! Each `LinkedRepo` represents a GitHub repository that a user or
//! organization has connected to QuangHub. It carries two remote endpoints:
//! the upstream GitHub URL and the internal Fluid Remote URL for agent access.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::repo_settings::RepoSettings;

/// Unique identifier for a linked repository.
pub type RepoId = String;

/// Connection status between QuangHub and the upstream GitHub repo.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RepoConnectionStatus {
    /// Repo is linked and reachable
    Connected,
    /// Repo is linked but GitHub is unreachable
    Disconnected,
    /// Initial sync with Fluid Remote is in progress
    Syncing,
    /// OAuth token has expired or was revoked
    AuthFailed,
    /// Repo was intentionally disconnected
    Archived,
}

impl std::fmt::Display for RepoConnectionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RepoConnectionStatus::Connected => write!(f, "connected"),
            RepoConnectionStatus::Disconnected => write!(f, "disconnected"),
            RepoConnectionStatus::Syncing => write!(f, "syncing"),
            RepoConnectionStatus::AuthFailed => write!(f, "auth_failed"),
            RepoConnectionStatus::Archived => write!(f, "archived"),
        }
    }
}

/// A GitHub repository linked to QuangHub.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedRepo {
    /// QuangHub internal repo ID (UUID)
    pub id: RepoId,
    /// GitHub owner (user or org)
    pub owner: String,
    /// GitHub repo name
    pub name: String,
    /// Full GitHub URL
    pub url: String,
    /// Default branch (e.g. "main" or "master")
    pub default_branch: String,
    /// Whether the repo is currently linked and active
    pub is_linked: bool,
    /// Connection status
    pub status: RepoConnectionStatus,
    /// Optional description synced from GitHub
    pub description: Option<String>,
    /// Primary language (from GitHub API)
    pub language: Option<String>,
    /// Star count from GitHub
    pub stars: u64,
    /// Fork count from GitHub
    pub forks: u64,
    /// Whether this repo is private on GitHub
    pub is_private: bool,
    /// GitHub API URL for this repo
    pub api_url: String,
    /// GitHub clone URL (HTTPS)
    pub clone_url: String,
    /// Fluid Remote URL (internal QuangHub mirror)
    pub fluid_remote_url: Option<String>,
    /// Repo settings (mirror, deploy, webhook)
    pub settings: RepoSettings,
    /// GitHub OAuth token (encrypted at rest, never exposed to frontend)
    #[serde(skip_serializing)]
    pub access_token: Option<String>,
    /// The QuangHub workspace this repo belongs to
    pub workspace_id: Option<String>,
    /// Timestamps
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// Last successful sync with Fluid Remote
    pub last_synced_at: Option<DateTime<Utc>>,
}

impl LinkedRepo {
    /// Create a new `LinkedRepo` from GitHub API data.
    pub fn new(
        owner: &str,
        name: &str,
        default_branch: &str,
        is_private: bool,
        access_token: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            owner: owner.to_string(),
            name: name.to_string(),
            url: format!("https://github.com/{}/{}", owner, name),
            default_branch: default_branch.to_string(),
            is_linked: true,
            status: RepoConnectionStatus::Syncing,
            description: None,
            language: None,
            stars: 0,
            forks: 0,
            is_private,
            api_url: format!("https://api.github.com/repos/{}/{}", owner, name),
            clone_url: format!("https://github.com/{}/{}.git", owner, name),
            fluid_remote_url: None,
            settings: RepoSettings::default(),
            access_token,
            workspace_id: None,
            created_at: now,
            updated_at: now,
            last_synced_at: None,
        }
    }

    /// Full name in `owner/name` format.
    pub fn full_name(&self) -> String {
        format!("{}/{}", self.owner, self.name)
    }

    /// Mark the repo as synced with the Fluid Remote.
    pub fn mark_synced(&mut self) {
        self.last_synced_at = Some(Utc::now());
        if self.status == RepoConnectionStatus::Syncing {
            self.status = RepoConnectionStatus::Connected;
        }
        self.updated_at = Utc::now();
    }

    /// Set the Fluid Remote URL after creating the mirror.
    pub fn set_fluid_remote(&mut self, url: &str) {
        self.fluid_remote_url = Some(url.to_string());
        self.updated_at = Utc::now();
    }

    /// Update metadata from GitHub API response.
    pub fn update_from_github(
        &mut self,
        description: Option<String>,
        language: Option<String>,
        stars: u64,
        forks: u64,
    ) {
        self.description = description;
        self.language = language;
        self.stars = stars;
        self.forks = forks;
        self.updated_at = Utc::now();
    }
}

/// Summary view of a LinkedRepo (for list displays).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkedRepoSlim {
    pub id: RepoId,
    pub owner: String,
    pub name: String,
    pub full_name: String,
    pub default_branch: String,
    pub status: RepoConnectionStatus,
    pub language: Option<String>,
    pub is_private: bool,
}

impl From<LinkedRepo> for LinkedRepoSlim {
    fn from(r: LinkedRepo) -> Self {
        let full_name = format!("{}/{}", r.owner, r.name);
        Self {
            id: r.id,
            owner: r.owner,
            name: r.name,
            full_name,
            default_branch: r.default_branch,
            status: r.status,
            language: r.language,
            is_private: r.is_private,
        }
    }
}
