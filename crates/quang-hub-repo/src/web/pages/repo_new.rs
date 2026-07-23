//! RepoNew — create or link a new GitHub repository to QuangHub.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::web::components::repo_search_bar::RepoSearchBar;

/// Form state for linking a new repository.
struct LinkForm {
    owner: String,
    repo_name: String,
    is_private: bool,
    default_branch: String,
    enable_mirror: bool,
    enable_auto_deploy: bool,
    access_token: String,
}

impl Default for LinkForm {
    fn default() -> Self {
        Self {
            owner: String::new(),
            repo_name: String::new(),
            is_private: false,
            default_branch: "main".to_string(),
            enable_mirror: true,
            enable_auto_deploy: false,
            access_token: String::new(),
        }
    }
}

/// Page for linking a new GitHub repository to QuangHub.
#[component]
pub fn RepoNew() -> Element {
    let form = use_signal(LinkForm::default);
    let linking = use_signal(|| false);
    let error = use_signal(|| None::<String>);

    let is_valid = use_memo(move || {
        !form.read().owner.is_empty()
            && !form.read().repo_name.is_empty()
            && !form.read().access_token.is_empty()
    });

    rsx! {
        div { class: "repo-new",
            style: { REPO_NEW_STYLES }

            // ── Breadcrumb ──
            div { class: "repo-new-breadcrumb",
                Link { to: "/repo", "Repositories" }
                span { " / " }
                span { "Link New Repository" }
            }

            // ── Form ──
            div { class: "repo-new-card card",
                h2 { "Link a GitHub Repository" }
                p { class: "repo-new-desc",
                    "Connect a GitHub repository to QuangHub. This enables repo browsing, agent task execution, auto-deploy, and adaptive app rendering."
                }

                if let Some(ref err) = *error.read() {
                    div { class: "repo-new-error",
                        span { "⚠" }
                        p { "{err}" }
                    }
                }

                // Owner / Organization
                div { class: "form-group",
                    label { "GitHub Owner (user or organization)" }
                    input {
                        class: "form-input",
                        placeholder: "e.g. my-org or my-username",
                        value: "{form.read().owner}",
                        oninput: move |e| form.write().owner = e.value(),
                        disabled: *linking.read(),
                    }
                }

                // Repository name
                div { class: "form-group",
                    label { "Repository Name" }
                    div { class: "repo-name-input",
                        span { class: "repo-name-prefix",
                            "{if form.read().owner.is_empty() { \"owner\" } else { &form.read().owner }}/"
                        }
                        input {
                            class: "form-input",
                            placeholder: "e.g. my-project",
                            value: "{form.read().repo_name}",
                            oninput: move |e| form.write().repo_name = e.value(),
                            disabled: *linking.read(),
                        }
                    }
                }

                // Default branch
                div { class: "form-group",
                    label { "Default Branch" }
                    input {
                        class: "form-input",
                        placeholder: "main",
                        value: "{form.read().default_branch}",
                        oninput: move |e| form.write().default_branch = e.value(),
                        disabled: *linking.read(),
                    }
                }

                // Access token
                div { class: "form-group",
                    label { "GitHub Personal Access Token" }
                    input {
                        class: "form-input",
                        input_type: "password",
                        placeholder: "ghp_... or github_pat_...",
                        value: "{form.read().access_token}",
                        oninput: move |e| form.write().access_token = e.value(),
                        disabled: *linking.read(),
                    }
                    span { class: "form-hint",
                        "Requires repo scope. "
                        a { href: "https://github.com/settings/tokens", target: "_blank",
                            "Create a token"
                        }
                    }
                }

                // Options
                div { class: "form-options",
                    div { class: "form-checkbox",
                        input {
                            input_type: "checkbox",
                            id: "mirror",
                            checked: form.read().enable_mirror,
                            onchange: move |e| form.write().enable_mirror = e.checked(),
                            disabled: *linking.read(),
                        }
                        label { "for": "mirror", "Enable Fluid Remote mirroring" }
                    }
                    div { class: "form-checkbox",
                        input {
                            input_type: "checkbox",
                            id: "deploy",
                            checked: form.read().enable_auto_deploy,
                            onchange: move |e| form.write().enable_auto_deploy = e.checked(),
                            disabled: *linking.read(),
                        }
                        label { "for": "deploy", "Enable auto-deploy to Cloudflare Pages" }
                    }
                }

                // Actions
                div { class: "form-actions",
                    Link {
                        to: "/repo",
                        class: "btn-ghost",
                        "Cancel"
                    }
                    button {
                        class: "btn-primary",
                        disabled: !*is_valid.read() || *linking.read(),
                        onclick: move |_| {
                            if *is_valid.read() {
                                linking.set(true);
                                // TODO: Call API to link repo
                                // let client = use_shared_state::<QuangHubClient>().unwrap();
                                // let result = client.link_repo(...).await;
                                spawn(async move {
                                    gloo_timers::future::TimeoutFuture::new(1000).await;
                                    linking.set(false);
                                    // Navigate to the new repo on success
                                });
                            }
                        },
                        if *linking.read() { "Linking..." } else { "Link Repository" }
                    }
                }
            }
        }
    }
}

const REPO_NEW_STYLES: &str = "
<style>
  .repo-new {
    display: flex;
    flex-direction: column;
    gap: 20px;
    max-width: 640px;
  }

  .repo-new-breadcrumb {
    font-size: 13px;
    color: var(--q-text-secondary);
  }

  .repo-new-breadcrumb a {
    color: var(--q-text-secondary);
  }

  .repo-new-breadcrumb a:hover {
    color: var(--q-primary);
  }

  .repo-new-card {
    display: flex;
    flex-direction: column;
    gap: 20px;
  }

  .repo-new-card h2 {
    font-size: 22px;
    font-weight: 600;
  }

  .repo-new-desc {
    color: var(--q-text-secondary);
    font-size: 14px;
    line-height: 1.5;
  }

  .repo-new-error {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px;
    background: rgba(255, 107, 107, 0.1);
    border: 1px solid var(--q-danger);
    border-radius: var(--q-radius);
    color: var(--q-danger);
    font-size: 13px;
  }

  .form-group {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .form-group label {
    font-size: 13px;
    font-weight: 500;
    color: var(--q-text-secondary);
  }

  .form-input {
    width: 100%;
  }

  .repo-name-input {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .repo-name-prefix {
    color: var(--q-text-secondary);
    font-size: 14px;
    white-space: nowrap;
    font-family: monospace;
  }

  .repo-name-input .form-input {
    flex: 1;
  }

  .form-hint {
    font-size: 12px;
    color: var(--q-text-secondary);
  }

  .form-hint a {
    color: var(--q-primary);
  }

  .form-options {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .form-checkbox {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 13px;
  }

  .form-checkbox label {
    cursor: pointer;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    padding-top: 8px;
    border-top: 1px solid var(--q-border);
  }
</style>
";
