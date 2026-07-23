//! Repo CRUD handlers — create, read, list, update, delete linked repos.

use worker::*;

use crate::linked_repo::{LinkedRepo, RepoConnectionStatus, RepoId};
use crate::repo_event::RepoEvent;
use crate::repo_settings::RepoSettings;

/// Create a new linked repository.
pub async fn handle_create_repo(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    // Parse request body
    let body: serde_json::Value = req.json().await.map_err(|e| {
        Error::from(format!("Invalid request body: {}", e))
    })?;

    let owner = body["owner"].as_str().ok_or_else(|| Error::from("Missing 'owner'"))?;
    let name = body["name"].as_str().ok_or_else(|| Error::from("Missing 'name'"))?;
    let default_branch = body["default_branch"].as_str().unwrap_or("main");
    let is_private = body["is_private"].as_bool().unwrap_or(false);
    let access_token = body["access_token"].as_str().map(|s| s.to_string());
    let workspace_id = body["workspace_id"].as_str().map(|s| s.to_string());

    let mut repo = LinkedRepo::new(owner, name, default_branch, is_private, access_token);
    repo.workspace_id = workspace_id;

    // TODO: Store in D1 or KV
    // let d1 = ctx.d1("REPO_DB")?;
    // d1.insert("repos", &repo.id, &repo).await?;

    // Kick off Fluid Remote sync
    // spawn(async move { sync_fluid_remote(repo.id.clone()).await });

    let response = serde_json::json!({
        "success": true,
        "repo": repo,
    });

    Response::from_json(&response)
}

/// Get a single linked repository by ID.
pub async fn handle_get_repo(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;

    // TODO: Fetch from D1
    // let d1 = ctx.d1("REPO_DB")?;
    // let repo: Option<LinkedRepo> = d1.select("repos", repo_id).await?;

    let repo = LinkedRepo::new("owner", "repo", "main", false, None);

    Response::from_json(&repo)
}

/// List all linked repositories, optionally filtered by workspace.
pub async fn handle_list_repos(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let workspace_id = req.url()?.query_pairs()
        .find(|(k, _)| k == "workspace_id")
        .map(|(_, v)| v.to_string());

    // TODO: Fetch from D1
    let repos: Vec<LinkedRepo> = Vec::new();

    let response = serde_json::json!({
        "success": true,
        "repos": repos,
        "total": repos.len(),
    });

    Response::from_json(&response)
}

/// Update a linked repository's settings or metadata.
pub async fn handle_update_repo(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;
    let body: serde_json::Value = req.json().await.map_err(|e| {
        Error::from(format!("Invalid request body: {}", e))
    })?;

    // TODO: Fetch existing repo, merge updates, persist

    let response = serde_json::json!({
        "success": true,
        "repo_id": repo_id,
    });

    Response::from_json(&response)
}

/// Delete (unlink) a repository.
pub async fn handle_delete_repo(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;

    // TODO: Delete from D1, remove webhook, clean up Fluid Remote

    let response = serde_json::json!({
        "success": true,
        "deleted": repo_id,
    });

    Response::from_json(&response)
}

/// Trigger a Fluid Remote sync for a repo.
pub async fn handle_sync_repo(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;

    // TODO: Spawn sync task
    // spawn(async move { sync_fluid_remote(repo_id.clone()).await });

    let response = serde_json::json!({
        "success": true,
        "message": format!("Sync initiated for repo {}", repo_id),
    });

    Response::from_json(&response)
}
