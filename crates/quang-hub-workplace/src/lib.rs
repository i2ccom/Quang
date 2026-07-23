//! quang-hub-workplace — the collaboration graph ecosystem for QuangHub.
//!
//! This crate provides the core data models, graph engine, event system,
//! and view projections for the Workplace collaboration tools.
//!
//! ## Architecture
//!
//! - **HyperGraph** — All state lives in a typed graph. Nodes are entities
//!   (WorkSpace, Team, Task, etc.), edges are typed relationships.
//! - **EventBus** — All mutations emit typed events for WebSocket push,
//!   AI agent triggers, summary generation, and audit logging.
//! - **View projections** — The same graph data can be viewed as Table,
//!   Kanban, Chart, or Gantt. New view types can be registered at runtime.
//! - **Dual interface** — Every entity is serializable (serde) for both
//!   human UI (Yew components) and agent tools (JSON schema).
//!
//! ## Entity hierarchy
//!
//! ```text
//! WorkSpace
//!   ├── Teams (humans + agents with roles)
//!   ├── Projects (time-bounded containers)
//!   │   ├── Tasks (assignable work units)
//!   │   ├── Goals (OKR-aligned objectives)
//!   │   └── Reviews (approval gates)
//!   └── Channels (topic-based communication)
//!       └── ChatMessages (threaded conversations)
//! Summaries (AI-generated or human-written digests)
//! ```

pub mod actor;
pub mod agent;
pub mod audit;
pub mod channel;
pub mod chat;
pub mod compensation;
pub mod contract;
pub mod event;
pub mod goal;
pub mod graph;
pub mod hub;
pub mod human;
pub mod project;
pub mod review;
pub mod skill;
pub mod summary;
pub mod task;
pub mod team;
pub mod view;
pub mod views;
pub mod worklog;
pub mod workspace;

// Feature-gated modules
#[cfg(feature = "server")]
pub mod server;
#[cfg(feature = "web")]
pub mod web;

pub use actor::*;
pub use agent::*;
pub use audit::*;
pub use channel::*;
pub use chat::*;
pub use compensation::*;
pub use contract::*;
pub use event::*;
pub use goal::*;
pub use graph::*;
pub use hub::*;
pub use human::*;
pub use project::*;
pub use review::*;
pub use skill::*;
pub use summary::*;
pub use task::*;
pub use team::*;
pub use view::*;
pub use views::*;
pub use worklog::*;
pub use workspace::*;
