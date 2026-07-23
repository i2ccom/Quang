//! quang-hub-repo server module — Cloudflare Workers handlers for repo operations.
//!
//! This module is gated behind the `server` feature flag. It provides
//! Cloudflare Workers request handlers for GitHub API proxying, repo CRUD,
//! webhook handling, and Cloudflare Pages auto-deploy.

pub mod deploy;
pub mod github_client;
pub mod handlers;
pub mod webhook;

pub use deploy::*;
pub use github_client::*;
pub use handlers::*;
pub use webhook::*;
