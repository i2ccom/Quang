//! Login page — OAuth login with Google and GitHub providers.
//!
//! This page handles the OAuth flow interaction. It reads the `provider`
//! query parameter and redirects to the appropriate OAuth endpoint.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

/// Login page component with OAuth provider selection.
#[component]
pub fn Login() -> Element {
    let mut provider = use_signal(|| String::new());
    let mut email = use_signal(|| String::new());
    let mut password = use_signal(|| String::new());
    let mut error = use_signal(|| String::new());
    let mut is_loading = use_signal(|| false);

    // Read query param for provider
    use_effect(move || {
        // In a real app, read from URL query params
        // For now, this is a stub that would be wired to the router
    });

    let on_provider_click = move |prov: &str| {
        let prov = prov.to_string();
        is_loading.set(true);
        error.set(String::new());
        // In a real app, redirect to the OAuth endpoint:
        // let redirect_url = format!("/api/auth/{}", prov);
        // web_sys::window().unwrap().location().set_href(&redirect_url).unwrap();
        provider.set(prov);
        is_loading.set(false);
    };

    let on_submit = move |_| {
        is_loading.set(true);
        error.set(String::new());
        // Stub — in production, call backend auth endpoint
        if email().is_empty() || password().is_empty() {
            error.set("Email and password are required.".to_string());
            is_loading.set(false);
        } else {
            // Simulate auth delay
            is_loading.set(false);
            let _ = navigator().push("/workspaces");
        }
    };

    rsx! {
        div {
            class: "login-page",
            style: "
                display: flex;
                flex-direction: column;
                align-items: center;
                justify-content: center;
                min-height: 100vh;
                background: var(--q-bg, #0f0f1a);
                color: var(--q-text, #e0e0e0);
                font-family: 'Inter', system-ui, sans-serif;
                padding: 2rem;
            ",

            div {
                class: "login-card",
                style: "
                    background: var(--q-surface, #1a1a2e);
                    border: 1px solid var(--q-surface-border, #333);
                    border-radius: 16px;
                    padding: 2.5rem;
                    width: 100%;
                    max-width: 400px;
                    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.3);
                ",

                // Header
                div {
                    style: "text-align: center; margin-bottom: 2rem;",
                    h1 {
                        style: "
                            font-size: 1.5rem;
                            font-weight: 700;
                            margin: 0 0 0.25rem 0;
                            color: var(--q-primary, #6c5ce7);
                        ",
                        "Welcome back"
                    }
                    p {
                        style: "
                            font-size: 0.9rem;
                            color: var(--q-text-secondary, #888);
                            margin: 0;
                        ",
                        "Sign in to your QuangHub workspace"
                    }
                }

                // Error message
                if !error().is_empty() {
                    div {
                        style: "
                            background: rgba(255, 71, 87, 0.1);
                            border: 1px solid rgba(255, 71, 87, 0.3);
                            border-radius: 8px;
                            padding: 0.75rem;
                            margin-bottom: 1rem;
                            color: #ff4757;
                            font-size: 0.85rem;
                            text-align: center;
                        ",
                        "{error}"
                    }
                }

                // OAuth Buttons
                div {
                    style: "
                        display: flex;
                        flex-direction: column;
                        gap: 0.75rem;
                        margin-bottom: 1.5rem;
                    ",

                    OAuthButton {
                        label: "Continue with Google",
                        icon: "G",
                        disabled: is_loading(),
                        on_click: {
                            let p = "google";
                            move |_| on_provider_click(p)
                        }
                    },
                    OAuthButton {
                        label: "Continue with GitHub",
                        icon: "GH",
                        disabled: is_loading(),
                        on_click: {
                            let p = "github";
                            move |_| on_provider_click(p)
                        }
                    }
                }

                // Divider
                div {
                    style: "
                        display: flex;
                        align-items: center;
                        gap: 1rem;
                        margin-bottom: 1.5rem;
                        color: var(--q-text-muted, #555);
                        font-size: 0.8rem;
                    ",
                    div { style: "flex: 1; height: 1px; background: var(--q-surface-border, #333);" }
                    span { "or" }
                    div { style: "flex: 1; height: 1px; background: var(--q-surface-border, #333);" }
                }

                // Email/Password form
                form {
                    onsubmit: on_submit,
                    style: "
                        display: flex;
                        flex-direction: column;
                        gap: 1rem;
                    ",

                    div {
                        style: "display: flex; flex-direction: column; gap: 0.35rem;",
                        label {
                            r#for: "email",
                            style: "
                                font-size: 0.85rem;
                                font-weight: 500;
                                color: var(--q-text-secondary, #aaa);
                            ",
                            "Email"
                        }
                        input {
                            id: "email",
                            r#type: "email",
                            placeholder: "you@example.com",
                            value: email(),
                            oninput: move |e| email.set(e.value()),
                            style: "
                                padding: 0.65rem 0.85rem;
                                border-radius: 8px;
                                border: 1px solid var(--q-surface-border, #333);
                                background: var(--q-bg, #0f0f1a);
                                color: var(--q-text, #e0e0e0);
                                font-size: 0.9rem;
                                outline: none;
                                transition: border-color 0.2s;
                            "
                        }
                    }

                    div {
                        style: "display: flex; flex-direction: column; gap: 0.35rem;",
                        label {
                            r#for: "password",
                            style: "
                                font-size: 0.85rem;
                                font-weight: 500;
                                color: var(--q-text-secondary, #aaa);
                            ",
                            "Password"
                        }
                        input {
                            id: "password",
                            r#type: "password",
                            placeholder: "••••••••",
                            value: password(),
                            oninput: move |e| password.set(e.value()),
                            style: "
                                padding: 0.65rem 0.85rem;
                                border-radius: 8px;
                                border: 1px solid var(--q-surface-border, #333);
                                background: var(--q-bg, #0f0f1a);
                                color: var(--q-text, #e0e0e0);
                                font-size: 0.9rem;
                                outline: none;
                                transition: border-color 0.2s;
                            "
                        }
                    }

                    button {
                        r#type: "submit",
                        disabled: is_loading(),
                        style: if is_loading() {
                            "
                                padding: 0.75rem;
                                border-radius: 8px;
                                border: none;
                                background: var(--q-primary-dim, #4a3db5);
                                color: var(--q-text-dim, #888);
                                font-size: 1rem;
                                font-weight: 600;
                                cursor: not-allowed;
                                transition: all 0.2s;
                            "
                        } else {
                            "
                                padding: 0.75rem;
                                border-radius: 8px;
                                border: none;
                                background: var(--q-primary, #6c5ce7);
                                color: #fff;
                                font-size: 1rem;
                                font-weight: 600;
                                cursor: pointer;
                                transition: all 0.2s;
                            "
                        },
                        if is_loading() { "Signing in..." } else { "Sign in" }
                    }
                }

                // Sign up link
                div {
                    style: "
                        text-align: center;
                        margin-top: 1.5rem;
                        font-size: 0.85rem;
                        color: var(--q-text-secondary, #888);
                    ",
                    span { "Don't have an account? " }
                    a {
                        href: "/signup",
                        style: "
                            color: var(--q-primary, #6c5ce7);
                            text-decoration: none;
                            font-weight: 500;
                        ",
                        "Create one"
                    }
                }
            }
        }
    }
}

/// OAuth provider button component.
#[component]
fn OAuthButton(
    label: String,
    icon: String,
    disabled: bool,
    on_click: EventHandler<MouseEvent>,
) -> Element {
    rsx! {
        button {
            disabled,
            onclick: move |e| on_click.call(e),
            style: if disabled {
                "
                    display: flex;
                    align-items: center;
                    gap: 0.75rem;
                    padding: 0.75rem 1.25rem;
                    border-radius: 8px;
                    border: 1px solid var(--q-surface-border, #333);
                    background: var(--q-surface, #1a1a2e);
                    color: var(--q-text-dim, #555);
                    font-size: 0.9rem;
                    font-weight: 500;
                    cursor: not-allowed;
                    width: 100%;
                    justify-content: center;
                    transition: all 0.2s;
                "
            } else {
                "
                    display: flex;
                    align-items: center;
                    gap: 0.75rem;
                    padding: 0.75rem 1.25rem;
                    border-radius: 8px;
                    border: 1px solid var(--q-surface-border, #333);
                    background: var(--q-surface, #1a1a2e);
                    color: var(--q-text, #e0e0e0);
                    font-size: 0.9rem;
                    font-weight: 500;
                    cursor: pointer;
                    width: 100%;
                    justify-content: center;
                    transition: all 0.2s;
                "
            },
            div {
                style: "
                    width: 24px;
                    height: 24px;
                    border-radius: 4px;
                    background: var(--q-primary, #6c5ce7);
                    color: #fff;
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    font-size: 0.7rem;
                    font-weight: 700;
                ",
                "{icon}"
            }
            span { "{label}" }
        }
    }
}
