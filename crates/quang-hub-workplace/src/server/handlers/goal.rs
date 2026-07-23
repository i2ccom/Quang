//! Goal REST handlers — CRUD and key-result management for Goal entities.

use serde::Deserialize;
use worker::*;

/// Request body for creating a goal.
#[derive(Debug, Deserialize)]
pub struct CreateGoalRequest {
    pub title: String,
    pub description: String,
    pub period: Option<String>,
}

/// Request body for updating a goal.
#[derive(Debug, Deserialize)]
pub struct UpdateGoalRequest {
    pub title: Option<String>,
    pub description: Option<String>,
    pub status: Option<String>,
}

/// Request body for adding a key result.
#[derive(Debug, Deserialize)]
pub struct AddKeyResultRequest {
    pub title: String,
    pub target_value: f64,
    pub unit: String,
}

/// List goals in a project.
pub async fn list(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _proj_id = req.param("proj_id").unwrap_or("");

    let goals = serde_json::json!([
        {
            "id": "goal_001",
            "title": "Launch MVP of agent collaboration platform",
            "description": "Ship the core platform with task management, chat, and agent integration",
            "status": "on_track",
            "period": "2026-Q2",
            "owner": "human:alice",
            "progress": 0.65,
            "key_results": [
                {"id": "kr_001", "title": "Ship task management", "target_value": 100.0, "current_value": 80.0, "unit": "%", "progress": 0.8},
                {"id": "kr_002", "title": "Onboard 10 beta teams", "target_value": 10.0, "current_value": 4.0, "unit": "teams", "progress": 0.4},
                {"id": "kr_003", "title": "Agent chat integration", "target_value": 5.0, "current_value": 3.0, "unit": "channels", "progress": 0.6}
            ],
            "created_at": "2026-04-01T00:00:00Z",
            "updated_at": "2026-05-28T00:00:00Z"
        },
        {
            "id": "goal_002",
            "title": "Achieve 99.9% API uptime",
            "description": "Improve reliability and monitoring to meet SLO targets",
            "status": "on_track",
            "period": "2026-Q2",
            "owner": "human:bob",
            "progress": 0.92,
            "key_results": [
                {"id": "kr_004", "title": "Reduce p95 latency <200ms", "target_value": 200.0, "current_value": 145.0, "unit": "ms", "progress": 0.73},
                {"id": "kr_005", "title": "Zero critical incidents", "target_value": 0.0, "current_value": 1.0, "unit": "incidents", "progress": 0.0}
            ],
            "created_at": "2026-04-01T00:00:00Z",
            "updated_at": "2026-05-27T00:00:00Z"
        }
    ]);

    Response::ok(serde_json::to_string(&goals).unwrap())
}

/// Create a new goal.
pub async fn create(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _proj_id = req.param("proj_id").unwrap_or("");
    let body: CreateGoalRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("goal_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let goal = serde_json::json!({
        "id": id,
        "title": body.title,
        "description": body.description,
        "status": "draft",
        "period": body.period.unwrap_or_else(|| "2026-Q2".to_string()),
        "owner": "human:current_user",
        "progress": 0.0,
        "key_results": [],
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&goal).unwrap())
}

/// Get a specific goal by ID.
pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let goal = serde_json::json!({
        "id": id,
        "title": "Sample Goal",
        "description": "A sample OKR-aligned goal",
        "status": "active",
        "period": "2026-Q2",
        "owner": "human:demo",
        "progress": 0.45,
        "key_results": [
            {"id": "kr_demo_1", "title": "Key result 1", "target_value": 100.0, "current_value": 45.0, "unit": "%", "progress": 0.45}
        ],
        "created_at": "2026-04-15T00:00:00Z",
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&goal).unwrap())
}

/// Update a goal.
pub async fn update(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: UpdateGoalRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    tracing::info!("Updating goal {:?}: {:?}", _id, body);
    Response::ok(r#"{"status":"updated"}"#)
}

/// Add a key result to a goal.
pub async fn add_key_result(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let _id = req.param("id").unwrap_or("");
    let body: AddKeyResultRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let kr_id = format!("kr_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));
    let key_result = serde_json::json!({
        "id": kr_id,
        "title": body.title,
        "target_value": body.target_value,
        "current_value": 0.0,
        "unit": body.unit,
        "progress": 0.0,
        "created_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&key_result).unwrap())
}
