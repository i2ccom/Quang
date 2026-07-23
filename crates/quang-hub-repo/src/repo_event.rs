//! RepoEvent — typed events for repository mutations and changes.
//!
//! Every mutation to a LinkedRepo emits a `RepoEvent` which propagates
//! via the QuangHub event bus to update the UI, trigger Fluid Remote syncs,
//! fire webhooks, and kick off agent tasks.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::linked_repo::RepoId;
use crate::q_task::QTaskId;

pub type RepoEventId = String;

/// All events that can occur on linked repositories.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "data")]
pub enum RepoEvent {
    // ── Repo lifecycle ──
    /// A repo was linked to QuangHub
    RepoLinked {
        repo_id: RepoId,
        owner: String,
        name: String,
    },
    /// A repo was unlinked from QuangHub
    RepoUnlinked { repo_id: RepoId },
    /// Repo metadata was updated (stars, description, etc.)
    RepoUpdated { repo_id: RepoId },
    /// Repo connection status changed
    RepoStatusChanged {
        repo_id: RepoId,
        old_status: String,
        new_status: String,
    },

    // ── Sync events ──
    /// Fluid Remote sync started
    FluidSyncStarted { repo_id: RepoId },
    /// Fluid Remote sync completed
    FluidSyncCompleted { repo_id: RepoId, commit_count: u64 },
    /// Fluid Remote sync failed
    FluidSyncFailed { repo_id: RepoId, error: String },

    // ── Branch / Push events ──
    /// A push event was received (via webhook or poll)
    PushReceived {
        repo_id: RepoId,
        branch: String,
        commit_sha: String,
        pusher: String,
        commit_count: u64,
    },
    /// A new branch was created
    BranchCreated { repo_id: RepoId, branch: String },
    /// A branch was deleted
    BranchDeleted { repo_id: RepoId, branch: String },
    /// A tag was created
    TagCreated { repo_id: RepoId, tag: String },

    // ── Pull Request events ──
    /// A PR was opened
    PullRequestOpened {
        repo_id: RepoId,
        pr_number: u64,
        title: String,
        author: String,
    },
    /// A PR was merged
    PullRequestMerged {
        repo_id: RepoId,
        pr_number: u64,
        branch: String,
        merged_by: String,
    },
    /// A PR was closed without merging
    PullRequestClosed { repo_id: RepoId, pr_number: u64 },

    // ── Webhook events ──
    /// Webhook was registered with GitHub
    WebhookRegistered { repo_id: RepoId, webhook_id: u64 },
    /// Webhook received a ping
    WebhookPing { repo_id: RepoId },

    // ── Deploy events ──
    /// Auto-deploy started
    DeployStarted {
        repo_id: RepoId,
        branch: String,
        commit_sha: String,
    },
    /// Auto-deploy completed
    DeployCompleted {
        repo_id: RepoId,
        branch: String,
        deploy_url: String,
    },
    /// Auto-deploy failed
    DeployFailed {
        repo_id: RepoId,
        branch: String,
        error: String,
    },

    // ── QTask events ──
    /// A QTask was created for this repo
    QTaskCreated {
        repo_id: RepoId,
        task_id: QTaskId,
        title: String,
    },
    /// A QTask changed state
    QTaskStateChanged {
        repo_id: RepoId,
        task_id: QTaskId,
        old_state: String,
        new_state: String,
    },
    /// A QTask completed
    QTaskCompleted {
        repo_id: RepoId,
        task_id: QTaskId,
        commit_sha: Option<String>,
    },

    // ── Adaptive app events ──
    /// A qh.app manifest was discovered or updated
    AppManifestUpdated {
        repo_id: RepoId,
        app_id: String,
        app_name: String,
    },
}

