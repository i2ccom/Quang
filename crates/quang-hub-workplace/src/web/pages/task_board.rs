//! Task Board page — Kanban board for tasks with drag-ready columns.
//!
//! Columns: Backlog | Ready | In Progress | Review | Done
//! Each column shows a count and the task cards within it.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::web::components::kanban_column::KanbanColumn;
use crate::web::components::task_card::TaskCard;

/// Stub task data for the Kanban board.
#[derive(Clone, Debug)]
struct TaskStub {
    id: String,
    title: String,
    description: String,
    status: String,
    priority: String,
    assignee: Option<String>,
    tags: Vec<String>,
}

/// Sample tasks to populate the board.
fn sample_tasks() -> Vec<TaskStub> {
    vec![
        TaskStub {
            id: "t1".into(), title: "Design landing page hero".into(),
            description: "Create the hero section with animated gradient background".into(),
            status: "backlog".into(), priority: "medium".into(),
            assignee: None, tags: vec!["design".into(), "frontend".into()],
        },
        TaskStub {
            id: "t2".into(), title: "Implement OAuth flow".into(),
            description: "Wire up Google and GitHub OAuth login callbacks".into(),
            status: "ready".into(), priority: "high".into(),
            assignee: Some("alice".into()), tags: vec!["auth".into(), "backend".into()],
        },
        TaskStub {
            id: "t3".into(), title: "Build GraphQL schema for tasks".into(),
            description: "Define task CRUD operations in the GraphQL schema".into(),
            status: "in_progress".into(), priority: "high".into(),
            assignee: Some("bob".into()), tags: vec!["graphql".into(), "api".into()],
        },
        TaskStub {
            id: "t4".into(), title: "Add task status transitions".into(),
            description: "Implement the state machine for task lifecycle".into(),
            status: "in_review".into(), priority: "medium".into(),
            assignee: Some("carol".into()), tags: vec!["backend".into()],
        },
        TaskStub {
            id: "t5".into(), title: "Set up D1 database schema".into(),
            description: "Create the initial D1 tables for workspace entities".into(),
            status: "done".into(), priority: "critical".into(),
            assignee: Some("alice".into()), tags: vec!["infra".into(), "database".into()],
        },
        TaskStub {
            id: "t6".into(), title: "Agent chat integration".into(),
            description: "Allow agents to send and receive messages in channels".into(),
            status: "backlog".into(), priority: "low".into(),
            assignee: None, tags: vec!["agents".into(), "chat".into()],
        },
        TaskStub {
            id: "t7".into(), title: "Write unit tests for HyperGraph".into(),
            description: "Add comprehensive tests for node/edge operations".into(),
            status: "ready".into(), priority: "medium".into(),
            assignee: Some("dave".into()), tags: vec!["testing".into()],
        },
    ]
}

/// Column definitions for the Kanban board.
const COLUMNS: &[(&str, &str, &str)] = &[
    ("backlog", "Backlog", "#636e72"),
    ("ready", "Ready", "#00b894"),
    ("in_progress", "In Progress", "#0984e3"),
    ("in_review", "Review", "#fdcb6e"),
    ("done", "Done", "#6c5ce7"),
];

/// Task Board page — full Kanban view.
#[component]
pub fn TaskBoard() -> Element {
    let tasks = use_signal(|| sample_tasks());
    let mut search = use_signal(|| String::new());

    rsx! {
        div {
            class: "task-board",
            style: "
                min-height: 100vh;
                background: var(--q-bg, #0f0f1a);
                color: var(--q-text, #e0e0e0);
                font-family: 'Inter', system-ui, sans-serif;
            ",

            // ── Top bar ──
            div {
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 0.75rem 2rem;
                    background: var(--q-surface, #1a1a2e);
                    border-bottom: 1px solid var(--q-surface-border, #333);
                ",
                h1 {
                    style: "
                        font-size: 1.25rem;
                        font-weight: 600;
                        margin: 0;
                    ",
                    "Task Board"
                }
                div {
                    style: "display: flex; align-items: center; gap: 1rem;",
                    input {
                        placeholder: "Search tasks...",
                        value: search(),
                        oninput: move |e| search.set(e.value()),
                        style: "
                            padding: 0.45rem 0.75rem;
                            border-radius: 6px;
                            border: 1px solid var(--q-surface-border, #333);
                            background: var(--q-bg, #0f0f1a);
                            color: var(--q-text, #e0e0e0);
                            font-size: 0.85rem;
                            outline: none;
                            width: 200px;
                        "
                    }
                    button {
                        style: "
                            padding: 0.45rem 1rem;
                            border-radius: 6px;
                            border: none;
                            background: var(--q-primary, #6c5ce7);
                            color: #fff;
                            font-size: 0.85rem;
                            font-weight: 500;
                            cursor: pointer;
                        ",
                        "+ Add Task"
                    }
                }
            }

            // ── Kanban Columns ──
            div {
                style: "
                    display: flex;
                    gap: 1rem;
                    padding: 1.5rem 2rem;
                    overflow-x: auto;
                    min-height: calc(100vh - 80px);
                ",
                for (status_key, label, color) in COLUMNS {
                    let status_tasks: Vec<_> = tasks()
                        .iter()
                        .filter(|t| {
                            let matches_status = t.status == *status_key;
                            if search().is_empty() {
                                matches_status
                            } else {
                                matches_status && (t.title.to_lowercase().contains(&search().to_lowercase())
                                    || t.tags.iter().any(|tag| tag.to_lowercase().contains(&search().to_lowercase())))
                            }
                        })
                        .cloned()
                        .collect::<Vec<_>>();

                    let mut on_drop = move |task_id: String, new_status: String| {
                        if let Some(task) = tasks.write().iter_mut().find(|t| t.id == task_id) {
                            task.status = new_status;
                        }
                    };

                    KanbanColumn {
                        key: "{status_key}",
                        title: label.to_string(),
                        color: color.to_string(),
                        count: status_tasks.len(),
                        status: status_key.to_string(),
                        tasks: status_tasks,
                        on_drop: move |task_id| {
                            on_drop(task_id, status_key.to_string());
                        }
                    }
                }
            }
        }
    }
}
