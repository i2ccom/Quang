//! Team REST handlers — CRUD for Team entities and member management.

use serde::Deserialize;
use worker::*;

use crate::store::WorkplaceStore;

/// Request body for creating a team.
#[derive(Debug, Deserialize)]
pub struct CreateTeamRequest {
    pub name: String,
    pub description: String,
}

/// Request body for adding a team member.
#[derive(Debug, Deserialize)]
pub struct AddMemberRequest {
    pub actor_id: String,
    pub role: String,
    pub display_name: String,
}

/// List teams in a workspace.
pub async fn list(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _ws_id = req.param("ws_id").unwrap_or("");

    let teams = serde_json::json!([
        {
            "id": "team_demo_1",
            "name": "Engineering",
            "description": "Core engineering team",
            "member_count": 8,
            "human_count": 6,
            "agent_count": 2,
            "created_at": "2026-01-15T00:00:00Z"
        },
        {
            "id": "team_demo_2",
            "name": "Design",
            "description": "Product design team",
            "member_count": 4,
            "human_count": 4,
            "agent_count": 0,
            "created_at": "2026-02-01T00:00:00Z"
        }
    ]);

    Response::ok(serde_json::to_string(&teams).unwrap())
}

/// Create a new team in a workspace.
pub async fn create(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _ws_id = req.param("ws_id").unwrap_or("");
    let body: CreateTeamRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("team_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let team = serde_json::json!({
        "id": id,
        "name": body.name,
        "description": body.description,
        "members": [],
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&team).unwrap())
}

/// Get a specific team by ID.
pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let team = serde_json::json!({
        "id": id,
        "name": "Engineering Team",
        "description": "The core engineering team",
        "members": [
            {"actor": "human:alice", "role": "owner", "display_name": "Alice", "joined_at": "2026-01-15T00:00:00Z"},
            {"actor": "human:bob", "role": "admin", "display_name": "Bob", "joined_at": "2026-01-20T00:00:00Z"},
            {"actor": "agent:agent-x", "role": "agent", "display_name": "Agent-X", "joined_at": "2026-03-01T00:00:00Z"}
        ],
        "created_at": "2026-01-15T00:00:00Z",
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&team).unwrap())
}

/// Update a team.
pub async fn update(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: serde_json::Value = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    tracing::info!("Updating team: {:?}", body);
    Response::ok(r#"{"status":"updated"}"#)
}

/// Add a member to a team.
pub async fn add_member(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _team_id = req.param("id").unwrap_or("");
    let body: AddMemberRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let member = serde_json::json!({
        "actor": body.actor_id,
        "role": body.role,
        "display_name": body.display_name,
        "joined_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&member).unwrap())
}

/// Remove a member from a team.
pub async fn remove_member(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _team_id = req.param("id").unwrap_or("");
    let _actor_id = req.param("actor_id").unwrap_or("");

    Response::ok(r#"{"status":"member_removed"}"#)
}
