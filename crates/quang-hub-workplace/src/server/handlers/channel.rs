//! Channel REST handlers — CRUD for Channel entities.

use serde::Deserialize;
use worker::*;

/// Request body for creating a channel.
#[derive(Debug, Deserialize)]
pub struct CreateChannelRequest {
    pub name: String,
    pub topic: String,
    pub kind: Option<String>,
    pub is_private: Option<bool>,
}

/// List channels in a workspace.
pub async fn list(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _ws_id = req.param("ws_id").unwrap_or("");

    let channels = serde_json::json!([
        {
            "id": "ch_001",
            "name": "general",
            "topic": "General workspace discussion",
            "kind": "general",
            "is_private": false,
            "message_count": 145,
            "created_at": "2026-01-01T00:00:00Z",
            "last_activity": "2026-05-28T10:32:00Z"
        },
        {
            "id": "ch_002",
            "name": "engineering",
            "topic": "Engineering team chat",
            "kind": "team",
            "is_private": false,
            "message_count": 89,
            "created_at": "2026-01-15T00:00:00Z",
            "last_activity": "2026-05-28T09:15:00Z"
        },
        {
            "id": "ch_003",
            "name": "agent-logs",
            "topic": "AI agent activity and logs",
            "kind": "agent",
            "is_private": false,
            "message_count": 312,
            "created_at": "2026-03-01T00:00:00Z",
            "last_activity": "2026-05-28T11:00:00Z"
        }
    ]);

    Response::ok(serde_json::to_string(&channels).unwrap())
}

/// Create a new channel.
pub async fn create(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _ws_id = req.param("ws_id").unwrap_or("");
    let body: CreateChannelRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("ch_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let channel = serde_json::json!({
        "id": id,
        "name": body.name,
        "topic": body.topic,
        "kind": body.kind.unwrap_or_else(|| "general".to_string()),
        "is_private": body.is_private.unwrap_or(false),
        "message_count": 0,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339(),
        "last_activity": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&channel).unwrap())
}

/// Get a specific channel by ID.
pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let channel = serde_json::json!({
        "id": id,
        "name": "general",
        "topic": "General workspace discussion",
        "kind": "general",
        "is_private": false,
        "message_count": 145,
        "created_at": "2026-01-01T00:00:00Z",
        "last_activity": "2026-05-28T10:32:00Z"
    });

    Response::ok(serde_json::to_string(&channel).unwrap())
}

/// Update a channel.
pub async fn update(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: serde_json::Value = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    tracing::info!("Updating channel: {:?}", body);
    Response::ok(r#"{"status":"updated"}"#)
}
