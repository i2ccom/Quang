# QuangHub Workplace Architecture

## Overview

The `quang-hub-workplace` crate is the collaboration graph engine for the QuangHub platform. It provides a unified data model, event system, view projections, and dual frontend/backend interfaces for building human-AI collaborative workspaces.

```
┌─────────────────────────────────────────────────────────────┐
│                    Dioxus Web UI (src-web/)                 │
│  Pages: Welcome, Dashboard, TaskBoard, GoalBoard, Channel   │
│  Components: Cards, Kanban, Chat, Agents, Progress Bars     │
│  Agent Panels: Chat, Goals, Tasks, Design                   │
│  Compiles with cfg(feature = "web")                         │
└──────────────────────────┬──────────────────────────────────┘
                           │ HTTP / WebSocket
┌──────────────────────────▼──────────────────────────────────┐
│              Cloudflare Worker API (src-server/)            │
│  REST + GraphQL handlers │ Auth (OAuth) │ Store (D1/KV/R2) │
│  Compiles with cfg(feature = "server")                     │
└──────────────────────────┬──────────────────────────────────┘
                           │
┌──────────────────────────▼──────────────────────────────────┐
│                 Core Engine (src/)                          │
│  ┌─────────┐ ┌──────────┐ ┌──────────────┐ ┌────────────┐  │
│  │ HyperGraph│ │ EventBus │ │ ViewRegistry │ │WorkplaceHub│  │
│  └─────────┘ └──────────┘ └──────────────┘ └────────────┘  │
│  │  ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐ ┌──────┐          │
│  │  │Worksp│ │ Team │ │Proj. │ │ Task │ │ Goal │   ...     │
│  │  └──────┘ └──────┘ └──────┘ └──────┘ └──────┘          │
└─────────────────────────────────────────────────────────────┘
```

## Core Concepts

### HyperGraph

All state lives in a typed directed graph. Nodes are entities (WorkSpace, Team, Task, Goal, etc.), edges are typed relationships (BelongsTo, AssignedTo, DependsOn, etc.). This makes the system:

- **Schema-flexible** — New entity types can be added without migrations
- **Traversable** — Any relationship can be queried in both directions
- **Projectable** — Views select subsets of nodes/edges for rendering

```rust
pub struct HyperGraph {
    pub nodes: HashMap<NodeId, serde_json::Value>,
    pub node_kinds: HashMap<NodeId, NodeKind>,
    pub edges: Vec<Edge>,
    pub out_edges: HashMap<NodeId, Vec<usize>>,
    pub in_edges: HashMap<NodeId, Vec<usize>>,
}
```

### EventBus

Every mutation emits a typed event. Events drive:
- **WebSocket pushes** — Real-time UI updates
- **AI agent triggers** — Agents subscribe to events they handle
- **Summary generation** — Periodic or event-driven digests
- **Audit logging** — Complete history of all changes

Events are tagged with an `ActorId` (human or agent) for provenance.

### View Projections

Views are graph projections that render the same underlying data in different layouts. Built-in views include Table, Kanban, Chart, and Gantt. Agents can register new view types at runtime via the `ViewRegistry`.

```rust
pub trait View: Send + Sync {
    fn view_type(&self) -> &str;
    fn display_name(&self) -> &str;
    fn project(&self, graph: &HyperGraph, config: &ViewConfig) -> ViewProjection;
}
```

### Actor-Native Design

Humans and AI agents share the `Actor` identity model. Both have:
- Unified `ActorProfile` (name, email, avatar, bio)
- Membership in `Team`s with explicit roles
- `Rank`/grade levels
- `ActorId` discrimination for provenance tracking

Agents additionally have:
- `AgentCapability` (model info, tools, languages)
- `AgentResourceAllocation` (token budgets, rate limits)
- `AgentReputation` (trust scores, accuracy, reviews)
- `AgentOwnership` (who created/governs the agent)

## Module Layout

