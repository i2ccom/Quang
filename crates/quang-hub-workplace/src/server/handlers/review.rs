//! Review REST handlers — CRUD, approval, and commenting for Review entities.

use serde::Deserialize;
use worker::*;

/// Request body for creating a review.
#[derive(Debug, Deserialize)]
pub struct CreateReviewRequest {
    pub title: String,
    pub target_kind: Option<String>,
}

/// Request body for requesting changes.
#[derive(Debug, Deserialize)]
pub struct RequestChangesRequest {
    pub comment: String,
}

/// Request body for rejecting a review.
#[derive(Debug, Deserialize)]
pub struct RejectRequest {
    pub reason: String,
}

/// Request body for adding a comment.
#[derive(Debug, Deserialize)]
pub struct AddCommentRequest {
    pub body: String,
    pub location: Option<String>,
}

/// List all reviews.
pub async fn list(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let reviews = serde_json::json!([
        {
            "id": "rev_001",
            "title": "Review: Task status transitions",
            "target_kind": "code_change",
            "target_id": "task_001",
            "status": "pending",
            "reviewers": ["human:carol", "human:dave"],
            "required_approvals": 2,
            "approvals_received": 0,
            "comment_count": 0,
            "created_by": "human:bob",
            "created_at": "2026-05-27T14:00:00Z"
        },
        {
            "id": "rev_002",
            "title": "Review: OAuth flow implementation",
            "target_kind": "code_change",
            "target_id": "task_002",
            "status": "in_progress",
            "reviewers": ["human:carol", "human:dave"],
            "required_approvals": 2,
            "approvals_received": 1,
            "comment_count": 3,
            "created_by": "human:alice",
            "created_at": "2026-05-27T15:00:00Z"
        },
        {
            "id": "rev_003",
            "title": "Review: Landing page design",
            "target_kind": "document",
            "target_id": "task_003",
            "status": "approved",
            "reviewers": ["human:alice"],
            "required_approvals": 1,
            "approvals_received": 1,
            "comment_count": 2,
            "created_by": "human:carol",
            "created_at": "2026-05-26T10:00:00Z"
        }
    ]);

    Response::ok(serde_json::to_string(&reviews).unwrap())
}

/// Create a new review for a target (task, document, etc.).
pub async fn create(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let target_id = req.param("target_id").unwrap_or("");
    let body: CreateReviewRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("rev_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let review = serde_json::json!({
        "id": id,
        "title": body.title,
        "target_kind": body.target_kind.unwrap_or_else(|| "task_completion".to_string()),
        "target_id": target_id,
        "status": "pending",
        "reviewers": [],
        "required_approvals": 1,
        "approvals_received": 0,
        "comments": [],
        "created_by": "human:current_user",
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&review).unwrap())
}

/// Get a specific review by ID.
pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let review = serde_json::json!({
        "id": id,
        "title": "Code Review",
        "target_kind": "code_change",
        "target_id": "task_demo",
        "status": "in_progress",
        "reviewers": ["human:reviewer1", "human:reviewer2"],
        "required_approvals": 2,
        "approvals_received": 1,
        "comments": [
            {"id": "c1", "author": "human:reviewer1", "body": "Looks good! One minor nit on line 42.", "location": "42", "is_resolved": false, "created_at": "2026-05-28T09:00:00Z"}
        ],
        "created_by": "human:author",
        "created_at": "2026-05-27T00:00:00Z",
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&review).unwrap())
}

/// Approve a review.
pub async fn approve(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");

    let response = serde_json::json!({
        "id": _id,
        "status": "approved",
        "completed_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&response).unwrap())
}

/// Request changes on a review.
pub async fn request_changes(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: RequestChangesRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let response = serde_json::json!({
        "id": _id,
        "status": "changes_requested",
        "comment": body.comment,
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&response).unwrap())
}

/// Reject a review.
pub async fn reject(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: RejectRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let response = serde_json::json!({
        "id": _id,
        "status": "rejected",
        "reason": body.reason,
        "completed_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&response).unwrap())
}

/// Add a comment to a review.
pub async fn add_comment(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: AddCommentRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let comment = serde_json::json!({
        "id": format!("c_{}", uuid::Uuid::new_v4().to_string().replace('-', "")),
        "review_id": _id,
        "author": "human:current_user",
        "body": body.body,
        "location": body.location,
        "is_resolved": false,
        "created_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&comment).unwrap())
}
