//! quang-hub-repo — repository integration & adaptive apps for QuangHub.
//!
//! This crate provides the core data models, Dioxus web components, and
//! Cloudflare Workers server handlers for managing GitHub-linked repos,
//! browsing code, running agent tasks against repos, and rendering
//! adaptive repo apps from `qh.app` manifests.
//!
//! ## Architecture
//!
//! ```text
//! quang-hub-repo
//!   ├── src/
//!   │   ├── lib.rs          — Core data models (this file)
//!   │   ├── linked_repo.rs  — GitHub connection model
//!   │   ├── repo_file.rs    — File/folder tree entries
//!   │   ├── repo_branch.rs  — Branch model
//!   │   ├── repo_commit.rs  — Commit model
//!   │   ├── repo_app.rs     — qh.app manifest model
//!   │   ├── repo_settings.rs— Mirror / deploy / webhook settings
//!   │   ├── q_task.rs       — Agent task tied to repo
//!   │   └── repo_event.rs   — Repo change events
//!   ├── src-web/            — Dioxus web components (feature "web")
//!   ├── src-server/         — Cloudflare Workers handlers (feature "server")
//!   └── docs/               — Architecture & plan docs
//! ```
//!
//! ## Dual Remote Model
//!
//! Every linked repo has two remote references:
//! - **Upstream** — The canonical GitHub remote (read/write via OAuth)
//! - **Fluid Remote** — An internal QuangHub mirror that enables fast clones,
//!   AI agent write access, pre-commit analysis, and auto-deploy triggers
//!   without granting AI agents direct GitHub push access.

pub mod linked_repo;
pub mod q_task;
pub mod repo_app;
pub mod repo_branch;
pub mod repo_commit;
pub mod repo_event;
pub mod repo_file;
pub mod repo_settings;

// Feature-gated modules
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "web")]
pub mod web;

pub use linked_repo::*;
pub use q_task::*;
pub use repo_app::*;
pub use repo_branch::*;
pub use repo_commit::*;
pub use repo_event::*;
pub use repo_file::*;
pub use repo_settings::*;
