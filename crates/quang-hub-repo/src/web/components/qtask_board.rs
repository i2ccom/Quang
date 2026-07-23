//! QTaskBoard — agent task board specific to repositories.

use dioxus::prelude::*;

/// A QTask item for display in the board.
#[derive(Clone, PartialEq, Debug)]
pub struct QTaskItem {
    pub id: String,
    pub title: String,
    pub action: String,
    pub state: String,
    pub priority: u8,
    pub assigned_agent: Option<String>,
    pub created_at: String,
}

/// Agent task board for a specific repository.
#[component]
pub fn QTaskBoard(
    repo_id: String,
    filter: String,
) -> Element {
    let tasks = use_signal(|| {
        // Mock task data
        vec![
            QTaskItem {
                id: "task-001".to_string(),
                title: "Refactor authentication module".to_string(),
                action: "refactor".to_string(),
                state: "running".to_string(),
                priority: 1,
                assigned_agent: Some("Agent Alpha".to_string()),
                created_at: "2h ago".to_string(),
            },
            QTaskItem {
                id: "task-002".to_string(),
                title: "Add tests for FileTree component".to_string(),
                action: "test".to_string(),
                state: "pending".to_string(),
                priority: 2,
                assigned_agent: None,
                created_at: "5h ago".to_string(),
            },
            QTaskItem {
                id: "task-003".to_string(),
                title: "Fix memory leak in event bus".to_string(),
                action: "bugfix".to_string(),
                state: "pending".to_string(),
                priority: 1,
                assigned_agent: Some("Agent Beta".to_string()),
                created_at: "1d ago".to_string(),
            },
            QTaskItem {
                id: "task-004".to_string(),
                title: "Generate API documentation".to_string(),
                action: "docs".to_string(),
                state: "completed".to_string(),
                priority: 3,
                assigned_agent: Some("Agent Alpha".to_string()),
                created_at: "3d ago".to_string(),
            },
            QTaskItem {
                id: "task-005".to_string(),
                title: "Implement syntax highlighting for Python files".to_string(),
                action: "feature".to_string(),
                state: "running".to_string(),
                priority: 2,
                assigned_agent: Some("Agent Gamma".to_string()),
                created_at: "4h ago".to_string(),
            },
        ]
    });

    let filtered = use_memo(move || {
        let f = filter.to_lowercase();
        tasks
            .read()
            .iter()
            .filter(|t| f == "all" || t.state == f)
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "qtask-board",
            style: { QTASK_BOARD_STYLES }

            if filtered.read().is_empty() {
                div { class: "qtask-empty",
                    span { class: "empty-icon", "🤖" }
                    h3 { "No tasks found" }
                    p { "Create a new agent task to get started." }
                }
            } else {
                div { class: "qtask-list",
                    for task in filtered.read().iter() {
                        div {
                            class: "qtask-item",
                            class: if task.state == "running" { "qtask-running" }
                                else if task.state == "completed" { "qtask-completed" }
                                else if task.state == "pending" { "qtask-pending" }
                                else { "" },

                            // Priority badge
                            div { class: "qtask-priority",
                                class: if task.priority <= 2 { "priority-high" }
                                    else if task.priority == 3 { "priority-medium" }
                                    else { "priority-low" },
                                "P{task.priority}"
                            }

                            // Content
                            div { class: "qtask-content",
                                div { class: "qtask-title", "{task.title}" }
                                div { class: "qtask-meta",
                                    span { class: "qtask-action", "{task.action}" }
                                    if let Some(ref agent) = task.assigned_agent {
                                        span { " · " }
                                        span { class: "qtask-agent", "🧑‍💻 {agent}" }
                                    }
                                    span { " · " }
                                    span { class: "qtask-time", "{task.created_at}" }
                                }
                            }

                            // Status badge
                            div { class: "qtask-state",
                                class: if task.state == "running" { "state-running" }
                                    else if task.state == "completed" { "state-completed" }
                                    else if task.state == "pending" { "state-pending" }
                                    else { "" },
                                "{task.state}"
                            }
                        }
                    }
                }
            }
        }
    }
}

const QTASK_BOARD_STYLES: &str = "
<style>
  .qtask-board {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .qtask-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 48px;
    text-align: center;
  }

  .qtask-empty .empty-icon {
    font-size: 40px;
    opacity: 0.4;
  }

  .qtask-empty h3 {
    font-size: 18px;
    font-weight: 600;
  }

  .qtask-empty p {
    color: var(--q-text-secondary);
    font-size: 14px;
  }

  .qtask-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .qtask-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 12px 16px;
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius);
    transition: all 0.1s;
  }

  .qtask-item:hover {
    background: var(--q-surface-hover);
  }

  .qtask-item.qtask-running {
    border-left: 3px solid var(--q-primary);
  }

  .qtask-item.qtask-completed {
    border-left: 3px solid var(--q-success);
    opacity: 0.7;
  }

  .qtask-item.qtask-pending {
    border-left: 3px solid var(--q-warning);
  }

  .qtask-priority {
    font-size: 10px;
    font-weight: 700;
    padding: 2px 6px;
    border-radius: 4px;
    flex-shrink: 0;
  }

  .priority-high {
    background: rgba(255, 107, 107, 0.2);
    color: var(--q-danger);
  }

  .priority-medium {
    background: rgba(252, 196, 25, 0.2);
    color: var(--q-warning);
  }

  .priority-low {
    background: rgba(81, 207, 102, 0.2);
    color: var(--q-success);
  }

  .qtask-content {
    flex: 1;
    min-width: 0;
  }

  .qtask-title {
    font-size: 14px;
    font-weight: 500;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .qtask-meta {
    font-size: 11px;
    color: var(--q-text-secondary);
    margin-top: 4px;
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .qtask-action {
    text-transform: uppercase;
    font-size: 10px;
    font-weight: 600;
  }

  .qtask-state {
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    padding: 3px 10px;
    border-radius: 10px;
    flex-shrink: 0;
  }

  .state-running {
    background: rgba(108, 92, 231, 0.15);
    color: var(--q-primary);
  }

  .state-completed {
    background: rgba(81, 207, 102, 0.15);
    color: var(--q-success);
  }

  .state-pending {
    background: rgba(252, 196, 25, 0.15);
    color: var(--q-warning);
  }
</style>
";
