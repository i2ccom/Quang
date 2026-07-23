//! Route definitions for the Workplace Cloudflare Worker.
//!
//! All REST endpoints are defined here and dispatched to the appropriate
//! handler modules. GraphQL is served at `/api/graphql` and OAuth at
//! `/api/auth/*`.

use worker::*;

use crate::handlers;

/// Base API path prefix.
const API_PREFIX: &str = "/api";

/// Register all routes on the given Router.
pub fn register_routes(router: Router<()>) -> Router<()> {
    router
        // ── Health ──
        .get_async("/health", |_req, _ctx| async move {
            Response::ok(r#"{"status":"ok","service":"quang-hub-workplace"}"#)
                .map(|r| r.with_headers(cors_headers()))
        })

        // ── Workspace routes ──
        .get_async("/api/workspaces", |req, ctx| async move {
            handlers::workspace::list(req, ctx).await
        })
        .post_async("/api/workspaces", |req, ctx| async move {
            handlers::workspace::create(req, ctx).await
        })
        .get_async("/api/workspaces/:id", |req, ctx| async move {
            handlers::workspace::get(req, ctx).await
        })
        .put_async("/api/workspaces/:id", |req, ctx| async move {
            handlers::workspace::update(req, ctx).await
        })
        .delete_async("/api/workspaces/:id", |req, ctx| async move {
            handlers::workspace::delete(req, ctx).await
        })

        // ── Team routes ──
        .get_async("/api/workspaces/:ws_id/teams", |req, ctx| async move {
            handlers::team::list(req, ctx).await
        })
        .post_async("/api/workspaces/:ws_id/teams", |req, ctx| async move {
            handlers::team::create(req, ctx).await
        })
        .get_async("/api/teams/:id", |req, ctx| async move {
            handlers::team::get(req, ctx).await
        })
        .put_async("/api/teams/:id", |req, ctx| async move {
            handlers::team::update(req, ctx).await
        })
        .post_async("/api/teams/:id/members", |req, ctx| async move {
            handlers::team::add_member(req, ctx).await
        })
        .delete_async("/api/teams/:id/members/:actor_id", |req, ctx| async move {
            handlers::team::remove_member(req, ctx).await
        })

        // ── Project routes ──
        .get_async("/api/workspaces/:ws_id/projects", |req, ctx| async move {
            handlers::project::list(req, ctx).await
        })
        .post_async("/api/workspaces/:ws_id/projects", |req, ctx| async move {
            handlers::project::create(req, ctx).await
        })
        .get_async("/api/projects/:id", |req, ctx| async move {
            handlers::project::get(req, ctx).await
        })
        .put_async("/api/projects/:id", |req, ctx| async move {
            handlers::project::update(req, ctx).await
        })
        .delete_async("/api/projects/:id", |req, ctx| async move {
            handlers::project::delete(req, ctx).await
        })

        // ── Channel routes ──
        .get_async("/api/workspaces/:ws_id/channels", |req, ctx| async move {
            handlers::channel::list(req, ctx).await
        })
        .post_async("/api/workspaces/:ws_id/channels", |req, ctx| async move {
            handlers::channel::create(req, ctx).await
        })
        .get_async("/api/channels/:id", |req, ctx| async move {
            handlers::channel::get(req, ctx).await
        })
        .put_async("/api/channels/:id", |req, ctx| async move {
            handlers::channel::update(req, ctx).await
        })

        // ── Chat / Message routes ──
        .get_async("/api/channels/:ch_id/messages", |req, ctx| async move {
            handlers::chat::list(req, ctx).await
        })
        .post_async("/api/channels/:ch_id/messages", |req, ctx| async move {
            handlers::chat::send(req, ctx).await
        })
        .put_async("/api/messages/:id", |req, ctx| async move {
            handlers::chat::update(req, ctx).await
        })
        .delete_async("/api/messages/:id", |req, ctx| async move {
            handlers::chat::delete(req, ctx).await
        })
        .post_async("/api/messages/:id/reactions", |req, ctx| async move {
            handlers::chat::add_reaction(req, ctx).await
        })

        // ── Task routes ──
        .get_async("/api/projects/:proj_id/tasks", |req, ctx| async move {
            handlers::task::list(req, ctx).await
        })
        .post_async("/api/projects/:proj_id/tasks", |req, ctx| async move {
            handlers::task::create(req, ctx).await
        })
        .get_async("/api/tasks/:id", |req, ctx| async move {
            handlers::task::get(req, ctx).await
        })
        .put_async("/api/tasks/:id", |req, ctx| async move {
            handlers::task::update(req, ctx).await
        })
        .post_async("/api/tasks/:id/transition", |req, ctx| async move {
            handlers::task::transition(req, ctx).await
        })
        .post_async("/api/tasks/:id/assign", |req, ctx| async move {
            handlers::task::assign(req, ctx).await
        })

        // ── Goal routes ──
        .get_async("/api/projects/:proj_id/goals", |req, ctx| async move {
            handlers::goal::list(req, ctx).await
        })
        .post_async("/api/projects/:proj_id/goals", |req, ctx| async move {
            handlers::goal::create(req, ctx).await
        })
        .get_async("/api/goals/:id", |req, ctx| async move {
            handlers::goal::get(req, ctx).await
        })
        .put_async("/api/goals/:id", |req, ctx| async move {
            handlers::goal::update(req, ctx).await
        })
        .post_async("/api/goals/:id/key-results", |req, ctx| async move {
            handlers::goal::add_key_result(req, ctx).await
        })

        // ── Review routes ──
        .get_async("/api/reviews", |req, ctx| async move {
            handlers::review::list(req, ctx).await
        })
        .post_async("/api/tasks/:target_id/reviews", |req, ctx| async move {
            handlers::review::create(req, ctx).await
        })
        .get_async("/api/reviews/:id", |req, ctx| async move {
            handlers::review::get(req, ctx).await
        })
        .post_async("/api/reviews/:id/approve", |req, ctx| async move {
            handlers::review::approve(req, ctx).await
        })
        .post_async("/api/reviews/:id/request-changes", |req, ctx| async move {
            handlers::review::request_changes(req, ctx).await
        })
        .post_async("/api/reviews/:id/reject", |req, ctx| async move {
            handlers::review::reject(req, ctx).await
        })
        .post_async("/api/reviews/:id/comments", |req, ctx| async move {
            handlers::review::add_comment(req, ctx).await
        })

        // ── Summary routes ──
        .get_async("/api/summaries", |req, ctx| async move {
            handlers::summary::list(req, ctx).await
        })
        .post_async("/api/summaries", |req, ctx| async move {
            handlers::summary::create(req, ctx).await
        })
        .get_async("/api/summaries/:id", |req, ctx| async move {
            handlers::summary::get(req, ctx).await
        })

        // ── GraphQL ──
        .post_async("/api/graphql", |req, ctx| async move {
            crate::graphql::handle_graphql(req, ctx).await
        })

        // ── Auth routes ──
        .get_async("/api/auth/:provider", |req, ctx| async move {
            crate::auth::handle_oauth_start(req, ctx).await
        })
        .get_async("/api/auth/:provider/callback", |req, ctx| async move {
            crate::auth::handle_oauth_callback(req, ctx).await
        })
        .post_async("/api/auth/login", |req, ctx| async move {
            crate::auth::handle_login(req, ctx).await
        })
        .post_async("/api/auth/register", |req, ctx| async move {
            crate::auth::handle_register(req, ctx).await
        })
}

/// CORS headers for API responses.
fn cors_headers() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Access-Control-Allow-Origin", "*"),
        ("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"),
        ("Access-Control-Allow-Headers", "Content-Type, Authorization"),
        ("Access-Control-Max-Age", "86400"),
        ("Content-Type", "application/json"),
    ]
}
