//! GitHub API proxy handlers — proxy requests to the GitHub API with OAuth tokens.

use worker::*;

use crate::github_client::GitHubClient;

/// Proxy a request to the GitHub API, injecting the repo's OAuth token.
pub async fn handle_github_proxy(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;
    let path = ctx.param("*").unwrap_or("");

    // TODO: Fetch repo and its access token
    // let d1 = ctx.d1("REPO_DB")?;
    // let repo: Option<LinkedRepo> = d1.select("repos", repo_id).await?;
    // let token = repo.access_token.ok_or_else(|| Error::from("No access token"))?;

    let token = "mock_token".to_string();
    let client = GitHubClient::new(&token);

    let result = client.get(path).await?;

    Response::from_json(&result)
}

/// Get file contents from a repo via the GitHub API.
pub async fn handle_get_file(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;
    let path = ctx.param("path").ok_or_else(|| Error::from("Missing path"))?;
    let branch = req.url()?.query_pairs()
        .find(|(k, _)| k == "ref")
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "main".to_string());

    // TODO: Fetch from GitHub API or Fluid Remote
    let response = serde_json::json!({
        "path": path,
        "branch": branch,
        "content": "// File content would be returned here",
        "encoding": "utf-8",
    });

    Response::from_json(&response)
}

/// Get the file tree for a repo path via GitHub API.
pub async fn handle_get_tree(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;
    let path = ctx.param("path").unwrap_or("");
    let branch = req.url()?.query_pairs()
        .find(|(k, _)| k == "ref")
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "main".to_string());

    // TODO: Fetch tree from GitHub API or Fluid Remote
    let response = serde_json::json!({
        "path": path,
        "branch": branch,
        "entries": [],
        "total_count": 0,
    });

    Response::from_json(&response)
}

/// Get recent commits for a repo branch.
pub async fn handle_get_commits(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;
    let branch = req.url()?.query_pairs()
        .find(|(k, _)| k == "branch")
        .map(|(_, v)| v.to_string())
        .unwrap_or_else(|| "main".to_string());

    // TODO: Fetch commits from GitHub API
    let response = serde_json::json!({
        "commits": [],
        "total_count": 0,
        "branch": branch,
    });

    Response::from_json(&response)
}

/// Get branches for a repo.
pub async fn handle_get_branches(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let repo_id = ctx.param("repo_id").ok_or_else(|| Error::from("Missing repo_id"))?;

    // TODO: Fetch branches from GitHub API
    let response = serde_json::json!({
        "branches": [],
        "total_count": 0,
    });

    Response::from_json(&response)
}
