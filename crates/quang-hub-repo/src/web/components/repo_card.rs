//! RepoCard — card showing repo info with connection status.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

/// Card displaying a linked repository's summary information.
#[component]
pub fn RepoCard(
    id: String,
    owner: String,
    name: String,
    full_name: String,
    status: String,
    default_branch: String,
    language: Option<String>,
    is_private: bool,
    stars: u64,
) -> Element {
    let status_color = match status.as_str() {
        "connected" => "var(--q-success)",
        "syncing" => "var(--q-warning)",
        "disconnected" | "auth_failed" => "var(--q-danger)",
        _ => "var(--q-text-secondary)",
    };

    let status_label = match status.as_str() {
        "connected" => "Connected",
        "disconnected" => "Disconnected",
        "syncing" => "Syncing",
        "auth_failed" => "Auth Failed",
        "archived" => "Archived",
        _ => &status,
    };

    rsx! {
        Link {
            to: "/repo/{id}",
            class: "repo-card",
            style: { REPO_CARD_STYLES }

            div { class: "repo-card-header",
                div { class: "repo-card-icon",
                    if is_private { "🔒" } else { "📂" }
                }
                div { class: "repo-card-info",
                    div { class: "repo-card-name", "{full_name}" }
                    if let Some(ref lang) = language {
                        div { class: "repo-card-language", "{lang}" }
                    }
                }
                div { class: "repo-card-status",
                    span {
                        class: "status-dot",
                        style: "background: {status_color};"
                    }
                    span { style: "color: {status_color}; font-size: 12px;",
                        "{status_label}"
                    }
                }
            }

            div { class: "repo-card-details",
                div { class: "repo-card-branch",
                    span { "🌿 " }
                    span { "{default_branch}" }
                }
                div { class: "repo-card-stars",
                    span { "⭐ " }
                    span { "{stars}" }
                }
            }
        }
    }
}

const REPO_CARD_STYLES: &str = "
<style>
  .repo-card {
    display: flex;
    flex-direction: column;
    gap: 12px;
    padding: 16px;
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius-lg);
    text-decoration: none;
    color: var(--q-text);
    transition: all 0.15s ease;
    cursor: pointer;
  }

  .repo-card:hover {
    border-color: var(--q-primary);
    background: var(--q-surface-hover);
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(108, 92, 231, 0.15);
  }

  .repo-card-header {
    display: flex;
    align-items: flex-start;
    gap: 10px;
  }

  .repo-card-icon {
    font-size: 24px;
    flex-shrink: 0;
    width: 36px;
    height: 36px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--q-bg);
    border-radius: var(--q-radius);
  }

  .repo-card-info {
    flex: 1;
    min-width: 0;
  }

  .repo-card-name {
    font-size: 15px;
    font-weight: 600;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .repo-card-language {
    font-size: 12px;
    color: var(--q-text-secondary);
    margin-top: 2px;
  }

  .repo-card-status {
    display: flex;
    align-items: center;
    gap: 6px;
    flex-shrink: 0;
  }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    display: inline-block;
  }

  .repo-card-details {
    display: flex;
    justify-content: space-between;
    font-size: 12px;
    color: var(--q-text-secondary);
  }

  .repo-card-branch,
  .repo-card-stars {
    display: flex;
    align-items: center;
    gap: 4px;
  }
</style>
";