impl RepoEvent {
    /// Human-readable description of the event.
    pub fn description(&self) -> String {
        match self {
            RepoEvent::RepoLinked { owner, name, .. } => {
                format!("Repo {}/{} linked", owner, name)
            }
            RepoEvent::RepoUnlinked { repo_id } => format!("Repo {} unlinked", repo_id),
            RepoEvent::RepoUpdated { repo_id } => format!("Repo {} updated", repo_id),
            RepoEvent::RepoStatusChanged {
                repo_id,
                new_status,
                ..
            } => format!("Repo {} status: {}", repo_id, new_status),
            RepoEvent::FluidSyncStarted { repo_id } => {
                format!("Fluid sync started for repo {}", repo_id)
            }
            RepoEvent::FluidSyncCompleted {
                repo_id,
                commit_count,
                ..
            } => format!(
                "Fluid sync completed for repo {} ({} commits)",
                repo_id, commit_count
            ),
            RepoEvent::FluidSyncFailed { repo_id, error } => {
                format!("Fluid sync failed for repo {}: {}", repo_id, error)
            }
            RepoEvent::PushReceived {
                repo_id, branch, ..
            } => format!("Push to {} on {}", repo_id, branch),
            RepoEvent::BranchCreated { repo_id, branch } => {
                format!("Branch {} created in {}", branch, repo_id)
            }
            RepoEvent::BranchDeleted { repo_id, branch } => {
                format!("Branch {} deleted in {}", branch, repo_id)
            }
            RepoEvent::TagCreated { repo_id, tag } => format!("Tag {} created in {}", tag, repo_id),
            RepoEvent::PullRequestOpened {
                repo_id, pr_number, ..
            } => format!("PR #{} opened in {}", pr_number, repo_id),
            RepoEvent::PullRequestMerged {
                repo_id, pr_number, ..
            } => format!("PR #{} merged in {}", pr_number, repo_id),
            RepoEvent::PullRequestClosed { repo_id, pr_number } => {
                format!("PR #{} closed in {}", pr_number, repo_id)
            }
            RepoEvent::WebhookRegistered { repo_id, .. } => {
                format!("Webhook registered for repo {}", repo_id)
            }
            RepoEvent::WebhookPing { repo_id } => {
                format!("Webhook ping received for repo {}", repo_id)
            }
            RepoEvent::DeployStarted {
                repo_id, branch, ..
            } => format!("Deploy started for {} ({})", repo_id, branch),
            RepoEvent::DeployCompleted {
                repo_id, branch, ..
            } => format!("Deploy completed for {} ({})", repo_id, branch),
            RepoEvent::DeployFailed {
                repo_id,
                branch,
                error,
            } => format!("Deploy failed for {} ({}): {}", repo_id, branch, error),
            RepoEvent::QTaskCreated { repo_id, title, .. } => {
                format!("QTask \"{}\" created in {}", title, repo_id)
            }
            RepoEvent::QTaskStateChanged {
                repo_id,
                task_id,
                new_state,
                ..
            } => format!("QTask {} in repo {} is now {}", task_id, repo_id, new_state),
            RepoEvent::QTaskCompleted {
                repo_id, task_id, ..
            } => {
                format!("QTask {} completed in repo {}", task_id, repo_id)
            }
            RepoEvent::AppManifestUpdated {
                repo_id, app_name, ..
            } => format!("App \"{}\" updated in repo {}", app_name, repo_id),
        }
    }
}

/// An event envelope with metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoEventEnvelope {
    pub id: RepoEventId,
    pub event: RepoEvent,
    pub timestamp: DateTime<Utc>,
    pub sequence: u64,
}

impl RepoEventEnvelope {
    pub fn new(event: RepoEvent, sequence: u64) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            event,
            timestamp: Utc::now(),
            sequence,
        }
    }
}

/// A simple event bus for repo events.
#[derive(Debug, Clone)]
pub struct RepoEventBus {
    sequence: u64,
    pub history: Vec<RepoEventEnvelope>,
}

impl Default for RepoEventBus {
    fn default() -> Self {
        Self::new()
    }
}

impl RepoEventBus {
    pub fn new() -> Self {
        Self {
            sequence: 0,
            history: Vec::new(),
        }
    }

    /// Emit a repo event and return the envelope.
    pub fn emit(&mut self, event: RepoEvent) -> RepoEventEnvelope {
        self.sequence += 1;
        let envelope = RepoEventEnvelope::new(event, self.sequence);
        self.history.push(envelope.clone());
        envelope
    }

    /// Get all events since a given sequence number.
    pub fn since(&self, sequence: u64) -> Vec<&RepoEventEnvelope> {
        self.history
            .iter()
            .filter(|e| e.sequence > sequence)
            .collect()
    }

    /// Get events filtered by repo ID.
    pub fn for_repo(&self, repo_id: &RepoId) -> Vec<&RepoEventEnvelope> {
        self.history
            .iter()
            .filter(
                |e| matches!(&e.event, RepoEvent::RepoLinked { repo_id: id, .. } if id == repo_id),
            )
            .collect()
    }
}
