//! Route handlers for all Workplace entities.
//!
//! Each handler module provides CRUD operations for a specific entity
//! type. Handlers receive a `Request` and `RouteContext`, interact with
//! the store (D1/KV/R2), and return a `Result<Response>`.

pub mod channel;
pub mod chat;
pub mod goal;
pub mod project;
pub mod review;
pub mod summary;
pub mod task;
pub mod team;
pub mod workspace;

pub use channel::*;
pub use chat::*;
pub use goal::*;
pub use project::*;
pub use review::*;
pub use summary::*;
pub use task::*;
pub use team::*;
pub use workspace::*;
