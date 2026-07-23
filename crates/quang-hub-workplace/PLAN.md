# QuangHub Workplace Implementation Plan

## Overview

Phase-based plan for building the `quang-hub-workplace` crate — the collaboration graph ecosystem for the QuangHub platform. The crate supports humans and AI agents as first-class collaborators with a unified data model, event-driven architecture, view projections, and dual frontend/backend interfaces.

### Current Status

- **Core data models**: ✅ Complete (HyperGraph, EventBus, all entities)
- **Built-in views**: ✅ Complete (Table, Kanban, Chart, Gantt)
- **Web UI (Dioxus)**: 🚧 In progress (Phase 2)
- **Server handlers (Cloudflare)**: 🚧 In progress (Phase 3)
- **Agent integration**: 📅 Planned (Phase 4)
- **GraphQL API**: 🚧 In progress (Phase 3)
- **Deployment**: 📅 Planned (Phase 5)

---

## Phase 1: Core Data Models ✅ [Done]

**Goal**: All entity types, HyperGraph, EventBus, View registry.

### Completed
- [x] `graph.rs` — HyperGraph, NodeId, Edge, ActorId, typed nodes/edges
- [x] `event.rs` — CollabEvent enum, EventBus, EventEnvelope
- [x] `view.rs` — View trait, ViewRegistry, ViewConfig, ViewProjection
- [x] `workspace.rs` — WorkSpace with settings and metadata
- [x] `team.rs` — Team, TeamMember, TeamRole with human/agent discrimination
- [x] `project.rs` — Project with status machine and priority
- [x] `task.rs` — Full state machine (9 states with valid transitions), priority, size, evidence, skills, budget
- [x] `goal.rs` — Goal with Key Results, progress computation, status auto-calculation
- [x] `review.rs` — Review with comments, approval chains, change requests
- [x] `channel.rs` — Channel with kinds and member management
- [x] `chat.rs` — ChatMessage with content types, threads, reactions
- [x] `summary.rs` — Summary with sections and references
- [x] `hub.rs` — WorkplaceHub orchestrator wrapping Graph + Events + Views
- [x] `actor.rs` — ActorProfile, Organization, Rank (shared human/agent)
- [x] `agent.rs` — AgentCapability, AgentResourceAllocation, AgentReputation, AgentOwnership
- [x] `human.rs` — HumanIdentity, ComplianceDocument, LeaveRecord
- [x] `views/` — Table, Kanban, Chart, Gantt view implementations
- [x] `audit.rs`, `compensation.rs`, `contract.rs`, `skill.rs`, `worklog.rs`

---

## Phase 2: Web UI (Dioxus) 🚧 [In Progress]

**Goal**: Full Dioxus web interface replacing Yew components.

### Pages
- [x] `welcome.rs` — Hero section, QuangHub branding, OAuth login buttons (Google/GitHub)
- [x] `login.rs` — Login card with OAuth buttons, email/password form, validation
- [x] `workspace_dashboard.rs` — Workspace grid, create dialog modal, search
- [x] `task_board.rs` — Kanban board with 5 columns, search filter, drag-ready
- [x] `goal_board.rs` — Goal listing with progress bars, key results, status badges
- [x] `review_board.rs` — Reviews grouped by status, filter, approval tracking
- [x] `channel_view.rs` — Chat interface with channel sidebar, messages, input

### Components
- [x] `workspace_card.rs` — Clickable card with stats, owner badge
- [x] `team_card.rs` — Team display with human/agent composition bar, avatars
- [x] `project_card.rs` — Project card with status, priority, progress bar
- [x] `task_card.rs` — Draggable card with priority colors, tags, assignee
- [x] `kanban_column.rs` — Drop-target column with drag-over state
- [x] `chat_panel.rs` — Message feed with agent/human styling, reactions
- [x] `channel_list.rs` — Sidebar with unread badges, active state
- [x] `goal_progress.rs` — Animated progress bar with color transitions, compact mode
- [x] `agent_chat_panel.rs` — Agent chat with code blocks, typing indicator
- [x] `agent_goal_panel.rs` — AI goal assistant with generate and assign
- [x] `agent_task_panel.rs` — AI task decomposer with skill matching
- [x] `agent_design_panel.rs` — AI design panel with tabs, suggestions

### Styling
- [x] CSS variables (`--q-bg`, `--q-surface`, `--q-primary`, `--q-text`, etc.)
- [x] Dark theme with consistent color palette
- [x] Responsive layouts (grid, flexbox)
- [x] Hover/active/transition animations

### Remaining
- [ ] Drag-and-drop (HTML5 drag API) — currently wired but needs real task ID transfer
- [ ] Router integration — page components use `navigator().push()` but need route definitions
- [ ] Backend data binding — components use stub data; need `fetch()` calls to REST API
- [ ] Real-time WebSocket updates — EventBus should push to connected clients
- [ ] Testing — Dioxus component tests with `dioxus_testing`

---

## Phase 3: Server Handlers (Cloudflare Workers) 🚧 [In Progress]

**Goal**: REST + GraphQL API on Cloudflare Workers with D1, KV, R2.

