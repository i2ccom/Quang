//! RepoAppViewer — adaptive application viewer for qh.app manifests.

use dioxus::prelude::*;

/// Tab within the app viewer.
#[derive(Clone, PartialEq)]
struct AppTab {
    id: String,
    label: String,
    icon: Option<String>,
}

/// Renders an adaptive repo app from a qh.app manifest.
///
/// This component discovers `qh.app.json` or `.qh.app.toml` manifests
/// in the repository and renders the app's UI as defined by the manifest.
#[component]
pub fn RepoAppViewer(repo_id: String, app_id: Option<String>) -> Element {
    let apps = use_signal(|| {
        // Mock app data — in production, fetch from API
        vec![
            AppInfo {
                id: "dashboard".to_string(),
                name: "Project Dashboard".to_string(),
                description: Some("Real-time project metrics and charts.".to_string()),
                icon: Some("📊".to_string()),
                tabs: vec![
                    AppTab {
                        id: "overview".to_string(),
                        label: "Overview".to_string(),
                        icon: Some("📈".to_string()),
                    },
                    AppTab {
                        id: "metrics".to_string(),
                        label: "Metrics".to_string(),
                        icon: Some("📉".to_string()),
                    },
                ],
            },
            AppInfo {
                id: "playground".to_string(),
                name: "API Playground".to_string(),
                description: Some("Interactive API testing environment.".to_string()),
                icon: Some("🧪".to_string()),
                tabs: vec![
                    AppTab {
                        id: "request".to_string(),
                        label: "Request".to_string(),
                        icon: Some("➡️".to_string()),
                    },
                    AppTab {
                        id: "response".to_string(),
                        label: "Response".to_string(),
                        icon: Some("⬅️".to_string()),
                    },
                ],
            },
        ]
    });

    let selected_app = use_signal(|| {
        app_id
            .clone()
            .or_else(|| apps.read().first().map(|a| a.id.clone()))
    });

    let active_tab = use_signal(|| 0usize);

    let current_app = use_memo(move || {
        let sid = selected_app.read().clone();
        apps.read()
            .iter()
            .find(|a| Some(a.id.clone()) == sid)
            .cloned()
    });

    rsx! {
        div { class: "repo-app-viewer",
            style: { REPO_APP_VIEWER_STYLES }

            if apps.read().is_empty() {
                div { class: "app-viewer-empty",
                    div { class: "empty-icon", "📱" }
                    h3 { "No adaptive apps found" }
                    p { "This repository does not have any qh.app manifests. Create a qh.app.json or .qh.app.toml file to define adaptive apps." }
                }
            } else {
                // App selector
                div { class: "app-selector",
                    for app in apps.read().iter() {
                        button {
                            class: if Some(app.id.clone()) == *selected_app.read() { "app-btn-active" } else { "app-btn" },
                            onclick: {
                                let id = app.id.clone();
                                move |_| {
                                    selected_app.set(Some(id.clone()));
                                    active_tab.set(0);
                                }
                            },
                            span { "{app.icon.clone().unwrap_or_default()}" }
                            span { "{app.name}" }
                        }
                    }
                }

                if let Some(ref app) = *current_app.read() {
                    // App content area
                    div { class: "app-content",
                        // Tab navigation
                        div { class: "app-tabs",
                            for (i, tab) in app.tabs.iter().enumerate() {
                                button {
                                    class: if i == *active_tab.read() { "tab-active" } else { "" },
                                    onclick: move |_| active_tab.set(i),
                                    if let Some(ref icon) = tab.icon {
                                        span { "{icon} " }
                                    }
                                    span { "{tab.label}" }
                                }
                            }
                        }

                        // Tab content
                        div { class: "app-tab-content",
                            if let Some(tab) = app.tabs.get(*active_tab.read()) {
                                div { class: "app-tab-placeholder",
                                    p { "Adaptive app content for \"{tab.label}\" tab." }
                                    p { class: "app-hint",
                                        "This area renders the component or page defined in the qh.app manifest entry point."
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Internal type for app info display.
#[derive(Clone, PartialEq)]
struct AppInfo {
    id: String,
    name: String,
    description: Option<String>,
    icon: Option<String>,
    tabs: Vec<AppTab>,
}

const REPO_APP_VIEWER_STYLES: &str = "
<style>
  .repo-app-viewer {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .app-viewer-empty {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 64px 40px;
    text-align: center;
  }

  .app-viewer-empty .empty-icon {
    font-size: 48px;
    opacity: 0.4;
  }

  .app-viewer-empty h3 {
    font-size: 18px;
    font-weight: 600;
  }

  .app-viewer-empty p {
    color: var(--q-text-secondary);
    max-width: 480px;
    line-height: 1.5;
    font-size: 14px;
  }

  .app-selector {
    display: flex;
    gap: 8px;
    flex-wrap: wrap;
  }

  .app-btn,
  .app-btn-active {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 8px 16px;
    border-radius: var(--q-radius);
    font-size: 13px;
    font-weight: 500;
    cursor: pointer;
    transition: all 0.15s;
    border: 1px solid var(--q-border);
  }

  .app-btn {
    background: var(--q-surface);
    color: var(--q-text);
  }

  .app-btn:hover {
    background: var(--q-surface-hover);
  }

  .app-btn-active {
    background: var(--q-primary);
    color: white;
    border-color: var(--q-primary);
  }

  .app-content {
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius-lg);
    overflow: hidden;
  }

  .app-tabs {
    display: flex;
    gap: 0;
    background: var(--q-surface);
    border-bottom: 1px solid var(--q-border);
  }

  .app-tabs button {
    background: transparent;
    border: none;
    padding: 10px 20px;
    color: var(--q-text-secondary);
    font-size: 13px;
    cursor: pointer;
    border-bottom: 2px solid transparent;
    margin-bottom: -1px;
    transition: all 0.1s;
  }

  .app-tabs button:hover {
    color: var(--q-text);
  }

  .app-tabs button.tab-active {
    color: var(--q-primary);
    border-bottom-color: var(--q-primary);
  }

  .app-tab-content {
    background: var(--q-bg);
    min-height: 200px;
  }

  .app-tab-placeholder {
    padding: 48px;
    text-align: center;
    color: var(--q-text);
    font-size: 14px;
  }

  .app-hint {
    color: var(--q-text-secondary);
    font-size: 12px;
    margin-top: 8px;
  }
</style>
";
