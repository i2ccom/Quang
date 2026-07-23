//! VideoTile — single video/avatar tile with name overlay, mute indicator.
//!
//! Renders a square/rectangular tile that shows either a live <video>
//! element or a fallback avatar with the user's initial. Overlays show
//! the participant name, mute icon, and hand-raise indicator.

use dioxus::prelude::*;

use crate::web::webrtc::MediaTrackId;

/// Display data for a participant inside the video grid.
#[derive(Debug, Clone, PartialEq)]
pub struct ParticipantView {
    pub id: String,
    pub display_name: String,
    pub avatar_url: Option<String>,
    pub is_mic_on: bool,
    pub is_camera_on: bool,
    pub is_screen_sharing: bool,
    pub is_hand_raised: bool,
    pub is_local: bool,
    pub is_agent: bool,
    pub video_track_id: Option<String>,
}

/// A single video or avatar tile in the meeting grid.
#[component]
pub fn VideoTile(
    participant: ParticipantView,
    is_screen_tile: Option<bool>,
    tile_size: Option<String>,
) -> Element {
    let is_screen = is_screen_tile.unwrap_or(false);

    // Determine the visual state
    let show_video = participant.is_camera_on && participant.video_track_id.is_some();
    let initial = participant
        .display_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    // Size: if screen share tile, use wider aspect; else square-ish
    let aspect_style = if is_screen {
        "aspect-ratio: 16 / 9;"
    } else {
        "aspect-ratio: 4 / 3;"
    };

    rsx! {
        div {
            class: "video-tile",
            style: "position: relative; border-radius: var(--q-radius-lg); overflow: hidden; background: var(--q-surface); border: 1px solid var(--q-border); {aspect_style} min-width: 0; display: flex; align-items: center; justify-content: center;",

            // Border highlight for screen sharer / speaker
            if participant.is_screen_sharing {
                div {
                    style: "position: absolute; inset: 0; border: 2px solid var(--q-primary); border-radius: var(--q-radius-lg); pointer-events: none; z-index: 2;",
                }
            }

            // ── Video element ──
            if show_video {
                video {
                    style: "width: 100%; height: 100%; object-fit: cover; background: #000;",
                    autoplay: true,
                    muted: participant.is_local,
                    playsinline: true,
                    // In real impl, bind MediaStream to this element
                    id: "video-{participant.id}",
                }
            }

            // ── Fallback avatar (no camera or no track) ──
            if !show_video {
                div {
                    style: "display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 8px; color: var(--q-text-secondary);",

                    // Avatar circle with initial
                    div {
                        style: "width: 64px; height: 64px; border-radius: 50%; background: var(--q-bg); border: 2px solid {if participant.is_agent { \"var(--q-primary)\" } else { \"var(--q-border)\" }}; display: flex; align-items: center; justify-content: center; font-size: 24px; font-weight: 600; color: var(--q-text);",
                        "{initial}"
                    }
                }
            }

            // ── Name overlay (bottom-left) ──
            div {
                style: "position: absolute; bottom: 8px; left: 8px; right: 8px; display: flex; align-items: center; justify-content: space-between; gap: 6px; z-index: 3;",

                // Name pill
                div {
                    style: "display: flex; align-items: center; gap: 6px; background: rgba(0, 0, 0, 0.6); padding: 4px 10px; border-radius: 999px; font-size: 12px; font-weight: 500; color: white; backdrop-filter: blur(4px);",

                    if !participant.is_mic_on {
                        span { style: "font-size: 11px; color: var(--q-danger);", "🔇" }
                    }
                    span { "{participant.display_name}" }
                    if participant.is_agent {
                        span { style: "font-size: 10px; color: var(--q-primary); font-weight: 600;", "AI" }
                    }
                }

                // Hand raise indicator
                if participant.is_hand_raised {
                    div {
                        style: "background: rgba(252, 196, 25, 0.8); padding: 4px 8px; border-radius: 999px; font-size: 14px;",
                        "✋"
                    }
                }
            }

            // ── Screen share badge ──
            if participant.is_screen_sharing {
                div {
                    style: "position: absolute; top: 8px; left: 8px; background: var(--q-primary); padding: 3px 8px; border-radius: 999px; font-size: 10px; font-weight: 600; color: white; z-index: 3; display: flex; align-items: center; gap: 4px;",
                    span { "🖥" }
                    span { "Screen" }
                }
            }

            // ── Local "You" badge ──
            if participant.is_local {
                div {
                    style: "position: absolute; top: 8px; right: 8px; background: rgba(0, 0, 0, 0.5); padding: 2px 8px; border-radius: 999px; font-size: 10px; color: var(--q-text-secondary); z-index: 3;",
                    "You"
                }
            }
        }
    }
}
