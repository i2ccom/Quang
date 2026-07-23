//! ProjectCard — displays a project with status, priority, and progress.

use dioxus::prelude::*;

/// Props for the ProjectCard component.
#[derive(Clone, PartialEq, Props)]
pub struct ProjectCardProps {
    pub id: String,
    pub name: String,
    pub description: String,
    pub status: String,
    pub priority: String,
    pub owner: String,
    pub task_count: usize,
    pub completed_task_count: usize,
    pub progress: f64,
    pub on_click: EventHandler<String>,
}

/// A card representing a project with its status and progress indicators.
#[component]
pub fn ProjectCard(props: ProjectCardProps) -> Element {
    let status_color = match props.status.as_str() {
        "active" => "#00b894",
        "planning" => "#0984e3",
        "paused" => "#fdcb6e",
        "completed" => "#6c5ce7",
        "cancelled" => "#ff4757",
        "archived" => "#636e72",
        _ => "#636e72",
    };

    let priority_color = match props.priority.as_str() {
        "critical" => "#ff4757",
        "high" => "#e17055",
        "medium" => "#fdcb6e",
        "low" => "#636e72",
        _ => "#636e72",
    };

    let progress_pct = (props.progress * 100.0).min(100.0);

    rsx! {
        div {
            class: "project-card",
            onclick: move |_| props.on_click.call(props.id.clone()),
            style: "
                background: var(--q-surface, #1a1a2e);
                border: 1px solid var(--q-surface-border, #333);
                border-left: 3px solid {status_color};
                border-radius: 10px;
                padding: 1rem;
                cursor: pointer;
                transition: all 0.2s ease;
            ",

            // Header row
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    margin-bottom: 0.3rem;
                ",
                h3 {
                    style: "
                        font-size: 0.95rem;
                        font-weight: 600;
                        margin: 0;
                        color: var(--q-text, #e0e0e0);
                    ",
                    "{props.name}"
                }
                div {
                    style: "display: flex; align-items: center; gap: 0.35rem;",
                    span {
                        style: "
                            font-size: 0.7rem;
                            padding: 0.1rem 0.4rem;
                            border-radius: 3px;
                            background: {priority_color}22;
                            color: {priority_color};
                            border: 1px solid {priority_color}44;
                            text-transform: uppercase;
                            font-weight: 500;
                        ",
                        "{props.priority}"
                    }
                }
            }

            // Description
            p {
                style: "
                    font-size: 0.8rem;
                    color: var(--q-text-secondary, #888);
                    margin: 0 0 0.75rem 0;
                    line-height: 1.4;
                ",
                "{props.description}"
            }

            // Owner
            div {
                style: "
                    font-size: 0.75rem;
                    color: var(--q-text-muted, #555);
                    margin-bottom: 0.75rem;
                ",
                "Lead: {props.owner}"
            }

            // Progress bar
            div {
                style: "margin-bottom: 0.5rem;",
                div {
                    style: "
                        display: flex;
                        justify-content: space-between;
                        font-size: 0.75rem;
                        margin-bottom: 0.25rem;
                        color: var(--q-text-secondary, #888);
                    ",
                    span { "{props.completed_task_count}/{props.task_count} tasks" }
                    span { "{progress_pct:.0}%" }
                }
                div {
                    style: "
                        height: 5px;
                        background: var(--q-bg, #0f0f1a);
                        border-radius: 3px;
                        overflow: hidden;
                    ",
                    div {
                        style: "
                            height: 100%;
                            width: {progress_pct.to_string()}%;
                            background: {status_color};
                            border-radius: 3px;
                            transition: width 0.4s ease;
                        "
                    }
                }
            }
        }
    }
}
