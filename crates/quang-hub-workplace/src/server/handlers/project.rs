//! Project REST handlers — CRUD for Project entities.

use serde::Deserialize;
use worker::*;

use crate::store::WorkplaceStore;

/// Request body for creating a project.
#[derive(Debug, Deserialize)]
pub struct CreateProjectRequest {
    pub name: String,
    pub description: String,
    pub priority: Option<String>,
}

/// Request body for updating a project.
#[derive(Debug, Deserialize)]
pub struct UpdateProjectRequest {
    pub name: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
    pub priority: Option<String>,
}

/// List projects in a workspace.
pub async fn list(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _ws_id = req.param("ws_id").unwrap_or("");

    let projects = serde_json::json!([
        {
            "id": "proj_001",
            "name": "QuangHub MVP",
            "description": "Minimum viable product for the QuangHub collaboration platform",
            "status": "active",
            "priority": "high",
            "owner": "human:alice",
            "task_count": 12,
            "completed_task_count": 5,
            "progress": 0.42,
            "start_date": "2026-04-01T00:00:00Z",
            "end_date": "2026-07-01T00:00:00Z",
            "created_at": "2026-04-01T00:00:00Z"
        },
        {
            "id": "proj_002",
            "name": "Agent Integration Suite",
            "description": "AI agent integration framework and SDK",
            "status": "planning",
            "priority": "medium",
            "owner": "human:bob",
            "task_count": 8,
            "completed_task_count": 0,
            "progress": 0.0,
            "start_date": null,
            "end_date": null,
            "created_at": "2026-05-15T00:00:00Z"
        }
    ]);

    Response::ok(serde_json::to_string(&projects).unwrap())
}

/// Create a new project.
pub async fn create(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _ws_id = req.param("ws_id").unwrap_or("");
    let body: CreateProjectRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("proj_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let project = serde_json::json!({
        "id": id,
        "name": body.name,
        "description": body.description,
        "status": "planning",
        "priority": body.priority.unwrap_or_else(|| "medium".to_string()),
        "owner": "human:current_user",
        "task_count": 0,
        "completed_task_count": 0,
        "progress": 0.0,
        "start_date": null,
        "end_date": null,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&project).unwrap())
}

/// Get a specific project by ID.
pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let project = serde_json::json!({
        "id": id,
        "name": "Sample Project",
        "description": "A sample project for demonstration",
        "status": "active",
        "priority": "medium",
        "owner": "human:demo",
        "task_count": 10,
        "completed_task_count": 4,
        "progress": 0.4,
        "start_date": "2026-01-01T00:00:00Z",
        "end_date": "2026-06-30T00:00:00Z",
        "created_at": "2026-01-01T00:00:00Z",
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&project).unwrap())
}

/// Update a project.
pub async fn update(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: UpdateProjectRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    tracing::info!("Updating project {:?}: {:?}", _id, body);
    Response::ok(r#"{"status":"updated"}"#)
}

/// Delete a project.
pub async fn delete(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    Response::ok(r#"{"status":"deleted"}"#)
}
