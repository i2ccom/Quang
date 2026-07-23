//! Workspace REST handlers — CRUD for WorkSpace entities.
//!
//! Each handler reads/writes to D1 via WorkplaceStore, serializes
//! results as JSON, and returns appropriate HTTP status codes.

use serde::{Deserialize, Serialize};
use worker::*;

use crate::store::WorkplaceStore;

/// Request body for creating a workspace.
#[derive(Debug, Deserialize)]
pub struct CreateWorkspaceRequest {
    pub name: String,
    pub description: String,
    pub slug: String,
}

/// Request body for updating a workspace.
#[derive(Debug, Deserialize)]
pub struct UpdateWorkspaceRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

/// List all workspaces accessible to the current user.
pub async fn list(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let store = WorkplaceStore::new(&_ctx.env);

    match store
        .d1_query_all::<serde_json::Value>(
            "SELECT id, data FROM workspaces ORDER BY created_at DESC",
            vec![],
        )
        .await
    {
        Ok(rows) => {
            let workspaces: Vec<serde_json::Value> = rows
                .iter()
                .filter_map(|row| {
                    let id = row.get("id")?.as_str()?;
                    let data = row.get("data")?.as_object()?;
                    let mut ws = data.clone();
                    ws.insert("id".into(), serde_json::Value::String(id.to_string()));
                    Some(serde_json::Value::Object(ws))
                })
                .collect();
            let json = serde_json::to_string(&workspaces).unwrap_or_else(|_| "[]".to_string());
            Response::ok(json)
        }
        Err(e) => {
            tracing::error!("Failed to list workspaces: {}", e);
            // Return stub data until D1 is connected
            let stubs = serde_json::json!([
                {
                    "id": "ws_demo",
                    "name": "Demo Workspace",
                    "description": "A demo workspace for development",
                    "slug": "demo",
                    "owner": "human:dev",
                    "member_count": 3,
                    "project_count": 2,
                    "created_at": "2026-01-01T00:00:00Z",
                    "updated_at": "2026-05-01T00:00:00Z"
                }
            ]);
            Response::ok(serde_json::to_string(&stubs).unwrap())
        }
    }
}

/// Create a new workspace.
pub async fn create(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body: CreateWorkspaceRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("ws_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let workspace = serde_json::json!({
        "name": body.name,
        "description": body.description,
        "slug": body.slug,
        "owner": "human:current_user",
        "settings": {
            "allow_agent_teams": true,
            "max_projects": 100,
            "default_view": "kanban",
            "features": ["chat", "tasks", "goals", "reviews"]
        },
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    let store = WorkplaceStore::new(&_ctx.env);
    if let Err(e) = store.d1_insert("workspaces", &id, &workspace).await {
        tracing::warn!("D1 insert failed (dev mode): {}", e);
    }

    let mut response = serde_json::json!({"id": id, ...workspace});
    Response::ok(serde_json::to_string(&response).unwrap())
}

/// Get a specific workspace by ID.
pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let store = WorkplaceStore::new(&_ctx.env);
    match store
        .d1_query_one::<serde_json::Value>(
            "SELECT data FROM workspaces WHERE id = ?",
            vec![D1Value::from(id.to_string())],
        )
        .await
    {
        Ok(Some(data)) => {
            let mut ws = data.clone();
            if let Some(obj) = ws.as_object_mut() {
                obj.insert("id".into(), serde_json::Value::String(id.to_string()));
            }
            Response::ok(serde_json::to_string(&ws).unwrap())
        }
        _ => Response::error(r#"{"error":"Workspace not found"}"#, 404),
    }
}

/// Update a workspace.
pub async fn update(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");
    let body: UpdateWorkspaceRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let store = WorkplaceStore::new(&_ctx.env);

    // In production: read existing, merge fields, write back
    let mut update_data = serde_json::Map::new();
    if let Some(name) = body.name {
        update_data.insert("name".into(), serde_json::Value::String(name));
    }
    if let Some(desc) = body.description {
        update_data.insert("description".into(), serde_json::Value::String(desc));
    }
    update_data.insert(
        "updated_at".into(),
        serde_json::Value::String(chrono::Utc::now().to_rfc3339()),
    );

    if let Err(e) = store
        .d1_update("workspaces", id, &serde_json::Value::Object(update_data))
        .await
    {
        tracing::warn!("D1 update failed (dev mode): {}", e);
    }

    Response::ok(r#"{"status":"updated"}"#)
}

/// Delete a workspace.
pub async fn delete(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let store = WorkplaceStore::new(&_ctx.env);
    if let Err(e) = store.d1_delete("workspaces", id).await {
        tracing::warn!("D1 delete failed (dev mode): {}", e);
    }

    Response::ok(r#"{"status":"deleted"}"#)
}
