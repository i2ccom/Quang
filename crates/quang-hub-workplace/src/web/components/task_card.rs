//! TaskCard — a compact task card for Kanban board columns.

use dioxus::prelude::*;

/// Props for the TaskCard component.
#[derive(Clone, PartialEq, Props)]
pub struct TaskCardProps {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub assignee: Option<String>,
    pub tags: Vec<String>,
    pub on_click: Option<EventHandler<String>>,
}

/// A draggable task card used in Kanban columns and task lists.
#[component]
pub fn TaskCard(props: TaskCardProps) -> Element {
    let priority_color = match props.priority.as_str() {
        "critical" => "#ff4757",
        "high" => "#e17055",
        "medium" => "#fdcb6e",
        "low" => "#636e72",
        _ => "#636e72",
    };

    let priority_label = match props.priority.as_str() {
        "critical" => "CRIT",
        "high" => "HIGH",
        "medium" => "MED",
        _ => "LOW",
    };

    rsx! {
        div {
            class: "task-card",
            draggable: true,
            onclick: move |_| {
                if let Some(ref handler) = props.on_click {
                    handler.call(props.id.clone());
                }
            },
            style: "
                background: var(--q-surface, #1a1a2e);
                border: 1px solid var(--q-surface-border, #333);
                border-radius: 8px;
                padding: 0.75rem;
                cursor: grab;
                transition: all 0.15s ease;
                user-select: none;
            ",

            // Priority badge + assignee
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    margin-bottom: 0.4rem;
                ",
                span {
                    style: "
                        font-size: 0.65rem;
                        font-weight: 600;
                        padding: 0.1rem 0.35rem;
                        border-radius: 3px;
                        background: {priority_color}22;
                        color: {priority_color};
                        border: 1px solid {priority_color}44;
                    ",
                    "{priority_label}"
                }
                if let Some(ref assignee) = props.assignee {
                    div {
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 0.3rem;
                            font-size: 0.7rem;
                            color: var(--q-text-secondary, #888);
                        ",
                        div {
                            style: "
                                width: 18px;
                                height: 18px;
                                border-radius: 4px;
                                background: var(--q-primary, #6c5ce7);
                                display: flex;
                                align-items: center;
                                justify-content: center;
                                font-size: 0.55rem;
                                font-weight: 600;
                                color: #fff;
                            ",
                            "{assignee.chars().next().unwrap_or('?')}"
                        }
                        span { "{assignee}" }
                    }
                }
            }

            // Title
            h4 {
                style: "
                    font-size: 0.85rem;
                    font-weight: 500;
                    margin: 0 0 0.3rem 0;
                    color: var(--q-text, #e0e0e0);
                    line-height: 1.35;
                ",
                "{props.title}"
            }

            // Tags
            if !props.tags.is_empty() {
                div {
                    style: "
                        display: flex;
                        flex-wrap: wrap;
                        gap: 0.3rem;
                        margin-top: 0.4rem;
                    ",
                    for tag in &props.tags {
                        span {
                            style: "
                                font-size: 0.65rem;
                                padding: 0.1rem 0.4rem;
                                border-radius: 3px;
                                background: var(--q-bg, #0f0f1a);
                                color: var(--q-text-muted, #666);
                                border: 1px solid var(--q-surface-border, #333);
                            ",
                            "{tag}"
                        }
                    }
                }
            }
        }
    }
}
