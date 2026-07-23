# Quang Workplace Collaboration — PLAN & Implementation

## Vision

Quang Workplace is the **collaboration layer** of QuangHub — an agent-native, graph-based ecosystem where humans and AI agents collaborate through shared workspaces, channels, tasks, goals, reviews, and meetings. Everything is a typed node in a HyperGraph, enabling flexible views (Table, Kanban, Chart, Gantt) that adapt to context.

## Core Concepts

### Graph-First Data Model

Every entity is a **typed node** in a HyperGraph:
- **WorkSpace** — Top-level org unit (company, open-source org)
- **Team** — Group of humans + agents with roles
- **Project** — Time-bounded container with goals
- **Channel** — Communication stream (chat, events, topics)
- **Task** — Assignable unit of work (state machine: Backlog → Ready → InProgress → Review → Done)
- **Goal** — High-level objective (aligned to OKR pattern)
- **Review** — Approval/feedback gate (code review, task review, milestone gate)
- **Summary** — AI-generated or human-written digest of work
- **Meeting** — Real-time call with participants, media, recording

Relationships are **typed edges** (e.g., `belongs_to`, `assigned_to`, `depends_on`, `reviewed_by`, `generated_from`).

### Dual Interface

Every entity exposes:
- **Human UI** — Yew components (Table, Kanban, Chart, Gantt)
- **Agent Interface** — Rust types + JSON schema + MCP tools

### Flexible Views (Graph Projections)

| View | Description | Use Case |
|------|-------------|----------|
| **Table** | Sortable, filterable columns | Task list, member list |
| **Kanban** | Column-by-status cards | Task board, sprint board |
| **Chart** | Bar/line/pie aggregation | Burndown, velocity |
| **Gantt** | Timeline bars by date | Project timeline, dependency |
| **Dashboard** | Composite of above | Project overview |

Views are **graph projections** — they select a subset of nodes/edges and render them.

### Event-Driven Execution

All mutations emit typed events (`CollabEvent`):
- `workspace.created`, `task.moved`, `goal.updated`, `review.submitted`
- Events are consumed by: WebSocket push, AI agent triggers, summary generation, view refresh

### Agent Collaboration Flows

```
Goal → Task Decomposition → Agent Assignment → Execution → Review → Merge → Summary
                                                         ↑                        ↓
                                                  Human Approval Gate      Learning Memory
```

## Architecture

```
quang-hub-workplace          quang-hub-meet
    │                             │
    └───────────┬─────────────────┘
                │
        quang-hub (orchestrator)
                │
        quang-web (Yew UI)
```

### quang-hub-workplace crate

| Module | Responsibility |
|--------|---------------|
| `graph.rs` | Core Node/Edge/HyperGraph types (serde, graph operations) |
| `workspace.rs` | WorkSpace entity, lifecycle, membership |
| `team.rs` | Team entity, member roster, roles |
| `project.rs` | Project entity, goals, timeline |
| `channel.rs` | Channel entity, message bus |
| `chat.rs` | Chat message, thread, reactions |
| `task.rs` | Task entity, state machine, dependencies |
| `goal.rs` | Goal entity, OKR-aligned, progress |
| `review.rs` | Review entity, approval chains |
| `summary.rs` | Summary entity, generation traits |
| `event.rs` | CollabEvent enum, event bus |
| `hub.rs` | WorkplaceHub — top-level orchestrator |
| `view.rs` | View trait, registry, projection engine |
| `views/` | Concrete view implementations (Table, Kanban, Chart, Gantt) |

### quang-hub-meet crate

| Module | Responsibility |
|--------|---------------|
| `room.rs` | MeetingRoom, lifecycle, scheduling |
| `participant.rs` | Participant, roles, permissions |
| `media.rs` | MediaStream, track, mute state |
| `recording.rs` | Recording, transcript, AI notes |
| `chat.rs` | In-call chat, handraise, polls |
| `event.rs` | MeetingEvent enum |
| `hub.rs` | MeetHub — meeting orchestrator |

## Implementation Order

### Phase 1 — Core Data Models (this PR)
1. `graph.rs` — Base graph types (NodeId, Node, Edge, HyperEdge, Graph)
2. `workspace.rs` — WorkSpace with membership
3. `team.rs` — Team with roles
4. `project.rs` — Project with lifecycle
5. `channel.rs` — Channel with messages
6. `chat.rs` — Chat messages and threads
7. `task.rs` — Full task state machine
8. `goal.rs` — Goal with progress tracking
9. `review.rs` — Review with approval chains
10. `summary.rs` — Summary structure
11. `event.rs` — Event system
12. `hub.rs` — Workplace hub orchestrator
13. `view.rs` — View trait + projection engine
14. `views/` — Table, Kanban, Chart, Gantt projections

### Phase 2 — Meeting Crates (this PR)
1. `room.rs` — Meeting room
2. `participant.rs` — Participants
3. `media.rs` — Media streams
4. `recording.rs` — Recordings
5. `chat.rs` — In-call chat
6. `event.rs` — Meeting events
7. `hub.rs` — Meet hub

### Phase 3 — Yew Web Views (future)
- Yew components for Table, Kanban, Chart, Gantt
- WebSocket event subscription
- Agent chat panel
- Task board drag-and-drop

### Phase 4 — Agent Integration (future)
- MCP tool definitions
- AI summary generation
- Agent task execution loop
- Smart task assignment

## Design Principles

1. **Graph-native** — All state lives in a typed graph. Views are projections.
2. **Agent-ready** — Every struct implements serialize/deserialize. Events are typed.
3. **Composable** — Small focused entities compose into complex workflows.
4. **Auditable** — All state transitions emit events with timestamps and actors.
5. **Flexible** — View registry allows agents to register new view types at runtime.
