//! MeetingInfoPanel — meeting details, recording status, duration.
//!
//! Sidebar panel showing the current meeting's metadata: room ID,
//! duration, participant count, recording state, and quick actions
//! like copying the invite link.

use dioxus::prelude::*;

/// Meeting info sidebar panel.
#[component]
pub fn MeetingInfoPanel(
    room_id: String,
    duration_seconds: u64,
    is_recording: bool,
    participant_count: usize,
) -> Element {
    let copy_label = use_signal(|| "Copy".to_string());

    let handle_copy_link = move |_| {
        // Build the meeting invite link
        let link = format!("{}/meet/{}", get_base_url(), room_id);
        // Copy to clipboard via Web API
        if let Some(window) = web_sys::window() {
            if let Some(clipboard) = window.navigator().clipboard() {
                let _ = clipboard.write_text(&link);
                copy_label.set("Copied!".to_string());
                // Reset after 2 seconds
                let mut label = copy_label.clone();
                spawn(async move {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    label.set("Copy".to_string());
                });
            }
        }
    };

    rsx! {
        div {
            class: "meeting-info-panel",
            style: "display: flex; flex-direction: column; height: 100%;",

            // Header
            div {
                style: "padding: 16px; border-bottom: 1px solid var(--q-border); font-weight: 600; font-size: 14px;",
                "Meeting Info"
            }

            // Content
            div {
                style: "padding: 16px; display: flex; flex-direction: column; gap: 16px; overflow-y: auto;",

                // ── Meeting link ──
                div { style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--q-text-secondary);", "Invite Link" }
                    div { style: "display: flex; gap: 8px;",
                        input {
                            value: "{get_base_url()}/meet/{room_id}",
                            readonly: true,
                            style: "flex: 1; font-size: 11px; padding: 6px 8px;",
                            onclick: move |e| {
                                let input = e.as_ref(); // get HTMLInputElement
                                // Select the text
                                _ = input.select();
                            },
                        }
                        button {
                            class: "btn-ghost",
                            style: "padding: 6px 12px; font-size: 12px; white-space: nowrap;",
                            onclick: handle_copy_link,
                            "{copy_label}"
                        }
                    }
                }

                // ── Statistics ──
                div { style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--q-text-secondary);", "Statistics" }

                    StatRow { label: "Duration".into(), value: format_duration(duration_seconds) }
                    StatRow { label: "Participants".into(), value: format!("{}", participant_count) }
                    StatRow {
                        label: "Recording".into(),
                        value: if is_recording { "Active" } else { "Off" }.into(),
                        value_color: if is_recording { Some("var(--q-danger)") } else { None },
                    }
                }

                // ── Recording status ──
                if is_recording {
                    div {
                        style: "display: flex; align-items: center; gap: 8px; padding: 12px; background: rgba(255, 107, 107, 0.1); border: 1px solid rgba(255, 107, 107, 0.3); border-radius: var(--q-radius);",
                        div { style: "width: 8px; height: 8px; border-radius: 50%; background: var(--q-danger); animation: q-pulse 1.5s ease-in-out infinite;", "" }
                        div { style: "display: flex; flex-direction: column; gap: 2px;",
                            span { style: "font-size: 13px; font-weight: 600; color: var(--q-danger);", "Recording in progress" }
                            span { style: "font-size: 11px; color: var(--q-text-secondary);", "The meeting is being recorded to R2 storage." }
                        }
                    }
                }

                // ── Room ID ──
                div { style: "display: flex; flex-direction: column; gap: 6px;",
                    label { style: "font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.05em; color: var(--q-text-secondary);", "Room ID" }
                    code {
                        style: "font-size: 12px; padding: 6px 10px; background: var(--q-bg); border-radius: var(--q-radius); color: var(--q-text-secondary); word-break: break-all;",
                        "{room_id}"
                    }
                }
            }
        }
    }
}

/// A single statistics row (label + value).
#[component]
fn StatRow(label: String, value: String, value_color: Option<String>) -> Element {
    rsx! {
        div { style: "display: flex; justify-content: space-between; align-items: center; padding: 4px 0;",
            span { style: "font-size: 13px; color: var(--q-text-secondary);", "{label}" }
            span { style: "font-size: 13px; font-weight: 500; color: {value_color.unwrap_or_else(|| \"var(--q-text)\".to_string())};", "{value}" }
        }
    }
}

/// Get the base URL for building invite links.
fn get_base_url() -> String {
    if cfg!(target_arch = "wasm32") {
        if let Some(location) = web_sys::window().and_then(|w| w.location()) {
            let proto = location.protocol().unwrap_or_default();
            let host = location.host().unwrap_or_default();
            return format!("{}//{}", proto.trim_end_matches(':'), host);
        }
    }
    "https://quanghub.app".to_string()
}

/// Format seconds as HH:MM:SS or MM:SS.
fn format_duration(total_seconds: u64) -> String {
    let hours = total_seconds / 3600;
    let mins = (total_seconds % 3600) / 60;
    let secs = total_seconds % 60;
    if hours > 0 {
        format!("{:02}:{:02}:{:02}", hours, mins, secs)
    } else {
        format!("{:02}:{:02}", mins, secs)
    }
}
