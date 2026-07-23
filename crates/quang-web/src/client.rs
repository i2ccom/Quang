//! QuangHubClient — typed API + GraphQL client.
//!
//! Provides a unified HTTP/GraphQL/Event client using the browser Fetch API.
//! All requests go through the QuangHub backend with auth headers attached.

use serde::{de::DeserializeOwned, Serialize};
use std::collections::HashMap;

/// Unified API client for QuangHub. Uses browser Fetch API on wasm.
#[derive(Clone)]
pub struct QuangHubClient {
    base_url: String,
    token: Option<String>,
}

impl QuangHubClient {
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.trim_end_matches('/').to_string(),
            token: None,
        }
    }

    pub fn with_token(mut self, token: &str) -> Self {
        self.token = Some(token.to_string());
        self
    }

    pub fn set_token(&mut self, token: &str) {
        self.token = Some(token.to_string());
    }

    async fn fetch_json(
        &self,
        method: &str,
        path: &str,
        body: Option<&str>,
    ) -> Result<serde_json::Value, String> {
        let url = format!("{}{}", self.base_url, path);

        let mut opts = web_sys::RequestInit::new();
        opts.set_method(method);

        let headers =
            web_sys::Headers::new().map_err(|_| "Failed to create headers".to_string())?;
        headers
            .set("Content-Type", "application/json")
            .map_err(|_| "Failed to set content type".to_string())?;
        if let Some(token) = &self.token {
            headers
                .set("Authorization", &format!("Bearer {}", token))
                .map_err(|_| "Failed to set auth header".to_string())?;
        }
        opts.set_headers(&headers);

        if let Some(body_str) = body {
            opts.set_body(&wasm_bindgen::JsValue::from_str(body_str));
        }

        let request = web_sys::Request::new_with_str_and_init(&url, &opts)
            .map_err(|_| format!("Failed to create request: {}", url))?;

        let window = web_sys::window().ok_or("No window".to_string())?;
        let resp = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .map_err(|e| format!("Fetch error: {:?}", e))?;

        let resp: web_sys::Response =
            wasm_bindgen::JsCast::dyn_into(resp).map_err(|_| "Not a response".to_string())?;

        if !resp.ok() {
            return Err(format!("HTTP {}", resp.status()));
        }

        let json = wasm_bindgen_futures::JsFuture::from(
            resp.json()
                .map_err(|_| "Failed to parse JSON".to_string())?,
        )
        .await
        .map_err(|e| format!("JSON error: {:?}", e))?;

        serde_wasm_bindgen::from_value(json).map_err(|e| format!("Deserialize error: {}", e))
    }

    pub async fn get<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let json = self.fetch_json("GET", path, None).await?;
        serde_json::from_value(json).map_err(|e| e.to_string())
    }

    pub async fn post<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, String> {
        let body_str = serde_json::to_string(body).map_err(|e| e.to_string())?;
        let json = self.fetch_json("POST", path, Some(&body_str)).await?;
        serde_json::from_value(json).map_err(|e| e.to_string())
    }

    pub async fn put<T: DeserializeOwned, B: Serialize>(
        &self,
        path: &str,
        body: &B,
    ) -> Result<T, String> {
        let body_str = serde_json::to_string(body).map_err(|e| e.to_string())?;
        let json = self.fetch_json("PUT", path, Some(&body_str)).await?;
        serde_json::from_value(json).map_err(|e| e.to_string())
    }

    pub async fn delete<T: DeserializeOwned>(&self, path: &str) -> Result<T, String> {
        let json = self.fetch_json("DELETE", path, None).await?;
        serde_json::from_value(json).map_err(|e| e.to_string())
    }

    pub async fn graphql<T: DeserializeOwned>(
        &self,
        query: &str,
        variables: HashMap<String, serde_json::Value>,
    ) -> Result<T, String> {
        let body = serde_json::json!({
            "query": query,
            "variables": variables,
        });
        let body_str = serde_json::to_string(&body).map_err(|e| e.to_string())?;
        let json = self.fetch_json("POST", "/graphql", Some(&body_str)).await?;

        if let Some(errors) = json.get("errors").and_then(|e| e.as_array()) {
            let msgs: Vec<String> = errors
                .iter()
                .filter_map(|e| e.get("message").and_then(|m| m.as_str()))
                .map(String::from)
                .collect();
            if !msgs.is_empty() {
                return Err(format!("GraphQL errors: {}", msgs.join("; ")));
            }
        }

        let data = json
            .get("data")
            .ok_or_else(|| "GraphQL: no data returned".to_string())?
            .clone();
        serde_json::from_value(data).map_err(|e| e.to_string())
    }

    pub fn event_stream(&self, topic: &str) -> EventStream {
        EventStream::new(&self.base_url, topic, self.token.as_deref())
    }
}

pub struct EventStream {
    url: String,
    topic: String,
    token: Option<String>,
}

impl EventStream {
    pub fn new(base_url: &str, topic: &str, token: Option<&str>) -> Self {
        Self {
            url: format!(
                "{}/api/events/subscribe?topic={}",
                base_url.trim_end_matches('/'),
                topic
            ),
            topic: topic.to_string(),
            token: token.map(String::from),
        }
    }

    pub fn topic(&self) -> &str {
        &self.topic
    }

    pub fn open(&self) -> web_sys::EventSource {
        let mut url = self.url.clone();
        if let Some(token) = &self.token {
            url.push_str(&format!("&token={}", token));
        }
        web_sys::EventSource::new(&url).expect("Failed to create EventSource")
    }
}
