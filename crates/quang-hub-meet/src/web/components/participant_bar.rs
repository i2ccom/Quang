//! ParticipantBar — participant list sidebar for the meeting room.
//!
//! Shows all participants with their media state, role badges,
//! and context actions (mute, promote, remove for hosts).

use dioxus::prelude::*;

use crate::web::components::video_tile::ParticipantView;

/// Sidebar that lists all participants in the meeting.
#[component]
pub fn ParticipantBar(participants: Vec<ParticipantView>) -> Element {
    // Split into groups: hosts, presenters, then everyone else
    // For simplicity, we show a flat list with role indicators.

    rsx! {
        div {
            class: "participant-bar",
            style: "display: flex; flex-direction: column; height: 100%;",

            // Header
            div {
                style: "padding: 16px; border-bottom: 1px solid var(--q-border); font-weight: 600; font-size: 14px; display: flex; align-items: center; justify-content: space-between;",
                span { "Participants" }
                span { style: "font-weight: 400; color: var(--q-text-secondary); font-size: 13px;",
                    "{participants.len()}"
                }
            }

            // List
            div {
                style: "flex: 1; overflow-y: auto; padding: 8px; display: flex; flex-direction: column; gap: 2px;",

                if participants.is_empty() {
                    div { style: "padding: 24px; text-align: center; color: var(--q-text-secondary); font-size: 13px;",
                        "No participants yet"
                    }
                }

                for p in participants.iter() {
                    ParticipantRow { participant: p.clone() }
                }
            }
        }
    }
}

/// A single participant row in the sidebar.
#[component]
fn ParticipantRow(participant: ParticipantView) -> Element {
    let initial = participant
        .display_name
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .unwrap_or_else(|| "?".to_string());

    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 10px; padding: 8px 10px; border-radius: var(--q-radius); transition: background 0.15s;",

            // Avatar / initial circle
            div {
                style: "width: 32px; height: 32px; border-radius: 50%; background: var(--q-bg); border: 1px solid {if participant.is_agent { \"var(--q-primary)\" } else { \"var(--q-border)\" }}; display: flex; align-items: center; justify-content: center; font-size: 13px; font-weight: 600; flex-shrink: 0;",
                "{initial}"
            }

            // Name + role
            div { style: "flex: 1; min-width: 0;",
                div { style: "display: flex; align-items: center; gap: 6px;",
                    span { style: "font-size: 13px; font-weight: 500; white-space: nowrap; overflow: hidden; text-overflow: ellipsis;", "{participant.display_name}" }
                    if participant.is_local {
                        span { style: "font-size: 10px; color: var(--q-text-secondary);", "(You)" }
                    }
                }
                if participant.is_agent {
                    span { style: "font-size: 10px; color: var(--q-primary); font-weight: 500;", "AI Agent" }
                }
            }

            // Media indicators
            div { style: "display: flex; gap: 6px; flex-shrink: 0;",
                // Mic
                div {
                    style: "width: 24px; height: 24px; border-radius: 50%; background: var(--q-bg); display: flex; align-items: center; justify-content: center; font-size: 11px; {if !participant.is_mic_on { \"color: var(--q-danger);\" }}",
                    if participant.is_mic_on { "🎤" } else { "🔇" }
                }
                // Camera
                div {
                    style: "width: 24px; height: 24px; border-radius: 50%; background: var(--q-bg); display: flex; align-items: center; justify-content: center; font-size: 11px; {if !participant.is_camera_on { \"color: var(--q-danger);\" }}",
                    if participant.is_camera_on { "📷" } else { "🚫" }
                }
                // Hand raise
                if participant.is_hand_raised {
                    div {
                        style: "width: 24px; height: 24px; border-radius: 50%; background: rgba(252, 196, 25, 0.2); display: flex; align-items: center; justify-content: center; font-size: 12px;",
                        "✋"
                    }
                }
            }
        }
    }
}
