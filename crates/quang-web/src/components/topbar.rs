//! TopBar — horizontal top navigation bar.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::auth::{login, logout, IdentityProvider, UserSession};
use crate::components::avatar::Avatar;

#[component]
pub fn TopBar() -> Element {
    let session: Signal<Option<UserSession>> = use_context::<Signal<Option<UserSession>>>();
    let mut show_dropdown = use_signal(|| false);

    rsx! {
        header {
            class: "topbar",
            style: "position: fixed; top: 0; left: 0; right: 0; height: var(--q-topbar-height); background: rgba(18, 18, 28, 0.85); backdrop-filter: blur(20px); -webkit-backdrop-filter: blur(20px); border-bottom: 1px solid var(--q-border); display: flex; align-items: center; padding: 0 24px; z-index: 100; gap: 8px;",

            Link {
                to: "/",
                style: "display: flex; align-items: center; gap: 10px; font-weight: 700; font-size: 18px; color: var(--q-text); text-decoration: none; padding: 8px 12px; border-radius: var(--q-radius);",
                span { style: "font-size: 20px; background: linear-gradient(135deg, var(--q-primary), var(--q-accent)); -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;", "✦" }
                span { style: "letter-spacing: -0.02em;", "QuangHub" }
            }

            nav { style: "display: flex; gap: 2px; margin-left: 16px;",
                NavLink { to: "/workspace", label: "Workplace" }
                NavLink { to: "/meet", label: "Meet" }
                NavLink { to: "/repo", label: "Repos" }
                NavLink { to: "/agent/chat", label: "AI Agents" }
            }

            div { style: "flex: 1;" }

            div { style: "position: relative;",
                if let Some(user) = session.read().as_ref() {
                    button {
                        class: "btn-ghost",
                        style: "display: flex; align-items: center; gap: 8px; padding: 6px 12px; border-radius: 100px;",
                        onclick: move |_| show_dropdown.set(!show_dropdown()),
                        Avatar {
                            src: user.avatar_url.clone(),
                            name: user.display_name.clone(),
                            size: Some("28px".to_string()),
                            is_agent: Some(user.is_agent),
                        }
                        span { style: "font-size: 13px; font-weight: 500;", "{user.display_name}" }
                    }
                } else {
                    button {
                        class: "btn-primary",
                        style: "padding: 8px 20px; border-radius: 100px; font-size: 13px;",
                        onclick: move |_| login(IdentityProvider::Google),
                        "Sign in"
                    }
                }

                if show_dropdown() {
                    div {
                        style: "position: absolute; top: calc(100% + 8px); right: 0; background: rgba(22, 22, 34, 0.95); backdrop-filter: blur(20px); border: 1px solid var(--q-border); border-radius: var(--q-radius-lg); padding: 6px; min-width: 180px; box-shadow: var(--q-shadow-lg); z-index: 200; animation: scaleIn 0.15s ease;",
                        onmouseleave: move |_| show_dropdown.set(false),
                        DropdownItem { to: "/profile", label: "Profile", icon: "👤" }
                        DropdownItem { to: "/settings", label: "Settings", icon: "⚙️" }
                        div { style: "height: 1px; background: var(--q-border); margin: 4px 6px;" }
                        button {
                            class: "btn-ghost",
                            style: "width: 100%; text-align: left; justify-content: flex-start; padding: 8px 12px; border-radius: var(--q-radius); font-size: 13px;",
                            onclick: move |_| { logout(); show_dropdown.set(false); },
                            span { style: "margin-right: 8px;", "🚪" }
                            "Sign out"
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn NavLink(to: String, label: String) -> Element {
    rsx! {
        Link {
            to: to,
            style: "padding: 8px 14px; border-radius: var(--q-radius); font-size: 13px; font-weight: 500; color: var(--q-text-secondary); text-decoration: none; transition: all var(--q-transition);",
            "{label}"
        }
    }
}

#[component]
fn DropdownItem(to: String, label: String, icon: String) -> Element {
    rsx! {
        Link {
            to: to,
            style: "display: flex; align-items: center; gap: 10px; padding: 8px 12px; border-radius: var(--q-radius); font-size: 13px; color: var(--q-text-secondary); text-decoration: none; transition: all var(--q-transition);",
            span { "{icon}" }
            span { "{label}" }
        }
    }
}
