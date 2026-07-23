//! RepoDetail — repository overview with branches, recent commits, quick actions.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::web::components::commit_list::CommitList;
use crate::web::components::repo_app_viewer::RepoAppViewer;
use quang_web::components::loading::LoadingSpinner;

/// Repository detail page showing overview, branches, and recent commits.
#[component]
pub fn RepoDetail(repo_id: String) -> Element {
    let active_tab = use_signal(|| RepoDetailTab::Overview);
    let loading = use_signal(|| true);

    // Simulate loading
    use_effect(move || {
        let repo_id = repo_id.clone();
        spawn(async move {
            // TODO: Fetch repo data from API
            gloo_timers::future::TimeoutFuture::new(300).await;
            loading.set(false);
        });
    });

    let repo_full_name = format!("repo/{}", repo_id); // placeholder

    rsx! {
        div { class: "repo-detail",
            style: { REPO_DETAIL_STYLES }

            // ── Header ──
            div { class: "repo-detail-header",
                div { class: "repo-detail-breadcrumb",
                    Link { to: "/repo", "Repositories" }
                    span { " / " }
                    span { "{repo_full_name}" }
                }
                div { class: "repo-detail-info",
                    h1 { "{repo_full_name}" }
                    div { class: "repo-detail-meta",
                        span { class: "repo-badge", "main" }
                        span { class: "repo-private-badge", "🔒 Private" }
                        span { class: "repo-stars", "⭐ 0" }
                    }
                }
                div { class: "repo-detail-actions",
                    Link {
                        to: "/repo/{repo_id}/browse/main",
                        class: "btn-primary",
                        "Browse Files"
                    }
                    Link {
                        to: "/repo/{repo_id}/tasks",
                        class: "btn-ghost",
                        "Agent Tasks"
                    }
                    button { class: "btn-ghost", "⚙ Settings" }
                }
            }

            if *loading.read() {
                LoadingSpinner { label: Some("Loading repository...".to_string()) }
            } else {
                // ── Tab navigation ──
                div { class: "repo-detail-tabs",
                    button {
                        class: if *active_tab.read() == RepoDetailTab::Overview { "tab-active" } else { "" },
                        onclick: move |_| active_tab.set(RepoDetailTab::Overview),
                        "Overview"
                    }
                    button {
                        class: if *active_tab.read() == RepoDetailTab::Branches { "tab-active" } else { "" },
                        onclick: move |_| active_tab.set(RepoDetailTab::Branches),
                        "Branches"
                    }
                    button {
                        class: if *active_tab.read() == RepoDetailTab::Commits { "tab-active" } else { "" },
                        onclick: move |_| active_tab.set(RepoDetailTab::Commits),
                        "Commits"
                    }
                    button {
                        class: if *active_tab.read() == RepoDetailTab::Apps { "tab-active" } else { "" },
                        onclick: move |_| active_tab.set(RepoDetailTab::Apps),
                        "Apps"
                    }
                }

                // ── Tab content ──
                div { class: "repo-detail-content",
                    match *active_tab.read() {
                        RepoDetailTab::Overview => rsx! {
                            RepoOverview { repo_id: repo_id.clone() }
                        },
                        RepoDetailTab::Branches => rsx! {
                            RepoBranchesTab { repo_id: repo_id.clone() }
                        },
                        RepoDetailTab::Commits => rsx! {
                            CommitList { repo_id: repo_id.clone() }
                        },
                        RepoDetailTab::Apps => rsx! {
                            RepoAppViewer { repo_id: repo_id.clone() }
                        },
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum RepoDetailTab {
    Overview,
    Branches,
    Commits,
    Apps,
}

/// Overview tab showing repo stats and quick actions.
#[component]
fn RepoOverview(repo_id: String) -> Element {
    rsx! {
        div { class: "repo-overview",
            div { class: "overview-grid",
                // Description
                div { class: "card overview-section",
                    h3 { "About" }
                    p { style: "color: var(--q-text-secondary);",
                        "This repository is linked to QuangHub. Browse files, run agent tasks, and manage deployments."
                    }
                    div { class: "overview-stats",
                        div { class: "stat-item",
                            span { class: "stat-value", "0" }
                            span { class: "stat-label", "Stars" }
                        }
                        div { class: "stat-item",
                            span { class: "stat-value", "0" }
                            span { class: "stat-label", "Forks" }
                        }
                        div { class: "stat-item",
                            span { class: "stat-value", "1" }
                            span { class: "stat-label", "Branches" }
                        }
                        div { class: "stat-item",
                            span { class: "stat-value", "0" }
                            span { class: "stat-label", "Apps" }
                        }
                    }
                }

                // Quick actions
                div { class: "card overview-section",
                    h3 { "Quick Actions" }
                    div { class: "quick-actions",
                        Link {
                            to: "/repo/{repo_id}/browse/main",
                            class: "quick-action-btn",
                            "📂 Browse Files"
                        }
                        Link {
                            to: "/repo/{repo_id}/tasks",
                            class: "quick-action-btn",
                            "🤖 Agent Tasks"
                        }
                        button { class: "quick-action-btn", "🔄 Sync Now" }
                        button { class: "quick-action-btn", "⚡ Deploy" }
                    }
                }

                // Recent activity
                div { class: "card overview-section overview-wide",
                    h3 { "Recent Activity" }
                    p { style: "color: var(--q-text-secondary); padding: 24px 0; text-align: center;",
                        "No recent activity to display."
                    }
                }

                // Settings summary
                div { class: "card overview-section",
                    h3 { "Settings" }
                    div { class: "settings-summary",
                        div { class: "setting-row",
                            span { "Mirror" }
                            span { class: "setting-value", "Enabled" }
                        }
                        div { class: "setting-row",
                            span { "Auto-deploy" }
                            span { class: "setting-value", "Disabled" }
                        }
                        div { class: "setting-row",
                            span { "Webhook" }
                            span { class: "setting-value", "Active" }
                        }
                    }
                }
            }
        }
    }
}

/// Branches tab showing all branches.
#[component]
fn RepoBranchesTab(repo_id: String) -> Element {
    rsx! {
        div { class: "repo-branches",
            div { class: "card",
                h3 { "Branches" }
                p { style: "color: var(--q-text-secondary); padding: 24px 0; text-align: center;",
                    "Branch listing will appear here."
                }
            }
        }
    }
}

const REPO_DETAIL_STYLES: &str = "
<style>
  .repo-detail {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 1200px;
  }

  .repo-detail-header {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .repo-detail-breadcrumb {
    font-size: 13px;
    color: var(--q-text-secondary);
  }

  .repo-detail-breadcrumb a {
    color: var(--q-text-secondary);
  }

  .repo-detail-breadcrumb a:hover {
    color: var(--q-primary);
  }

  .repo-detail-info {
    display: flex;
    align-items: center;
    gap: 16px;
    flex-wrap: wrap;
  }

  .repo-detail-info h1 {
    font-size: 24px;
    font-weight: 700;
  }

  .repo-detail-meta {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .repo-badge {
    background: var(--q-primary);
    color: white;
    font-size: 11px;
    padding: 2px 8px;
    border-radius: 10px;
    font-weight: 600;
  }

  .repo-private-badge {
    font-size: 12px;
    color: var(--q-text-secondary);
  }

  .repo-stars {
    font-size: 13px;
    color: var(--q-text-secondary);
  }

  .repo-detail-actions {
    display: flex;
    gap: 8px;
  }

  .repo-detail-tabs {
    display: flex;
    gap: 4px;
    border-bottom: 1px solid var(--q-border);
    padding-bottom: 0;
  }

  .repo-detail-tabs button {
    background: transparent;
    color: var(--q-text-secondary);
    border: none;
    border-radius: 0;
    padding: 10px 16px;
    font-size: 14px;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
  }

  .repo-detail-tabs button:hover {
    color: var(--q-text);
    background: var(--q-surface-hover);
  }

  .repo-detail-tabs button.tab-active {
    color: var(--q-primary);
    border-bottom-color: var(--q-primary);
  }

  .repo-detail-content {
    flex: 1;
  }

  .overview-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 16px;
  }

  .overview-section {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .overview-wide {
    grid-column: 1 / -1;
  }

  .overview-stats {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
  }

  .stat-item {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 4px;
  }

  .stat-value {
    font-size: 24px;
    font-weight: 700;
    color: var(--q-text);
  }

  .stat-label {
    font-size: 12px;
    color: var(--q-text-secondary);
  }

  .quick-actions {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 8px;
  }

  .quick-action-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 8px;
    padding: 12px;
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius);
    color: var(--q-text);
    font-size: 13px;
    text-decoration: none;
    cursor: pointer;
    transition: all 0.15s;
  }

  .quick-action-btn:hover {
    background: var(--q-surface-hover);
    border-color: var(--q-primary);
  }

  .settings-summary {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .setting-row {
    display: flex;
    justify-content: space-between;
    font-size: 13px;
  }

  .setting-value {
    color: var(--q-text-secondary);
  }

  @media (max-width: 768px) {
    .overview-grid {
      grid-template-columns: 1fr;
    }
    .quick-actions {
      grid-template-columns: 1fr;
    }
  }
</style>
";
