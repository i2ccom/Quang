//! Workspace Dashboard page — lists workspaces with a create dialog.
//!
//! Shows all workspaces the current user has access to, plus a button
//! to create a new workspace via an inline modal dialog.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::web::components::workspace_card::WorkspaceCard;

/// Stub workspace data used until backend integration is wired.
#[derive(Clone, Debug)]
struct WorkspaceStub {
    id: String,
    name: String,
    description: String,
    slug: String,
    member_count: usize,
    project_count: usize,
    is_owner: bool,
}

/// Workspace Dashboard page — main landing after login.
#[component]
pub fn WorkspaceDashboard() -> Element {
    let mut show_create = use_signal(|| false);
    let mut workspaces = use_signal(|| {
        vec![
            WorkspaceStub {
                id: "ws_001".into(),
                name: "Acme Corp".into(),
                description: "Main engineering workspace for Acme Corporation".into(),
                slug: "acme-corp".into(),
                member_count: 24,
                project_count: 7,
                is_owner: true,
            },
            WorkspaceStub {
                id: "ws_002".into(),
                name: "Open Source Community".into(),
                description: "Contributor workspace for the QuangHub open source project".into(),
                slug: "quanghub-oss".into(),
                member_count: 156,
                project_count: 12,
                is_owner: false,
            },
            WorkspaceStub {
                id: "ws_003".into(),
                name: "Design Team".into(),
                description: "Product design and UX research workspace".into(),
                slug: "design-team".into(),
                member_count: 8,
                project_count: 3,
                is_owner: true,
            },
        ]
    });

    let mut new_name = use_signal(|| String::new());
    let mut new_slug = use_signal(|| String::new());
    let mut new_desc = use_signal(|| String::new());
    let mut create_error = use_signal(|| String::new());

    let on_create_submit = move |_| {
        if new_name().trim().is_empty() || new_slug().trim().is_empty() {
            create_error.set("Name and slug are required.".to_string());
            return;
        }
        let ws = WorkspaceStub {
            id: format!("ws_{:04}", workspaces().len() + 1),
            name: new_name().clone(),
            description: new_desc().clone(),
            slug: new_slug().clone(),
            member_count: 1,
            project_count: 0,
            is_owner: true,
        };
        workspaces.write().push(ws);
        new_name.set(String::new());
        new_slug.set(String::new());
        new_desc.set(String::new());
        create_error.set(String::new());
        show_create.set(false);
    };

    rsx! {
        div {
            class: "workspace-dashboard",
            style: "
                min-height: 100vh;
                background: var(--q-bg, #0f0f1a);
                color: var(--q-text, #e0e0e0);
                font-family: 'Inter', system-ui, sans-serif;
            ",

            // ── Top Navigation Bar ──
            nav {
                class: "top-nav",
                style: "
                    display: flex;
                    align-items: center;
                    justify-content: space-between;
                    padding: 0.75rem 2rem;
                    background: var(--q-surface, #1a1a2e);
                    border-bottom: 1px solid var(--q-surface-border, #333);
                ",
                div {
                    style: "display: flex; align-items: center; gap: 0.75rem;",
                    span {
                        style: "
                            font-size: 1.25rem;
                            font-weight: 700;
                            background: linear-gradient(135deg, var(--q-primary, #6c5ce7), var(--q-accent, #00cec9));
                            -webkit-background-clip: text;
                            -webkit-text-fill-color: transparent;
                            background-clip: text;
                        ",
                        "QuangHub"
                    }
                }
                div {
                    style: "
                        width: 32px;
                        height: 32px;
                        border-radius: 50%;
                        background: var(--q-primary, #6c5ce7);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        font-size: 0.8rem;
                        font-weight: 600;
                        cursor: pointer;
                    ",
                    "U"
                }
            }

            // ── Main Content ──
            div {
                style: "
                    max-width: 960px;
                    margin: 0 auto;
                    padding: 2rem;
                ",

                // Header row
                div {
                    style: "
                        display: flex;
                        align-items: center;
                        justify-content: space-between;
                        margin-bottom: 2rem;
                    ",
                    h1 {
                        style: "
                            font-size: 1.75rem;
                            font-weight: 700;
                            margin: 0;
                        ",
                        "Workspaces"
                    }
                    button {
                        onclick: move |_| show_create.set(true),
                        style: "
                            display: flex;
                            align-items: center;
                            gap: 0.5rem;
                            padding: 0.6rem 1.25rem;
                            border-radius: 8px;
                            border: none;
                            background: var(--q-primary, #6c5ce7);
                            color: #fff;
                            font-size: 0.9rem;
                            font-weight: 600;
                            cursor: pointer;
                            transition: background 0.2s;
                        ",
                        "+ New Workspace"
                    }
                }

                // Workspace grid
                if workspaces().is_empty() {
                    div {
                        style: "
                            text-align: center;
                            padding: 4rem 2rem;
                            color: var(--q-text-muted, #555);
                        ",
                        p { style: "font-size: 1.1rem;", "No workspaces yet." }
                        p { style: "font-size: 0.9rem;", "Create one to get started." }
                    }
                } else {
                    div {
                        style: "
                            display: grid;
                            grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
                            gap: 1rem;
                        ",
                        for ws in workspaces() {
                            WorkspaceCard {
                                id: ws.id.clone(),
                                name: ws.name.clone(),
                                description: ws.description.clone(),
                                slug: ws.slug.clone(),
                                member_count: ws.member_count,
                                project_count: ws.project_count,
                                is_owner: ws.is_owner,
                                on_click: {
                                    let id = ws.id.clone();
                                    move || {
                                        // Navigate to workspace detail
                                        let _ = navigator().push("/workspace/{id}");
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // ── Create Workspace Modal ──
            if show_create() {
                div {
                    class: "modal-overlay",
                    style: "
                        position: fixed;
                        inset: 0;
                        background: rgba(0, 0, 0, 0.6);
                        display: flex;
                        align-items: center;
                        justify-content: center;
                        z-index: 1000;
                    ",
                    onclick: move |_| show_create.set(false),

                    div {
                        class: "modal-content",
                        onclick: move |e| e.stop_propagation(),
                        style: "
                            background: var(--q-surface, #1a1a2e);
                            border: 1px solid var(--q-surface-border, #333);
                            border-radius: 16px;
                            padding: 2rem;
                            width: 100%;
                            max-width: 480px;
                            box-shadow: 0 16px 48px rgba(0, 0, 0, 0.4);
                        ",

                        h2 {
                            style: "
                                font-size: 1.25rem;
                                font-weight: 600;
                                margin: 0 0 1.5rem 0;
                            ",
                            "Create Workspace"
                        }

                        if !create_error().is_empty() {
                            div {
                                style: "
                                    background: rgba(255, 71, 87, 0.1);
                                    border: 1px solid rgba(255, 71, 87, 0.3);
                                    border-radius: 8px;
                                    padding: 0.6rem;
                                    margin-bottom: 1rem;
                                    color: #ff4757;
                                    font-size: 0.85rem;
                                ",
                                "{create_error}"
                            }
                        }

                        form {
                            onsubmit: on_create_submit,
                            style: "display: flex; flex-direction: column; gap: 1rem;",

                            input {
                                placeholder: "Workspace Name",
                                value: new_name(),
                                oninput: move |e| new_name.set(e.value()),
                                style: "
                                    padding: 0.65rem 0.85rem;
                                    border-radius: 8px;
                                    border: 1px solid var(--q-surface-border, #333);
                                    background: var(--q-bg, #0f0f1a);
                                    color: var(--q-text, #e0e0e0);
                                    font-size: 0.9rem;
                                    outline: none;
                                "
                            }
                            input {
                                placeholder: "slug-name",
                                value: new_slug(),
                                oninput: move |e| new_slug.set(e.value()),
                                style: "
                                    padding: 0.65rem 0.85rem;
                                    border-radius: 8px;
                                    border: 1px solid var(--q-surface-border, #333);
                                    background: var(--q-bg, #0f0f1a);
                                    color: var(--q-text, #e0e0e0);
                                    font-size: 0.9rem;
                                    outline: none;
                                "
                            }
                            textarea {
                                placeholder: "Description (optional)",
                                value: new_desc(),
                                oninput: move |e| new_desc.set(e.value()),
                                rows: 3,
                                style: "
                                    padding: 0.65rem 0.85rem;
                                    border-radius: 8px;
                                    border: 1px solid var(--q-surface-border, #333);
                                    background: var(--q-bg, #0f0f1a);
                                    color: var(--q-text, #e0e0e0);
                                    font-size: 0.9rem;
                                    outline: none;
                                    resize: vertical;
                                    font-family: inherit;
                                "
                            }

                            div {
                                style: "
                                    display: flex;
                                    gap: 0.75rem;
                                    justify-content: flex-end;
                                    margin-top: 0.5rem;
                                ",
                                button {
                                    r#type: "button",
                                    onclick: move |_| {
                                        show_create.set(false);
                                        create_error.set(String::new());
                                    },
                                    style: "
                                        padding: 0.6rem 1.25rem;
                                        border-radius: 8px;
                                        border: 1px solid var(--q-surface-border, #333);
                                        background: transparent;
                                        color: var(--q-text-secondary, #888);
                                        font-size: 0.9rem;
                                        cursor: pointer;
                                    ",
                                    "Cancel"
                                }
                                button {
                                    r#type: "submit",
                                    style: "
                                        padding: 0.6rem 1.5rem;
                                        border-radius: 8px;
                                        border: none;
                                        background: var(--q-primary, #6c5ce7);
                                        color: #fff;
                                        font-size: 0.9rem;
                                        font-weight: 600;
                                        cursor: pointer;
                                    ",
                                    "Create"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
