//! quang-web — shared Dioxus web layer for QuangHub.
//!
//! This crate provides common UI components, GraphQL client, auth helpers,
//! and event bus integration shared across all QuangHub modules
//! (Workplace, Meet, Repo).
//!
//! ## Architecture
//!
//! ```text
//! quang-web
//!   ├── app.rs          — Root App shell + router
//!   ├── auth.rs         — Auth provider (Google, GitHub OAuth)
//!   ├── client.rs       — GraphQL / API client
//!   ├── components/     — Shared Dioxus components
//!   │   ├── mod.rs
//!   │   ├── app_shell.rs
//!   │   ├── sidebar.rs
//!   │   ├── topbar.rs
//!   │   ├── avatar.rs
//!   │   └── loading.rs
//!   ├── event.rs        — Shared event bus (WebSocket + Signals)
//!   ├── graphql/        — GraphQL client & queries
//!   │   ├── mod.rs
//!   │   └── schema.rs
//!   ├── hooks/          — Shared Dioxus hooks
//!   │   ├── mod.rs
//!   │   └── use_auth.rs
//!   └── router.rs       — App router configuration
//! ```

#![cfg_attr(target_arch = "wasm32", allow(unused_imports))]

pub mod auth;
pub mod client;
pub mod components;
pub mod event;
pub mod graphql;
pub mod hooks;
pub mod router;

// Re-export common types
pub use auth::AuthProvider;
pub use client::QuangHubClient;
pub use event::EventBusSignal;
pub use router::AppRoute;
