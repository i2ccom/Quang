//! RepoBranch — branch information for a linked repository.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A branch in a linked repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoBranch {
    /// Branch name (e.g. "main", "feature/xyz")
    pub name: String,
    /// SHA of the latest commit on this branch
    pub sha: String,
    /// Full commit SHA of the latest commit
    pub commit_sha: String,
    /// URL to view the branch on GitHub
    pub url: String,
    /// Whether this is the default branch
    pub is_default: bool,
    /// Whether this branch is protected
    pub is_protected: bool,
    /// Latest commit message
    pub last_commit_message: Option<String>,
    /// Latest commit author
    pub last_commit_author: Option<String>,
    /// Timestamp of the latest commit
    pub last_commit_at: Option<DateTime<Utc>>,
    /// Number of commits ahead of the default branch
    pub ahead_by: Option<i64>,
    /// Number of commits behind the default branch
    pub behind_by: Option<i64>,
}

impl RepoBranch {
    /// Create a new branch entry.
    pub fn new(name: &str, sha: &str, is_default: bool) -> Self {
        Self {
            name: name.to_string(),
            sha: sha.to_string(),
            commit_sha: sha.to_string(),
            url: String::new(),
            is_default,
            is_protected: false,
            last_commit_message: None,
            last_commit_author: None,
            last_commit_at: None,
            ahead_by: None,
            behind_by: None,
        }
    }

    /// Build the GitHub URL for this branch.
    pub fn with_github_url(mut self, owner: &str, repo: &str) -> Self {
        self.url = format!("https://github.com/{}/{}/tree/{}", owner, repo, self.name);
        self
    }
}

/// A paginated list of branches.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoBranchList {
    pub branches: Vec<RepoBranch>,
    pub total_count: usize,
    pub page: usize,
    pub per_page: usize,
}
