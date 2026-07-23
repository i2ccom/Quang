//! RepoSettings — mirror, auto-deploy, and webhook configuration for a linked repo.

use serde::{Deserialize, Serialize};

/// Configuration for a linked repository's mirroring and auto-deploy behavior.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoSettings {
    /// Whether to mirror this repo to the Fluid Remote
    pub mirror_enabled: bool,
    /// Sync frequency in minutes (0 = real-time via webhook)
    pub mirror_interval_minutes: u32,
    /// Whether to enable auto-deploy to Cloudflare Pages
    pub auto_deploy_enabled: bool,
    /// Auto-deploy branch pattern (glob, e.g. "main" or "release/*")
    pub auto_deploy_branch: String,
    /// Cloudflare Pages project name
    pub pages_project_name: Option<String>,
    /// Cloudflare account ID
    pub pages_account_id: Option<String>,
    /// Webhook configuration
    pub webhook: WebhookConfig,
    /// Whether to enable pre-commit analysis by AI agents
    pub pre_commit_analysis: bool,
    /// Whether to enable post-merge summaries
    pub post_merge_summary: bool,
    /// Whether to enable Dependabot-style dependency scanning
    pub dependency_scanning: bool,
    /// Branch protection rules (custom config)
    pub branch_protection: BranchProtection,
    /// Custom environment variables injected into auto-deploy builds
    pub env_vars: Vec<EnvVar>,
}

impl Default for RepoSettings {
    fn default() -> Self {
        Self {
            mirror_enabled: true,
            mirror_interval_minutes: 5,
            auto_deploy_enabled: false,
            auto_deploy_branch: "main".to_string(),
            pages_project_name: None,
            pages_account_id: None,
            webhook: WebhookConfig::default(),
            pre_commit_analysis: false,
            post_merge_summary: false,
            dependency_scanning: false,
            branch_protection: BranchProtection::default(),
            env_vars: Vec::new(),
        }
    }
}

/// Webhook configuration for the linked repo.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Whether the webhook is active
    pub enabled: bool,
    /// GitHub webhook secret
    pub secret: Option<String>,
    /// GitHub webhook ID (for management)
    pub github_webhook_id: Option<u64>,
    /// The QuangHub webhook endpoint URL
    pub webhook_url: Option<String>,
    /// Events to subscribe to
    pub events: Vec<String>,
}

impl Default for WebhookConfig {
    fn default() -> Self {
        Self {
            enabled: true,
            secret: None,
            github_webhook_id: None,
            webhook_url: None,
            events: vec![
                "push".to_string(),
                "pull_request".to_string(),
                "create".to_string(),
            ],
        }
    }
}

/// Branch protection rules.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BranchProtection {
    /// Branches matching this pattern are protected
    pub pattern: String,
    /// Require pull request reviews before merging
    pub require_reviews: bool,
    /// Number of required reviewers
    pub required_reviewers: u32,
    /// Dismiss stale reviews when new commits are pushed
    pub dismiss_stale_reviews: bool,
    /// Require status checks to pass before merging
    pub require_status_checks: bool,
    /// List of required status check names
    pub required_status_checks: Vec<String>,
    /// Require branches to be up to date before merging
    pub require_up_to_date: bool,
    /// Do not allow bypassing the above settings
    pub enforce_for_admins: bool,
}

impl Default for BranchProtection {
    fn default() -> Self {
        Self {
            pattern: "main".to_string(),
            require_reviews: false,
            required_reviewers: 1,
            dismiss_stale_reviews: false,
            require_status_checks: false,
            required_status_checks: Vec::new(),
            require_up_to_date: false,
            enforce_for_admins: false,
        }
    }
}

/// An environment variable for auto-deploy builds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVar {
    pub key: String,
    pub value: String,
    /// Whether this value is encrypted
    pub is_secret: bool,
}
