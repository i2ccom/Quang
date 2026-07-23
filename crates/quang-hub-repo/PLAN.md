# quang-hub-repo Implementation Plan

## Overview

This plan outlines the phased implementation of the `quang-hub-repo` crate. Each phase builds on the previous one, delivering incremental value while maintaining a shippable state at every milestone.

## Phase 1: GitHub Linking (Week 1)

**Goal**: Users can link a GitHub repository to QuangHub.

### Deliverables
- [x] Core data models: `LinkedRepo`, `RepoConnectionStatus`, `RepoSettings`, `WebhookConfig`, `BranchProtection`
- [x] `src-server/github_client.rs` — GitHub API client (get repo, get contents, get tree, get commits, get branches)
- [x] `src-server/handlers/repo_crud.rs` — CRUD handlers for linked repos
- [x] `src-web/pages/repo_new.rs` — Link new repo form
- [x] `src-web/pages/repo_home.rs` — List linked repos
- [x] `src-web/components/repo_card.rs` — Repo card component
- [x] `src-web/components/link_repo_dialog.rs` — Link repo dialog
- [x] `src-web/components/repo_search_bar.rs` — Search/filter repos

### Acceptance Criteria
- [ ] User can enter GitHub URL + access token to link a repo
- [ ] Linked repos appear in the repo home page
- [ ] Repo status (connected/syncing/disconnected) is displayed
- [ ] Search/filter works across all linked repos

### Dependencies
- `quang-graphql` or direct D1 storage for persistence
- Cloudflare Workers routing setup

---

## Phase 2: Repo Browser (Week 2)

**Goal**: Users can browse files, view code, and see commit history.

### Deliverables
- [x] `src-web/pages/repo_detail.rs` — Repo overview with tabs
- [x] `src-web/pages/repo_browse.rs` — File tree + code viewer layout
- [x] `src-web/components/file_tree.rs` — Expandable file/folder tree
- [x] `src-web/components/file_viewer.rs` — Code viewer with line numbers
- [x] `src-web/components/code_line.rs` — Individual code line rendering
- [x] `src-web/components/diff_view.rs` — Unified diff viewer
- [x] `src-web/components/commit_list.rs` — Commit history list
- [x] `src-server/handlers/proxy.rs` — GitHub API proxy handlers

### Acceptance Criteria
- [ ] File tree shows repo structure with expandable directories
- [ ] Clicking a file opens the code viewer with syntax highlighting
- [ ] Line numbers, file toolbar (copy, permalink, raw)
- [ ] Commit history with author, time, verification status
- [ ] Diff view for commit comparisons
- [ ] Branch selector to switch between branches

### Dependencies
- Phase 1 complete (repo must be linkable first)
- Fluid Remote or direct GitHub API for file content

---

## Phase 3: QTask — Agent Tasks (Week 3)

**Goal**: Users can create and monitor AI agent tasks against repos.

### Deliverables
- [x] Core model: `QTask`, `QTaskScope`, `QTaskAction`, `QTaskExecutionState`, `QTaskToolCall`
- [x] `src-web/pages/repo_tasks.rs` — Task board page
- [x] `src-web/components/qtask_board.rs` — Task list with filtering
- [ ] Task creation form (via API)
- [ ] Agent execution engine integration
- [ ] Task status updates via SSE

### Acceptance Criteria
- [ ] User can create a QTask (title, description, scope, action)
- [ ] Tasks appear in the board with priority, status, and assigned agent
- [ ] Filter by state (all/pending/running/completed)
- [ ] Agent tool calls are recorded and visible
- [ ] Task completion includes result summary and commit SHA

### Dependencies
- Phase 1-2 complete
- AI agent infrastructure (Shai agent system)
- Fluid Remote for agent file access

---

## Phase 4: Adaptive Repo Apps (Week 3-4)

**Goal**: Repos can define interactive apps via `qh.app` manifests.

### Deliverables
- [x] Core model: `RepoAppManifest`, `AppEntry`, `AppEntryType`, `AppRenderMode`, `AppNavItem`
- [x] `src-web/components/repo_app_viewer.rs` — Manifest renderer
- [ ] Manifest discovery (scan repo for `qh.app.json`/`.qh.app.toml`)
- [ ] Entry point rendering (component, page, markdown, HTML)
- [ ] App permissions system
- [ ] App actions integration with QuangHub UI

### Acceptance Criteria
- [ ] `qh.app.json` manifests are discovered in linked repos
- [ ] App viewer shows app selector with navigation tabs
- [ ] Apps render inline, as full page, sidebar, or modal per manifest config
- [ ] Theme overrides from manifest are applied
- [ ] App actions trigger QTask or other QuangHub features

### Dependencies
- Phase 1-2 complete
- Manifest parsing utilities
- Sandbox for third-party app rendering

---

## Phase 5: Fluid Remote (Week 4)

**Goal**: Internal mirror for fast agent access and safe write operations.

