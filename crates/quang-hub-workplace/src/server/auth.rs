//! OAuth authentication handlers for Google and GitHub providers.
//!
//! Handles the OAuth 2.0 authorization flow: redirect users to the
//! provider's consent page, handle the callback with the auth code,
//! exchange it for tokens, and create/manage user sessions.

use serde::{Deserialize, Serialize};
use worker::*;

/// OAuth provider identifiers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OAuthProvider {
    Google,
    GitHub,
}

impl OAuthProvider {
    fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "google" => Some(Self::Google),
            "github" => Some(Self::GitHub),
            _ => None,
        }
    }

    fn auth_url(&self, client_id: &str, redirect_uri: &str, state: &str) -> String {
        match self {
            OAuthProvider::Google => {
                format!(
                    "https://accounts.google.com/o/oauth2/v2/auth?\
                     client_id={}&redirect_uri={}&response_type=code&\
                     scope=openid%20email%20profile&state={}",
                    client_id, redirect_uri, state
                )
            }
            OAuthProvider::GitHub => {
                format!(
                    "https://github.com/login/oauth/authorize?\
                     client_id={}&redirect_uri={}&scope=read:user+user:email&state={}",
                    client_id, redirect_uri, state
                )
            }
        }
    }
}

/// User info returned after successful OAuth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub provider: String,
    pub provider_id: String,
    pub email: String,
    pub name: String,
    pub avatar_url: Option<String>,
}

/// Start the OAuth flow by redirecting the user to the provider.
pub async fn handle_oauth_start(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let provider_name = req.param("provider").unwrap_or("google");
    let provider = OAuthProvider::from_str(provider_name)
        .ok_or_else(|| worker::Error::RustError(format!("Unknown provider: {}", provider_name)))?;

    // In production, get these from environment secrets (worker::Secret)
    let client_id = ctx
        .var("OAUTH_CLIENT_ID")
        .map(|v| v.to_string())
        .unwrap_or_else(|_| "dev-client-id".to_string());

    let redirect_uri = format!(
        "{}/api/auth/{}/callback",
        ctx.var("APP_URL")
            .map(|v| v.to_string())
            .unwrap_or_else(|_| "http://localhost:8787".to_string()),
        provider_name
    );

    let state = uuid::Uuid::new_v4().to_string();

    // Store state in KV for CSRF validation
    // In production: ctx.kv("sessions")?.put(&state, "pending")?.expiration_ttl(300)?;

    let auth_url = provider.auth_url(&client_id, &redirect_uri, &state);

    Response::redirect(301, auth_url.as_str())
}

/// Handle the OAuth callback from the provider.
pub async fn handle_oauth_callback(req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let query = req.url()?.query().unwrap_or_default().to_string();
    let params: Vec<(String, String)> = url::form_urlencoded::parse(query.as_bytes())
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect();

    let code = params
        .iter()
        .find(|(k, _)| k == "code")
        .map(|(_, v)| v.clone())
        .ok_or_else(|| worker::Error::RustError("Missing authorization code".to_string()))?;

    let _state = params
        .iter()
        .find(|(k, _)| k == "state")
        .map(|(_, v)| v.clone())
        .ok_or_else(|| worker::Error::RustError("Missing state parameter".to_string()))?;

    // In production: validate state against KV store, then exchange code for tokens
    // let token_response = exchange_code_for_token(provider, &code).await?;
    // let user_info = fetch_user_info(provider, &token_response.access_token).await?;
    // let session_token = create_session(&user_info).await?;
    // Set session cookie and redirect to app

    // Stub: return success
    Response::redirect(301, "/workspaces")
}

/// Handle email/password login.
pub async fn handle_login(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body: serde_json::Value = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid JSON body: {}", e)))?;

    let email = body["email"].as_str().unwrap_or("");
    let _password = body["password"].as_str().unwrap_or("");

    if email.is_empty() {
        return Response::error(r#"{"error":"Email is required"}"#, 400);
    }

    // In production: validate credentials against D1, create session
    let session = serde_json::json!({
        "token": uuid::Uuid::new_v4().to_string(),
        "user": {
            "email": email,
            "name": email.split('@').next().unwrap_or("User"),
        }
    });

    Response::ok(serde_json::to_string(&session).unwrap()).map(|r| r.with_headers(cors_headers()))
}

/// Handle user registration.
pub async fn handle_register(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body: serde_json::Value = req
        .json()
        .await
        .map_err(|e| worker::Error::RustError(format!("Invalid JSON body: {}", e)))?;

    let email = body["email"].as_str().unwrap_or("");
    let _password = body["password"].as_str().unwrap_or("");
    let name = body["name"].as_str().unwrap_or("User");

    if email.is_empty() || _password.is_empty() || name.is_empty() {
        return Response::error(r#"{"error":"Email, password, and name are required"}"#, 400);
    }

    // In production: create user in D1, hash password, create session
    let response = serde_json::json!({
        "token": uuid::Uuid::new_v4().to_string(),
        "user": {
            "email": email,
            "name": name,
        }
    });

    Response::ok(serde_json::to_string(&response).unwrap()).map(|r| r.with_headers(cors_headers()))
}

/// CORS headers helper.
fn cors_headers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Access-Control-Allow-Origin", "*"),
        (
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ),
        ("Content-Type", "application/json"),
    ]
}
