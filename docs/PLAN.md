# Quang — Project Plan

> **Quang** (Vietnamese: *Light*) is an AI-first, agent-native web framework built by i2c.
> One prompt creates a full-stack app. Agents build, humans review, the platform ships.

---

## Vision

Quang compresses the fragmented modern web stack into one unified, typed, graph-aware platform:

```
Prompt / Goal
    ↓
Agent builds App Graph
    ↓
QuangHub (server + events + AI tools)
    ↓
Data Core (FractalDB / HyperGraph / FluidArray via Kitchen)
    ↓
QuangWeb (HTMX-like UI + Signals + TSX)
    ↓
Deploy (i2c-forge / Cloudflare / AWS / GCP)
```

**Philosophy:** Lightweight like Hono, reactive like Signals, hypermedia-simple like HTMX, strongly typed like Rust, productive like TypeScript, AI-native by default. *Light first, power when needed.*

---

## Ecosystem Context

Quang is one pillar of the i2c ecosystem:

| Pillar | Role | MVP Substitute |
|--------|------|---------------|
| **Fractal** | Multi-resolution compression / navigation | (future) |
| **Fluid** | Git-compatible versioned data / file system | **git** |
| **Hyper / HyperGraph** | Graph-native knowledge & execution layer | **deep_causality** (Rust crate) |
| **HyperAI** | Unified framework for scaling AI & distributed computing | Rust workspace (`hypernn-core`, `quadtree-dist`, …) |
| **Quang** | AI-first web / app / cloud framework | This project |
| **Minh / MinhAI** | Efficient graph-native intelligence layer | Any LLM provider via API key |
| **RsTs / rsts aio** | TypeScript+Rust unified language & toolchain | **bun** (all-in-one TS runtime) |
| **Kitchen** | Query planner over FluidArray+HyperGraph+FractalDB | Embedded in Quang Data Core |
| **Fluid** (DevOps) | Cross-platform git-like collaboration | **git** |
| **shai** | AI-native IDE / agent control shell | Claude Code + mobile notifications |
| **i2c-forge** | Primary hosting platform | Cloudflare / AWS / GCP Firebase fallback |
| **fractalDB / Kitchen** | Adaptive multi-resolution database | Postgres + pgvector (MVP) |

---

## Core Concepts

### App as Graph
Every Quang app is a **typed living graph** — not a folder of files.  
One capability auto-generates: human UI + API route + agent tool + event + policy + test + docs.

```
Capability = UI + API + Tool + Event + Policy + Test
```

### Dual Interface
Every feature can be consumed two ways:
- **Human UI** — pages, forms, dashboards, tables, chat panels
- **Agent Interface** — typed tools, resources, schemas, MCP-compatible endpoints

### One Schema, Many Outputs
A single `@model` declaration generates: DB schema, GraphQL type, REST validation, AI tool schema, admin UI, typed client, test stubs.

---

## Architecture

```
┌──────────────────────────────────────────────────────────────┐
│                      Quang IDE (shai)                        │
│              AI agent control + graph dev view               │
└────────────────────────────┬─────────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────────┐
│                    RsTs / TypeScript (MVP: bun)               │
│        Rust safety + TS syntax + Wasm target (future)        │
└────────────────────────────┬─────────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────────┐
│                         QuangHub                             │
│    Hono-style routes │ Events │ Streams │ AI tools │ MCP     │
└────────────────────────────┬─────────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────────┐
│                      Quang Data Core                         │
│  FractalDB │ HyperGraph (deep_causality) │ FluidArray        │
│  Kitchen (query planner) │ External DB adapters              │
└────────────────────────────┬─────────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────────┐
│                    HyperAI Compute Layer                     │
│  Distributed Tasks │ Actor mesh │ GPU-poor node pool         │
│  hypernn-core │ quadtree-dist │ hypergs │ ghost-frame        │
│  Train │ Serve │ Tune │ Data pipeline │ WebGPU shaders       │
└────────────────────────────┬─────────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────────┐
│                         QuangWeb                             │
│  HTMX-like q-* │ Signals │ TSX │ GraphQL │ Wasm islands      │
└────────────────────────────┬─────────────────────────────────┘
                             │
┌────────────────────────────▼─────────────────────────────────┐
│                    Deployment Targets                        │
│  i2c-forge │ Cloudflare │ AWS │ GCP Firebase │ Local         │
└──────────────────────────────────────────────────────────────┘
```

