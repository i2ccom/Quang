//! App router — top-level route definitions for QuangHub.
//!
//! Defines all navigable routes across Workplace, Meet, Repo, and Agents.
//! Uses Dioxus Router with the Routable derive macro.

use dioxus::prelude::*;
use dioxus_router::prelude::*;

use crate::pages::*;

/// Top-level application routes.
#[derive(Debug, Clone, PartialEq, Routable)]
pub enum AppRoute {
    // ── Public routes ──
    #[route("/")]
    Welcome {},

    #[route("/login")]
    Login {},

    #[route("/privacy")]
    Privacy {},

    #[route("/terms")]
    Terms {},

    // ── Workplace routes ──
    #[route("/workspace")]
    #[layout(AppShell)]
    WorkplaceHome {},

    #[route("/workspace/:workspace_id")]
    WorkplaceDetail { workspace_id: String },

    #[route("/workspace/:workspace_id/team/:team_id")]
    TeamDetail {
        workspace_id: String,
        team_id: String,
    },

    #[route("/workspace/:workspace_id/project/:project_id")]
    ProjectDetail {
        workspace_id: String,
        project_id: String,
    },

    #[route("/workspace/:workspace_id/channel/:channel_id")]
    ChannelDetail {
        workspace_id: String,
        channel_id: String,
    },

    #[route("/workspace/:id/tasks")]
    TaskBoard { id: String },

    #[route("/workspace/:id/goals")]
    GoalBoard { id: String },

    #[route("/workspace/:id/reviews")]
    ReviewBoard { id: String },

    // ── Meet routes ──
    #[route("/meet")]
    #[layout(AppShell)]
    MeetHome {},

    #[route("/meet/:room_id")]
    MeetRoom { room_id: String },

    #[route("/meet/schedule")]
    MeetSchedule {},

    // ── Repo routes ──
    #[route("/repo")]
    #[layout(AppShell)]
    RepoHome {},

    #[route("/repo/:repo_id")]
    RepoDetail { repo_id: String },

    #[route("/repo/:repo_id/browse/*path")]
    RepoBrowse { repo_id: String, path: Vec<String> },

    #[route("/repo/:repo_id/tasks")]
    RepoTasks { repo_id: String },

    #[route("/repo/new")]
    RepoNew {},

    // ── Agent routes ──
    #[route("/agent/chat")]
    AgentChat {},

    #[route("/agent/goal")]
    AgentGoal {},

    #[route("/agent/task")]
    AgentTask {},

    #[route("/agent/design")]
    AgentDesign {},

    // ── Settings / Profile ──
    #[route("/settings")]
    #[layout(AppShell)]
    Settings {},

    #[route("/settings/organization")]
    OrgSettings {},

    #[route("/profile")]
    #[layout(AppShell)]
    Profile {},

    // ── API routes (show backend-needed message) ──
    #[route("/api/:..path")]
    ApiRoute { path: Vec<String> },

    // ── 404 ──
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

/// Shared layout shell for authenticated routes.
#[component]
pub fn AppShell() -> Element {
    rsx! {
        div { class: "quang-app-shell",
            quang_web::components::TopBar {}
            div { class: "quang-app-body",
                quang_web::components::Sidebar {}
                main { class: "quang-app-content",
                    Outlet::<AppRoute> {}
                }
            }
        }
    }
}
