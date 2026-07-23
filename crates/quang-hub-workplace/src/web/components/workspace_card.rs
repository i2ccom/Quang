//! WorkspaceCard — a clickable card that represents a WorkSpace.
//!
//! Shows the workspace name, description, slug, member/project count,
//! and an owner badge. Clicking navigates to the workspace detail view.

use dioxus::prelude::*;

/// Props for the WorkspaceCard component.
#[derive(Clone, PartialEq, Props)]
pub struct WorkspaceCardProps {
    pub id: String,
    pub name: String,
    pub description: String,
    pub slug: String,
    pub member_count: usize,
    pub project_count: usize,
    pub is_owner: bool,
    pub on_click: EventHandler<()>,
}

/// A card representing a workspace in the dashboard grid.
#[component]
pub fn WorkspaceCard(props: WorkspaceCardProps) -> Element {
    rsx! {
        div {
            class: "workspace-card",
            onclick: move |_| props.on_click.call(()),
            style: "
                background: var(--q-surface, #1a1a2e);
                border: 1px solid var(--q-surface-border, #333);
                border-radius: 12px;
                padding: 1.25rem;
                cursor: pointer;
                transition: all 0.2s ease;
                position: relative;
                overflow: hidden;
            ",

            // Owner badge
            if props.is_owner {
                div {
                    style: "
                        position: absolute;
                        top: 0.75rem;
                        right: 0.75rem;
                        font-size: 0.7rem;
                        padding: 0.15rem 0.5rem;
                        border-radius: 4px;
                        background: rgba(108, 92, 231, 0.15);
                        color: var(--q-primary, #6c5ce7);
                        border: 1px solid rgba(108, 92, 231, 0.3);
                        font-weight: 500;
                    ",
                    "Owner"
                }
            }

            // Name
            h3 {
                style: "
                    font-size: 1.1rem;
                    font-weight: 600;
                    margin: 0 0 0.35rem 0;
                    color: var(--q-text, #e0e0e0);
                ",
                "{props.name}"
            }

            // Description
            p {
                style: "
                    font-size: 0.85rem;
                    color: var(--q-text-secondary, #888);
                    margin: 0 0 0.75rem 0;
                    line-height: 1.5;
                    display: -webkit-box;
                    -webkit-line-clamp: 2;
                    -webkit-box-orient: vertical;
                    overflow: hidden;
                ",
                "{props.description}"
            }

            // Slug
            div {
                style: "
                    font-size: 0.75rem;
                    color: var(--q-text-muted, #555);
                    margin-bottom: 1rem;
                    font-family: 'Fira Code', 'Consolas', monospace;
                ",
                "quanghub.io/{props.slug}"
            }

            // Stats row
            div {
                style: "
                    display: flex;
                    gap: 1.25rem;
                    font-size: 0.8rem;
                    color: var(--q-text-secondary, #888);
                    border-top: 1px solid var(--q-surface-border, #333);
                    padding-top: 0.75rem;
                ",
                div {
                    style: "display: flex; align-items: center; gap: 0.35rem;",
                    span { "👥" }
                    span { "{props.member_count} members" }
                }
                div {
                    style: "display: flex; align-items: center; gap: 0.35rem;",
                    span { "📁" }
                    span { "{props.project_count} projects" }
                }
            }
        }
    }
}
