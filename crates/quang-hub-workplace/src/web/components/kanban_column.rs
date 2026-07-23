//! KanbanColumn — a drop-target column for the task board.
//!
//! Each column has a header with title, color, and count, followed
//! by a list of TaskCard components. Cards are rendered and arranged
//! vertically within the column.

use dioxus::prelude::*;

use crate::web::components::task_card::TaskCard;

/// Stub task data used by the Kanban column.
#[derive(Clone, PartialEq, Debug)]
pub struct KanbanTask {
    pub id: String,
    pub title: String,
    pub description: String,
    pub priority: String,
    pub assignee: Option<String>,
    pub tags: Vec<String>,
}

/// Props for the KanbanColumn component.
#[derive(Clone, PartialEq, Props)]
pub struct KanbanColumnProps {
    pub title: String,
    pub color: String,
    pub count: usize,
    pub status: String,
    pub tasks: Vec<KanbanTask>,
    pub on_drop: EventHandler<String>,
}

/// A Kanban column that serves as a drop target for task cards.
#[component]
pub fn KanbanColumn(props: KanbanColumnProps) -> Element {
    let mut is_dragover = use_signal(|| false);

    let handle_drag_over = move |_| {
        is_dragover.set(true);
    };

    let handle_drag_leave = move |_| {
        is_dragover.set(false);
    };

    let handle_drop = move |e: Event<DragData>| {
        is_dragover.set(false);
        // The dragged task ID would be stored via dragstart dataTransfer
        // For now, we use a simplified approach — the task knows its own ID
        if let Some(task_id) = e.data().get_data("text/plain") {
            props.on_drop.call(task_id);
        }
    };

    rsx! {
        div {
            class: "kanban-column",
            ondragover: handle_drag_over,
            ondragleave: handle_drag_leave,
            ondrop: handle_drop,
            style: if is_dragover() {
                "
                    min-width: 260px;
                    max-width: 300px;
                    flex: 1;
                    background: var(--q-surface, #1a1a2e);
                    border: 2px dashed {props.color};
                    border-radius: 12px;
                    display: flex;
                    flex-direction: column;
                    transition: all 0.2s;
                "
            } else {
                "
                    min-width: 260px;
                    max-width: 300px;
                    flex: 1;
                    background: var(--q-surface, #1a1a2e);
                    border: 1px solid var(--q-surface-border, #333);
                    border-radius: 12px;
                    display: flex;
                    flex-direction: column;
                    transition: all 0.2s;
                "
            },

            // Column header
            div {
                style: "
                    padding: 0.75rem 1rem;
                    border-bottom: 2px solid {props.color};
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                ",
                div {
                    style: "display: flex; align-items: center; gap: 0.5rem;",
                    div {
                        style: "
                            width: 8px;
                            height: 8px;
                            border-radius: 50%;
                            background: {props.color};
                        "
                    }
                    h3 {
                        style: "
                            font-size: 0.85rem;
                            font-weight: 600;
                            margin: 0;
                            color: var(--q-text, #e0e0e0);
                        ",
                        "{props.title}"
                    }
                }
                span {
                    style: "
                        font-size: 0.75rem;
                        font-weight: 500;
                        padding: 0.1rem 0.5rem;
                        border-radius: 4px;
                        background: {props.color}22;
                        color: {props.color};
                    ",
                    "{props.count}"
                }
            }

            // Task list
            div {
                style: "
                    padding: 0.75rem;
                    flex: 1;
                    display: flex;
                    flex-direction: column;
                    gap: 0.5rem;
                    overflow-y: auto;
                    min-height: 200px;
                ",
                if props.tasks.is_empty() {
                    div {
                        style: "
                            padding: 2rem 1rem;
                            text-align: center;
                            color: var(--q-text-muted, #555);
                            font-size: 0.8rem;
                            border: 1px dashed var(--q-surface-border, #333);
                            border-radius: 8px;
                        ",
                        "Drop tasks here"
                    }
                } else {
                    for task in &props.tasks {
                        TaskCard {
                            key: "{task.id}",
                            id: task.id.clone(),
                            title: task.title.clone(),
                            description: task.description.clone(),
                            priority: task.priority.clone(),
                            assignee: task.assignee.clone(),
                            tags: task.tags.clone(),
                            on_click: None
                        }
                    }
                }
            }
        }
    }
}
