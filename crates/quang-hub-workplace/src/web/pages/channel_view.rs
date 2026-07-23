//! Channel View page — chat channel with message list and input.
//!
//! Displays a channel's messages in a scrollable list with a message
//! composition input at the bottom. Supports reactions via emoji buttons.

use dioxus::prelude::*;

use crate::web::components::channel_list::ChannelList;
use crate::web::components::chat_panel::ChatPanel;

/// Stub channel data.
#[derive(Clone, Debug)]
struct ChannelStub {
    id: String,
    name: String,
    topic: String,
    is_private: bool,
    unread: usize,
}

/// Stub message data.
#[derive(Clone, Debug)]
struct MessageStub {
    id: String,
    author: String,
    author_avatar: String,
    content: String,
    timestamp: String,
    reaction_count: usize,
    is_edited: bool,
}

/// Sample channels.
fn sample_channels() -> Vec<ChannelStub> {
    vec![
        ChannelStub {
            id: "ch1".into(),
            name: "general".into(),
            topic: "General discussion".into(),
            is_private: false,
            unread: 0,
        },
        ChannelStub {
            id: "ch2".into(),
            name: "engineering".into(),
            topic: "Engineering team chat".into(),
            is_private: false,
            unread: 3,
        },
        ChannelStub {
            id: "ch3".into(),
            name: "design".into(),
            topic: "Design critiques".into(),
            is_private: false,
            unread: 1,
        },
        ChannelStub {
            id: "ch4".into(),
            name: "agent-logs".into(),
            topic: "AI agent activity feed".into(),
            is_private: false,
            unread: 7,
        },
        ChannelStub {
            id: "ch5".into(),
            name: "standup".into(),
            topic: "Daily standup updates".into(),
            is_private: true,
            unread: 0,
        },
        ChannelStub {
            id: "ch6".into(),
            name: "releases".into(),
            topic: "Release announcements".into(),
            is_private: false,
            unread: 2,
        },
    ]
}

/// Sample messages for the active channel.
fn sample_messages() -> Vec<MessageStub> {
    vec![
        MessageStub {
            id: "m1".into(), author: "Alice".into(), author_avatar: "A".into(),
            content: "Hey team, I just pushed the OAuth flow implementation to the review board. Can someone take a look?".into(),
            timestamp: "10:32 AM".into(), reaction_count: 2, is_edited: false,
        },
        MessageStub {
            id: "m2".into(), author: "Bob".into(), author_avatar: "B".into(),
            content: "Sure, I'll review it after standup. The GraphQL schema changes look good too.".into(),
            timestamp: "10:33 AM".into(), reaction_count: 1, is_edited: false,
        },
        MessageStub {
            id: "m3".into(), author: "Carol".into(), author_avatar: "C".into(),
            content: "I noticed the task status transitions PR is in review. @Bob can you check the state machine logic? I want to make sure `changes_requested` → `in_review` works correctly.".into(),
            timestamp: "10:35 AM".into(), reaction_count: 0, is_edited: false,
        },
        MessageStub {
            id: "m4".into(), author: "Agent-X".into(), author_avatar: "X".into(),
            content: "🤖 **Agent Alert:** I've completed the dependency graph analysis for the HyperGraph module. No circular dependencies detected. Full report is in #agent-logs.".into(),
            timestamp: "10:38 AM".into(), reaction_count: 4, is_edited: true,
        },
        MessageStub {
            id: "m5".into(), author: "Alice".into(), author_avatar: "A".into(),
            content: "Great work Agent-X! Let's plan to merge the OAuth PR by EOD if we get two approvals.".into(),
            timestamp: "10:40 AM".into(), reaction_count: 3, is_edited: false,
        },
        MessageStub {
            id: "m6".into(), author: "Dave".into(), author_avatar: "D".into(),
            content: "I've updated the D1 schema to include the new `review_comments` table. Ready for review.".into(),
            timestamp: "10:45 AM".into(), reaction_count: 1, is_edited: false,
        },
    ]
}

