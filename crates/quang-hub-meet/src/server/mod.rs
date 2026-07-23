//! quang-hub-meet-server — Cloudflare Workers + DurableObjects for WebRTC signaling.
//!
//! Server-side module for the Meet feature. Handles:
//! - Room CRUD via REST API
//! - WebRTC signaling relay via Durable Objects
//! - TURN server integration (Coturn / Cloudflare)
//! - Recording management with R2 storage
//! - AI transcription pipeline trigger
//!
//! ## Features
//!
//! - `server` — enabled by default when the `server` feature is active
//!   (see `Cargo.toml`)
//!
//! ## Module structure
//!
//! ```text
//! src-server/
//!   ├── lib.rs                         — Entry point (this file)
//!   ├── handlers/
//!   │   ├── mod.rs                     — Route handler exports
//!   │   ├── room_handlers.rs           — Room create/join/leave/end
//!   │   └── signaling_handlers.rs      — WebSocket upgrade + signaling
//!   ├── durable_object.rs              — DO for room state + signaling
//!   ├── turns.rs                       — TURN credentials + integration
//!   └── recording.rs                   — R2 recording management
//! ```

pub mod durable_object;
pub mod handlers;
pub mod recording;
pub mod turns;

// Re-export key types
pub use durable_object::MeetRoomDO;
pub use handlers::room_handlers;
pub use handlers::signaling_handlers;
