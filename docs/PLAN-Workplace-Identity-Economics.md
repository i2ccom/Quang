# Quang Workplace Identity & Economics — Actor Model Design

## The Core Insight

**Humans and Agents are both Actors.** Many traditional ERP/HR concepts apply to both, with different concrete realizations. Instead of duplicating, we build a shared Actor abstraction with specialized extensions.

```
Actor (abstract)
  ├── Human
  │   ├── Legal identity (name, gov ID, tax info)
  │   ├── Employment (contract, department, rank, salary)
  │   ├── Benefits (insurance, leave, retirement)
  │   └── HR records (emergency contact, documents)
  │
  └── Agent
      ├── Capability manifest (skills, tools, models)
      ├── Resource allocation (compute, tokens, rate limits)
      ├── Trust/reputation (accuracy, reliability, safety)
      └── Owner relationship (creator, governed-by)
```

## Shared Concepts (Actor trait / base)

| Concept | Human Expression | Agent Expression |
|---------|-----------------|------------------|
| **Identity** | Name, email, gov ID, legal entity | Agent ID, capability hash, provider, version |
| **Profile** | Avatar, bio, timezone, contact | Avatar, description, model info, languages |
| **Organization** | Department, division, cost center | Capability group, cluster, agent team |
| **Rank / Grade** | Job title, level (L4, Director) | Capability tier (T1, T2), trust level |
| **Skill** | Skill name, certs, years exp | Tool skill, domain expertise, model capability |
| **Contract** | Employment agreement (terms, hours) | Service agreement (SLA, scope, limitations) |
| **Plan** | Performance goals, development plan | Work plan, task queue, goal alignment |
| **Budget** | Expense budget, project allocation | Compute budget, token quota, API credits |
| **Compensation** | Salary, bonus, equity | Resource allocation, priority access |
| **Incentive** | Promotion, bonus, recognition | Reputation score, capability unlock, priority |
| **WorkLog** | Time tracking, billable hours | Activity log, token usage, execution trace |
| **Audit** | Decision log, approvals, evidence | Action log, tool calls, reasoning trace |

## Data Model

### Actor — unified base

```rust
pub struct ActorProfile {
    pub actor_id: ActorId,
    pub display_name: String,
    pub email: Option<String>,
    pub avatar_url: Option<String>,
    pub bio: String,
    pub organization_id: Option<NodeId>,
    pub rank_id: Option<NodeId>,
    pub created_at: Timestamp,
}
```

### Skill — shared by both

```rust
pub struct Skill {
    pub id: String,
    pub name: String,
    pub category: String,
    pub proficiency: f64,          // 0.0 - 1.0
    pub certifications: Vec<String>,
    pub years_experience: Option<f64>,
    // Agent-specific: tool/API this skill maps to
    pub tool_id: Option<String>,
}
```

### WorkLog — shared activity tracking

```rust
pub struct WorkLogEntry {
    pub id: String,
    pub actor: ActorId,
    pub activity_type: String,     // "task_work", "meeting", "review", "tool_call"
    pub description: String,
    pub start_time: Timestamp,
    pub end_time: Option<Timestamp>,
    pub duration_minutes: Option<f64>,
    pub billable: bool,
    pub metadata: serde_json::Value,
}
```

### AuditEntry — shared audit trail

```rust
pub struct AuditEntry {
    pub id: String,
    pub actor: ActorId,
    pub action: String,
    pub target_type: String,       // "task", "project", "budget", "contract"
    pub target_id: NodeId,
    pub decision: Option<String>,
    pub rationale: Option<String>,
    pub evidence: Vec<String>,
    pub timestamp: Timestamp,
}
```

### Compensation — abstract for both

```rust
pub enum CompensationPeriod { Hourly, Monthly, Yearly, PerTask, PerToken }

pub struct CompensationPlan {
    pub actor: ActorId,
    pub base_amount: f64,
    pub currency: String,
    pub period: CompensationPeriod,
    pub effective_from: Timestamp,
    pub metadata: serde_json::Value,
}
```

### Budget — shared resource allocation

```rust
pub struct Budget {
    pub id: NodeId,
    pub owner: ActorId,
    pub total_amount: f64,
    pub spent_amount: f64,
    pub currency: String,
    pub period_start: Timestamp,
    pub period_end: Timestamp,
    pub budget_type: String,       // "project", "department", "compute", "tokens"
}
```

### Human-specific extensions

```rust
pub struct HumanIdentity {
    pub actor_id: ActorId,
    pub legal_name: String,
    pub government_id: Option<String>,
    pub tax_id: Option<String>,
    pub nationality: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub emergency_contact: Option<String>,
}

pub struct EmploymentContract {
    pub id: NodeId,
    pub human_id: ActorId,
    pub contract_type: String,     // "full_time", "contractor", "intern"
    pub department: String,
    pub title: String,
    pub manager: Option<ActorId>,
    pub start_date: NaiveDate,
    pub end_date: Option<NaiveDate>,
    pub working_hours_per_week: f64,
    pub probation_end: Option<NaiveDate>,
}

pub struct Salary {
    pub id: NodeId,
    pub actor_id: ActorId,
    pub base_salary: f64,
    pub currency: String,
    pub period: CompensationPeriod,
    pub last_review_date: Option<Timestamp>,
    pub next_review_date: Option<Timestamp>,
}
```

### Agent-specific extensions

```rust
pub struct AgentCapability {
    pub actor_id: ActorId,
    pub model_name: String,
    pub model_provider: String,
    pub model_version: String,
    pub tools: Vec<String>,
    pub max_tokens_per_request: u64,
    pub max_context_length: u64,
    pub supported_languages: Vec<String>,
    pub trust_score: f64,
    pub reliability_score: f64,
    pub owner: ActorId,
}

pub struct AgentResourceAllocation {
    pub actor_id: ActorId,
    pub compute_priority: u32,
    pub api_rate_limit: u32,       // requests per minute
    pub token_budget_daily: u64,
    pub token_used_today: u64,
    pub concurrent_task_limit: u32,
}
```

## Implementation Order

### Phase 1 (this PR)
1. `actor.rs` — `ActorProfile`, `Organization`, `Rank`
2. `skill.rs` — `Skill`, `SkillCategory`
3. `worklog.rs` — `WorkLogEntry`, time tracking
4. `audit.rs` — `AuditEntry`, evidence chain
5. `compensation.rs` — `CompensationPlan`, `Budget`, `Incentive`
6. `contract.rs` — `Contract` (shared), `EmploymentContract` (human)
7. `human.rs` — `HumanIdentity` (legal, tax, benefits)
8. `agent.rs` — `AgentCapability`, `AgentResourceAllocation`

### Phase 2 (future)
- Payroll integration (payslips, tax reporting)
- Leave/absence management
- Performance review cycles
- Agent reputation marketplace
- Token economy / micro-transactions
