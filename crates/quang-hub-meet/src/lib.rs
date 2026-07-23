//! quang-hub-meet — real-time meeting and video call collaboration.
//!
//! This crate provides the data models and flows for real-time meetings
//! between humans and agents: rooms, participants, media streams,
//! recordings, and in-call chat/events.
//!
//! Meetings are HyperGraph nodes in the Workplace graph, connected via
//! edges to channels, projects, and participants.

pub mod chat;
pub mod event;
pub mod hub;
pub mod media;
pub mod participant;
pub mod recording;
pub mod room;

// Feature-gated modules
#[cfg(feature = "web")]
pub mod web;
#[cfg(feature = "server")]
pub mod server;

pub use chat::*;
pub use event::*;
pub use hub::*;
pub use media::*;
pub use participant::*;
pub use recording::*;
pub use room::*;
