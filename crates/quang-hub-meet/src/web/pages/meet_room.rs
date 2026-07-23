//! MeetRoom — active meeting room with video grid, controls, chat sidebar.
//!
//! The main in-call experience. Manages the WebRTC peer connection,
//! local media streams, participant grid, media controls, chat,
//! screen share, and meeting info panel.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::web::components::in_call_chat::InCallChat;
use crate::web::components::media_controls::MediaControls;
use crate::web::components::meeting_info_panel::MeetingInfoPanel;
use crate::web::components::participant_bar::ParticipantBar;
use crate::web::components::screen_share_overlay::ScreenShareOverlay;
use crate::web::components::video_grid::VideoGrid;
use crate::web::components::video_tile::ParticipantView;
use crate::web::webrtc::PeerConnectionManager;

/// Room-level state shared across all MeetRoom child components.
pub struct RoomState {
    pub room_id: String,
    pub local_participant_id: String,
    pub participants: Vec<ParticipantView>,
    pub is_mic_on: bool,
    pub is_camera_on: bool,
    pub is_screen_sharing: bool,
    pub is_recording: bool,
    pub duration_seconds: u64,
    pub show_chat: bool,
    pub show_participants: bool,
    pub show_info: bool,
    pub show_screen_share: bool,
    pub screen_share_participant: Option<String>,
    pub peer_manager: Option<PeerConnectionManager>,
}

/// Props injected by the Dioxus router.
#[derive(Props, Clone, PartialEq)]
pub struct MeetRoomProps {
    pub room_id: String,
}

