//! Sidebar — left navigation panel for workspace/meet/repo modules.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

#[component]
pub fn Sidebar() -> Element {
    rsx! {
        aside {
            class: "sidebar",
            style: "position: fixed; left: 0; top: var(--q-topbar-height); bottom: 0; width: var(--q-sidebar-width); background: rgba(14, 14, 22, 0.9); border-right: 1px solid var(--q-border); padding: 20px 12px; overflow-y: auto; display: flex; flex-direction: column; gap: 2px; z-index: 50;",

            SidebarSection { title: "Workplace",
                SidebarLink { to: "/workspace", label: "Dashboard", icon: "◆" }
                SidebarLink { to: "/workspace/all/tasks", label: "Tasks", icon: "☐" }
                SidebarLink { to: "/workspace/all/goals", label: "Goals", icon: "★" }
                SidebarLink { to: "/workspace/all/reviews", label: "Reviews", icon: "◎" }
            }

            SidebarSection { title: "Meet",
                SidebarLink { to: "/meet", label: "Rooms", icon: "📹" }
                SidebarLink { to: "/meet/schedule", label: "Schedule", icon: "📅" }
            }

            SidebarSection { title: "Repos",
                SidebarLink { to: "/repo", label: "My Repos", icon: "📁" }
                SidebarLink { to: "/repo/new", label: "New Repo", icon: "➕" }
            }

            SidebarSection { title: "AI Agents",
                SidebarLink { to: "/agent/chat", label: "Chat", icon: "💬" }
                SidebarLink { to: "/agent/goal", label: "Goals", icon: "🎯" }
                SidebarLink { to: "/agent/task", label: "Tasks", icon: "⚡" }
                SidebarLink { to: "/agent/design", label: "Design", icon: "🎨" }
            }

            div { style: "flex: 1;" }
            div { style: "border-top: 1px solid var(--q-border); padding-top: 8px; margin-top: 8px;" }
            SidebarLink { to: "/settings", label: "Settings", icon: "⚙️" }
        }
    }
}

#[component]
fn SidebarSection(title: String, children: Element) -> Element {
    rsx! {
        div { style: "margin-bottom: 4px;",
            div { style: "padding: 8px 10px 4px 10px; font-size: 11px; font-weight: 600; text-transform: uppercase; letter-spacing: 0.08em; color: var(--q-text-muted);", "{title}" }
            {children}
        }
    }
}

#[component]
fn SidebarLink(to: String, label: String, icon: String) -> Element {
    let color = "var(--q-text-secondary)".to_string();
    let bg = "transparent".to_string();

    rsx! {
        Link {
            to: to,
            style: "display: flex; align-items: center; gap: 10px; padding: 8px 10px; border-radius: var(--q-radius); font-size: 13px; font-weight: 500; color: {color}; background: {bg}; text-decoration: none; transition: all var(--q-transition);",
            span { style: "font-size: 16px; width: 20px; text-align: center; opacity: 0.7;", "{icon}" }
            span { "{label}" }
        }
    }
}