/// Channel View page — the main chat interface.
#[component]
pub fn ChannelView() -> Element {
    let channels = use_signal(|| sample_channels());
    let messages = use_signal(|| sample_messages());
    let mut active_channel = use_signal(|| "ch1".to_string());
    let mut input_text = use_signal(|| String::new());

    let current_channel = channels()
        .iter()
        .find(|c| c.id == active_channel())
        .cloned();

    let on_send = move |_| {
        let text = input_text().trim().to_string();
        if text.is_empty() {
            return;
        }
        let msg = MessageStub {
            id: format!("m_{}", messages().len() + 1),
            author: "You".into(),
            author_avatar: "U".into(),
            content: text,
            timestamp: "Just now".into(),
            reaction_count: 0,
            is_edited: false,
        };
        messages.write().push(msg);
        input_text.set(String::new());
    };

    rsx! {
        div {
            class: "channel-view",
            style: "
                display: flex;
                min-height: 100vh;
                background: var(--q-bg, #0f0f1a);
                color: var(--q-text, #e0e0e0);
                font-family: 'Inter', system-ui, sans-serif;
            ",

            // ── Sidebar: Channel List ──
            ChannelList {
                channels: channels(),
                active_channel_id: active_channel(),
                on_channel_click: move |id| {
                    active_channel.set(id);
                }
            }

            // ── Main: Chat Panel ──
            div {
                style: "
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                ",

                // Channel header
                if let Some(ref ch) = current_channel {
                    div {
                        style: "
                            padding: 0.75rem 1.5rem;
                            background: var(--q-surface, #1a1a2e);
                            border-bottom: 1px solid var(--q-surface-border, #333);
                            display: flex;
                            align-items: center;
                            gap: 0.75rem;
                        ",
                        span {
                            style: "
                                font-size: 0.85rem;
                                color: var(--q-text-muted, #555);
                            ",
                            "#"
                        }
                        span {
                            style: "
                                font-size: 1.05rem;
                                font-weight: 600;
                            ",
                            "{ch.name}"
                        }
                        span {
                            style: "
                                font-size: 0.8rem;
                                color: var(--q-text-muted, #555);
                                margin-left: 0.5rem;
                            ",
                            "{ch.topic}"
                        }
                        if ch.is_private {
                            span {
                                style: "
                                    font-size: 0.7rem;
                                    padding: 0.1rem 0.35rem;
                                    border-radius: 3px;
                                    background: rgba(255, 71, 87, 0.15);
                                    color: #ff4757;
                                ",
                                "private"
                            }
                        }
                    }
                }

                // Messages area
                div {
                    style: "
                        flex: 1;
                        overflow-y: auto;
                        padding: 1rem 1.5rem;
                        display: flex;
                        flex-direction: column;
                        gap: 0.75rem;
                    ",
                    for msg in messages() {
                        ChatMessageCard { message: msg }
                    }
                }

                // Input area
                div {
                    style: "
                        padding: 0.75rem 1.5rem;
                        background: var(--q-surface, #1a1a2e);
                        border-top: 1px solid var(--q-surface-border, #333);
                    ",
                    form {
                        onsubmit: on_send,
                        style: "display: flex; gap: 0.75rem;",
                        input {
                            placeholder: "Message #{current_channel.as_ref().map(|c| c.name.as_str()).unwrap_or("channel")}",
                            value: input_text(),
                            oninput: move |e| input_text.set(e.value()),
                            style: "
                                flex: 1;
                                padding: 0.65rem 0.85rem;
                                border-radius: 8px;
                                border: 1px solid var(--q-surface-border, #333);
                                background: var(--q-bg, #0f0f1a);
                                color: var(--q-text, #e0e0e0);
                                font-size: 0.9rem;
                                outline: none;
                            "
                        }
                        button {
                            r#type: "submit",
                            style: "
                                padding: 0.65rem 1.25rem;
                                border-radius: 8px;
                                border: none;
                                background: var(--q-primary, #6c5ce7);
                                color: #fff;
                                font-size: 0.9rem;
                                font-weight: 500;
                                cursor: pointer;
                            ",
                            "Send"
                        }
                    }
                }
            }
        }
    }
}

/// A single chat message card.
#[component]
fn ChatMessageCard(message: MessageStub) -> Element {
    let is_agent = message.author.starts_with("Agent");
    let avatar_color = if is_agent {
        "var(--q-accent, #00cec9)"
    } else {
        "var(--q-primary, #6c5ce7)"
    };

    rsx! {
        div {
            class: "chat-message",
            style: "
                display: flex;
                gap: 0.75rem;
                padding: 0.5rem 0.75rem;
                border-radius: 8px;
                transition: background 0.15s;
            ",

            // Avatar
            div {
                style: "
                    width: 32px;
                    height: 32px;
                    border-radius: 6px;
                    background: {avatar_color};
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 0.75rem;
                    font-weight: 700;
                    flex-shrink: 0;
                    color: #fff;
                ",
                "{message.author_avatar}"
            }

            // Content
            div {
                style: "flex: 1; min-width: 0;",
                div {
                    style: "
                        display: flex;
                        align-items: baseline;
                        gap: 0.5rem;
                        margin-bottom: 0.15rem;
                    ",
                    span {
                        style: "
                            font-weight: 600;
                            font-size: 0.85rem;
                        ",
                        "{message.author}"
                    }
                    span {
                        style: "
                            font-size: 0.75rem;
                            color: var(--q-text-muted, #555);
                        ",
                        "{message.timestamp}"
                    }
                    if message.is_edited {
                        span {
                            style: "
                                font-size: 0.7rem;
                                color: var(--q-text-muted, #555);
                                font-style: italic;
                            ",
                            "(edited)"
                        }
                    }
                }
                p {
                    style: "
                        font-size: 0.9rem;
                        margin: 0;
                        line-height: 1.5;
                        color: var(--q-text, #e0e0e0);
                        white-space: pre-wrap;
                        word-break: break-word;
                    ",
                    "{message.content}"
                }
                if message.reaction_count > 0 {
                    div {
                        style: "
                            margin-top: 0.35rem;
                            display: flex;
                            align-items: center;
                            gap: 0.35rem;
                            font-size: 0.8rem;
                            color: var(--q-text-secondary, #888);
                        ",
                        span {
                            style: "
                                padding: 0.1rem 0.45rem;
                                border-radius: 4px;
                                background: var(--q-bg, #0f0f1a);
                                border: 1px solid var(--q-surface-border, #333);
                                cursor: pointer;
                            ",
                            "👍 {message.reaction_count}"
                        }
                    }
                }
            }
        }
    }
}
