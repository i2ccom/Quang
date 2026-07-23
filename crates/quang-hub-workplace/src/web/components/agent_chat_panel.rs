//! AgentChatPanel — an agent-specific chat panel for human-agent communication.
//!
//! This panel shows conversations between humans and AI agents in a
//! dedicated interface. It features agent typing indicators, structured
//! message formatting (code blocks, JSON, markdown), and capability buttons.

use dioxus::prelude::*;

/// An agent chat message.
#[derive(Clone, PartialEq, Debug)]
pub struct AgentChatMessage {
    pub id: String,
    pub agent_name: String,
    pub agent_avatar: String,
    pub content: String,
    pub timestamp: String,
    pub is_code: bool,
    pub language: Option<String>,
    pub is_typing: bool,
}

/// Props for the AgentChatPanel component.
#[derive(Clone, PartialEq, Props)]
pub struct AgentChatPanelProps {
    pub agent_name: String,
    pub agent_avatar: String,
    pub messages: Vec<AgentChatMessage>,
    pub on_send: EventHandler<String>,
}

/// A chat panel specialized for human-agent conversations.
#[component]
pub fn AgentChatPanel(props: AgentChatPanelProps) -> Element {
    let mut input = use_signal(|| String::new());

    let handle_send = move |_| {
        let text = input().trim().to_string();
        if !text.is_empty() {
            props.on_send.call(text);
            input.set(String::new());
        }
    };

    rsx! {
        div {
            class: "agent-chat-panel",
            style: "
                display: flex;
                flex-direction: column;
                height: 100%;
                background: var(--q-surface, #1a1a2e);
                border-radius: 12px;
                border: 1px solid var(--q-surface-border, #333);
                overflow: hidden;
            ",

            // Header
            div {
                style: "
                    display: flex;
                    align-items: center;
                    gap: 0.75rem;
                    padding: 0.75rem 1rem;
                    background: linear-gradient(135deg, rgba(0, 206, 201, 0.1), rgba(108, 92, 231, 0.1));
                    border-bottom: 1px solid var(--q-surface-border, #333);
                ",
                div {
                    style: "
                        width: 32px;
                        height: 32px;
                        border-radius: 8px;
                        background: linear-gradient(135deg, var(--q-accent, #00cec9), var(--q-primary, #6c5ce7));
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 0.75rem;
                        font-weight: 700;
                        color: #fff;
                    ",
                    "{props.agent_avatar}"
                }
                div {
                    font_size: "0.95rem",
                    font_weight: "600",
                    "{props.agent_name}"
                }
                span {
                    style: "
                        font-size: 0.7rem;
                        padding: 0.1rem 0.4rem;
                        border-radius: 3px;
                        background: rgba(0, 206, 201, 0.15);
                        color: var(--q-accent, #00cec9);
                    ",
                    "Agent"
                }
            }

            // Messages
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 1rem;
                    display: flex;
                    flex-direction: column;
                    gap: 0.75rem;
                ",
                for msg in &props.messages {
                    AgentMessageBubble { message: msg.clone() }
                }

                // Typing indicator
                if props.messages.last().map_or(false, |m| m.is_typing) {
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 0.5rem;
                            color: var(--q-accent, #00cec9);
                            font-size: 0.8rem;
                            padding: 0.5rem;
                        ",
                        div {
                            style: "
                                display: flex;
                                gap: 0.2rem;
                            ",
                            div { style: "width: 6px; height: 6px; border-radius: 50%; background: var(--q-accent, #00cec9); animation: bounce 1.4s infinite;" }
                            div { style: "width: 6px; height: 6px; border-radius: 50%; background: var(--q-accent, #00cec9); animation: bounce 1.4s 0.2s infinite;" }
                            div { style: "width: 6px; height: 6px; border-radius: 50%; background: var(--q-accent, #00cec9); animation: bounce 1.4s 0.4s infinite;" }
                        }
                        span { "{props.agent_name} is typing..." }
                    }
                }
            }

            // Input
            div {
                style: "
                    padding: 0.75rem 1rem;
                    border-top: 1px solid var(--q-surface-border, #333);
                ",
                form {
                    onsubmit: handle_send,
                    style: "display: flex; gap: 0.5rem;",
                    input {
                        placeholder: "Ask {props.agent_name} something...",
                        value: input(),
                        oninput: move |e| input.set(e.value()),
                        style: "
                            flex: 1;
                            padding: 0.55rem 0.75rem;
                            border-radius: 8px;
                            border: 1px solid var(--q-surface-border, #333);
                            background: var(--q-bg, #0f0f1a);
                            color: var(--q-text, #e0e0e0);
                            font-size: 0.85rem;
                            outline: none;
                        "
                    }
                    button {
                        r#type: "submit",
                        style: "
                            padding: 0.55rem 1rem;
                            border-radius: 8px;
                            border: none;
                            background: var(--q-accent, #00cec9);
                            color: #fff;
                            font-size: 0.85rem;
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

/// A single message bubble in the agent chat, with code block support.
#[component]
fn AgentMessageBubble(message: AgentChatMessage) -> Element {
    let is_user = !message.is_code && message.agent_name == "You";

    rsx! {
        div {
            class: "agent-message-bubble",
            style: if is_user {
                "
                    display: flex;
                    flex-direction: column;
                    align-items: flex-end;
                "
            } else {
                "
                    display: flex;
                    flex-direction: column;
                    align-items: flex-start;
                "
            },

            div {
                style: if is_user {
                    "
                        max-width: 80%;
                        padding: 0.6rem 0.85rem;
                        border-radius: 12px 4px 12px 12px;
                        background: var(--q-primary, #6c5ce7);
                        color: #fff;
                        font-size: 0.85rem;
                        line-height: 1.45;
                    "
                } else if message.is_code {
                    "
                        max-width: 90%;
                        padding: 0.75rem;
                        border-radius: 8px;
                        background: #0d1117;
                        border: 1px solid var(--q-surface-border, #333);
                        font-family: 'Fira Code', 'Consolas', monospace;
                        font-size: 0.8rem;
                        color: #e6edf3;
                        overflow-x: auto;
                        white-space: pre;
                    "
                } else {
                    "
                        max-width: 80%;
                        padding: 0.6rem 0.85rem;
                        border-radius: 4px 12px 12px 12px;
                        background: var(--q-bg, #0f0f1a);
                        color: var(--q-text, #e0e0e0);
                        font-size: 0.85rem;
                        line-height: 1.45;
                        border: 1px solid var(--q-surface-border, #333);
                    "
                },
                if message.is_code && message.language.is_some() {
                    div {
                        style: "
                            font-size: 0.7rem;
                            color: var(--q-text-muted, #555);
                            margin-bottom: 0.4rem;
                            text-transform: uppercase;
                            font-family: 'Inter', sans-serif;
                        ",
                        "{message.language.as_ref().unwrap()}"
                    }
                }
                "{message.content}"
            }

            // Timestamp
            span {
                style: "
                    font-size: 0.65rem;
                    color: var(--q-text-muted, #555);
                    margin-top: 0.2rem;
                    padding: 0 0.25rem;
                ",
                "{message.timestamp}"
            }
        }
    }
}
