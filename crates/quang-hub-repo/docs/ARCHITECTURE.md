# quang-hub-repo Architecture

## Overview

`quang-hub-repo` is the repository integration crate for QuangHub. It provides a complete system for linking GitHub repositories, browsing code, running AI agent tasks against repos, rendering adaptive repo apps from `qh.app` manifests, and deploying to Cloudflare Pages — all within the QuangHub collaboration platform.

## Dual Remote Model

Every linked repository has **two remote references**:

```
┌─────────────────────────────────────────────────────────┐
│                   LinkedRepo                            │
│                                                         │
│  ┌─────────────────┐      ┌──────────────────────────┐  │
│  │   Upstream      │      │    Fluid Remote          │  │
│  │   (GitHub)      │      │    (QuangHub Internal)   │  │
│  │                 │      │                          │  │
│  │  • Canonical    │      │  • Fast clone for agents │  │
│  │  • Read/write   │      │  • No direct GitHub push │  │
│  │    via OAuth    │      │  • Pre-commit analysis   │  │
│  │  • Source of    │      │  • Auto-deploy trigger   │  │
│  │    truth        │      │  • Syncs via webhook     │  │
│  └─────────────────┘      └──────────────────────────┘  │
└─────────────────────────────────────────────────────────┘
```

### Why Dual Remote?

- **Security**: AI agents never get direct GitHub push access. They write to the Fluid Remote, which acts as a review gate.
- **Speed**: The Fluid Remote lives in the same infrastructure as QuangHub, enabling sub-second clone times for agent tools.
- **Analysis**: Pre-commit hooks, dependency scanning, and summary generation all happen on the Fluid Remote before changes touch GitHub.
- **GitHub-free operation**: The Fluid Remote can operate even if GitHub is unreachable, with changes synced back when connectivity restores.

## Feature Architecture

The crate is organized into three feature-gated modules:

```
quang-hub-repo
├── src/              # Core data models (always compiled)
│   ├── lib.rs
│   ├── linked_repo.rs
│   ├── repo_file.rs
│   ├── repo_branch.rs
│   ├── repo_commit.rs
│   ├── repo_app.rs
│   ├── repo_settings.rs
│   ├── q_task.rs
│   └── repo_event.rs
│
├── src-web/          # Dioxus UI components (feature = "web")
│   ├── lib.rs
│   ├── pages/
│   │   ├── repo_home.rs
│   │   ├── repo_detail.rs
│   │   ├── repo_browse.rs
│   │   ├── repo_tasks.rs
│   │   └── repo_new.rs
│   └── components/
│       ├── repo_card.rs
│       ├── file_tree.rs
│       ├── file_viewer.rs
│       ├── code_line.rs
│       ├── diff_view.rs
│       ├── commit_list.rs
│       ├── repo_search_bar.rs
│       ├── link_repo_dialog.rs
│       ├── qtask_board.rs
│       └── repo_app_viewer.rs
│
├── src-server/       # Cloudflare Workers handlers (feature = "server")
│   ├── lib.rs
│   ├── github_client.rs
│   ├── webhook.rs
│   ├── deploy.rs
│   └── handlers/
│       ├── repo_crud.rs
│       └── proxy.rs
│
└── docs/
    └── ARCHITECTURE.md
```

## Data Model

### LinkedRepo

The central entity representing a GitHub repository connection:

```
LinkedRepo
├── id: RepoId (UUID)
├── owner: String (GitHub user/org)
├── name: String (repo name)
├── url: String (GitHub URL)
├── default_branch: String
├── is_linked: bool
├── status: RepoConnectionStatus
│   ├── Connected
│   ├── Disconnected
│   ├── Syncing
│   ├── AuthFailed
│   └── Archived
├── fluid_remote_url: Option<String>
├── settings: RepoSettings
│   ├── mirror_enabled: bool
│   ├── auto_deploy_enabled: bool
│   ├── webhook: WebhookConfig
│   ├── branch_protection: BranchProtection
│   └── ...
├── access_token: Option<String> (server-only, encrypted)
└── workspace_id: Option<String>
```

### QTask — Agent Tasks for Repos

QTask extends the workplace `Task` model with repo-specific fields:

```
QTask
├── id: QTaskId (UUID)
├── parent_task_id: Option<String> (links to workplace Task)
├── repo_id: RepoId
├── title: String
├── description: String
├── scope: QTaskScope
│   ├── Repository (entire repo)
│   ├── Directory(path)
│   ├── File(path)
│   ├── Branch(name)
│   └── PullRequest(number)
├── action: QTaskAction
│   ├── Refactor
│   ├── Feature
│   ├── BugFix
│   ├── Test
│   ├── Documentation
│   ├── Review
│   ├── Analyze
│   ├── Deploy
│   ├── Summarize
│   └── Custom(String)
├── state: QTaskExecutionState
│   ├── Pending → Running → Completed
│   ├── Running → AwaitingInput → Running
│   └── Running/Failed/Cancelled (terminal)
├── tool_calls: Vec<QTaskToolCall>
├── affected_files: Vec<String>
└── result_summary: Option<String>
```

