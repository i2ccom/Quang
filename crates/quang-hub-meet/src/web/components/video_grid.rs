//! VideoGrid — responsive grid of video tiles.
//!
//! Arranges participant video/avatar tiles in a responsive grid that
//! adapts to the number of participants. When someone is screen sharing,
//! the grid switches to a main-screen + stacked sidebar layout.

use dioxus::prelude::*;

use crate::web::components::video_tile::{ParticipantView, VideoTile};

/// Responsive video tile grid for the meeting room.
#[component]
pub fn VideoGrid(participants: Vec<ParticipantView>, screen_sharing: bool) -> Element {
    // ── Separate screen sharer from others ──
    let (sharers, others): (Vec<_>, Vec<_>) = participants
        .iter()
        .cloned()
        .partition(|p| p.is_screen_sharing);

    // If screen sharing is active, use spotlight layout
    if screen_sharing && !sharers.is_empty() {
        return rsx! {
            div {
                class: "video-grid-screen-share",
                style: "display: flex; gap: 8px; height: 100%; padding: 8px;",

                // Main screen share tile
                div {
                    style: "flex: 1; min-width: 0;",
                    VideoTile {
                        participant: sharers[0].clone(),
                        is_screen_tile: Some(true),
                    }
                }

                // Side column of other participants
                if !others.is_empty() {
                    div {
                        style: "width: 200px; flex-shrink: 0; display: flex; flex-direction: column; gap: 8px; overflow-y: auto;",
                        for p in others.iter() {
                            div {
                                style: "flex-shrink: 0;",
                                VideoTile {
                                    participant: p.clone(),
                                }
                            }
                        }
                    }
                }
            }
        };
    }

    // ── Standard grid layout ──
    let count = participants.len();
    if count == 0 {
        return rsx! {
            div {
                style: "display: flex; align-items: center; justify-content: center; height: 100%; color: var(--q-text-secondary); font-size: 14px;",
                "Waiting for participants..."
            }
        };
    }

    // Determine grid columns based on count
    let columns = match count {
        1 => "repeat(1, 1fr)",
        2 => "repeat(2, 1fr)",
        3 | 4 => "repeat(2, 1fr)",
        5..=9 => "repeat(3, 1fr)",
        _ => "repeat(4, 1fr)",
    };

    rsx! {
        div {
            class: "video-grid",
            style: "display: grid; grid-template-columns: {columns}; gap: 8px; padding: 8px; height: 100%; align-content: center;",

            for p in participants.iter() {
                VideoTile {
                    participant: p.clone(),
                }
            }
        }
    }
}
