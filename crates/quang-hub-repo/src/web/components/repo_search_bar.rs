//! RepoSearchBar — search within a repository or across linked repos.

use dioxus::prelude::*;

/// Search bar for filtering repositories or files.
#[component]
pub fn RepoSearchBar(
    value: String,
    on_change: EventHandler<String>,
    placeholder: Option<String>,
    on_submit: Option<EventHandler<String>>,
) -> Element {
    let focused = use_signal(|| false);
    let placeholder_text = placeholder.unwrap_or_else(|| "Search...".to_string());

    rsx! {
        div {
            class: "repo-search-bar",
            class: if *focused.read() { "repo-search-focused" } else { "" },
            style: { REPO_SEARCH_BAR_STYLES }

            span { class: "search-icon", "🔍" }
            input {
                class: "search-input",
                placeholder: "{placeholder_text}",
                value: "{value}",
                oninput: move |e| on_change.call(e.value()),
                onfocus: move |_| focused.set(true),
                onblur: move |_| focused.set(false),
                onkeydown: move |e| {
                    if e.key() == "Enter" {
                        if let Some(ref handler) = on_submit {
                            handler.call(value.clone());
                        }
                    }
                },
            }
            if !value.is_empty() {
                button {
                    class: "search-clear",
                    onclick: move |_| on_change.call(String::new()),
                    "✕"
                }
            }
        }
    }
}

const REPO_SEARCH_BAR_STYLES: &str = "
<style>
  .repo-search-bar {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius-lg);
    transition: all 0.15s ease;
  }

  .repo-search-focused {
    border-color: var(--q-primary);
    box-shadow: 0 0 0 3px rgba(108, 92, 231, 0.15);
  }

  .search-icon {
    font-size: 14px;
    opacity: 0.5;
    flex-shrink: 0;
  }

  .search-input {
    flex: 1;
    background: transparent;
    border: none;
    outline: none;
    color: var(--q-text);
    font-size: 14px;
  }

  .search-input::placeholder {
    color: var(--q-text-secondary);
    opacity: 0.6;
  }

  .search-clear {
    background: transparent;
    border: none;
    color: var(--q-text-secondary);
    font-size: 14px;
    cursor: pointer;
    padding: 2px 6px;
    border-radius: 4px;
  }

  .search-clear:hover {
    color: var(--q-text);
    background: var(--q-surface-hover);
  }
</style>
";
