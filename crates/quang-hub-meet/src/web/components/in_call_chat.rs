//! InCallChat — chat sidebar during a meeting.
//!
//! Displays the in-call message stream with text input at the bottom.
//! Supports sending public messages to the room.

use dioxus::prelude::*;

/// A single chat message inside the meeting.
#[derive(Debug, Clone)]
struct ChatMessage {
    id: String,
    author: String,
    text: String,
    is_local: bool,
    timestamp: String,
}

/// In-call chat sidebar component.
#[component]
pub fn InCallChat() -> Element {
    // ── Local state ──
    let messages = use_signal(|| {
        vec![
            ChatMessage {
                id: "1".into(),
                author: "Alice".into(),
                text: "Hey everyone!".into(),
                is_local: false,
                timestamp: "10:02 AM".into(),
            },
            ChatMessage {
                id: "2".into(),
                author: "Bob".into(),
                text: "I'll share my screen in a moment.".into(),
                is_local: false,
                timestamp: "10:03 AM".into(),
            },
        ]
    });

    let input_text = use_signal(|| String::new());
    let container_ref = use_node_ref();

    // ── Handlers ──

    let send_message = move |_| {
        let text = input_text.read().clone();
        if text.trim().is_empty() {
            return;
        }
        let now = chrono::Local::now().format("%I:%M %p").to_string();
        messages.write().push(ChatMessage {
            id: uuid::Uuid::new_v4().to_string(),
            author: "You".into(),
            text: text.trim().to_string(),
            is_local: true,
            timestamp: now,
        });
        input_text.set(String::new());

        // TODO: Send over signaling channel
        // signaling.send(ChatMessage { ... })
    };

    let handle_keydown = move |e: keyboard::KeyboardEvent| {
        if e.key() == "Enter" && !e.shift_key() {
            e.prevent_default();
            send_message(());
        }
    };

    rsx! {
        div {
            class: "in-call-chat",
            style: "display: flex; flex-direction: column; height: 100%;",

            // Header
            div {
                style: "padding: 16px; border-bottom: 1px solid var(--q-border); font-weight: 600; font-size: 14px;",
                "In-Call Chat"
            }

            // Messages area
            div {
                class: "chat-messages",
                style: "flex: 1; overflow-y: auto; padding: 12px; display: flex; flex-direction: column; gap: 8px;",

                for msg in messages.read().iter() {
                    div {
                        style: "display: flex; flex-direction: column; align-items: {if msg.is_local { \"flex-end\" } else { \"flex-start\" }}; gap: 2px;",

                        // Author + timestamp row
                        div {
                            style: "display: flex; align-items: center; gap: 6px; font-size: 11px; color: var(--q-text-secondary); padding: 0 4px;",
                            span { "{msg.author}" }
                            span { "{msg.timestamp}" }
                        }

                        // Message bubble
                        div {
                            style: "max-width: 85%; padding: 8px 12px; border-radius: 12px; font-size: 13px; line-height: 1.4; \
                                background: {if msg.is_local { \"var(--q-primary)\" } else { \"var(--q-bg)\" }}; \
                                color: {if msg.is_local { \"white\" } else { \"var(--q-text)\" }}; \
                                border: {if msg.is_local { \"none\" } else { \"1px solid var(--q-border)\" }};",
                            "{msg.text}"
                        }
                    }
                }
            }

            // Input area
            div {
                style: "padding: 12px; border-top: 1px solid var(--q-border); display: flex; gap: 8px; align-items: flex-end;",

                textarea {
                    placeholder: "Type a message...",
                    value: "{input_text}",
                    oninput: move |e| input_text.set(e.value()),
                    onkeydown: handle_keydown,
                    rows: 2,
                    style: "flex: 1; resize: none; font-family: inherit; font-size: 13px; padding: 8px 12px; border-radius: var(--q-radius); background: var(--q-bg); border: 1px solid var(--q-border); color: var(--q-text); outline: none;",
                }

                button {
                    class: "btn-primary",
                    style: "padding: 8px 16px; font-size: 13px; flex-shrink: 0; height: 36px;",
                    onclick: send_message,
                    disabled: input_text.read().trim().is_empty(),
                    "Send"
                }
            }
        }
    }
}
