//! Auth provider — Google & GitHub OAuth for QuangHub.
//!
//! Manages login state, token storage, and session management
//! for both human users and AI agent identities.

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};
use wasm_bindgen::JsCast;

/// Supported OAuth identity providers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IdentityProvider {
    Google,
    GitHub,
}

impl IdentityProvider {
    pub fn as_str(&self) -> &str {
        match self {
            IdentityProvider::Google => "google",
            IdentityProvider::GitHub => "github",
        }
    }
}

/// Authenticated user session.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub id: String,
    pub email: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub provider: IdentityProvider,
    pub token: String,
    pub is_agent: bool,
}

impl UserSession {
    pub fn is_authenticated(&self) -> bool {
        !self.token.is_empty()
    }
}

/// OAuth login flow — dev mode uses mock, production redirects to backend.
pub fn login(provider: IdentityProvider) {
    // DEV MODE: Set a mock session so the profile menu appears.
    // In production, this would redirect to the OAuth backend.
    let mock_session = UserSession {
        id: format!("mock_{}", provider.as_str()),
        email: format!("user@example.com"),
        display_name: format!("Demo User"),
        avatar_url: None,
        provider,
        token: "mock_token_123".to_string(),
        is_agent: false,
    };
    save_session(&mock_session);

    // Reload the page to pick up the session from localStorage
    if let Some(window) = web_sys::window() {
        let _ = window.location().reload();
    }
}

/// Logout — clear session and redirect.
pub fn logout() {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            let _ = storage.remove_item("quang_session");
        }
        let _ = window.location().set_href("/");
    }
}

/// Restore session from localStorage (survives page reloads).
pub fn restore_session() -> Option<UserSession> {
    let storage = web_sys::window()?.local_storage().ok()??;
    let data = storage.get_item("quang_session").ok()??;
    serde_json::from_str(&data).ok()
}

/// Save session to localStorage.
pub fn save_session(session: &UserSession) {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(storage)) = window.local_storage() {
            if let Ok(data) = serde_json::to_string(session) {
                let _ = storage.set_item("quang_session", &data);
            }
        }
    }
}

async fn fetch_session_via_fetch(token: &str) -> Result<UserSession, String> {
    let mut opts = web_sys::RequestInit::new();
    opts.set_method("GET");

    let headers = web_sys::Headers::new().map_err(|_| "Failed to create headers".to_string())?;
    headers
        .set("Authorization", &format!("Bearer {}", token))
        .map_err(|_| "Failed to set auth header".to_string())?;
    opts.set_headers(&headers);

    let request = web_sys::Request::new_with_str_and_init("/api/auth/session", &opts)
        .map_err(|_| "Failed to create request".to_string())?;

    let window = web_sys::window().ok_or("No window".to_string())?;
    let resp = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
        .await
        .map_err(|e| format!("Fetch error: {:?}", e))?;

    let resp: web_sys::Response = resp.dyn_into().map_err(|_| "Not a response".to_string())?;

    if !resp.ok() {
        return Err(format!("HTTP {}", resp.status()));
    }

    let json = wasm_bindgen_futures::JsFuture::from(
        resp.json()
            .map_err(|_| "Failed to parse JSON".to_string())?,
    )
    .await
    .map_err(|e| format!("JSON error: {:?}", e))?;

    let session: UserSession =
        serde_wasm_bindgen::from_value(json).map_err(|e| format!("Deserialize error: {}", e))?;
    Ok(session)
}

/// Auth provider component — wraps the app and provides auth context.
#[component]
pub fn AuthProvider(children: Element) -> Element {
    let mut session = use_signal(|| restore_session());

    use_effect(move || {
        let window = match web_sys::window() {
            Some(w) => w,
            None => return,
        };
        let location = window.location();
        let params = match location.search() {
            Ok(s) => s,
            Err(_) => return,
        };

        if params.contains("token=") {
            let token = match params.split("token=").nth(1) {
                Some(t) => t.split('&').next().unwrap_or("").to_string(),
                None => return,
            };

            if token.is_empty() {
                return;
            }

            wasm_bindgen_futures::spawn_local(async move {
                match fetch_session_via_fetch(&token).await {
                    Ok(user_session) => {
                        session.set(Some(user_session.clone()));
                        save_session(&user_session);
                        let _ = location.set_search("");
                    }
                    Err(e) => {
                        tracing::error!("Auth callback failed: {}", e);
                    }
                }
            });
        }
    });

    rsx! {
        {children}
    }
}
