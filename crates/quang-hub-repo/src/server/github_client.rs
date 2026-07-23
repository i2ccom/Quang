//! GitHub API client — octocrab-style GitHub API client for repo operations.

use serde::{Deserialize, Serialize};
use worker::*;

/// A lightweight GitHub API client for Cloudflare Workers.
///
/// Uses `reqwest` under the hood and injects OAuth tokens for authenticated
/// requests. Designed to mirror the octocrab API surface for familiarity.
pub struct GitHubClient {
    base_url: String,
    token: String,
    client: reqwest::Client,
}

impl GitHubClient {
    /// Create a new GitHub API client with the given OAuth token.
    pub fn new(token: &str) -> Self {
        Self {
            base_url: "https://api.github.com".to_string(),
            token: token.to_string(),
            client: reqwest::Client::new(),
        }
    }

    /// Set a custom base URL (for testing or enterprise GitHub).
    pub fn with_base_url(mut self, url: &str) -> Self {
        self.base_url = url.to_string();
        self
    }

    /// GET a path from the GitHub API.
    pub async fn get(&self, path: &str) -> Result<serde_json::Value> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let response = self
            .client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "QuangHub/1.0")
            .header("Accept", "application/vnd.github.v3+json")
            .send()
            .await
            .map_err(|e| Error::from(format!("GitHub API error: {}", e)))?;

        let status = response.status();
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::from(format!("Failed to parse response: {}", e)))?;

        if !status.is_success() {
            return Err(Error::from(format!(
                "GitHub API returned {}: {}",
                status,
                body.to_string()
            )));
        }

        Ok(body)
    }

    /// POST to the GitHub API with a JSON body.
    pub async fn post(&self, path: &str, body: &serde_json::Value) -> Result<serde_json::Value> {
        let url = format!("{}/{}", self.base_url, path.trim_start_matches('/'));
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.token))
            .header("User-Agent", "QuangHub/1.0")
            .header("Accept", "application/vnd.github.v3+json")
            .json(body)
            .send()
            .await
            .map_err(|e| Error::from(format!("GitHub API error: {}", e)))?;

        let status = response.status();
        let body: serde_json::Value = response
            .json()
            .await
            .map_err(|e| Error::from(format!("Failed to parse response: {}", e)))?;

        if !status.is_success() {
            return Err(Error::from(format!(
                "GitHub API returned {}: {}",
                status,
                body.to_string()
            )));
        }

        Ok(body)
    }

    /// Get repository information.
    pub async fn get_repo(&self, owner: &str, repo: &str) -> Result<GitHubRepoResponse> {
        let path = format!("repos/{}/{}", owner, repo);
        let response = self.get(&path).await?;
        serde_json::from_value(response)
            .map_err(|e| Error::from(format!("Failed to parse repo response: {}", e)))
    }

    /// Get repository file contents.
    pub async fn get_contents(
        &self,
        owner: &str,
        repo: &str,
        path: &str,
        branch: &str,
    ) -> Result<serde_json::Value> {
        let api_path = format!("repos/{}/{}/contents/{}", owner, repo, path);
        let url = format!("{}?ref={}", api_path, branch);
        self.get(&url).await
    }

    /// Get the file tree (recursive) for a repo.
    pub async fn get_tree(
        &self,
        owner: &str,
        repo: &str,
        branch_sha: &str,
        recursive: bool,
    ) -> Result<serde_json::Value> {
        let api_path = format!(
            "repos/{}/{}/git/trees/{}",
            owner, repo, branch_sha
        );
        let url = if recursive {
            format!("{}?recursive=1", api_path)
        } else {
            api_path
        };
        self.get(&url).await
    }

    /// Get commits for a repo.
    pub async fn get_commits(
        &self,
        owner: &str,
        repo: &str,
        branch: &str,
        per_page: u32,
    ) -> Result<serde_json::Value> {
        let path = format!(
            "repos/{}/{}/commits?sha={}&per_page={}",
            owner, repo, branch, per_page
        );
        self.get(&path).await
    }

    /// Get branches for a repo.
    pub async fn get_branches(
        &self,
        owner: &str,
        repo: &str,
    ) -> Result<serde_json::Value> {
        let path = format!("repos/{}/{}/branches", owner, repo);
        self.get(&path).await
    }

    /// Create a webhook on the repository.
    pub async fn create_webhook(
        &self,
        owner: &str,
        repo: &str,
        webhook_url: &str,
        secret: &str,
        events: &[String],
    ) -> Result<serde_json::Value> {
        let body = serde_json::json!({
            "name": "web",
            "active": true,
            "events": events,
            "config": {
                "url": webhook_url,
                "content_type": "json",
                "secret": secret,
                "insecure_ssl": "0"
            }
        });

        let path = format!("repos/{}/{}/hooks", owner, repo);
        self.post(&path, &body).await
    }
}

/// Response from the GitHub API for a single repository.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitHubRepoResponse {
    pub id: u64,
    pub node_id: String,
    pub name: String,
    pub full_name: String,
    pub private: bool,
    pub description: Option<String>,
    pub fork: bool,
    pub url: String,
    pub html_url: String,
    pub clone_url: String,
    pub default_branch: String,
    pub language: Option<String>,
    pub stargazers_count: u64,
    pub forks_count: u64,
    pub open_issues_count: u64,
    pub topics: Vec<String>,
    pub visibility: String,
    pub archived: bool,
    pub disabled: bool,
}
