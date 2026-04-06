# 2. Add implement-adr companion skill

Date: 2026-04-01

## Status

Proposed

## Context

The `author-adr` skill helps teams create, review, and manage Architectural
Decision Records. However, once a decision is recorded, there is no structured
path from decision to implementation. Engineers must manually interpret ADRs,
decompose the work, and plan tasks — a process that is error-prone and
inconsistent. The gap between "we decided X" and "X is built" is where
architectural drift, scope creep, and forgotten requirements occur.

A companion skill that consumes accepted ADRs and produces actionable
implementation plans would close this loop. It would operate downstream of
`author-adr` and leverage the structured format of decision records (context,
decision, consequences) to generate scoped, reviewable plans.

Key forces:

- **ADRs capture _what_ and _why_, but not _how_** — implementation details are
  intentionally out of scope for a decision record.
- **Planning quality varies** — without structured decomposition, tasks are
  often too large, under-specified, or missing test/acceptance criteria.
- **Agent-assisted coding benefits from structured plans** — a well-scoped
  `plan.md` with clear task boundaries enables more effective agent execution.
- **ADRs can be linked** — the Nygard format supports inter-ADR links, allowing
  implementation plans to trace back to the decisions they realize.
- **Testing expectations differ by code context** — user-input processing needs
  fuzz testing, hot paths need benchmarks, public APIs need unit tests, and an
  overall ~80% code coverage bar should be maintained.

## Decision

We will add a companion skill called `implement-adr` to this repository. This
skill will consume ADRs produced by `author-adr` and generate a structured
`plan.md` implementation plan. The skill will follow these design principles:

### 1. Staged Implementation Tree

The plan will organize work as a tree of **implementation stages**, where each
stage is broken down into discrete **tasks**. Stages represent logical phases
(e.g., "Data Layer", "API Surface", "Integration Tests") and tasks represent
individual units of work within a stage.

```
plan.md
├── Stage 1: Foundation
│   ├── Task 1.1: Set up project structure        [small]
│   ├── Task 1.2: Define core data models         [medium]
│   └── Task 1.3: Implement storage layer          [medium]
├── Stage 2: Business Logic
│   ├── Task 2.1: Implement validation rules       [small]
│   └── Task 2.2: Build processing pipeline        [heavy]
└── Stage 3: API & Integration
    ├── Task 3.1: Define public API surface         [medium]
    └── Task 3.2: End-to-end integration tests      [medium]
```

### 2. Detailed Task Plans

Each task will include a detailed implementation plan scoped strictly to that
task. If the executing agent is qualified to write code, the task plan may
include code snippets, interface definitions, or implementation sketches.
Regardless, the plan must contain enough detail for an engineer or agent to
execute the task without referring to other tasks or external context beyond the
linked ADRs.

### 3. Test and Acceptance Instructions

Each task will call out relevant testing and acceptance criteria appropriate to
the code context. The following guidelines apply:

- **User input processing** — include fuzz testing requirements
- **Hot path / performance-critical code** — include benchmarking tasks
- **Public APIs** — require unit tests with edge case coverage
- **Overall target** — maintain ~80% code coverage bar

The specific testing taxonomy and detailed guidance for each category will be
expanded in a follow-up ADR: *"Tasks should include test and acceptance
instructions"* (to be authored separately and linked to this record).

### 4. ADR Linkage

The generated `plan.md` will reference all ADRs consumed during planning. Each
referenced ADR will be linked using the standard Nygard link format so that
traceability is maintained from decision to implementation. If the plan is
generated from multiple ADRs, all are cited.

### 5. Gap Detection and ADR Recommendations

If the skill determines that existing decision records lack sufficient detail to
complete a stage or task plan (e.g., no decision on authentication strategy when
planning an auth module), it will recommend that the user author an additional
ADR to cover the gap before proceeding. The skill will not fabricate
implementation details for undecided architectural concerns.

### 6. Estimated Usage Cost

Each task will include an estimated usage cost using a simple t-shirt sizing
scheme:

| Size       | Description                                                   |
|------------|---------------------------------------------------------------|
| **small**  | Straightforward, well-defined, minimal ambiguity (~1 agent turn) |
| **medium** | Moderate complexity, some design choices within the task (~2-3 agent turns) |
| **heavy**  | Significant complexity, may require iteration or research (~4+ agent turns) |

These estimates help the user anticipate effort and prioritize work, and allow
billing-conscious users to plan agent usage accordingly.

## Consequences

**Positive:**

- Closes the gap between architectural decisions and implementation, reducing
  architectural drift.
- Produces consistent, reviewable implementation plans with clear task
  boundaries.
- Enables agent-assisted implementation by providing well-scoped task
  definitions that agents can execute independently.
- Maintains traceability from code back to the decisions that motivated it.
- Surfaces gaps in decision records early, prompting additional ADRs before
  implementation begins rather than discovering missing decisions mid-build.
- Cost estimates give users visibility into expected effort before committing.

**Negative / Risks:**

- Adds a second skill to maintain alongside `author-adr`; the two must stay
  compatible as ADR formats evolve.
- Plan quality depends on ADR quality — poorly written ADRs will produce
  under-specified plans (mitigated by the gap detection mechanism).
- The t-shirt sizing for cost is inherently imprecise and may set inaccurate
  expectations (mitigated by using coarse categories and documenting them as
  estimates).
- Testing guidance (fuzz, benchmark, unit test expectations) may not suit all
  project contexts — the follow-up ADR on test instructions should address
  configurability.

**Neutral:**

- The `implement-adr` skill does not replace engineering judgment; it structures
  the planning phase but the user retains full control over plan review and
  execution.
- A follow-up ADR for detailed test and acceptance instruction taxonomy is
  expected and should be linked to this record once authored.

---

## Comments

<!-- No review cycle on record. This ADR predates the formal review process. -->
