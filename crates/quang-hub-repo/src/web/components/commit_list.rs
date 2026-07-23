//! CommitList — commit history list for a repository.

use dioxus::prelude::*;

/// A commit item for display in the list.
#[derive(Clone, PartialEq)]
struct CommitItem {
    sha: String,
    short_sha: String,
    subject: String,
    author_name: String,
    author_avatar: Option<String>,
    time_ago: String,
    is_verified: bool,
}

/// Commit history list component.
#[component]
pub fn CommitList(repo_id: String, branch: Option<String>, max_count: Option<usize>) -> Element {
    let limit = max_count.unwrap_or(20);
    let commits = use_signal(|| {
        // Mock commit data
        vec![
            CommitItem {
                sha: "a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0".to_string(),
                short_sha: "a1b2c3d".to_string(),
                subject: "feat: add repository browser with file tree and diff view".to_string(),
                author_name: "Alice Chen".to_string(),
                author_avatar: None,
                time_ago: "2h ago".to_string(),
                is_verified: true,
            },
            CommitItem {
                sha: "b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1".to_string(),
                short_sha: "b2c3d4e".to_string(),
                subject: "fix: resolve edge case in QTask state transitions".to_string(),
                author_name: "Bob Smith".to_string(),
                author_avatar: None,
                time_ago: "5h ago".to_string(),
                is_verified: false,
            },
            CommitItem {
                sha: "c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2".to_string(),
                short_sha: "c3d4e5f".to_string(),
                subject: "docs: update ARCHITECTURE.md with dual remote model".to_string(),
                author_name: "Alice Chen".to_string(),
                author_avatar: None,
                time_ago: "1d ago".to_string(),
                is_verified: true,
            },
            CommitItem {
                sha: "d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3".to_string(),
                short_sha: "d4e5f6a".to_string(),
                subject: "refactor: extract FileViewer and CodeLine into separate components"
                    .to_string(),
                author_name: "Carol Davis".to_string(),
                author_avatar: None,
                time_ago: "2d ago".to_string(),
                is_verified: false,
            },
            CommitItem {
                sha: "e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4".to_string(),
                short_sha: "e5f6a7b".to_string(),
                subject: "Merge pull request #42 from feature/qtask-board".to_string(),
                author_name: "Alice Chen".to_string(),
                author_avatar: None,
                time_ago: "3d ago".to_string(),
                is_verified: true,
            },
        ]
    });

    rsx! {
        div { class: "commit-list",
            style: { COMMIT_LIST_STYLES }

            div { class: "commit-list-header",
                span { "Recent Commits" }
                if branch.is_some() {
                    span { class: "commit-branch", "🌿 {branch.clone().unwrap_or_default()}" }
                }
            }

            div { class: "commit-list-content",
                if commits.read().is_empty() {
                    div { class: "commit-list-empty",
                        p { "No commits found in this branch." }
                    }
                } else {
                    for commit in commits.read().iter().take(limit) {
                        div { class: "commit-item",
                            div { class: "commit-avatar",
                                if let Some(ref avatar) = commit.author_avatar {
                                    img { src: "{avatar}", alt: "{commit.author_name}" }
                                } else {
                                    div { class: "commit-avatar-placeholder",
                                        "{commit.author_name.chars().next().unwrap_or('?')}"
                                    }
                                }
                            }
                            div { class: "commit-info",
                                div { class: "commit-subject", "{commit.subject}" }
                                div { class: "commit-meta",
                                    span { "{commit.author_name}" }
                                    span { " · " }
                                    span { "{commit.time_ago}" }
                                    if commit.is_verified {
                                        span { class: "verified-badge", " ✓ Verified" }
                                    }
                                }
                            }
                            div { class: "commit-sha",
                                a {
                                    href: "#",
                                    class: "commit-sha-link",
                                    "{commit.short_sha}"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

const COMMIT_LIST_STYLES: &str = "
<style>
  .commit-list {
    display: flex;
    flex-direction: column;
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius-lg);
    overflow: hidden;
  }

  .commit-list-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 12px 16px;
    background: var(--q-surface);
    border-bottom: 1px solid var(--q-border);
    font-size: 13px;
    font-weight: 600;
  }

  .commit-branch {
    font-size: 12px;
    font-weight: 400;
    color: var(--q-text-secondary);
  }

  .commit-list-content {
    display: flex;
    flex-direction: column;
  }

  .commit-list-empty {
    padding: 32px;
    text-align: center;
    color: var(--q-text-secondary);
    font-size: 14px;
  }

  .commit-item {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 10px 16px;
    border-bottom: 1px solid var(--q-border);
    transition: background 0.1s;
  }

  .commit-item:last-child {
    border-bottom: none;
  }

  .commit-item:hover {
    background: var(--q-surface-hover);
  }

  .commit-avatar {
    flex-shrink: 0;
    width: 32px;
    height: 32px;
    border-radius: 50%;
    overflow: hidden;
  }

  .commit-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .commit-avatar-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--q-primary);
    color: white;
    font-size: 13px;
    font-weight: 600;
    border-radius: 50%;
  }

  .commit-info {
    flex: 1;
    min-width: 0;
  }

  .commit-subject {
    font-size: 14px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .commit-meta {
    font-size: 12px;
    color: var(--q-text-secondary);
    margin-top: 2px;
    display: flex;
    align-items: center;
    gap: 2px;
  }

  .verified-badge {
    color: var(--q-success);
    font-size: 11px;
  }

  .commit-sha {
    flex-shrink: 0;
  }

  .commit-sha-link {
    font-family: monospace;
    font-size: 12px;
    color: var(--q-primary);
    text-decoration: none;
  }

  .commit-sha-link:hover {
    text-decoration: underline;
  }
</style>
";
