//! Webhook — GitHub webhook handler for auto-sync, auto-deploy, and event dispatch.

use worker::*;

use crate::github_client::GitHubClient;
use crate::repo_event::{RepoEvent, RepoEventBus};

/// Handle an incoming GitHub webhook event.
///
/// Processes push, pull_request, create, and ping events to trigger
/// Fluid Remote syncs, auto-deploy, and event bus notifications.
pub async fn handle_webhook(mut req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let event_name = req
        .headers()
        .get("X-GitHub-Event")
        .ok()
        .flatten()
        .unwrap_or_else(|| "unknown".to_string());

    let delivery_id = req
        .headers()
        .get("X-GitHub-Delivery")
        .ok()
        .flatten()
        .unwrap_or_else(|| "unknown".to_string());

    let body: serde_json::Value = req
        .json()
        .await
        .map_err(|e| Error::from(format!("Failed to parse webhook body: {}", e)))?;

    tracing::info!(
        "Received webhook: event={}, delivery={}",
        event_name,
        delivery_id
    );

    match event_name.as_str() {
        "ping" => handle_ping_event(body).await,
        "push" => handle_push_event(body, &ctx).await,
        "pull_request" => handle_pull_request_event(body).await,
        "create" => handle_create_event(body).await,
        _ => Response::from_json(&serde_json::json!({
            "success": true,
            "message": format!("Unhandled event type: {}", event_name),
        })),
    }
}

/// Handle a ping event (used for webhook setup verification).
async fn handle_ping_event(body: serde_json::Value) -> Result<Response> {
    let hook_id = body["hook_id"].as_u64().unwrap_or(0);
    let zen = body["zen"].as_str().unwrap_or("");

    tracing::info!("Webhook ping: hook_id={}, zen={}", hook_id, zen);

    Response::from_json(&serde_json::json!({
        "success": true,
        "message": "pong",
        "hook_id": hook_id,
    }))
}

/// Handle a push event — trigger Fluid Remote sync and auto-deploy.
async fn handle_push_event(body: serde_json::Value, ctx: &RouteContext<()>) -> Result<Response> {
    let repo_name = body["repository"]["full_name"]
        .as_str()
        .unwrap_or("unknown");
    let ref_name = body["ref"].as_str().unwrap_or("");
    let branch = ref_name.strip_prefix("refs/heads/").unwrap_or(ref_name);
    let commit_sha = body["after"].as_str().unwrap_or("");
    let pusher = body["pusher"]["name"].as_str().unwrap_or("unknown");
    let commit_count = body["commits"]
        .as_array()
        .map(|a| a.len() as u64)
        .unwrap_or(0);

    tracing::info!(
        "Push event: repo={}, branch={}, sha={}, commits={}",
        repo_name,
        branch,
        commit_sha,
        commit_count
    );

    // TODO: Look up the LinkedRepo and trigger Fluid Remote sync
    // let repo = find_repo_by_full_name(repo_name).await?;
    // if repo.settings.mirror_enabled {
    //     spawn(async move { sync_fluid_remote(&repo).await });
    // }
    //
    // // Auto-deploy if branch matches
    // if repo.settings.auto_deploy_enabled && matches_branch_pattern(&branch, &repo.settings.auto_deploy_branch) {
    //     spawn(async move { trigger_deploy(&repo, &branch, commit_sha).await });
    // }

    Response::from_json(&serde_json::json!({
        "success": true,
        "message": format!("Push to {}/{} processed", repo_name, branch),
        "branch": branch,
        "commit_sha": commit_sha,
        "commit_count": commit_count,
    }))
}

/// Handle a pull request event.
async fn handle_pull_request_event(body: serde_json::Value) -> Result<Response> {
    let action = body["action"].as_str().unwrap_or("unknown");
    let pr_number = body["pull_request"]["number"].as_u64().unwrap_or(0);
    let repo_name = body["repository"]["full_name"]
        .as_str()
        .unwrap_or("unknown");
    let pr_title = body["pull_request"]["title"].as_str().unwrap_or("");

    tracing::info!(
        "PR event: repo={}, pr={}, action={}, title={}",
        repo_name,
        pr_number,
        action,
        pr_title
    );

    // Trigger post-merge summary if PR was merged
    if action == "closed" && body["pull_request"]["merged"].as_bool().unwrap_or(false) {
        // TODO: Trigger post-merge summary generation
        // spawn(async move { generate_post_merge_summary(repo_name, pr_number).await });
    }

    Response::from_json(&serde_json::json!({
        "success": true,
        "message": format!("PR #{} {} in {}", pr_number, action, repo_name),
    }))
}

/// Handle a create event (branch or tag created).
async fn handle_create_event(body: serde_json::Value) -> Result<Response> {
    let ref_type = body["ref_type"].as_str().unwrap_or("unknown");
    let ref_name = body["ref"].as_str().unwrap_or("");
    let repo_name = body["repository"]["full_name"]
        .as_str()
        .unwrap_or("unknown");

    tracing::info!(
        "Create event: repo={}, type={}, ref={}",
        repo_name,
        ref_type,
        ref_name
    );

    Response::from_json(&serde_json::json!({
        "success": true,
        "message": format!("{} {} created in {}", ref_type, ref_name, repo_name),
    }))
}