```
quang-hub-workplace/
├── Cargo.toml              # Features: web, server, full
├── PLAN.md                 # Implementation roadmap
├── docs/
│   └── ARCHITECTURE.md     # This file
├── src/
│   ├── lib.rs              # Module root + re-exports
│   ├── graph.rs            # HyperGraph, NodeId, Edge, ActorId
│   ├── event.rs            # CollabEvent, EventBus, EventEnvelope
│   ├── view.rs             # View trait, ViewRegistry, ViewProjection
│   ├── workspace.rs        # WorkSpace model + methods
│   ├── team.rs             # Team + TeamMember + TeamRole
│   ├── project.rs          # Project + ProjectStatus + Priority
│   ├── task.rs             # Task + TaskStatus (state machine) + TaskSize
│   ├── goal.rs             # Goal + KeyResult + GoalStatus
│   ├── review.rs           # Review + ReviewComment + ReviewStatus
│   ├── channel.rs          # Channel + ChannelKind
│   ├── chat.rs             # ChatMessage + MessageContent + Reaction
│   ├── summary.rs          # Summary + SummarySection + SummaryKind
│   ├── hub.rs              # WorkplaceHub (orchestrator)
│   ├── actor.rs            # ActorProfile + Organization + Rank
│   ├── agent.rs            # AgentCapability, ResourceAllocation, Reputation
│   ├── human.rs            # HumanIdentity + ComplianceDocument + LeaveRecord
│   ├── audit.rs            # Audit logging
│   ├── compensation.rs     # Compensation structures
│   ├── contract.rs         # Contract management
│   ├── skill.rs            # Skill taxonomy
│   ├── worklog.rs          # Work log / time tracking
│   └── views/              # Built-in view implementations
│       ├── mod.rs          # View registration
│       ├── table.rs        # Table view
│       ├── kanban.rs       # Kanban view
│       ├── chart.rs        # Chart view
│       └── gantt.rs        # Gantt view
├── src-web/                # Dioxus web UI (cfg feature = "web")
│   ├── lib.rs              # Web module root
│   ├── pages/              # Page components
│   │   ├── mod.rs
│   │   ├── welcome.rs
│   │   ├── login.rs
│   │   ├── workspace_dashboard.rs
│   │   ├── task_board.rs
│   │   ├── goal_board.rs
│   │   ├── review_board.rs
│   │   └── channel_view.rs
│   └── components/         # Reusable components
│       ├── mod.rs
│       ├── workspace_card.rs
│       ├── team_card.rs
│       ├── project_card.rs
│       ├── task_card.rs
│       ├── kanban_column.rs
│       ├── chat_panel.rs
│       ├── channel_list.rs
│       ├── goal_progress.rs
│       ├── agent_chat_panel.rs
│       ├── agent_goal_panel.rs
│       ├── agent_task_panel.rs
│       └── agent_design_panel.rs
└── src-server/             # Cloudflare Worker (cfg feature = "server")
    ├── lib.rs              # Server module root
    ├── worker.rs           # Cloudflare Worker entry point
    ├── routes.rs           # Route definitions
    ├── graphql.rs          # GraphQL schema + resolvers
    ├── auth.rs             # OAuth (Google, GitHub) handlers
    ├── store.rs            # D1 / KV / R2 storage
    └── handlers/           # REST handlers
        ├── mod.rs
        ├── workspace.rs
        ├── team.rs
        ├── project.rs
        ├── channel.rs
        ├── chat.rs
        ├── task.rs
        ├── goal.rs
        ├── review.rs
        └── summary.rs
```

## Data Flow

### Read Path (UI → Backend → Graph)

```
User clicks "View Tasks"
  → Dioxus component fires REST call
  → Worker handler queries D1 via WorkplaceStore
  → Deserializes entities from JSON
  → Returns JSON response
  → Dioxus deserializes into component state (Signals)
  → UI re-renders
```

### Write Path (UI → Backend → Event → Graph)

```
User creates a task
  → TaskBoard page calls POST /api/projects/:id/tasks
  → Worker handler:
      1. Deserializes request body
      2. Creates Task entity with NodeId
      3. Inserts into D1 via WorkplaceStore
      4. Emits CollabEvent::TaskCreated
      5. Returns created Task as JSON
  → Dioxus component appends task to Signal
  → WebSocket push notifies other clients (future)
  → Agent triggers on TaskCreated (future)
```

### Agent Interaction Flow

```
Human sends message to agent channel
  → AgentChatPanel sends via on_send callback
  → REST handler stores message, emits MessageSent
  → Agent task (background worker) receives event
  → Agent processes input, generates response
  → Agent posts response via REST API
  → UI receives response via poll or WebSocket (future)
```

## Entity Hierarchy

```
WorkSpace
├── Teams (humans + agents with roles)
│   ├── TeamMember (actor + role + joined_at)
│   └── TeamRole (Owner, Admin, Member, Viewer, Agent, Custom)
├── Projects (time-bounded containers)
│   ├── Tasks (assignable work units)
│   │   ├── TaskStatus (state machine with valid transitions)
│   │   ├── TaskPriority (Critical, High, Medium, Low)
│   │   ├── TaskSize (Tiny, Small, Medium, Large, Epic)
│   │   └── TaskEvidence (completion proof)
│   ├── Goals (OKR-aligned objectives)
│   │   └── KeyResult (measurable milestones)
│   └── Reviews (approval gates)
│       ├── ReviewComment (feedback items)
│       └── ReviewStatus (Pending → InProgress → Approved/Rejected)
├── Channels (topic-based communication)
│   ├── ChannelKind (General, Project, Team, Agent, Digest, DM)
│   └── ChatMessage (threaded conversations)
│       ├── MessageContent (Text, Markdown, Code, Reference, Attachment)
│       └── Reaction (emoji + actor list)
└── Summaries (AI-generated or human-written digests)
    ├── SummaryKind (DailyStandup, SprintReview, MeetingNotes, etc.)
    └── SummarySection (structured content with references)
```

## CSS Variables

All Dioxus components use CSS custom properties for theming:

| Variable | Default | Purpose |
|---|---|---|
| `--q-bg` | `#0f0f1a` | Page background |
| `--q-surface` | `#1a1a2e` | Card/surface background |
| `--q-surface-border` | `#333` | Border color |
| `--q-text` | `#e0e0e0` | Primary text |
| `--q-text-secondary` | `#888` | Secondary text |
| `--q-text-muted` | `#555` | Muted/disabled text |
| `--q-primary` | `#6c5ce7` | Primary accent (purple) |
| `--q-accent` | `#00cec9` | Accent (teal/cyan) |

## Key Design Decisions

1. **Graph over SQL** — The HyperGraph enables schema-flexible relationships that a fixed relational schema cannot easily model. D1 stores entities as JSON blobs for flexibility.

2. **Dual module compilation** — The `web` and `server` features keep platform dependencies (Dioxus, worker-rs) optional, so the core data models remain portable.

3. **State machine for tasks** — Task transitions are validated by the data model itself (`TaskStatus::valid_transitions()`), preventing invalid state changes at every layer.

4. **Views as projections** — The same graph can be rendered as Kanban, Table, Chart, or Gantt without duplicating data. New views can be registered by agents.

5. **Actor unification** — Humans and agents share identity, teams, and roles. This enables mixed teams and agent-native workflows from day one.
