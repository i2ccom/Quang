//! RepoHome — list linked repos with a "Link New Repo" dialog.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::web::components::link_repo_dialog::LinkRepoDialog;
use crate::web::components::repo_card::RepoCard;
use crate::web::components::repo_search_bar::RepoSearchBar;
use quang_web::components::loading::LoadingSpinner;

/// Mock data type for a linked repo summary (would come from context/API).
#[derive(Clone, PartialEq)]
struct RepoSummary {
    id: String,
    owner: String,
    name: String,
    full_name: String,
    default_branch: String,
    status: String,
    language: Option<String>,
    is_private: bool,
    stars: u64,
}

/// Home page showing all linked repos.
#[component]
pub fn RepoHome() -> Element {
    let show_link_dialog = use_signal(|| false);
    let search_query = use_signal(String::new);
    let repos = use_signal(|| {
        // In production, this would come from a resource/hook that fetches from the API
        Vec::<RepoSummary>::new()
    });
    let loading = use_signal(|| true);
    let error = use_signal(|| None::<String>);

    // Simulate loading
    use_effect(move || {
        spawn(async move {
            // TODO: Replace with actual API fetch
            // let client = use_shared_state::<QuangHubClient>().unwrap();
            // let result = client.get_linked_repos().await;
            gloo_timers::future::TimeoutFuture::new(500).await;
            loading.set(false);
        });
    });

    // Filter repos by search query
    let filtered = use_memo(move || {
        let query = search_query.read().to_lowercase();
        repos
            .read()
            .iter()
            .filter(|r| {
                query.is_empty()
                    || r.full_name.to_lowercase().contains(&query)
                    || r.name.to_lowercase().contains(&query)
                    || r.owner.to_lowercase().contains(&query)
                    || r
                        .language
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&query)
            })
            .cloned()
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "repo-home",
            style: { REPO_HOME_STYLES }

            // ── Header ──
            div { class: "repo-home-header",
                div { class: "repo-home-title-section",
                    h1 { "Repositories" }
                    p { class: "repo-home-subtitle", "Browse, manage, and run agent tasks on your linked GitHub repositories." }
                }
                div { class: "repo-home-actions",
                    button {
                        class: "btn-primary",
                        onclick: move |_| show_link_dialog.set(true),
                        "+ Link Repository"
                    }
                }
            }

            // ── Search bar ──
            RepoSearchBar {
                value: search_query(),
                on_change: move |v| search_query.set(v),
                placeholder: "Search repositories...",
            }

            // ── Content area ──
            div { class: "repo-home-content",
                if *loading.read() {
                    LoadingSpinner { label: Some("Loading repositories...".to_string()) }
                } else if let Some(ref err) = *error.read() {
                    div { class: "repo-home-error",
                        span { "⚠" }
                        p { "{err}" }
                        button {
                            class: "btn-ghost",
                            onclick: move |_| {
                                loading.set(true);
                                error.set(None);
                            },
                            "Retry"
                        }
                    }
                } else if filtered.read().is_empty() {
                    div { class: "repo-home-empty",
                        div { class: "empty-icon", "📦" }
                        h3 { "No repositories found" }
                        p { "Link your first GitHub repository to get started with QuangHub repo management." }
                        button {
                            class: "btn-primary",
                            onclick: move |_| show_link_dialog.set(true),
                            "Link Repository"
                        }
                    }
                } else {
                    div { class: "repo-grid",
                        for repo in filtered.read().iter() {
                            RepoCard {
                                key: "{repo.id}",
                                id: repo.id.clone(),
                                owner: repo.owner.clone(),
                                name: repo.name.clone(),
                                full_name: repo.full_name.clone(),
                                default_branch: repo.default_branch.clone(),
                                status: repo.status.clone(),
                                language: repo.language.clone(),
                                is_private: repo.is_private,
                                stars: repo.stars,
                            }
                        }
                    }
                }
            }
        }

        // ── Link Repo Dialog ──
        if *show_link_dialog.read() {
            LinkRepoDialog {
                on_close: move |_| show_link_dialog.set(false),
            }
        }
    }
}

const REPO_HOME_STYLES: &str = "
<style>
  .repo-home {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 1200px;
  }

  .repo-home-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-start;
    gap: 16px;
  }

  .repo-home-title-section h1 {
    font-size: 28px;
    font-weight: 700;
    margin: 0;
  }

  .repo-home-subtitle {
    color: var(--q-text-secondary);
    font-size: 14px;
    margin-top: 4px;
  }

  .repo-home-actions {
    display: flex;
    gap: 8px;
    flex-shrink: 0;
  }

  .repo-home-content {
    flex: 1;
  }

  .repo-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(340px, 1fr));
    gap: 16px;
  }

  .repo-home-error {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 48px;
    color: var(--q-danger);
    text-align: center;
  }

  .repo-home-error span {
    font-size: 32px;
  }

  .repo-home-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 12px;
    padding: 80px 40px;
    text-align: center;
  }

  .repo-home-empty .empty-icon {
    font-size: 48px;
    opacity: 0.5;
  }

  .repo-home-empty h3 {
    font-size: 20px;
    font-weight: 600;
  }

  .repo-home-empty p {
    color: var(--q-text-secondary);
    max-width: 400px;
  }
</style>
";
