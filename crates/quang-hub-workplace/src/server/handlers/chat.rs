//! Chat/Message REST handlers — send, list, update, and delete messages.

use serde::Deserialize;
use worker::*;

/// Request body for sending a message.
#[derive(Debug, Deserialize)]
pub struct SendMessageRequest {
    pub content: String,
    pub content_type: Option<String>,
}

/// Request body for adding a reaction.
#[derive(Debug, Deserialize)]
pub struct AddReactionRequest {
    pub emoji: String,
}

/// List messages in a channel.
pub async fn list(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _ch_id = req.param("ch_id").unwrap_or("");

    let messages = serde_json::json!([
        {
            "id": "msg_001",
            "channel_id": _ch_id,
            "author": "human:alice",
            "content": "Hey team, I just pushed the OAuth flow implementation!",
            "content_type": "text",
            "created_at": "2026-05-28T10:32:00Z",
            "edited_at": null,
            "reactions": ["👍", "🚀"]
        },
        {
            "id": "msg_002",
            "channel_id": _ch_id,
            "author": "human:bob",
            "content": "Great work! I'll review it after standup.",
            "content_type": "text",
            "created_at": "2026-05-28T10:33:00Z",
            "edited_at": null,
            "reactions": []
        },
        {
            "id": "msg_003",
            "channel_id": _ch_id,
            "author": "agent:agent-x",
            "content": "🤖 Dependency analysis complete. No circular dependencies found.",
            "content_type": "text",
            "created_at": "2026-05-28T10:38:00Z",
            "edited_at": "2026-05-28T10:39:00Z",
            "reactions": ["👍", "🎯", "🔥"]
        }
    ]);

    Response::ok(serde_json::to_string(&messages).unwrap())
}

/// Send a new message to a channel.
pub async fn send(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _ch_id = req.param("ch_id").unwrap_or("");
    let body: SendMessageRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("msg_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let message = serde_json::json!({
        "id": id,
        "channel_id": _ch_id,
        "author": "human:current_user",
        "content": body.content,
        "content_type": body.content_type.unwrap_or_else(|| "text".to_string()),
        "created_at": chrono::Utc::now().to_rfc3339(),
        "edited_at": null,
        "reactions": []
    });

    Response::ok(serde_json::to_string(&message).unwrap())
}

/// Update (edit) a message.
pub async fn update(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: serde_json::Value = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let content = body["content"].as_str().unwrap_or("");
    let response = serde_json::json!({
        "id": _id,
        "content": content,
        "edited_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&response).unwrap())
}

/// Delete a message.
pub async fn delete(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    Response::ok(r#"{"status":"deleted"}"#)
}

/// Add a reaction to a message.
pub async fn add_reaction(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: AddReactionRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let response = serde_json::json!({
        "message_id": _id,
        "emoji": body.emoji,
        "added": true
    });

    Response::ok(serde_json::to_string(&response).unwrap())
}
