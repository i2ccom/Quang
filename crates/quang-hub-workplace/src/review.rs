//! Review — an approval gate for tasks, changes, and milestones.
//!
//! Reviews provide a structured feedback and approval workflow.
//! They can be for code changes, task completion, milestone gates, or agent outputs.
//! Reviews support approval chains (single, multi-stage, or hierarchical).

use serde::{Deserialize, Serialize};

use crate::graph::{ActorId, NodeId, Timestamp, now};

pub type ReviewId = NodeId;

/// What type of thing is being reviewed.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewTargetKind {
    /// Review of a completed Task
    TaskCompletion,
    /// Code review / patch review
    CodeChange,
    /// Milestone gate review
    Milestone,
    /// Agent-generated output review
    AgentOutput,
    /// Document or spec review
    Document,
    /// Custom review type
    Custom(String),
}

impl std::fmt::Display for ReviewTargetKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewTargetKind::TaskCompletion => write!(f, "task_completion"),
            ReviewTargetKind::CodeChange => write!(f, "code_change"),
            ReviewTargetKind::Milestone => write!(f, "milestone"),
            ReviewTargetKind::AgentOutput => write!(f, "agent_output"),
            ReviewTargetKind::Document => write!(f, "document"),
            ReviewTargetKind::Custom(s) => write!(f, "custom_{}", s),
        }
    }
}

/// The state of a review.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReviewStatus {
    Pending,
    InProgress,
    Approved,
    ChangesRequested,
    Rejected,
    Cancelled,
}

impl std::fmt::Display for ReviewStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ReviewStatus::Pending => write!(f, "pending"),
            ReviewStatus::InProgress => write!(f, "in_progress"),
            ReviewStatus::Approved => write!(f, "approved"),
            ReviewStatus::ChangesRequested => write!(f, "changes_requested"),
            ReviewStatus::Rejected => write!(f, "rejected"),
            ReviewStatus::Cancelled => write!(f, "cancelled"),
        }
    }
}

/// A single review comment or feedback item.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewComment {
    pub id: String,
    pub author: ActorId,
    pub body: String,
    /// Optional line/field reference (e.g., code line number or form field name)
    pub location: Option<String>,
    pub is_resolved: bool,
    pub created_at: Timestamp,
    pub resolved_at: Option<Timestamp>,
}

impl ReviewComment {
    pub fn new(author: ActorId, body: &str) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            author,
            body: body.to_string(),
            location: None,
            is_resolved: false,
            created_at: now(),
            resolved_at: None,
        }
    }

    pub fn at_location(mut self, location: &str) -> Self {
        self.location = Some(location.to_string());
        self
    }

    pub fn resolve(&mut self) {
        self.is_resolved = true;
        self.resolved_at = Some(now());
    }
}

/// A structured approval gate for a task, change, or milestone.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub id: ReviewId,
    pub title: String,
    pub target_kind: ReviewTargetKind,
    /// The NodeId of the thing being reviewed (Task, etc.)
    pub target_id: NodeId,
    pub status: ReviewStatus,
    pub reviewers: Vec<ActorId>,
    pub required_approvals: u32,
    pub approvals_received: u32,
    pub comments: Vec<ReviewComment>,
    pub created_by: ActorId,
    pub created_at: Timestamp,
    pub updated_at: Timestamp,
    pub completed_at: Option<Timestamp>,
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

impl Review {
    pub fn new(
        title: &str,
        target_kind: ReviewTargetKind,
        target_id: NodeId,
        created_by: ActorId,
    ) -> Self {
        Self {
            id: NodeId::new("rev"),
            title: title.to_string(),
            target_kind,
            target_id,
            status: ReviewStatus::Pending,
            reviewers: Vec::new(),
            required_approvals: 1,
            approvals_received: 0,
            comments: Vec::new(),
            created_by,
            created_at: now(),
            updated_at: now(),
            completed_at: None,
            metadata: serde_json::Map::new(),
        }
    }

    pub fn add_reviewer(&mut self, reviewer: ActorId) {
        if !self.reviewers.contains(&reviewer) {
            self.reviewers.push(reviewer);
        }
        self.updated_at = now();
    }

    pub fn add_comment(&mut self, comment: ReviewComment) {
        self.comments.push(comment);
        self.updated_at = now();
        if self.status == ReviewStatus::Pending {
            self.status = ReviewStatus::InProgress;
        }
    }

    pub fn approve(&mut self, reviewer: &ActorId) -> Result<(), ReviewError> {
        if !self.reviewers.contains(reviewer) {
            return Err(ReviewError::NotAReviewer(reviewer.to_string()));
        }
        self.approvals_received += 1;
        self.updated_at = now();

        if self.approvals_received >= self.required_approvals {
            self.status = ReviewStatus::Approved;
            self.completed_at = Some(now());
        }
        Ok(())
    }

    pub fn request_changes(&mut self, reviewer: &ActorId, comment: &str) -> Result<(), ReviewError> {
        if !self.reviewers.contains(reviewer) {
            return Err(ReviewError::NotAReviewer(reviewer.to_string()));
        }
        self.status = ReviewStatus::ChangesRequested;
        self.comments.push(ReviewComment::new(reviewer.clone(), comment));
        self.updated_at = now();
        Ok(())
    }

    pub fn reject(&mut self, reviewer: &ActorId, reason: &str) -> Result<(), ReviewError> {
        if !self.reviewers.contains(reviewer) {
            return Err(ReviewError::NotAReviewer(reviewer.to_string()));
        }
        self.status = ReviewStatus::Rejected;
        self.completed_at = Some(now());
        self.comments.push(ReviewComment::new(reviewer.clone(), reason));
        self.updated_at = now();
        Ok(())
    }

    pub fn reset_for_rework(&mut self) {
        self.status = ReviewStatus::Pending;
        self.approvals_received = 0;
        self.completed_at = None;
        self.updated_at = now();
    }

    pub fn kind() -> &'static str {
        "review"
    }
}

#[derive(Debug, Clone, thiserror::Error)]
pub enum ReviewError {
    #[error("Actor {0} is not a reviewer for this review")]
    NotAReviewer(String),
}
