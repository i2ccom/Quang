//! MediaControls — mute/unmute, camera on/off, screen share, end call, leave.
//!
//! The bottom control bar during an active meeting. Provides toggles for
//! microphone, camera, screen sharing, recording, and leaving the call.

use dioxus::prelude::*;

/// Props for the MediaControls component.
#[derive(Props, Clone, PartialEq)]
pub struct MediaControlsProps {
    pub is_mic_on: bool,
    pub is_camera_on: bool,
    pub is_screen_sharing: bool,
    pub is_recording: bool,

    pub on_toggle_mic: EventHandler<()>,
    pub on_toggle_camera: EventHandler<()>,
    pub on_toggle_screen_share: EventHandler<()>,
    pub on_toggle_recording: EventHandler<()>,
    pub on_leave: EventHandler<()>,
}

/// Bottom media controls bar for the meeting room.
#[component]
pub fn MediaControls(
    is_mic_on: bool,
    is_camera_on: bool,
    is_screen_sharing: bool,
    is_recording: bool,
    on_toggle_mic: EventHandler<()>,
    on_toggle_camera: EventHandler<()>,
    on_toggle_screen_share: EventHandler<()>,
    on_toggle_recording: EventHandler<()>,
    on_leave: EventHandler<()>,
) -> Element {
    rsx! {
        div {
            class: "media-controls",
            style: "display: flex; align-items: center; justify-content: center; gap: 12px;",

            // ── Mic toggle ──
            ControlButton {
                icon: if is_mic_on { "🎤" } else { "🔇" },
                label: if is_mic_on { "Mute" } else { "Unmute" },
                is_active: is_mic_on,
                is_danger: !is_mic_on,
                on_click: move |_| on_toggle_mic.call(()),
            }

            // ── Camera toggle ──
            ControlButton {
                icon: if is_camera_on { "📷" } else { "🚫" },
                label: if is_camera_on { "Camera On" } else { "Camera Off" },
                is_active: is_camera_on,
                is_danger: !is_camera_on,
                on_click: move |_| on_toggle_camera.call(()),
            }

            // ── Separator ──
            div { style: "width: 1px; height: 32px; background: var(--q-border);" }

            // ── Screen share ──
            ControlButton {
                icon: "🖥",
                label: if is_screen_sharing { "Stop Share" } else { "Share Screen" },
                is_active: is_screen_sharing,
                is_danger: false,
                on_click: move |_| on_toggle_screen_share.call(()),
            }

            // ── Separator ──
            div { style: "width: 1px; height: 32px; background: var(--q-border);" }

            // ── Recording toggle (host only in real impl) ──
            ControlButton {
                icon: if is_recording { "⏹" } else { "⏺" },
                label: if is_recording { "Stop Recording" } else { "Record" },
                is_active: is_recording,
                is_danger: is_recording,
                on_click: move |_| on_toggle_recording.call(()),
            }

            // ── Spacer ──
            div { style: "flex: 1;" }

            // ── Leave call ──
            button {
                class: "btn-ghost",
                style: "display: flex; align-items: center; gap: 8px; padding: 10px 20px; border-radius: var(--q-radius); font-size: 14px; font-weight: 600; color: white; background: var(--q-danger); transition: all 0.15s; border: none; cursor: pointer;",
                onclick: move |_| on_leave.call(()),
                span { "📞" }
                span { "Leave" }
            }
        }
    }
}

/// A single circular control button (mic, camera, screen, record).
#[component]
fn ControlButton(
    icon: String,
    label: String,
    is_active: bool,
    is_danger: bool,
    on_click: EventHandler<()>,
) -> Element {
    let bg = if is_danger {
        "var(--q-danger)"
    } else if is_active {
        "var(--q-primary)"
    } else {
        "var(--q-surface)"
    };

    let border = if is_active && !is_danger {
        "2px solid var(--q-primary)"
    } else {
        "1px solid var(--q-border)"
    };

    rsx! {
        button {
            style: "display: flex; flex-direction: column; align-items: center; gap: 4px; background: transparent; border: none; cursor: pointer; padding: 4px;",
            onclick: move |_| on_click.call(()),
            title: "{label}",

            // Icon circle
            div {
                style: "width: 44px; height: 44px; border-radius: 50%; background: {bg}; border: {border}; display: flex; align-items: center; justify-content: center; font-size: 18px; transition: all 0.15s; color: {if is_danger { \"white\" } else { \"var(--q-text)\" }};",
                "{icon}"
            }

            // Label
            span { style: "font-size: 10px; color: var(--q-text-secondary); white-space: nowrap;", "{label}" }
        }
    }
}
