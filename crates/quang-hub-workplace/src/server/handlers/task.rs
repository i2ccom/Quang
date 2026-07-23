//! Task REST handlers — CRUD and state machine transitions for Task entities.

use serde::Deserialize;
use worker::*;

/// Request body for creating a task.
#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub title: String,
    pub description: String,
    pub priority: Option<String>,
    pub assignee: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Request body for updating a task.
#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub priority: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// Request body for transitioning task status.
#[derive(Debug, Deserialize)]
pub struct TransitionRequest {
    pub status: String,
}

/// Request body for assigning a task.
#[derive(Debug, Deserialize)]
pub struct AssignRequest {
    pub assignee: String,
}

/// List tasks in a project.
pub async fn list(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _proj_id = req.param("proj_id").unwrap_or("");

    let tasks = serde_json::json!([
        {
            "id": "task_001",
            "title": "Implement OAuth flow",
            "description": "Wire up Google and GitHub OAuth login callbacks",
            "status": "in_progress",
            "priority": "high",
            "assignee": "human:alice",
            "created_by": "human:bob",
            "tags": ["auth", "backend", "security"],
            "estimated_hours": 8.0,
            "spent_hours": 4.5,
            "due_date": "2026-06-01T00:00:00Z",
            "created_at": "2026-05-20T00:00:00Z",
            "updated_at": "2026-05-28T10:00:00Z"
        },
        {
            "id": "task_002",
            "title": "Design landing page hero",
            "description": "Create the hero section with animated gradient background",
            "status": "backlog",
            "priority": "medium",
            "assignee": null,
            "created_by": "human:carol",
            "tags": ["design", "frontend"],
            "estimated_hours": 6.0,
            "spent_hours": 0.0,
            "due_date": null,
            "created_at": "2026-05-25T00:00:00Z",
            "updated_at": "2026-05-25T00:00:00Z"
        },
        {
            "id": "task_003",
            "title": "Build GraphQL schema for tasks",
            "description": "Define task CRUD operations in the GraphQL schema",
            "status": "done",
            "priority": "high",
            "assignee": "human:bob",
            "created_by": "human:alice",
            "tags": ["graphql", "api"],
            "estimated_hours": 4.0,
            "spent_hours": 5.0,
            "due_date": "2026-05-28T00:00:00Z",
            "created_at": "2026-05-18T00:00:00Z",
            "completed_at": "2026-05-27T15:30:00Z",
            "updated_at": "2026-05-27T15:30:00Z"
        }
    ]);

    Response::ok(serde_json::to_string(&tasks).unwrap())
}

/// Create a new task.
pub async fn create(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _proj_id = req.param("proj_id").unwrap_or("");
    let body: CreateTaskRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("task_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let task = serde_json::json!({
        "id": id,
        "title": body.title,
        "description": body.description,
        "status": "backlog",
        "priority": body.priority.unwrap_or_else(|| "medium".to_string()),
        "assignee": body.assignee,
        "created_by": "human:current_user",
        "tags": body.tags.unwrap_or_default(),
        "estimated_hours": null,
        "spent_hours": null,
        "due_date": null,
        "completed_at": null,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&task).unwrap())
}

/// Get a specific task by ID.
pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let task = serde_json::json!({
        "id": id,
        "title": "Sample Task",
        "description": "A sample task for demonstration",
        "status": "in_progress",
        "priority": "medium",
        "assignee": "human:demo-user",
        "created_by": "human:admin",
        "tags": ["demo"],
        "estimated_hours": 4.0,
        "spent_hours": 2.0,
        "due_date": "2026-06-15T00:00:00Z",
        "completed_at": null,
        "created_at": "2026-05-01T00:00:00Z",
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&task).unwrap())
}

/// Update a task.
pub async fn update(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: UpdateTaskRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    tracing::info!("Updating task {:?}: {:?}", _id, body);
    Response::ok(r#"{"status":"updated"}"#)
}

/// Transition a task to a new status.
pub async fn transition(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: TransitionRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let valid_statuses = [
        "backlog",
        "ready",
        "in_progress",
        "in_review",
        "changes_requested",
        "done",
        "cancelled",
        "blocked",
    ];

    if !valid_statuses.contains(&body.status.as_str()) {
        return Response::error(
            &format!(r#"{{"error":"Invalid status: {}"}}"#, body.status),
            400,
        );
    }

    let response = serde_json::json!({
        "id": _id,
        "status": body.status,
        "updated_at": chrono::Utc::now().to_rfc3339(),
        "completed_at": if body.status == "done" {
            Some(chrono::Utc::now().to_rfc3339())
        } else {
            None
        }
    });

    Response::ok(serde_json::to_string(&response).unwrap())
}

/// Assign a task to an actor.
pub async fn assign(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: AssignRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let response = serde_json::json!({
        "id": _id,
        "assignee": body.assignee,
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&response).unwrap())
}
