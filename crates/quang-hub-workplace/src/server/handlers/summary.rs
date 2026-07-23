//! Summary REST handlers — CRUD for AI-generated and human-written summaries.

use serde::Deserialize;
use worker::*;

/// Request body for creating a summary.
#[derive(Debug, Deserialize)]
pub struct CreateSummaryRequest {
    pub title: String,
    pub source_id: String,
    pub kind: Option<String>,
    pub sections: Option<Vec<SummarySectionInput>>,
}

/// A single section in a summary.
#[derive(Debug, Deserialize)]
pub struct SummarySectionInput {
    pub title: String,
    pub content: String,
}

/// List all summaries.
pub async fn list(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let summaries = serde_json::json!([
        {
            "id": "sum_001",
            "title": "Sprint 12 Review",
            "kind": "sprint_review",
            "source_id": "proj_001",
            "is_ai_generated": true,
            "generated_by": "agent:summary-bot",
            "sections": [
                {"title": "Completed", "content": "3 tasks completed this sprint.", "references": ["task_001", "task_002", "task_003"]},
                {"title": "In Progress", "content": "2 tasks still in progress.", "references": ["task_004", "task_005"]},
                {"title": "Blockers", "content": "No blockers identified.", "references": []}
            ],
            "created_at": "2026-05-28T00:00:00Z"
        },
        {
            "id": "sum_002",
            "title": "Daily Standup — May 28",
            "kind": "daily_standup",
            "source_id": "ws_demo",
            "is_ai_generated": true,
            "generated_by": "agent:standup-bot",
            "sections": [
                {"title": "Team Updates", "content": "Alice: Working on OAuth. Bob: Reviewing PRs. Carol: Design reviews.", "references": []}
            ],
            "created_at": "2026-05-28T09:00:00Z"
        }
    ]);

    Response::ok(serde_json::to_string(&summaries).unwrap())
}

/// Create a new summary.
pub async fn create(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body: CreateSummaryRequest = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid request body: {}", e)))?;

    let id = format!("sum_{}", uuid::Uuid::new_v4().to_string().replace('-', ""));

    let sections: Vec<serde_json::Value> = body
        .sections
        .unwrap_or_default()
        .into_iter()
        .map(|s| {
            serde_json::json!({
                "title": s.title,
                "content": s.content,
                "references": []
            })
        })
        .collect();

    let summary = serde_json::json!({
        "id": id,
        "title": body.title,
        "kind": body.kind.unwrap_or_else(|| "custom".to_string()),
        "source_id": body.source_id,
        "is_ai_generated": true,
        "generated_by": "agent:summary-bot",
        "sections": sections,
        "created_at": chrono::Utc::now().to_rfc3339(),
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&summary).unwrap())
}

/// Get a specific summary by ID.
pub async fn get(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let id = req.param("id").unwrap_or("");

    let summary = serde_json::json!({
        "id": id,
        "title": "Project Status Summary",
        "kind": "project_status",
        "source_id": "proj_demo",
        "is_ai_generated": true,
        "generated_by": "agent:summary-bot",
        "sections": [
            {"title": "Overview", "content": "The project is on track with 65% completion.", "references": []},
            {"title": "Risks", "content": "No major risks identified at this time.", "references": []}
        ],
        "created_at": "2026-05-27T00:00:00Z",
        "updated_at": chrono::Utc::now().to_rfc3339()
    });

    Response::ok(serde_json::to_string(&summary).unwrap())
}