### RepoAppManifest — Adaptive Repo Apps

A `qh.app` manifest (`.qh.app.json` or `.qh.app.toml`) allows any repository to define interactive UI that renders within the QuangHub repo browser:

```
RepoAppManifest
├── version: String (schema version)
├── id: String (kebab-case app identifier)
├── name: String (display name)
├── entries: Vec<AppEntry>
│   ├── Component (Dioxus component)
│   ├── Page (full route)
│   ├── Script (sidecar script)
│   ├── Markdown (rendered content)
│   └── Html (embedded iframe)
├── renderMode: AppRenderMode
│   ├── Inline
│   ├── FullPage
│   ├── Sidebar
│   ├── Modal
│   └── Tab
├── permissions: Option<Vec<String>>
├── theme: Option<AppTheme>
└── actions: Option<Vec<AppAction>>
```

This enables "adaptive repo apps" — any repo can ship its own UI components, dashboards, playgrounds, or tools that render directly inside QuangHub without needing a separate frontend deployment.

## GitHub Mirror Flow

```
Push to GitHub
     │
     ▼
GitHub Webhook ──► QuangHub Webhook Handler
     │
     ├── Deploy? ──► Cloudflare Pages Deploy
     │
     └── Sync?  ──► Fluid Remote git pull
                          │
                          ▼
                    Pre-commit analysis
                    Dependency scanning
                    Summary generation
                          │
                          ▼
                    Notify event bus
                    Update UI via SSE
```

## Auto-Deploy Flow

```
1. Push to branch matching pattern (e.g. "main")
2. Webhook handler receives push event
3. If auto_deploy_enabled && branch matches auto_deploy_branch:
   a. Build the project
   b. Deploy to Cloudflare Pages via API v4
   c. Track deploy status
   d. Emit DeployStarted / DeployCompleted / DeployFailed event
   e. Update UI with deploy URL and status
```

## QTask Execution Flow

```
1. User creates QTask (via UI or API)
2. QTask is assigned to an agent (or queued)
3. Agent reads repo content via Fluid Remote (fast clone)
4. Agent makes tool calls (read_file, edit_file, run_command, etc.)
5. Each tool call is recorded in QTask.tool_calls
6. Agent writes changes to Fluid Remote
7. Agent completes task with summary
8. Changes can be pushed to GitHub via PR or direct push
9. RepoEvent emitted for each state change
```

## Event System

All repo mutations emit typed `RepoEvent` variants:

- **Repo lifecycle**: RepoLinked, RepoUnlinked, RepoUpdated, RepoStatusChanged
- **Sync**: FluidSyncStarted, FluidSyncCompleted, FluidSyncFailed
- **GitHub events**: PushReceived, BranchCreated, BranchDeleted, TagCreated
- **PR events**: PullRequestOpened, PullRequestMerged, PullRequestClosed
- **Webhook events**: WebhookRegistered, WebhookPing
- **Deploy events**: DeployStarted, DeployCompleted, DeployFailed
- **QTask events**: QTaskCreated, QTaskStateChanged, QTaskCompleted
- **Adaptive apps**: AppManifestUpdated

Events propagate through the QuangHub event bus (SSE + Signals) to update UI in real-time.

## CSS Design System

All components use CSS custom properties from `quang-web`:

```
--q-bg: #0f0f13
--q-surface: #1a1a22
--q-surface-hover: #22222c
--q-border: #2a2a35
--q-primary: #6c5ce7
--q-primary-hover: #7c6cf7
--q-accent: #00cec9
--q-text: #e8e8ed
--q-text-secondary: #9a9aa8
--q-danger: #ff6b6b
--q-success: #51cf66
--q-warning: #fcc419
--q-radius: 8px
--q-radius-lg: 12px
```

## Dependencies

### Core (always)
- serde + serde_json (serialization)
- chrono (timestamps)
- uuid (identifiers)
- thiserror (error handling)
- quang-hub-workplace (Task model reference)

### Web (feature = "web")
- dioxus (UI framework)
- dioxus-router (client-side routing)
- quang-web (shared components, event bus, auth)

### Server (feature = "server")
- worker (Cloudflare Workers SDK)
- reqwest (HTTP client for GitHub & Cloudflare APIs)
- git2 (Git operations for Fluid Remote sync)