### QuangHub Runtime Layers

| Layer | Contents |
|-------|----------|
| 0 — Core | Router, request/response, middleware, context, typed errors |
| 1 — Contracts | Schemas, validation, generated clients, GraphQL bridge |
| 2 — Events | Pub/sub, job queue, event sourcing, SSE, WebSocket streams |
| 3 — Data | FractalDB, HyperGraph, FluidArray, external DB adapters |
| 4 — AI | Tool registry, agent runtime, memory, evals, traces |
| 5 — Deploy | Local, edge, serverless, container, single binary |

---

## MVP Scope

> Start with TypeScript + bun. Prove the loop: *prompt → app → preview → approve → deploy.*

### MVP 1 — QuangHub Lite (TypeScript / bun)
- [ ] Hono-style router with typed route contracts
- [ ] Middleware pipeline
- [ ] HTML / TSX server-side rendering
- [ ] SSE and WebSocket helpers
- [ ] Auto-generated typed client
- [ ] File-based routing option
- Runs on: **bun**, Node, Deno, Cloudflare Workers

### MVP 2 — QuangWeb Lite
- [ ] `q-get`, `q-post`, `q-target`, `q-swap` HTMX-like attributes
- [ ] Signal-based islands for client interactivity
- [ ] Server-rendered TSX pages
- [ ] Small client runtime (< 15 kB)

### MVP 3 — App Graph & Agent Builder
- [ ] `app.graph.json` typed format
- [ ] TypeScript SDK for graph construction
- [ ] Capability system (`capability` = UI + API + Tool + Event)
- [ ] Component registry (Table, Form, Card, Chart, ChatBox, ApprovalPanel…)
- [ ] Page generator from semantic DSL
- [ ] Graph diff + validation engine
- [ ] Preview server
- [ ] Agent tool registry (`@tool` decorator, JSON Schema export)

### MVP 4 — Data Core (Postgres + pgvector bridge)
- [ ] FluidArray — in-memory hot/warm/cold adaptive cache + append log
- [ ] HyperGraph via **deep_causality** — entity/relationship/hyperedge store
- [ ] FractalDB — multi-resolution block summaries (Postgres-backed MVP)
- [ ] Kitchen — query planner routing between the three layers
- [ ] GraphQL endpoint auto-generated from `@model` declarations
- [ ] Event-driven re-indexing on data changes

### MVP 5 — AI Tool Layer
- [ ] `@tool` / `@requires` / `@redact` decorators
- [ ] JSON Schema + OpenAI / Anthropic tool schema auto-export
- [ ] Permission policy engine
- [ ] Trace logging (FluidArray-backed)
- [ ] Tool replay / audit
- [ ] MinhAI / Gemini / OpenAI / Claude integration via API key (provider-agnostic)

### MVP 6 — shai Agent Loop (Demo)
- [ ] Prompt input: `"Create a landing page"`
- [ ] Agent produces `app.graph.json` spec
- [ ] QuangHub compiles → routes + pages + tools
- [ ] Preview server opens live URL
- [ ] **shai-agent-mobile** pushes notification to user for review/approval
- [ ] User approves → deploy to i2c-forge or Cloudflare

---

## Short-Term Demo Goal

> **"From prompt to live landing page, with mobile approval notification."**

**Flow:**
1. User types: `"Create a landing page for Quang"`
2. shai agent plans app graph (models, pages, capabilities)
3. QuangHub generates server routes + QuangWeb HTML
4. Preview URL generated
5. shai-agent-mobile notifies user on phone: *"Preview ready — approve to deploy?"*
6. User approves → deploys to Cloudflare Pages / i2c-forge

