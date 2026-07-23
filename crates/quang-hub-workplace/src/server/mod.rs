//! quang-hub-workplace server module — Cloudflare Workers handlers.
//!
//! This module provides the server-side implementation for the QuangHub
//! Workplace platform, targeting Cloudflare Workers with D1, KV, and R2
//! storage backends. It includes REST + GraphQL route handlers, OAuth
//! authentication, and storage abstractions.
//!
//! ## Module Tree
//!
//! - `handlers/` — REST route handlers for all Workplace entities
//! - `routes.rs` — Route definitions and router setup
//! - `worker.rs` — Cloudflare Worker entry point
//! - `graphql.rs` — GraphQL schema, types, and resolvers
//! - `auth.rs` — OAuth (Google, GitHub) authentication handlers
//! - `store.rs` — D1 / KV / R2 storage abstractions

pub mod auth;
pub mod graphql;
pub mod handlers;
pub mod routes;
pub mod store;
pub mod worker;
