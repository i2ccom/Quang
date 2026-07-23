//! # quang-core
//!
//! The **core domain model** for the Quang platform.
//!
//! This crate defines the higher-level abstractions that all `quang-hub-*`
//! crates build on: Task, Job, Workflow, Agent, Cost, and Policy.
//!
//! It composes concepts from lower-level crates (jigsaw-core for evidence,
//! minh-agent for tool execution) without duplicating them. All source
//! stays in the original repos — quang-core defines **Quang's own types**.
//!
//! ## Module Map
//!
//! | Module | Purpose |
//! |--------|---------|
//! | [`task`] | Unified Task — lifecycle, priority, tool execution trace |
//! | [`agent`] | AgentDescriptor — full agent identity, role, tools, security |
//! | [`cost`] | Unified Cost — human, agent, and infrastructure spend |
//! | [`policy`] | Policy — governance rules composing Jigsaw evidence |
//! | [`job`] | Job — policy-governed DAG of Tasks |
//! | [`workflow`] | Workflow — reusable parameterized Job template |
//! | [`types`] | Shared primitives — IDs, ActorId, Timestamp, ExecutorKind |
//!
//! ## Dependency Philosophy
//!
//! ```text
//! jigsaw-core ──┐
//!   (Spike,      │
//!    Evidence,   │   quang-core ───────────────────────┐
//!    Weight)     ├──▶ (Task, Job, Workflow,            │
//!               │    Agent, Cost, Policy)              │
//! minh-agent ───┘                               ┌──────┴──────┐
//!   (Tool trait,                                │             │
//!    SecurityContext,                    quang-hub-      quang-hub-
//!    optional)                           workplace        repo
//! ```
//!
//! No source moves from lower crates. quang-core is 100% new types
//! that compose lower concepts into the Quang domain model.

pub mod types;
pub mod weight_serde;
pub mod task;
pub mod agent;
pub mod cost;
pub mod policy;
pub mod job;
pub mod workflow;

// Re-export everything for convenience
pub use types::*;
pub use task::*;
pub use agent::*;
pub use cost::*;
pub use policy::*;
pub use job::*;
pub use workflow::*;

// Re-export jigsaw-core types that Quang composes (not copies)
pub use jigsaw_core::{Weight, FluidId};
