# Planning Practices

Detailed guidance for decomposing ADRs into staged implementation plans.

**All sections in this reference must be consulted during plan generation. If a section is skipped, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

## Procedure

| ID | Description |
|----|-------------|
| PP-1 | Stage Decomposition — group tasks into ordered execution stages |
| PP-1a | Principles — stage ordering and sizing guidance |
| PP-1b | Common Stage Patterns — layered, vertical slice, setup-then-feature |
| PP-1c | Stage Naming — naming conventions |
| PP-2 | Task Scoping — define self-contained, estimable tasks |
| PP-2a | Self-Containment Checklist — verify task independence |
| PP-2b | Splitting Oversized Tasks — when and how to split |
| PP-3 | Gap Detection — identify missing decisions or information |
| PP-3a | Heuristic Checklist — common gap patterns |
| PP-3b | Gap Report Format — how to document gaps |
| PP-3c | Proceeding with Partial Plans — when to continue despite gaps |
| PP-4 | ADR Linkage — connect plan tasks to source ADR sections |
| PP-4a | Referencing ADRs in Plans — plan-level ADR references |
| PP-4b | Per-Task ADR References — task-level ADR section references |

```
PP-1 — Stage Decomposition
  ↓
PP-2 — Task Scoping
  ↓
PP-3 — Gap Detection
  ↓
PP-4 — ADR Linkage
```

## PP-1: Stage Decomposition

Stages represent logical phases of implementation. They impose order on work and make progress visible.

### PP-1a: Principles

1. **Foundation first** — Start with data models, configuration, and project scaffolding. Later stages depend on these.
2. **Inside out** — Build core logic before external interfaces. APIs, CLI commands, and UIs come after the engine works.
3. **Test alongside** — Don't defer all testing to a final stage. Each stage should leave the codebase in a testable state.
4. **2–5 tasks per stage** — Fewer than 2 means the stage is too narrow; more than 5 means it should be split.
5. **Minimize cross-stage dependencies** — Tasks within a stage may depend on each other, but stages should depend only on prior stages, not on partial completion of parallel stages.

### PP-1b: Common Stage Patterns

| Pattern | Stage Sequence | When to Use |
|---------|---------------|-------------|
| **Layered** | Data → Logic → API → UI | Traditional backend services |
| **Vertical Slice** | Feature A → Feature B → Feature C | Feature-driven delivery |
| **Infrastructure-first** | Infra → Core → Integration → Polish | Greenfield projects |
| **Migration** | Scaffold → Dual-write → Cutover → Cleanup | System migrations |

### PP-1c: Stage Naming

Use short, descriptive names that communicate the phase:

- ✅ "Data Layer", "Authentication", "API Surface", "Observability"
- ❌ "Phase 1", "Part A", "Misc", "Other Stuff"

## PP-2: Task Scoping

Each task must be independently executable. An engineer or agent should be able to pick up a task and complete it without reading other task plans.

### PP-2a: Self-Containment Checklist

- [ ] The task description explains _what_ to do without referencing other tasks
- [ ] Any interfaces or contracts from other tasks are restated in this task
- [ ] File paths or module names are explicitly mentioned
- [ ] The expected output or artifact is clearly described
- [ ] Test criteria are specific to this task's scope

### PP-2b: Splitting Oversized Tasks

If a task feels too large (rule of thumb: more than ~200 lines of code or touches more than 3 files), consider:

1. **Extract setup** — Move scaffolding, config, or boilerplate into a separate `[small]` task.
2. **Split by concern** — Separate validation, business logic, and persistence.
3. **Split by interface** — One task per public method or endpoint.

## PP-3: Gap Detection

Before generating tasks, check for missing decisions.

### PP-3a: Heuristic Checklist

For each major component implied by the ADR, verify:

| Question | If Missing |
|----------|-----------|
| Is the technology/framework chosen? | Need ADR on tech selection |
| Are data models or schemas defined? | Need ADR on data modeling |
| Is the authentication/authorization strategy decided? | Need ADR on auth |
| Are API contracts or interfaces specified? | Need ADR on API design |
| Is the deployment strategy clear? | Need ADR on deployment |
| Are non-functional requirements quantified? | Need ADR on NFRs |
| Is error handling / resilience approach decided? | Need ADR on resilience |
| Are there integration points with undefined contracts? | Need ADR on integration |

### PP-3b: Gap Report Format

When reporting gaps, use this structure:

```markdown
## ⚠️ Decision Gaps Detected

The following areas lack sufficient architectural decisions to generate
complete task plans:

1. **[Gap Title]** — [Brief explanation of what's missing and why it blocks
   planning]. _Recommended: Author an ADR on [topic]._

2. **[Gap Title]** — ...
```

### PP-3c: Proceeding with Partial Plans

If the user chooses to proceed despite gaps:

- Mark affected tasks with `⚠️ PARTIAL` in the title
- Add a note explaining which decision is missing
- Do not fabricate architectural choices — leave those sections as TODOs

## PP-4: ADR Linkage

### PP-4a: Referencing ADRs in Plans

Every plan must cite its source ADRs in the header. Use relative file paths:

```markdown
**Source ADRs:**
- [ADR-0002: Add implement-adr companion skill](docs/adr/0002-add-implement-adr-companion-skill.md)
- [ADR-0005: Use PostgreSQL for persistence](docs/adr/0005-use-postgresql-for-persistence.md)
```

### PP-4b: Per-Task ADR References

Each task should cite the specific ADR section that drives it:

```markdown
**ADR Reference:** ADR-0002, Decision §1 (Staged Implementation Tree)
```

This maintains traceability from implementation back to the decision that motivated it.

## Acceptance Criteria Categories

Every acceptance criterion that mentions manual verification must be classified into one of two categories. This prevents agents from treating unimplemented features as verification-only items.

| Category | Marker | Meaning | Agent Action |
|----------|--------|---------|--------------|
| Implement + verify manually | `[implement, verify-manual]` | The feature does not exist yet. Build it, then verify by hand. | Write the code, then note manual verification is needed. |
| Verify manually only | `[verify-manual]` | The feature is already implemented. It cannot be tested automatically. | Skip implementation — only note that manual verification is needed. |

**Why this matters:** A criterion like "Ctrl+C cancels the current line" looks like a verification item, but if no signal handling code exists, it's actually an implementation gap. Without the category marker, agents treat it as "already done, just needs testing" and move on.

**Rule:** When writing acceptance criteria, default to `[implement, verify-manual]`. Only use `[verify-manual]` when the code already exists and only needs human confirmation (e.g., visual layout checks, UX feel).

