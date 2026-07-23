//! ScreenShareOverlay — full-screen share view.
//!
//! Displays a participant's screen share in full-screen overlay mode,
//! with a close button and the sharer's name. The video grid is hidden
//! behind this overlay.

use dioxus::prelude::*;

/// Full-screen overlay for viewing a participant's shared screen.
#[component]
pub fn ScreenShareOverlay(participant_name: String, on_close: EventHandler<()>) -> Element {
    rsx! {
        div {
            class: "screen-share-overlay",
            style: "position: fixed; inset: 0; z-index: 500; background: var(--q-bg); display: flex; flex-direction: column;",

            // Top bar
            div {
                style: "display: flex; align-items: center; justify-content: space-between; padding: 12px 20px; background: var(--q-surface); border-bottom: 1px solid var(--q-border); flex-shrink: 0;",

                // Sharer info
                div { style: "display: flex; align-items: center; gap: 10px;",
                    span { style: "font-size: 16px;", "🖥" }
                    span { style: "font-size: 14px; font-weight: 600;",
                        "{participant_name} is presenting"
                    }
                }

                // Close button
                button {
                    class: "btn-ghost",
                    style: "padding: 8px 16px; font-size: 13px; display: flex; align-items: center; gap: 6px;",
                    onclick: move |_| on_close.call(()),
                    span { "✕" }
                    span { "Close" }
                }
            }

            // Screen share video container
            div {
                style: "flex: 1; display: flex; align-items: center; justify-content: center; padding: 24px;",

                // Placeholder: in real impl, bind the screen share MediaStream to a <video>
                div {
                    style: "width: 100%; max-width: 1200px; aspect-ratio: 16 / 9; background: #000; border-radius: var(--q-radius-lg); display: flex; align-items: center; justify-content: center; color: var(--q-text-secondary); font-size: 16px; border: 1px solid var(--q-border); position: relative;",

                    video {
                        style: "width: 100%; height: 100%; object-fit: contain; border-radius: var(--q-radius-lg);",
                        autoplay: true,
                        playsinline: true,
                        id: "screen-share-video",
                    }

                    // Placeholder text shown when no video stream attached yet
                    div {
                        class: "screen-share-placeholder",
                        style: "position: absolute; display: flex; flex-direction: column; align-items: center; gap: 12px; pointer-events: none;",
                        span { style: "font-size: 48px;", "🖥" }
                        span { style: "font-size: 14px; color: var(--q-text-secondary);", "Waiting for screen share stream..." }
                    }
                }
            }

            // Bottom bar with mini view of participants
            div {
                style: "padding: 12px 20px; background: var(--q-surface); border-top: 1px solid var(--q-border); display: flex; align-items: center; gap: 12px; overflow-x: auto; flex-shrink: 0;",

                span { style: "font-size: 12px; color: var(--q-text-secondary); flex-shrink: 0;", "In call:" }

                // Mini participant tiles (placeholder)
                div {
                    style: "display: flex; gap: 8px;",
                    MiniTile { initial: "Y".into(), name: "You".into(), is_active: false }
                    MiniTile { initial: "A".into(), name: "Alice".into(), is_active: true }
                }
            }
        }
    }
}

/// A tiny participant tile shown in the overlay bottom bar.
#[component]
fn MiniTile(initial: String, name: String, is_active: bool) -> Element {
    rsx! {
        div {
            style: "display: flex; align-items: center; gap: 6px; padding: 4px 10px; background: var(--q-bg); border-radius: 999px; border: 1px solid {if is_active { \"var(--q-primary)\" } else { \"var(--q-border)\" }}; font-size: 12px;",
            div {
                style: "width: 24px; height: 24px; border-radius: 50%; background: var(--q-surface); display: flex; align-items: center; justify-content: center; font-size: 11px; font-weight: 600;",
                "{initial}"
            }
            span { "{name}" }
        }
    }
}
