//! MeetHome — room listing, create room, join by link.
//!
//! Landing page for the Meet module. Shows active and scheduled rooms,
//! a form to create a new meeting, and a text input to join via room link.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

/// The meeting room list item type used for display.
struct RoomListItem {
    id: String,
    title: String,
    status: String,
    participant_count: u32,
    host_name: String,
    scheduled_at: Option<String>,
}

/// MeetHome page — list rooms, create, or join.
#[component]
pub fn MeetHome() -> Element {
    let navigator = use_navigator();

    // ── Local state ──
    let rooms = use_signal(|| {
        vec![
            RoomListItem {
                id: "demo-1".into(),
                title: "Weekly Standup".into(),
                status: "active".into(),
                participant_count: 5,
                host_name: "Alice".into(),
                scheduled_at: None,
            },
            RoomListItem {
                id: "demo-2".into(),
                title: "Design Review".into(),
                status: "scheduled".into(),
                participant_count: 0,
                host_name: "Bob".into(),
                scheduled_at: Some("2025-06-02 14:00".into()),
            },
        ]
    });

    let show_create = use_signal(|| false);
    let new_title = use_signal(|| String::new());
    let new_topic = use_signal(|| String::new());
    let join_link = use_signal(|| String::new());

    // ── Handlers ──

    let toggle_create = move |_| {
        show_create.set(!show_create());
    };

    let create_room = move |_| {
        let title = new_title.read().clone();
        let topic = new_topic.read().clone();
        if title.is_empty() {
            return;
        }
        // TODO: POST /api/meet/rooms -> receive room_id, navigate
        let room_id = uuid::Uuid::new_v4().to_string();
        navigator.push(route!(MeetRoom { room_id }));
    };

    let join_room = move |_| {
        let link = join_link.read().clone();
        if link.is_empty() {
            return;
        }
        // Parse room ID from link or use directly
        let room_id = link.trim().trim_start_matches('/').to_string();
        navigator.push(route!(MeetRoom { room_id }));
    };

    let join_room_link = move |room_id: String| {
        navigator.push(route!(MeetRoom { room_id }));
    };

    // ── Render ──

    rsx! {
        div { class: "meet-home",
            style: "max-width: 960px; margin: 0 auto; padding: 24px;",

            // ── Header ──
            div {
                style: "display: flex; align-items: center; justify-content: space-between; margin-bottom: 32px;",
                h1 { style: "font-size: 28px; font-weight: 700;", "Meetings" }
                div { style: "display: flex; gap: 12px;",
                    button {
                        class: "btn-primary",
                        style: "display: flex; align-items: center; gap: 8px; padding: 10px 20px; font-size: 15px;",
                        onclick: toggle_create,
                        span { "＋" }
                        span { "New Meeting" }
                    }
                }
            }

            // ── Create / Join forms ──
            if show_create() {
                div {
                    class: "card",
                    style: "margin-bottom: 24px; padding: 24px;",
                    h3 { style: "font-size: 18px; margin-bottom: 16px;", "Create a Meeting" }

                    div { style: "display: flex; flex-direction: column; gap: 12px;",
                        input {
                            placeholder: "Meeting title...",
                            value: "{new_title}",
                            oninput: move |e| new_title.set(e.value()),
                            style: "width: 100%;",
                        }
                        input {
                            placeholder: "Topic / agenda (optional)...",
                            value: "{new_topic}",
                            oninput: move |e| new_topic.set(e.value()),
                            style: "width: 100%;",
                        }
                        button {
                            class: "btn-primary",
                            style: "align-self: flex-start;",
                            onclick: create_room,
                            "Start Meeting"
                        }
                    }
                }
            }

            // ── Join by link ──
            div {
                class: "card",
                style: "margin-bottom: 24px; padding: 16px 24px; display: flex; align-items: center; gap: 12px;",
                span { style: "color: var(--q-text-secondary); font-size: 14px; white-space: nowrap;", "Join with link:" }
                input {
                    placeholder: "Paste meeting link or room ID...",
                    value: "{join_link}",
                    oninput: move |e| join_link.set(e.value()),
                    style: "flex: 1;",
                    onkeydown: move |e| {
                        if e.key() == "Enter" { join_room(); }
                    },
                }
                button {
                    class: "btn-primary",
                    onclick: join_room,
                    "Join"
                }
            }

            // ── Divider tabs ──
            div { style: "display: flex; gap: 24px; border-bottom: 1px solid var(--q-border); margin-bottom: 20px; padding-bottom: 8px;",
                span { style: "font-size: 14px; font-weight: 600; color: var(--q-primary); border-bottom: 2px solid var(--q-primary); padding-bottom: 8px; margin-bottom: -9px;", "Active" }
                span { style: "font-size: 14px; color: var(--q-text-secondary); cursor: pointer;", "Scheduled" }
                span { style: "font-size: 14px; color: var(--q-text-secondary); cursor: pointer;", "Past" }
            }

            // ── Room list ──
            div { style: "display: flex; flex-direction: column; gap: 12px;",
                for room in rooms.read().iter() {
                    div {
                        class: "card",
                        style: "display: flex; align-items: center; justify-content: space-between; padding: 16px 20px; cursor: pointer; transition: background 0.15s;",
                        onclick: {
                            let id = room.id.clone();
                            move |_| join_room_link(id.clone())
                        },

                        div { style: "display: flex; flex-direction: column; gap: 4px;",
                            div { style: "display: flex; align-items: center; gap: 8px;",
                                h3 { style: "font-size: 16px; font-weight: 600;", "{room.title}" }
                                StatusBadge { status: &room.status }
                            }
                            div { style: "display: flex; gap: 16px; font-size: 13px; color: var(--q-text-secondary);",
                                span { "Host: {room.host_name}" }
                                span { "{room.participant_count} participant{}", if room.participant_count != 1 { "s" } else { "" } }
                                if let Some(ref at) = room.scheduled_at {
                                    span { "Scheduled: {at}" }
                                }
                            }
                        }

                        button {
                            class: "btn-primary",
                            style: "padding: 8px 16px; font-size: 13px;",
                            onclick: {
                                let id = room.id.clone();
                                move |e| {
                                    e.stop_propagation();
                                    join_room_link(id.clone());
                                }
                            },
                            if room.status == "active" { "Join" } else { "View" }
                        }
                    }
                }
            }

            // ── Empty state ──
            if rooms.read().is_empty() {
                div { style: "text-align: center; padding: 64px 24px; color: var(--q-text-secondary);",
                    div { style: "font-size: 48px; margin-bottom: 16px;", "📹" }
                    h3 { style: "font-size: 18px; color: var(--q-text); margin-bottom: 8px;", "No meetings yet" }
                    p { style: "font-size: 14px;", "Create a new meeting or join one with a link." }
                }
            }
        }
    }
}

/// Small colored badge for room status.
#[component]
fn StatusBadge(status: &str) -> Element {
    let (bg, color, label) = match status {
        "active" => ("rgba(81, 207, 102, 0.15)", "var(--q-success)", "Live"),
        "scheduled" => ("rgba(108, 92, 231, 0.15)", "var(--q-primary)", "Scheduled"),
        "paused" => ("rgba(252, 196, 25, 0.15)", "var(--q-warning)", "Paused"),
        "ended" => (
            "rgba(154, 154, 168, 0.15)",
            "var(--q-text-secondary)",
            "Ended",
        ),
        _ => ("rgba(255, 107, 107, 0.15)", "var(--q-danger)", status),
    };

    rsx! {
        span {
            style: "font-size: 11px; font-weight: 600; padding: 2px 8px; border-radius: 999px; background: {bg}; color: {color};",
            "{label}"
        }
    }
}