### Deliverables
- [x] `LinkedRepo.fluid_remote_url` field
- [x] `RepoEvent::FluidSyncStarted/Completed/Failed` events
- [ ] Fluid Remote git server (internal)
- [ ] Sync service (spawned on webhook push or poll)
- [ ] Pre-commit analysis hook
- [ ] Dependency scanning integration

### Architecture
```
GitHub Push ──► Webhook ──► Fluid Remote git pull
                                   │
                          ┌────────┴────────┐
                          │                 │
                     Pre-commit         Dependency
                     Analysis           Scanning
                          │                 │
                          └────────┬────────┘
                                   ▼
                           Event Bus ──► UI Update
```

### Acceptance Criteria
- [ ] Fluid Remote syncs automatically on push events
- [ ] Agents can clone from Fluid Remote in <1s
- [ ] Pre-commit analysis runs before sync completes
- [ ] Dependency scanning detects known vulnerabilities
- [ ] Sync status is visible in the repo UI

### Dependencies
- Phase 1-3 complete
- Git server infrastructure
- Analysis pipeline integration

---

## Phase 6: Auto-Deploy (Week 5)

**Goal**: Automatic Cloudflare Pages deployment on push.

### Deliverables
- [x] `src-server/deploy.rs` — Cloudflare Pages deploy API client
- [x] `RepoSettings.auto_deploy_enabled`, `auto_deploy_branch`
- [x] `RepoEvent::DeployStarted/Completed/Failed`
- [ ] Deploy status dashboard in repo UI
- [ ] Environment variable management for builds
- [ ] Deploy history view

### Flow
```
Push → Webhook → Match branch pattern?
    │
    ├── Yes ──► Trigger Cloudflare Pages Deploy
    │               │
    │               ├── DeployStarted event
    │               ├── DeployCompleted/DeployFailed event
    │               └── UI shows deploy URL + status
    │
    └── No  ──► Skip deploy
```

### Acceptance Criteria
- [ ] Deploy triggers automatically on matching branch pushes
- [ ] Deploy status is tracked and displayed in real-time
- [ ] Deploy history is available (last N deployments)
- [ ] Environment variables can be configured per repo
- [ ] Manual deploy button in repo UI

### Dependencies
- Phase 1-2 complete (repo must be linked and synced)
- Phase 5 complete (Fluid Remote for build source)
- Cloudflare API credentials configured

---

## Phase 7: Polish & Production (Week 6)

**Goal**: Production-ready with error handling, performance, and docs.

### Deliverables
- [ ] Comprehensive error handling and user-friendly error messages
- [ ] Loading states and skeleton screens for all components
- [ ] Pagination for commits, branches, and task lists
- [ ] Responsive design for mobile repo browsing
- [ ] Keyboard shortcuts (e.g., `t` for file finder)
- [ ] End-to-end tests
- [ ] Documentation refinement

### Acceptance Criteria
- [ ] All API errors show user-friendly messages
- [ ] Components handle loading/empty/error states gracefully
- [ ] Responsive layout works on mobile
- [ ] Test coverage >80% for core model logic
- [ ] ARCHITECTURE.md and PLAN.md are complete and accurate

## Completed Features (Current)

The following have been implemented in the initial crate scaffolding:

### Core Data Models
- [x] `LinkedRepo` — Full GitHub repo connection model
- [x] `RepoFile` — File/folder entry with MIME detection
- [x] `RepoBranch` — Branch info with ahead/behind tracking
- [x] `RepoCommit` — Full commit info with time_ago
- [x] `RepoAppManifest` — qh.app manifest with all entry types
- [x] `RepoSettings` — Mirror, deploy, webhook, branch protection
- [x] `QTask` — Agent task with scope, action, tool calls, state machine
- [x] `RepoEvent` — All typed repo events

### Web Components (Dioxus)
- [x] `RepoHome` — Linked repo list page
- [x] `RepoDetail` — Repo overview with tabs (Overview, Branches, Commits, Apps)
- [x] `RepoBrowse` — File tree + code viewer layout
- [x] `RepoTasks` — QTask board page
- [x] `RepoNew` — Link new repo form
- [x] `RepoCard` — Repo card with status indicator
- [x] `FileTree` — Expandable file tree with file icons
- [x] `FileViewer` — Code viewer with toolbar
- [x] `CodeLine` — Individual line with syntax coloring
- [x] `DiffView` — Unified diff display
- [x] `CommitList` — Commit history with avatars
- [x] `RepoSearchBar` — Search bar with clear button
- [x] `LinkRepoDialog` — Modal link dialog
- [x] `QTaskBoard` — Agent task list with filters
- [x] `RepoAppViewer` — Adaptive app renderer

### Server Handlers (Cloudflare Workers)
- [x] `GitHubClient` — Octocrab-style GitHub API client
- [x] `handle_webhook` — GitHub webhook handler (push, PR, ping, create)
- [x] `trigger_deploy` — Cloudflare Pages deploy client
- [x] `handle_create/get/list/update/delete_repo` — Repo CRUD
- [x] `handle_github_proxy` — GitHub API proxy
- [x] `handle_get_file/tree/commits/branches` — File/proxy handlers
