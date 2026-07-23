//! ChannelList — a sidebar list of chat channels with unread indicators.

use dioxus::prelude::*;

/// A channel entry for display in the list.
#[derive(Clone, PartialEq, Debug)]
pub struct ChannelListItem {
    pub id: String,
    pub name: String,
    pub topic: String,
    pub is_private: bool,
    pub unread: usize,
}

/// Props for the ChannelList component.
#[derive(Clone, PartialEq, Props)]
pub struct ChannelListProps {
    pub channels: Vec<ChannelListItem>,
    pub active_channel_id: String,
    pub on_channel_click: EventHandler<String>,
}

/// A sidebar list of chat channels with icons and unread badges.
#[component]
pub fn ChannelList(props: ChannelListProps) -> Element {
    rsx! {
        div {
            class: "channel-list",
            style: "
                width: 240px;
                background: var(--q-surface, #1a1a2e);
                border-right: 1px solid var(--q-surface-border, #333);
                display: flex;
                flex-direction: column;
                flex-shrink: 0;
            ",

            // Header
            div {
                style: "
                    padding: 0.75rem 1rem;
                    border-bottom: 1px solid var(--q-surface-border, #333);
                    font-size: 0.8rem;
                    font-weight: 600;
                    text-transform: uppercase;
                    letter-spacing: 0.05em;
                    color: var(--q-text-secondary, #888);
                ",
                "Channels"
            }

            // Channel list
            div {
                style: "
                    flex: 1;
                    overflow-y: auto;
                    padding: 0.5rem;
                ",
                for ch in &props.channels {
                    let is_active = ch.id == props.active_channel_id;

                    div {
                        class: "channel-list-item",
                        onclick: {
                            let id = ch.id.clone();
                            move |_| props.on_channel_click.call(id.clone())
                        },
                        style: if is_active {
                            "
                                display: flex;
                                align-items: center;
                                justify-content: space-between;
                                padding: 0.5rem 0.65rem;
                                border-radius: 6px;
                                cursor: pointer;
                                background: var(--q-primary, #6c5ce7);
                                color: #fff;
                                margin-bottom: 0.2rem;
                                transition: all 0.15s;
                            "
                        } else {
                            "
                                display: flex;
                                align-items: center;
                                justify-content: space-between;
                                padding: 0.5rem 0.65rem;
                                border-radius: 6px;
                                cursor: pointer;
                                color: var(--q-text-secondary, #888);
                                margin-bottom: 0.2rem;
                                transition: all 0.15s;
                            "
                        },

                        // Channel name with icon
                        div {
                            style: "
                                display: flex;
                                align-items: center;
                                gap: 0.5rem;
                                min-width: 0;
                                overflow: hidden;
                            ",
                            // Icon
                            span {
                                style: "
                                    font-size: 0.85rem;
                                    flex-shrink: 0;
                                ",
                                if ch.is_private { "🔒" } else { "#" }
                            }
                            // Name
                            span {
                                style: "
                                    font-size: 0.85rem;
                                    font-weight: if is_active { 600 } else { 400 };
                                    overflow: hidden;
                                    text-overflow: ellipsis;
                                    white-space: nowrap;
                                ",
                                "{ch.name}"
                            }
                        }

                        // Unread badge
                        if ch.unread > 0 {
                            span {
                                style: if is_active {
                                    "
                                        font-size: 0.7rem;
                                        font-weight: 600;
                                        background: rgba(255, 255, 255, 0.2);
                                        padding: 0.1rem 0.45rem;
                                        border-radius: 4px;
                                        flex-shrink: 0;
                                    "
                                } else {
                                    "
                                        font-size: 0.7rem;
                                        font-weight: 600;
                                        background: rgba(255, 71, 87, 0.15);
                                        color: #ff4757;
                                        padding: 0.1rem 0.45rem;
                                        border-radius: 4px;
                                        flex-shrink: 0;
                                    "
                                },
                                "{ch.unread}"
                            }
                        }
                    }
                }
            }
        }
    }
}
