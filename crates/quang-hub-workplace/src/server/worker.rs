//! Cloudflare Worker entry point for the QuangHub Workplace API.
//!
//! This is the main entry point that Cloudflare Workers calls.
//! It sets up the Router, registers all routes, and configures
//! CORS, logging, and error handling.

use worker::*;

use crate::routes;

/// The main worker entry point — called by Cloudflare Workers runtime.
#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    // Initialize logging
    console_error_panic_hook::set_once();
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_target(false)
        .init();

    // Log incoming request
    tracing::info!("{} {}", req.method().to_string(), req.path());

    // Build the router with CORS preflight handling
    let router = Router::new();

    // Handle CORS preflight (OPTIONS) requests
    let router = router.options_async("/api/*", |_req, _ctx| async move {
        Response::empty()
            .map(|r| r.with_headers(cors_headers()))
    });

    // Register all application routes
    let router = routes::register_routes(router);

    // Run the router
    let response = router.run(req, env).await;

    // Attach CORS headers to the response
    match response {
        Ok(resp) => Ok(resp.with_headers(cors_headers())),
        Err(e) => {
            tracing::error!("Request failed: {}", e);
            Response::error(
                &format!(r#"{{"error":"{}"}}"#, e),
                500,
            )
            .map(|r| r.with_headers(cors_headers()))
        }
    }
}

/// CORS headers helper.
fn cors_headers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Access-Control-Allow-Origin", "*"),
        ("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"),
        ("Access-Control-Allow-Headers", "Content-Type, Authorization"),
        ("Access-Control-Max-Age", "86400"),
        ("Content-Type", "application/json"),
    ]
}
