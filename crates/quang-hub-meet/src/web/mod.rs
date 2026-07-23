//! quang-hub-meet-web — Dioxus web module for real-time video/audio meetings.
//!
//! Provides the browser UI for the Meet feature: room listing, active meeting,
//! scheduling, and all in-call components (video grid, participant bar,
//! media controls, chat, screen share, info panel).
//!
//! Also exposes the WebRTC helper module for peer connection management
//! on the Wasm side.
//!
//! ## Module structure
//!
//! ```text
//! src-web/
//!   ├── lib.rs                   — Entry point (this file)
//!   ├── pages/
//!   │   ├── mod.rs
//!   │   ├── meet_home.rs         — Room listing + create/join
//!   │   ├── meet_room.rs         — Active meeting room
//!   │   └── meet_schedule.rs     — Schedule a meeting
//!   ├── components/
//!   │   ├── mod.rs
//!   │   ├── video_grid.rs        — Responsive video tile grid
//!   │   ├── video_tile.rs        — Single video/avatar tile
//!   │   ├── participant_bar.rs   — Participant list sidebar
//!   │   ├── media_controls.rs    — Mute/camera/screen/share/end
//!   │   ├── in_call_chat.rs      — Chat sidebar during meeting
//!   │   ├── screen_share_overlay.rs — Full-screen share view
//!   │   └── meeting_info_panel.rs   — Meeting details + recording
//!   └── webrtc/
//!       └── mod.rs               — WebRTC peer connection helpers
//! ```

pub mod components;
pub mod pages;
pub mod webrtc;

// Re-export page components for the router
pub use pages::meet_home::MeetHome;
pub use pages::meet_room::MeetRoom;
pub use pages::meet_schedule::MeetSchedule;

// Re-export key components for external use
pub use components::media_controls::MediaControls;
pub use components::video_grid::VideoGrid;
pub use components::video_tile::VideoTile;
