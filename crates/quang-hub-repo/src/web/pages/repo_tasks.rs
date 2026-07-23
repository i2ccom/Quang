//! RepoTasks — QTask board for the repo (agent tasks).

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::web::components::qtask_board::QTaskBoard;
use quang_web::components::loading::LoadingSpinner;

/// Agent task board for a specific repository.
#[component]
pub fn RepoTasks(repo_id: String) -> Element {
    let loading = use_signal(|| true);
    let tasks = use_signal(Vec::new);
    let show_create = use_signal(|| false);
    let filter_state = use_signal(|| "all".to_string());

    // Simulate loading tasks
    use_effect(move || {
        let repo_id = repo_id.clone();
        spawn(async move {
            // TODO: Fetch tasks from API
            gloo_timers::future::TimeoutFuture::new(400).await;
            loading.set(false);
        });
    });

    let task_counts = use_memo(move || {
        let all = tasks.read().len();
        let pending = tasks
            .read()
            .iter()
            .filter(|t: &&crate::components::qtask_board::QTaskItem| t.state == "pending")
            .count();
        let running = tasks.read().iter().filter(|t| t.state == "running").count();
        let completed = tasks
            .read()
            .iter()
            .filter(|t| t.state == "completed")
            .count();
        (all, pending, running, completed)
    });

    rsx! {
        div { class: "repo-tasks",
            style: { REPO_TASKS_STYLES }

            // ── Header ──
            div { class: "repo-tasks-header",
                div { class: "repo-tasks-title-section",
                    h1 { "Agent Tasks" }
                    p { class: "repo-tasks-subtitle",
                        "AI agent tasks running against this repository."
                    }
                }
                div { class: "repo-tasks-actions",
                    button {
                        class: "btn-primary",
                        onclick: move |_| show_create.set(true),
                        "+ New Task"
                    }
                }
            }

            // ── Filter bar ──
            div { class: "repo-tasks-filters",
                div { class: "filter-tabs",
                    button {
                        class: if filter_state() == "all" { "filter-active" } else { "" },
                        onclick: move |_| filter_state.set("all".to_string()),
                        "All ({task_counts.read().0})"
                    }
                    button {
                        class: if filter_state() == "pending" { "filter-active" } else { "" },
                        onclick: move |_| filter_state.set("pending".to_string()),
                        "Pending ({task_counts.read().1})"
                    }
                    button {
                        class: if filter_state() == "running" { "filter-active" } else { "" },
                        onclick: move |_| filter_state.set("running".to_string()),
                        "Running ({task_counts.read().2})"
                    }
                    button {
                        class: if filter_state() == "completed" { "filter-active" } else { "" },
                        onclick: move |_| filter_state.set("completed".to_string()),
                        "Completed ({task_counts.read().3})"
                    }
                }
            }

            // ── Task board ──
            if *loading.read() {
                LoadingSpinner { label: Some("Loading agent tasks...".to_string()) }
            } else {
                QTaskBoard {
                    repo_id: repo_id.clone(),
                    filter: filter_state(),
                }
            }
        }

        // ── Create task dialog placeholder ──
        if *show_create.read() {
            div { class: "create-task-overlay",
                onclick: move |_| show_create.set(false),
                div { class: "create-task-dialog",
                    onclick: move |e| e.stop_propagation(),
                    h2 { "New Agent Task" }
                    // TODO: Full task creation form
                    p { "Task creation form will go here." }
                    div { class: "dialog-actions",
                        button {
                            class: "btn-ghost",
                            onclick: move |_| show_create.set(false),
                            "Cancel"
                        }
                        button { class: "btn-primary", "Create Task" }
                    }
                }
            }
        }
    }
}

const REPO_TASKS_STYLES: &str = "
<style>
  .repo-tasks {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 1200px;
  }

  .repo-tasks-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 16px;
  }

  .repo-tasks-title-section h1 {
    font-size: 24px;
    font-weight: 700;
    margin: 0;
  }

  .repo-tasks-subtitle {
    color: var(--q-text-secondary);
    font-size: 14px;
    margin-top: 4px;
  }

  .repo-tasks-actions {
    display: flex;
    gap: 8px;
    flex-shrink: 0;
  }

  .repo-tasks-filters {
    display: flex;
    gap: 8px;
  }

  .filter-tabs {
    display: flex;
    gap: 4px;
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius);
    padding: 3px;
  }

  .filter-tabs button {
    background: transparent;
    color: var(--q-text-secondary);
    border: none;
    padding: 6px 14px;
    font-size: 13px;
    border-radius: 6px;
    cursor: pointer;
  }

  .filter-tabs button:hover {
    color: var(--q-text);
  }

  .filter-tabs button.filter-active {
    background: var(--q-primary);
    color: white;
  }

  .create-task-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .create-task-dialog {
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius-lg);
    padding: 24px;
    min-width: 480px;
    max-width: 90vw;
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .create-task-dialog h2 {
    font-size: 20px;
    font-weight: 600;
  }

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 8px;
  }
</style>
";
