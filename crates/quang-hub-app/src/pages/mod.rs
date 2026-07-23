//! Page components for QuangHub routes.
//!
//! These are the top-level page components referenced by the router.
//! Most pages delegate to feature-specific components from the sub-crates.
//! These wrappers exist so the router can resolve all types in one crate.

use dioxus::prelude::*;

// ── Public pages ──

#[component]
pub fn Welcome() -> Element {
    rsx! {
        div { class: "welcome-page",
            h1 { "QuangHub" }
            p { class: "subtitle", "AI-native collaboration platform. Build, meet, and ship with intelligent agents." }
            div { class: "feature-list",
                div { class: "feature-chip",
                    div { class: "dot", style: "background: var(--q-primary);" }
                    "AI Agents"
                }
                div { class: "feature-chip",
                    div { class: "dot", style: "background: var(--q-accent);" }
                    "Workspace"
                }
                div { class: "feature-chip",
                    div { class: "dot", style: "background: var(--q-success);" }
                    "Video Meet"
                }
                div { class: "feature-chip",
                    div { class: "dot", style: "background: var(--q-warning);" }
                    "Repo Sync"
                }
                div { class: "feature-chip",
                    div { class: "dot", style: "background: var(--q-danger);" }
                    "GraphQL"
                }
                div { class: "feature-chip",
                    div { class: "dot", style: "background: #8d6cff;" }
                    "Cloudflare"
                }
            }
            div { class: "auth-buttons",
                button {
                    class: "btn-primary",
                    onclick: move |_| quang_web::auth::login(quang_web::auth::IdentityProvider::Google),
                    "Continue with Google"
                }
                button {
                    class: "btn-ghost",
                    onclick: move |_| quang_web::auth::login(quang_web::auth::IdentityProvider::GitHub),
                    "Continue with GitHub"
                }
            }
        }
    }
}

#[component]
pub fn Login() -> Element {
    rsx! {
        div { class: "welcome-page",
            h1 { "Sign in to QuangHub" }
            div { class: "auth-buttons",
                button {
                    class: "btn-primary",
                    onclick: move |_| quang_web::auth::login(quang_web::auth::IdentityProvider::Google),
                    "Google"
                }
                button {
                    class: "btn-ghost",
                    onclick: move |_| quang_web::auth::login(quang_web::auth::IdentityProvider::GitHub),
                    "GitHub"
                }
            }
        }
    }
}

#[component]
pub fn Privacy() -> Element {
    rsx! { div { "Privacy Policy — coming soon" } }
}

#[component]
pub fn Terms() -> Element {
    rsx! { div { "Terms of Service — coming soon" } }
}

// ── Workplace pages ──

#[component]
pub fn WorkplaceHome() -> Element {
    rsx! {
        div { class: "page",
            h2 { "Workplace" }
            p { "Your workspaces and teams." }
        }
    }
}

#[component]
pub fn WorkplaceDetail(workspace_id: String) -> Element {
    rsx! { div { class: "page", h2 { "Workspace: {workspace_id}" } } }
}

#[component]
pub fn TeamDetail(workspace_id: String, team_id: String) -> Element {
    rsx! { div { class: "page", h2 { "Team: {team_id}" } } }
}

#[component]
pub fn ProjectDetail(workspace_id: String, project_id: String) -> Element {
    rsx! { div { class: "page", h2 { "Project: {project_id}" } } }
}

#[component]
pub fn ChannelDetail(workspace_id: String, channel_id: String) -> Element {
    rsx! { div { class: "page", h2 { "Channel: {channel_id}" } } }
}

#[component]
pub fn TaskBoard(id: String) -> Element {
    rsx! {
        div { class: "page",
            h2 { "Task Board" }
            p { "Kanban board for workspace {id}" }
        }
    }
}

#[component]
pub fn GoalBoard(id: String) -> Element {
    rsx! {
        div { class: "page",
            h2 { "Goals" }
            p { "Goals for workspace {id}" }
        }
    }
}

#[component]
pub fn ReviewBoard(id: String) -> Element {
    rsx! {
        div { class: "page",
            h2 { "Reviews" }
            p { "Reviews for workspace {id}" }
        }
    }
}

