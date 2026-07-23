//! RepoCommit — commit information for a linked repository.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// A commit in a linked repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoCommit {
    /// Full commit SHA
    pub sha: String,
    /// Short commit SHA (first 7 characters)
    pub short_sha: String,
    /// Commit message (full)
    pub message: String,
    /// Commit message subject (first line)
    pub subject: String,
    /// Commit author name
    pub author_name: String,
    /// Commit author email
    pub author_email: Option<String>,
    /// Committer name (may differ from author)
    pub committer_name: Option<String>,
    /// Author GitHub avatar URL
    pub author_avatar: Option<String>,
    /// Commit timestamp
    pub authored_at: DateTime<Utc>,
    /// Commit timestamp (when it was added to the tree)
    pub committed_at: DateTime<Utc>,
    /// GitHub URL for this commit
    pub url: String,
    /// Number of files changed
    pub files_changed: Option<u64>,
    /// Number of additions
    pub additions: Option<u64>,
    /// Number of deletions
    pub deletions: Option<u64>,
    /// Whether this commit is a verified signed commit
    pub is_verified: bool,
    /// Parent commit SHAs
    pub parents: Vec<String>,
}

impl RepoCommit {
    /// Create a new commit from API data.
    pub fn new(sha: &str, message: &str, author_name: &str, authored_at: DateTime<Utc>) -> Self {
        let subject = message.lines().next().unwrap_or("").to_string();
        Self {
            sha: sha.to_string(),
            short_sha: sha.chars().take(7).collect(),
            message: message.to_string(),
            subject,
            author_name: author_name.to_string(),
            author_email: None,
            committer_name: None,
            author_avatar: None,
            authored_at,
            committed_at: authored_at,
            url: String::new(),
            files_changed: None,
            additions: None,
            deletions: None,
            is_verified: false,
            parents: Vec::new(),
        }
    }

    /// Set GitHub URL for this commit.
    pub fn with_github_url(mut self, owner: &str, repo: &str) -> Self {
        self.url = format!("https://github.com/{}/{}/commit/{}", owner, repo, self.sha);
        self
    }

    /// Time since this commit was authored, as a human-readable string.
    pub fn time_ago(&self) -> String {
        let now = Utc::now();
        let duration = now - self.authored_at;
        let seconds = duration.num_seconds().abs();

        if seconds < 60 {
            format!("{}s ago", seconds)
        } else if seconds < 3600 {
            format!("{}m ago", seconds / 60)
        } else if seconds < 86400 {
            format!("{}h ago", seconds / 3600)
        } else if seconds < 2592000 {
            format!("{}d ago", seconds / 86400)
        } else if seconds < 31536000 {
            format!("{}mo ago", seconds / 2592000)
        } else {
            format!("{}y ago", seconds / 31536000)
        }
    }
}

/// A paginated list of commits.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoCommitList {
    pub commits: Vec<RepoCommit>,
    pub total_count: usize,
    pub page: usize,
    pub per_page: usize,
    pub branch: String,
}