/// MeetRoom page — the active meeting.
#[component]
pub fn MeetRoom(room_id: String) -> Element {
    let loc = use_location();

    // ── Core room state ──
    // We provide this via context so child components can read/modify it.
    let participants = use_signal(|| {
        vec![
            ParticipantView {
                id: "local".into(),
                display_name: "You".into(),
                avatar_url: None,
                is_mic_on: true,
                is_camera_on: true,
                is_screen_sharing: false,
                is_hand_raised: false,
                is_local: true,
                is_agent: false,
                video_track_id: None,
            },
            ParticipantView {
                id: "remote-1".into(),
                display_name: "Alice".into(),
                avatar_url: None,
                is_mic_on: true,
                is_camera_on: true,
                is_screen_sharing: false,
                is_hand_raised: false,
                is_local: false,
                is_agent: false,
                video_track_id: None,
            },
        ]
    });
    let is_mic_on = use_signal(|| true);
    let is_camera_on = use_signal(|| true);
    let is_screen_sharing = use_signal(|| false);
    let is_recording = use_signal(|| false);
    let show_chat = use_signal(|| false);
    let show_participants = use_signal(|| true);
    let show_info = use_signal(|| false);
    let show_screen_share = use_signal(|| false);
    let screen_share_participant = use_signal(|| None::<String>);
    let duration_seconds = use_signal(|| 0u64);

    // ── Duration ticker ──
    use_effect(move || {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(1));
        spawn(async move {
            loop {
                interval.tick().await;
                duration_seconds += 1;
            }
        });
        // Effect runs once — the spawn continues indefinitely
        || ()
    });

    // ── WebRTC peer manager ──
    // In a real build, this would connect to the DurableObject signaling endpoint.
    let peer_manager = use_signal(|| None::<PeerConnectionManager>);

    // ── Context provider for child components ──
    // Provide a simplified context object that components can access.

    // ── Sidebar width based on what's open ──
    let sidebar_visible = show_chat() || show_participants() || show_info();
    let sidebar_width = if show_chat() || show_participants() {
        "320px"
    } else {
        "280px"
    };

    rsx! {
        div {
            class: "meet-room",
            style: "display: flex; height: calc(100vh - var(--q-topbar-height)); background: var(--q-bg); overflow: hidden; position: relative;",

            // ── Main video area ──
            div {
                style: "flex: 1; display: flex; flex-direction: column; min-width: 0;",

                // Top bar: room name, recording indicator, timer
                div {
                    style: "display: flex; align-items: center; justify-content: space-between; padding: 8px 16px; background: var(--q-surface); border-bottom: 1px solid var(--q-border); flex-shrink: 0;",

                    // Room title + recording indicator
                    div { style: "display: flex; align-items: center; gap: 12px;",
                        h2 { style: "font-size: 16px; font-weight: 600;", "Meeting Room" }
                        if is_recording() {
                            div { style: "display: flex; align-items: center; gap: 6px; color: var(--q-danger); font-size: 12px; font-weight: 600;",
                                div { style: "width: 8px; height: 8px; border-radius: 50%; background: var(--q-danger); animation: q-pulse 1.5s ease-in-out infinite;", "" }
                                span { "REC" }
                            }
                        }
                    }

                    // Duration
                    div { style: "font-size: 14px; color: var(--q-text-secondary); font-variant-numeric: tabular-nums;",
                        "{format_duration(duration_seconds())}"
                    }

                    // Action buttons
                    div { style: "display: flex; gap: 8px;",
                        button {
                            class: "btn-ghost",
                            style: "padding: 6px 12px; font-size: 13px; {if show_info() { \"background: var(--q-surface-hover); color: var(--q-primary);\" }}",
                            onclick: move |_| {
                                show_info.set(!show_info());
                                if show_info() {
                                    show_chat.set(false);
                                    show_participants.set(false);
                                }
                            },
                            "Info"
                        }
                        button {
                            class: "btn-ghost",
                            style: "padding: 6px 12px; font-size: 13px; {if show_participants() { \"background: var(--q-surface-hover); color: var(--q-primary);\" }}",
                            onclick: move |_| {
                                show_participants.set(!show_participants());
                                if show_participants() {
                                    show_chat.set(false);
                                    show_info.set(false);
                                }
                            },
                            "Participants ({participants_count})" where participants_count = participants.read().len()
                        }
                        button {
                            class: "btn-ghost",
                            style: "padding: 6px 12px; font-size: 13px; {if show_chat() { \"background: var(--q-surface-hover); color: var(--q-primary);\" }}",
                            onclick: move |_| {
                                show_chat.set(!show_chat());
                                if show_chat() {
                                    show_participants.set(false);
                                    show_info.set(false);
                                }
                            },
                            "Chat"
                        }
                    }
                }

                // ── Video grid (takes remaining space) ──
                div {
                    style: "flex: 1; overflow: hidden;",
                    VideoGrid {
                        participants: participants.read().clone(),
                        screen_sharing: is_screen_sharing(),
                    }
                }

                // ── Media controls bar (bottom) ──
                div {
                    style: "flex-shrink: 0; padding: 12px 16px; background: var(--q-surface); border-top: 1px solid var(--q-border);",
                    MediaControls {
                        is_mic_on: is_mic_on(),
                        is_camera_on: is_camera_on(),
                        is_screen_sharing: is_screen_sharing(),
                        is_recording: is_recording(),
                        on_toggle_mic: move |_| { is_mic_on.set(!is_mic_on()); },
                        on_toggle_camera: move |_| { is_camera_on.set(!is_camera_on()); },
                        on_toggle_screen_share: move |_| {
                            let new_val = !is_screen_sharing();
                            is_screen_sharing.set(new_val);
                            show_screen_share.set(new_val);
                        },
                        on_toggle_recording: move |_| { is_recording.set(!is_recording()); },
                        on_leave: move |_| {
                            // Navigate back to Meet home
                            let nav = use_navigator();
                            nav.push(route!(crate::pages::meet_home::MeetHome {}));
                        },
                    }
                }
            }

            // ── Right sidebar ──
            if show_chat() {
                div {
                    style: "width: 320px; flex-shrink: 0; border-left: 1px solid var(--q-border); background: var(--q-surface); display: flex; flex-direction: column;",
                    InCallChat {}
                }
            } else if show_participants() {
                div {
                    style: "width: 280px; flex-shrink: 0; border-left: 1px solid var(--q-border); background: var(--q-surface); overflow-y: auto;",
                    ParticipantBar {
                        participants: participants.read().clone(),
                    }
                }
            } else if show_info() {
                div {
                    style: "width: 280px; flex-shrink: 0; border-left: 1px solid var(--q-border); background: var(--q-surface); overflow-y: auto;",
                    MeetingInfoPanel {
                        room_id: room_id.clone(),
                        duration_seconds: duration_seconds(),
                        is_recording: is_recording(),
                        participant_count: participants.read().len(),
                    }
                }
            }

            // ── Screen share overlay ──
            if show_screen_share() {
                ScreenShareOverlay {
                    participant_name: "You".into(),
                    on_close: move |_| {
                        is_screen_sharing.set(false);
                        show_screen_share.set(false);
                    },
                }
            }
        }
    }
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