**Minimum pieces needed:**
- QuangHub Lite (bun)
- QuangWeb q-* runtime
- App Graph JSON spec + TypeScript SDK
- One AI provider wired (Claude via Anthropic API)
- shai-agent-mobile push notification (MVP: simple webhook → mobile app)
- Cloudflare Pages deploy step

---

## Technology Decisions (MVP)

| Concern | Long-Term Vision | MVP Choice |
|---------|-----------------|------------|
| Language | RsTs (Rust+TS+Wasm) | TypeScript + bun |
| Toolchain | rsts aio | bun |
| Graph layer | HyperGraph (custom) | deep_causality (Rust crate) |
| Distributed compute | HyperAI (Ray-like, Rust-native) | HyperAI Rust workspace (early) |
| Database | FractalDB (custom) | Postgres + pgvector |
| Version control | Fluid | git |
| AI provider | MinhAI | Any via API key (Claude / Gemini / OpenAI / Qwen) |
| IDE / DevX | shai full | Claude Code + shai-agent-mobile notifications |
| Hosting | i2c-forge | Cloudflare / AWS / GCP Firebase |
| Package manager | quang registry | npm + cargo |

---

## Phased Roadmap

### Phase 1 — 0–3 months (Foundation)
- QuangHub TypeScript package (bun-first)
- QuangWeb small client runtime
- Typed route contracts + server TSX rendering
- HTMX-like `q-*` attribute runtime
- SSE / WebSocket support
- Basic generated client
- App Graph JSON spec v0.1
- Demo: prompt → landing page → mobile approval

### Phase 2 — 3–6 months (Intelligence)
- AI tool registry + `@tool` decorator pipeline
- Schema compiler (model → GraphQL + DB + AI schema)
- FluidArray in-memory cache layer
- Event bus (pub/sub + job queue)
- GraphQL bridge
- Admin panel auto-generation
- Kitchen query planner v1

### Phase 3 — 6–12 months (Graph Core)
- deep_causality HyperGraph integration deepened
- FractalDB embedded storage (multi-resolution blocks)
- Wasm plugin runtime
- RsTs syntax parser prototype
- Quang IR exploration
- HyperGraph memory for agents
- Rust interop prototype

### Phase 4 — 12–24 months (Platform)
- Native RsTs compiler
- Edge runtime (Cloudflare Workers native)
- Distributed FractalDB
- AI graph debugger
- Visual app graph IDE (shai full)
- Single-binary deployment
- i2c-forge as primary hosting

---

## Key Design Rules

1. **Light first.** Small core, optional modules. No forced ORM, GraphQL, AI provider, or cloud.
2. **Capability over code.** Agents generate capabilities, not raw files. Capabilities compile into everything.
3. **Dual interface always.** Every feature serves humans (UI) and agents (tools/schemas) from one definition.
4. **App Graph is truth.** Natural language is input; the typed graph is the canonical representation.
5. **Permissioned by default.** AI tools are typed, permissioned, audited, and replayable — never raw API access.
6. **Migration-friendly.** Git-compatible, npm-compatible, GraphQL-compatible. Adoption doesn't require replacing everything.

---

## Positioning

> **Quang is an AI-native, Wasm-first, hypermedia web framework for building typed, graph-aware applications with minimal code.**

Or more poetically:

> **Quang turns your application into light: typed, fast, visible, and intelligent.**

---

## References

- Source docs: [`docs/Quang Web Framework.md`](../docs/Quang%20Web%20Framework.md)
- Ecosystem pitch: [`docs/Ecosystem Quang Fractal Fluid Hyper.md`](../docs/Ecosystem%20Quang%20Fractal%20Fluid%20Hyper.md)
- HyperAI plan: [`G:/i2c/PROJECTS/HyperAI/docs/PLAN.md`](../../HyperAI/docs/PLAN.md)
- deep_causality: https://github.com/deepcausality-rs/deep_causality
- Ray (HyperAI reference): https://github.com/ray-project/ray
- Hono (routing reference): https://hono.dev
- bun (rsts aio substitute): https://bun.sh
