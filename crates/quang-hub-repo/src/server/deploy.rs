//! Deploy — Cloudflare Pages auto-deploy integration for linked repos.

use worker::*;

/// Configuration for a Cloudflare Pages deployment.
#[derive(Debug, Clone)]
pub struct DeployConfig {
    pub account_id: String,
    pub project_name: String,
    pub branch: String,
    pub commit_sha: String,
    pub api_token: String,
}

/// Trigger a Cloudflare Pages deployment for a repo.
///
/// Uses the Cloudflare API v4 to trigger a new deployment for the given
/// project, branch, and commit SHA.
pub async fn trigger_deploy(config: &DeployConfig) -> Result<DeployResult> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/pages/projects/{}/deployments",
        config.account_id, config.project_name
    );

    let client = reqwest::Client::new();
    let body = serde_json::json!({
        "branch": config.branch,
        "commit_sha": config.commit_sha,
    });

    let response = client
        .post(&url)
        .header("Authorization", format!("Bearer {}", config.api_token))
        .header("Content-Type", "application/json")
        .json(&body)
        .send()
        .await
        .map_err(|e| Error::from(format!("Deploy request failed: {}", e)))?;

    let status = response.status();
    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| Error::from(format!("Failed to parse deploy response: {}", e)))?;

    if !status.is_success() {
        return Err(Error::from(format!(
            "Cloudflare API returned {}: {}",
            status,
            result.to_string()
        )));
    }

    let deploy_id = result["result"]["id"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let deploy_url = result["result"]["url"].as_str().unwrap_or("").to_string();

    tracing::info!(
        "Deploy triggered: project={}, branch={}, id={}, url={}",
        config.project_name,
        config.branch,
        deploy_id,
        deploy_url
    );

    Ok(DeployResult {
        deploy_id,
        deploy_url,
        status: "pending".to_string(),
    })
}

/// Check the status of a Cloudflare Pages deployment.
pub async fn check_deploy_status(config: &DeployConfig, deploy_id: &str) -> Result<DeployResult> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/pages/projects/{}/deployments/{}",
        config.account_id, config.project_name, deploy_id
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.api_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| Error::from(format!("Status check failed: {}", e)))?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| Error::from(format!("Failed to parse status response: {}", e)))?;

    let status = result["result"]["status"]
        .as_str()
        .unwrap_or("unknown")
        .to_string();
    let deploy_url = result["result"]["url"].as_str().unwrap_or("").to_string();

    Ok(DeployResult {
        deploy_id: deploy_id.to_string(),
        deploy_url,
        status,
    })
}

/// Result of a deployment operation.
#[derive(Debug, Clone)]
pub struct DeployResult {
    pub deploy_id: String,
    pub deploy_url: String,
    pub status: String,
}

/// List all deployments for a Cloudflare Pages project.
pub async fn list_deployments(config: &DeployConfig) -> Result<Vec<DeployResult>> {
    let url = format!(
        "https://api.cloudflare.com/client/v4/accounts/{}/pages/projects/{}/deployments",
        config.account_id, config.project_name
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("Authorization", format!("Bearer {}", config.api_token))
        .header("Content-Type", "application/json")
        .send()
        .await
        .map_err(|e| Error::from(format!("List deployments failed: {}", e)))?;

    let result: serde_json::Value = response
        .json()
        .await
        .map_err(|e| Error::from(format!("Failed to parse deployments: {}", e)))?;

    let deployments = result["result"]
        .as_array()
        .map(|arr| {
            arr.iter()
                .map(|d| DeployResult {
                    deploy_id: d["id"].as_str().unwrap_or("").to_string(),
                    deploy_url: d["url"].as_str().unwrap_or("").to_string(),
                    status: d["status"].as_str().unwrap_or("unknown").to_string(),
                })
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    Ok(deployments)
}
