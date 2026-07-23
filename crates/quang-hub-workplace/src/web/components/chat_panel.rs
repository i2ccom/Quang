//! ChatPanel — a composable chat message feed component.
//!
//! Renders a scrollable list of chat messages with author, timestamp,
//! and content. Supports agent/human differentiation via styling.

use dioxus::prelude::*;

/// A single chat message for display.
#[derive(Clone, PartialEq, Debug)]
pub struct ChatMessageItem {
    pub id: String,
    pub author: String,
    pub author_avatar: String,
    pub content: String,
    pub timestamp: String,
    pub is_agent: bool,
    pub is_edited: bool,
    pub reactions: Vec<String>,
}

/// Props for the ChatPanel component.
#[derive(Clone, PartialEq, Props)]
pub struct ChatPanelProps {
    pub messages: Vec<ChatMessageItem>,
}

/// A scrollable chat message feed.
#[component]
pub fn ChatPanel(props: ChatPanelProps) -> Element {
    rsx! {
        div {
            class: "chat-panel",
            style: "
                flex: 1;
                overflow-y: auto;
                padding: 1rem;
                display: flex;
                flex-direction: column;
                gap: 0.35rem;
            ",

            if props.messages.is_empty() {
                div {
                    style: "
                        flex: 1;
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        color: var(--q-text-muted, #555);
                        font-size: 0.9rem;
                    ",
                    "No messages yet. Start a conversation!"
                }
            } else {
                for msg in &props.messages {
                    ChatBubble { message: msg.clone() }
                }
            }
        }
    }
}

/// A single chat message bubble.
#[component]
fn ChatBubble(message: ChatMessageItem) -> Element {
    let avatar_color = if message.is_agent {
        "var(--q-accent, #00cec9)"
    } else {
        "var(--q-primary, #6c5ce7)"
    };

    rsx! {
        div {
            class: "chat-bubble",
            style: "
                display: flex;
                gap: 0.65rem;
                padding: 0.35rem 0.5rem;
                border-radius: 6px;
                transition: background 0.15s;
            ",

            // Avatar
            div {
                style: "
                    width: 30px;
                    height: 30px;
                    border-radius: 6px;
                    background: {avatar_color};
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 0.7rem;
                    font-weight: 700;
                    flex-shrink: 0;
                    color: #fff;
                ",
                "{message.author_avatar}"
            }

            // Content column
            div {
                style: "flex: 1; min-width: 0;",

                // Author + timestamp row
                div {
                    style: "
                        display: flex;
                        align-items: baseline;
                        gap: 0.5rem;
                        margin-bottom: 0.1rem;
                    ",
                    span {
                        style: "
                            font-weight: 600;
                            font-size: 0.82rem;
                            color: if message.is_agent {
                                "var(--q-accent, #00cec9)"
                            } else {
                                "var(--q-text, #e0e0e0)"
                            }
                        ",
                        "{message.author}"
                    }
                    span {
                        style: "
                            font-size: 0.7rem;
                            color: var(--q-text-muted, #555);
                        ",
                        "{message.timestamp}"
                    }
                    if message.is_edited {
                        span {
                            style: "
                                font-size: 0.65rem;
                                color: var(--q-text-muted, #555);
                                font-style: italic;
                            ",
                            "(edited)"
                        }
                    }
                }

                // Message text
                p {
                    style: "
                        font-size: 0.85rem;
                        margin: 0;
                        line-height: 1.45;
                        color: var(--q-text, #e0e0e0);
                        white-space: pre-wrap;
                        word-break: break-word;
                    ",
                    "{message.content}"
                }

                // Reactions row
                if !message.reactions.is_empty() {
                    div {
                        style: "
                            margin-top: 0.25rem;
                            display: flex;
                            gap: 0.25rem;
                            flex-wrap: wrap;
                        ",
                        for reaction in &message.reactions {
                            span {
                                style: "
                                    font-size: 0.75rem;
                                    padding: 0.05rem 0.35rem;
                                    border-radius: 4px;
                                    background: var(--q-bg, #0f0f1a);
                                    border: 1px solid var(--q-surface-border, #333);
                                    cursor: pointer;
                                ",
                                "{reaction}"
                            }
                        }
                    }
                }
            }
        }
    }
}
