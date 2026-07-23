//! MeetSchedule — schedule a meeting for a future date/time.
//!
//! Form for creating a scheduled meeting with title, description,
//! date/time, duration, participant invites, and optional
//! AI agent participation and recording preferences.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

/// MeetSchedule page — form to create a future meeting.
#[component]
pub fn MeetSchedule() -> Element {
    let navigator = use_navigator();

    // ── Form state ──
    let title = use_signal(|| String::new());
    let description = use_signal(|| String::new());
    let date = use_signal(|| String::new());
    let time = use_signal(|| String::new());
    let duration = use_signal(|| "30".to_string());
    let allow_ai = use_signal(|| true);
    let allow_recording = use_signal(|| true);
    let invite_emails = use_signal(|| String::new());

    let submitted = use_signal(|| false);

    // ── Handlers ──

    let handle_submit = move |_| {
        // Validation
        if title.read().is_empty() {
            return;
        }
        // TODO: POST /api/meet/rooms with scheduled_at field
        submitted.set(true);
    };

    let handle_schedule_another = move |_| {
        title.set(String::new());
        description.set(String::new());
        date.set(String::new());
        time.set(String::new());
        duration.set("30".to_string());
        invite_emails.set(String::new());
        submitted.set(false);
    };

    let handle_go_home = move |_| {
        navigator.push(route!(MeetHome {}));
    };

    // Populate default date/time if empty
    if date.read().is_empty() {
        let now = chrono::Local::now();
        // Set default to tomorrow at 10:00
        let tomorrow = now + chrono::Duration::days(1);
        date.set(tomorrow.format("%Y-%m-%d").to_string());
        time.set("10:00".to_string());
    }

    // ── Render ──

    if submitted() {
        return rsx! {
            div { style: "max-width: 600px; margin: 0 auto; padding: 48px 24px; text-align: center;",
                div { style: "font-size: 64px; margin-bottom: 24px;", "✅" }
                h1 { style: "font-size: 28px; font-weight: 700; margin-bottom: 12px;", "Meeting Scheduled!" }
                p { style: "color: var(--q-text-secondary); font-size: 16px; margin-bottom: 8px;",
                    "\"{title_read}\" has been scheduled." where title_read = title.read().clone()
                }
                p { style: "color: var(--q-text-secondary); font-size: 14px; margin-bottom: 32px;",
                    "Invitations will be sent to attendees."
                }
                div { style: "display: flex; gap: 12px; justify-content: center;",
                    button {
                        class: "btn-primary",
                        onclick: handle_schedule_another,
                        "Schedule Another"
                    }
                    button {
                        class: "btn-ghost",
                        onclick: handle_go_home,
                        "Back to Meetings"
                    }
                }
            }
        };
    }

    rsx! {
        div { class: "meet-schedule",
            style: "max-width: 640px; margin: 0 auto; padding: 24px;",

            h1 { style: "font-size: 28px; font-weight: 700; margin-bottom: 8px;", "Schedule a Meeting" }
            p { style: "color: var(--q-text-secondary); font-size: 14px; margin-bottom: 32px;",
                "Set up a meeting for a future time. Attendees will receive a link."
            }

            div { class: "card", style: "padding: 32px;",
                form {
                    style: "display: flex; flex-direction: column; gap: 20px;",
                    onsubmit: move |e| {
                        e.prevent_default();
                        handle_submit(());
                    },

                    // Title
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "font-size: 13px; font-weight: 600; color: var(--q-text-secondary);", "Meeting Title *" }
                        input {
                            placeholder: "e.g. Sprint Planning",
                            value: "{title}",
                            oninput: move |e| title.set(e.value()),
                            required: true,
                            style: "width: 100%;",
                        }
                    }

                    // Description
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "font-size: 13px; font-weight: 600; color: var(--q-text-secondary);", "Description (optional)" }
                        textarea {
                            placeholder: "Agenda, goals, or notes...",
                            value: "{description}",
                            oninput: move |e| description.set(e.value()),
                            rows: 3,
                            style: "width: 100%; resize: vertical; font-family: inherit;",
                        }
                    }

                    // Date, time, duration in a row
                    div { style: "display: flex; gap: 16px; flex-wrap: wrap;",
                        div { style: "flex: 1; min-width: 160px; display: flex; flex-direction: column; gap: 6px;",
                            label { style: "font-size: 13px; font-weight: 600; color: var(--q-text-secondary);", "Date *" }
                            input {
                                r#type: "date",
                                value: "{date}",
                                oninput: move |e| date.set(e.value()),
                                required: true,
                                style: "width: 100%;",
                            }
                        }
                        div { style: "flex: 1; min-width: 120px; display: flex; flex-direction: column; gap: 6px;",
                            label { style: "font-size: 13px; font-weight: 600; color: var(--q-text-secondary);", "Time *" }
                            input {
                                r#type: "time",
                                value: "{time}",
                                oninput: move |e| time.set(e.value()),
                                required: true,
                                style: "width: 100%;",
                            }
                        }
                        div { style: "flex: 1; min-width: 120px; display: flex; flex-direction: column; gap: 6px;",
                            label { style: "font-size: 13px; font-weight: 600; color: var(--q-text-secondary);", "Duration" }
                            select {
                                value: "{duration}",
                                oninput: move |e| duration.set(e.value()),
                                style: "width: 100%;",
                                option { value: "15", "15 min" }
                                option { value: "30", "30 min" }
                                option { value: "45", "45 min" }
                                option { value: "60", "1 hour" }
                                option { value: "90", "1.5 hours" }
                                option { value: "120", "2 hours" }
                            }
                        }
                    }

                    // Invite emails
                    div { style: "display: flex; flex-direction: column; gap: 6px;",
                        label { style: "font-size: 13px; font-weight: 600; color: var(--q-text-secondary);", "Invite by email (comma-separated)" }
                        input {
                            placeholder: "alice@example.com, bob@example.com",
                            value: "{invite_emails}",
                            oninput: move |e| invite_emails.set(e.value()),
                            style: "width: 100%;",
                        }
                    }

                    // Toggles row
                    div { style: "display: flex; gap: 24px; flex-wrap: wrap;",
                        label { style: "display: flex; align-items: center; gap: 8px; font-size: 14px; cursor: pointer;",
                            input {
                                r#type: "checkbox",
                                checked: "{allow_ai}",
                                oninput: move |e| allow_ai.set(e.checked()),
                            }
                            span { "Allow AI agent" }
                        }
                        label { style: "display: flex; align-items: center; gap: 8px; font-size: 14px; cursor: pointer;",
                            input {
                                r#type: "checkbox",
                                checked: "{allow_recording}",
                                oninput: move |e| allow_recording.set(e.checked()),
                            }
                            span { "Allow recording" }
                        }
                    }

                    // Submit
                    div { style: "display: flex; gap: 12px; justify-content: flex-end; padding-top: 8px;",
                        Link {
                            to: route!(MeetHome {}),
                            class: "btn-ghost",
                            style: "padding: 10px 20px; display: inline-flex; align-items: center; text-decoration: none;",
                            "Cancel"
                        }
                        button {
                            class: "btn-primary",
                            style: "padding: 10px 24px; font-size: 15px;",
                            r#type: "submit",
                            "Schedule Meeting"
                        }
                    }
                }
            }
        }
    }
}