### Infrastructure
- [x] `worker.rs` — Worker entry point, router setup, CORS, logging
- [x] `routes.rs` — All REST endpoint definitions, health check
- [x] `store.rs` — D1 (query, insert, update, delete), KV (put/get/delete), R2 (put/get/delete/list)

### Handlers
- [x] `workspace.rs` — List, Create, Get, Update, Delete with D1 integration
- [x] `team.rs` — List, Create, Get, Update, AddMember, RemoveMember
- [x] `project.rs` — List, Create, Get, Update, Delete with progress
- [x] `channel.rs` — List, Create, Get, Update
- [x] `chat.rs` — List, Send, Update, Delete, AddReaction
- [x] `task.rs` — List, Create, Get, Update, Transition (state machine), Assign
- [x] `goal.rs` — List, Create, Get, Update, AddKeyResult
- [x] `review.rs` — List, Create, Get, Approve, RequestChanges, Reject, AddComment
- [x] `summary.rs` — List, Create, Get with sections

### Auth
- [x] `auth.rs` — OAuth start/callback (Google, GitHub), login, register
- [ ] Session management with KV
- [ ] Token refresh

### GraphQL
- [x] `graphql.rs` — Schema with QueryRoot and MutationRoot, type definitions
- [ ] Resolver implementations (currently return stubs)
- [ ] Subscription support for real-time events

### Remaining
- [ ] D1 schema migrations (CREATE TABLE statements for each entity)
- [ ] Rate limiting
- [ ] Request validation middleware
- [ ] Error response standardization
- [ ] API documentation (OpenAPI/Swagger)

---

## Phase 4: Agent Integration 📅 [Planned]

**Goal**: AI agents as active participants in the workplace.

### Agent Capabilities
- [ ] Agent registration via `AgentCapability` manifest
- [ ] Tool/function calling framework (agents call REST APIs)
- [ ] Task assignment to agents via `Task::required_skills` + `AgentCapability::tools`
- [ ] Agent budget tracking (`Task::budget` + `AgentResourceAllocation`)

### Agent Event Subscriptions
- [ ] EventBus → Agent trigger pipeline
- [ ] Agent listens for `TaskCreated` with matching skills
- [ ] Agent automatically picks up assignable tasks
- [ ] Agent posts results as `ChatMessage` / `TaskEvidence`

### Agent Reputation
- [ ] `AgentReputation` tracking (trust scores, accuracy)
- [ ] Human review of agent outputs
- [ ] Automatic score adjustment

### Agent UI
- [x] Agent chat panel (src-web)
- [x] Agent goal panel (src-web)
- [x] Agent task panel (src-web)
- [x] Agent design panel (src-web)
- [ ] Agent status dashboard
- [ ] Agent resource usage visualization

---

## Phase 5: GraphQL API Completion 📅 [Planned]

**Goal**: Full GraphQL interface with all resolvers and subscriptions.

- [ ] Implement all QueryRoot resolvers (workspace, project, task, goal, etc.)
- [ ] Implement all MutationRoot resolvers
- [ ] Add SubscriptionRoot (real-time events via WebSocket)
- [ ] N+1 query optimization (DataLoader pattern)
- [ ] GraphQL schema documentation
- [ ] Rate limiting and depth limiting
- [ ] Federation support (Apollo Federation)

---

## Phase 6: Testing & QA 📅 [Planned]

**Goal**: Comprehensive test coverage across all layers.

### Unit Tests
- [ ] Task state machine transitions
- [ ] Goal progress computation
- [ ] Review approval logic
- [ ] HyperGraph node/edge operations
- [ ] EventBus emit/replay
- [ ] View projections (Table, Kanban, Chart, Gantt)

### Integration Tests
- [ ] REST API endpoint tests (with mock D1)
- [ ] GraphQL query/mutation tests
- [ ] Auth flow tests
- [ ] Agent capability matching

### Web Tests
- [ ] Dioxus component rendering tests
- [ ] User interaction tests (click, input, drag)
- [ ] Accessibility audit

---

## Phase 7: Deployment 📅 [Planned]

**Goal**: Production-ready deployment on Cloudflare Workers.

- [ ] `wrangler.toml` configuration
- [ ] D1 database creation and migration
- [ ] KV namespace setup for sessions
- [ ] R2 bucket for file attachments
- [ ] Environment secrets (OAuth client IDs, API keys)
- [ ] CI/CD pipeline (GitHub Actions)
- [ ] Custom domain + SSL
- [ ] Monitoring and logging (Cloudflare Analytics + Sentry)
- [ ] Rate limiting (Cloudflare WAF)
- [ ] Backup strategy (D1 exports)

---

## Dependency Graph

```
Phase 1: Core Models
    └── Phase 2: Web UI (depends on models)
    └── Phase 3: Server Handlers (depends on models)
        └── Phase 4: Agent Integration (depends on server + models)
        └── Phase 5: GraphQL Completion (depends on server)
    └── Phase 6: Testing (depends on all above)
    └── Phase 7: Deployment (depends on all above)
```

## File Count Summary

| Area | Files | Status |
|---|---|---|
| Core data models (`src/`) | 24 | ✅ Done |
| Web UI (`src-web/`) | 20 | 🚧 95% done |
| Server handlers (`src-server/`) | 15 | 🚧 80% done |
| Documentation | 2 | ✅ Done |
| **Total** | **61** | |
