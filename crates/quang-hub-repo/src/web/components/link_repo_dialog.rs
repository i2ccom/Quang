//! LinkRepoDialog — modal dialog for linking a GitHub repository to QuangHub.

use dioxus::prelude::*;

/// Modal dialog for linking a new GitHub repository.
#[component]
pub fn LinkRepoDialog(on_close: EventHandler<()>) -> Element {
    let repo_url = use_signal(String::new);
    let access_token = use_signal(String::new);
    let enable_mirror = use_signal(|| true);
    let linking = use_signal(|| false);
    let error = use_signal(|| None::<String>);

    let is_valid = use_memo(move || !repo_url.read().is_empty() && !access_token.read().is_empty());

    let handle_link = move |_| {
        if !*is_valid.read() {
            return;
        }
        linking.set(true);
        error.set(None);

        // TODO: Call actual API to link the repo
        // let client = use_shared_state::<QuangHubClient>().unwrap();
        // let result = client.link_repo(url, token, mirror).await;
        spawn(async move {
            gloo_timers::future::TimeoutFuture::new(1500).await;
            linking.set(false);
            // On success, close dialog
            on_close.call(());
        });
    };

    rsx! {
        div { class: "link-repo-overlay",
            onclick: move |_| on_close.call(()),
            style: { LINK_REPO_DIALOG_STYLES }

            div { class: "link-repo-dialog",
                onclick: move |e| e.stop_propagation(),

                // Header
                div { class: "link-repo-header",
                    h2 { "Link GitHub Repository" }
                    button {
                        class: "dialog-close-btn",
                        onclick: move |_| on_close.call(()),
                        "✕"
                    }
                }

                // Description
                p { class: "link-repo-desc",
                    "Paste a GitHub repository URL and a personal access token to link it to QuangHub."
                }

                // Error
                if let Some(ref err) = *error.read() {
                    div { class: "link-repo-error",
                        span { "⚠ {err}" }
                    }
                }

                // Repo URL
                div { class: "form-group",
                    label { "Repository URL" }
                    input {
                        class: "form-input",
                        placeholder: "https://github.com/owner/repo",
                        value: "{repo_url.read()}",
                        oninput: move |e| repo_url.set(e.value()),
                        disabled: *linking.read(),
                    }
                }

                // Access token
                div { class: "form-group",
                    label { "Personal Access Token" }
                    input {
                        class: "form-input",
                        input_type: "password",
                        placeholder: "ghp_... or github_pat_...",
                        value: "{access_token.read()}",
                        oninput: move |e| access_token.set(e.value()),
                        disabled: *linking.read(),
                    }
                    span { class: "form-hint",
                        "Requires repo and admin:repo_hook scopes."
                    }
                }

                // Mirror option
                div { class: "form-checkbox",
                    input {
                        input_type: "checkbox",
                        id: "dlg-mirror",
                        checked: *enable_mirror.read(),
                        onchange: move |e| enable_mirror.set(e.checked()),
                        disabled: *linking.read(),
                    }
                    label { "for": "dlg-mirror",
                        "Enable Fluid Remote mirroring for fast AI agent access"
                    }
                }

                // Actions
                div { class: "dialog-actions",
                    button {
                        class: "btn-ghost",
                        onclick: move |_| on_close.call(()),
                        disabled: *linking.read(),
                        "Cancel"
                    }
                    button {
                        class: "btn-primary",
                        disabled: !*is_valid.read() || *linking.read(),
                        onclick: handle_link,
                        if *linking.read() {
                            "Linking..."
                        } else {
                            "Link Repository"
                        }
                    }
                }
            }
        }
    }
}

const LINK_REPO_DIALOG_STYLES: &str = "
<style>
  .link-repo-overlay {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.6);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
    backdrop-filter: blur(4px);
  }

  .link-repo-dialog {
    background: var(--q-surface);
    border: 1px solid var(--q-border);
    border-radius: var(--q-radius-lg);
    padding: 24px;
    min-width: 480px;
    max-width: 90vw;
    display: flex;
    flex-direction: column;
    gap: 16px;
    box-shadow: 0 20px 60px rgba(0, 0, 0, 0.4);
  }

  .link-repo-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .link-repo-header h2 {
    font-size: 20px;
    font-weight: 600;
  }

  .dialog-close-btn {
    background: transparent;
    border: none;
    color: var(--q-text-secondary);
    font-size: 18px;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
  }

  .dialog-close-btn:hover {
    color: var(--q-text);
    background: var(--q-surface-hover);
  }

  .link-repo-desc {
    color: var(--q-text-secondary);
    font-size: 14px;
    line-height: 1.5;
  }

  .link-repo-error {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 10px 12px;
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

  .form-hint {
    font-size: 12px;
    color: var(--q-text-secondary);
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

  .dialog-actions {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    padding-top: 8px;
    border-top: 1px solid var(--q-border);
  }

  @media (max-width: 600px) {
    .link-repo-dialog {
      min-width: unset;
      width: 90vw;
    }
  }
</style>
";