// ── Meet pages ──

#[component]
pub fn MeetHome() -> Element {
    rsx! {
        div { class: "page",
            h2 { "Meet" }
            p { "Video rooms and meetings." }
        }
    }
}

#[component]
pub fn MeetRoom(room_id: String) -> Element {
    rsx! { div { class: "page", h2 { "Room: {room_id}" } } }
}

#[component]
pub fn MeetSchedule() -> Element {
    rsx! {
        div { class: "page",
            h2 { "Schedule a Meeting" }
        }
    }
}

// ── Repo pages ──

#[component]
pub fn RepoHome() -> Element {
    rsx! {
        div { class: "page",
            h2 { "Repositories" }
            p { "Your linked GitHub repos." }
        }
    }
}

#[component]
pub fn RepoDetail(repo_id: String) -> Element {
    rsx! { div { class: "page", h2 { "Repo: {repo_id}" } } }
}

#[component]
pub fn RepoBrowse(repo_id: String, path: Vec<String>) -> Element {
    let path_str = path.join("/");
    rsx! { div { class: "page", h2 { "Repo: {repo_id} / {path_str}" } } }
}

#[component]
pub fn RepoTasks(repo_id: String) -> Element {
    rsx! { div { class: "page", h2 { "Repo Tasks: {repo_id}" } } }
}

#[component]
pub fn RepoNew() -> Element {
    rsx! {
        div { class: "page",
            h2 { "Link New Repository" }
        }
    }
}

// ── Agent pages ──

#[component]
pub fn AgentChat() -> Element {
    rsx! {
        div { class: "page",
            h2 { "AI Chat" }
            p { "Chat with AI agents." }
        }
    }
}

#[component]
pub fn AgentGoal() -> Element {
    rsx! {
        div { class: "page",
            h2 { "AI Goals" }
        }
    }
}

#[component]
pub fn AgentTask() -> Element {
    rsx! {
        div { class: "page",
            h2 { "AI Tasks" }
        }
    }
}

#[component]
pub fn AgentDesign() -> Element {
    rsx! {
        div { class: "page",
            h2 { "AI Design" }
        }
    }
}

// ── Settings / Profile ──

#[component]
pub fn Settings() -> Element {
    rsx! {
        div { class: "page",
            h2 { "Settings" }
        }
    }
}

#[component]
pub fn OrgSettings() -> Element {
    rsx! {
        div { class: "page",
            h2 { "Organization Settings" }
        }
    }
}

#[component]
pub fn Profile() -> Element {
    rsx! {
        div { class: "page",
            h2 { "Profile" }
        }
    }
}

// ── API route ──

#[component]
pub fn ApiRoute(path: Vec<String>) -> Element {
    let full_path = path.join("/");
    rsx! {
        div { class: "page", style: "display: flex; flex-direction: column; align-items: center; justify-content: center; min-height: 60vh; gap: 16px; text-align: center;",
            div { style: "font-size: 48px; opacity: 0.4;", "🔌" }
            h2 { "API Backend Required" }
            p { style: "color: var(--q-text-secondary); max-width: 480px;",
                "The endpoint "
                code { style: "color: var(--q-primary); font-family: monospace; padding: 2px 6px; background: var(--q-surface); border-radius: 4px;", "/api/{full_path}" }
                " needs a running Cloudflare Workers backend.\nStart it separately with "
                code { style: "color: var(--q-accent); font-family: monospace; padding: 2px 6px; background: var(--q-surface); border-radius: 4px;", "npx wrangler dev" }
                "."
            }
            div { class: "auth-buttons", style: "margin-top: 8px;",
                Link {
                    to: "/",
                    style: "padding: 8px 20px; border-radius: 100px; font-size: 13px; background: linear-gradient(135deg, var(--q-primary), #6c3ce0); color: white; text-decoration: none; font-weight: 500;",
                    "Back to Home"
                }
            }
        }
    }
}

// ── 404 ──

#[component]
pub fn PageNotFound(route: Vec<String>) -> Element {
    let path = route.join("/");
    rsx! {
        div { class: "page",
            h2 { "404 — Page Not Found" }
            p { "No route matches: /{path}" }
        }
    }
}
