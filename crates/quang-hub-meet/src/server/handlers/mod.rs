//! HTTP request handlers for the Meet REST API.
//!
//! Uses the `worker` crate (cloudflare-workers) for route definitions
//! and HTTP primitives.

pub mod room_handlers;
pub mod signaling_handlers;
