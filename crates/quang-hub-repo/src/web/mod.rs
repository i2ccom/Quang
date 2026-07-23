//! quang-hub-repo web module — Dioxus UI for repo browsing and management.
//!
//! This module is gated behind the `web` feature flag. It provides
//! Dioxus components for the QuangHub repo browser, file tree, code viewer,
//! diff view, commit history, QTask board, and adaptive repo app viewer.

pub mod components;
pub mod pages;

// Re-export pages for router integration
pub use pages::repo_browse::RepoBrowse;
pub use pages::repo_detail::RepoDetail;
pub use pages::repo_home::RepoHome;
pub use pages::repo_new::RepoNew;
pub use pages::repo_tasks::RepoTasks;
