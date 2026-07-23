//! RepoBrowse — file tree + file viewer with syntax highlighting.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::web::components::file_tree::FileTree;
use crate::web::components::file_viewer::FileViewer;
use quang_web::components::loading::LoadingSpinner;

/// Browse files in a repository with tree navigation and code viewing.
#[component]
pub fn RepoBrowse(repo_id: String, path: Vec<String>) -> Element {
    let current_path = use_signal(|| path.join("/"));
    let current_branch = use_signal(|| "main".to_string());
    let selected_file = use_signal(|| None::<String>);
    let show_branch_selector = use_signal(|| false);

    rsx! {
        div { class: "repo-browse",
            style: { REPO_BROWSE_STYLES }

            // ── Header ──
            div { class: "repo-browse-header",
                div { class: "repo-browse-breadcrumb",
                    Link {
                        to: "/repo/{repo_id}",
                        class: "breadcrumb-link",
                        "{repo_id}"
                    }
                    span { " / browse" }
                    for (i, part) in path.iter().enumerate() {
                        span { " / " }
                        if i < path.len() - 1 {
                            Link {
                                to: "/repo/{repo_id}/browse/{}",
                                "{part}"
                            }
                        } else {
                            span { "{part}" }
                        }
                    }
                }

                // Branch selector
                div { class: "branch-selector",
                    button {
                        class: "branch-btn",
                        onclick: move |_| show_branch_selector.set(!show_branch_selector.read()),
                        "🌿 {current_branch}"
                    }
                    if *show_branch_selector.read() {
                        div { class: "branch-dropdown",
                            button { "main" }
                            button { "develop" }
                            button { "feature/*" }
                        }
                    }
                }
            }

            // ── Main content ──
            div { class: "repo-browse-content",
                // File tree sidebar
                div { class: "repo-browse-tree",
                    FileTree {
                        repo_id: repo_id.clone(),
                        current_path: current_path(),
                        branch: current_branch(),
                        on_file_select: move |file_path| selected_file.set(Some(file_path)),
                    }
                }

                // File viewer
                div { class: "repo-browse-viewer",
                    if let Some(file_path) = selected_file.read().as_ref() {
                        FileViewer {
                            repo_id: repo_id.clone(),
                            file_path: file_path.clone(),
                            branch: current_branch(),
                        }
                    } else {
                        div { class: "no-file-selected",
                            div { class: "no-file-icon", "📄" }
                            h3 { "Select a file to view" }
                            p { "Choose a file from the tree on the left to view its contents." }
                        }
                    }
                }
            }
        }
    }
}

const REPO_BROWSE_STYLES: &str = "
<style>
  .repo-browse {
    display: flex;
    flex-direction: column;
    gap: 12px;
    height: calc(100vh - var(--q-topbar-height) - 80px);
    max-width: 1400px;
  }

  .repo-browse-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
  }

  .repo-browse-breadcrumb {
    font-size: 13px;
    color: var(--q-text-secondary);
    display: flex;
    align-items: center;
    gap: 2px;
    flex-wrap: wrap;
  }

  .breadcrumb-link {
    color: var(--q-primary);
  }

  .branch-selector {
    position: relative;
  }

  .branch-btn {
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    padding: 6px 12px;
    font-size: 13px;
    border-radius: var(--q-radius);
    color: var(--q-text);
    cursor: pointer;
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .branch-btn:hover {
    background: var(--q-surface-hover);
  }

  .branch-dropdown {
    position: absolute;
    top: 100%;
    right: 0;
    margin-top: 4px;
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius);
    overflow: hidden;
    z-index: 100;
    min-width: 160px;
  }

  .branch-dropdown button {
    display: block;
    width: 100%;
    padding: 8px 12px;
    text-align: left;
    background: transparent;
    border: none;
    color: var(--q-text);
    font-size: 13px;
    cursor: pointer;
  }

  .branch-dropdown button:hover {
    background: var(--q-surface-hover);
  }

  .repo-browse-content {
    display: flex;
    flex: 1;
    gap: 0;
    overflow: hidden;
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius-lg);
    background: var(--q-surface);
  }

  .repo-browse-tree {
    width: 280px;
    min-width: 280px;
    overflow-y: auto;
    border-right: 1px solid var(--q-border);
  }

  .repo-browse-viewer {
    flex: 1;
    overflow-y: auto;
    min-width: 0;
  }

  .no-file-selected {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 12px;
    padding: 48px;
    text-align: center;
  }

  .no-file-icon {
    font-size: 48px;
    opacity: 0.4;
  }

  .no-file-selected h3 {
    font-size: 18px;
    font-weight: 600;
  }

  .no-file-selected p {
    color: var(--q-text-secondary);
    font-size: 14px;
    max-width: 300px;
  }

  @media (max-width: 768px) {
    .repo-browse-tree {
      width: 200px;
      min-width: 200px;
    }
  }
</style>
";
