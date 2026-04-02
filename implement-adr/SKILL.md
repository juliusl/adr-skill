---
name: implement-adr
description: >-
  Use this skill when the user needs to turn Architectural Decision Records
  (ADRs) into actionable implementation plans — including decomposing decisions
  into staged work, generating plan.md files with scoped tasks, estimating
  effort (small/medium/heavy), identifying gaps in decision records, or
  understanding how to go from "we decided X" to "X is built." Activate this
  skill when the user says things like "implement this ADR," "plan the work
  for this decision," "break this decision into tasks," "what's missing before
  I can build this," or "generate an implementation plan." Also use when the
  user asks about ADR-to-code traceability, task decomposition from decisions,
  or test and acceptance criteria for planned work. Do not use for authoring,
  reviewing, or managing ADRs themselves — use the author-adr skill for that.
  Do not use for general project management, sprint planning, or work that is
  not grounded in an architectural decision record.
license: CC-BY-4.0
metadata:
  version: "1.0"
---

# Implement ADR — From Decisions to Plans

You are an expert at turning Architectural Decision Records into structured,
actionable implementation plans. You bridge the gap between _what was decided_
and _how to build it_.

This skill consumes ADRs produced by the `author-adr` skill (or any
Nygard/MADR-formatted ADR) and generates a `plan.md` with staged tasks, test
criteria, cost estimates, and full traceability back to the source decisions.

## Agent Workflow

```
User request
├─ docs/adr/ exists? ────────────► List ADRs → user selects scope
├─ docs/adr/ missing? ──────────► Recommend: use author-adr skill first
│
├─ "Implement this ADR" ────────► Go to: Generating an Implementation Plan
├─ "What's missing?" ───────────► Go to: Gap Detection
├─ "Estimate effort" ───────────► Go to: Cost Estimation
├─ "Explain plan structure" ────► Go to: Plan Structure
└─ "Show plan template" ────────► Go to: Template Reference
```

### Step 0 — Locate ADRs

1. Check for `docs/adr/` directory in the repository.
2. **If missing:** Tell the user no ADRs were found. Recommend using the
   `author-adr` skill to create decision records before planning
   implementation. Stop here.
3. **If present:** List ADRs using:
   ```bash
   ls docs/adr/*.md
   ```
4. Ask the user which ADR(s) to implement. Accept one or more by number.

### Step 1 — Read and Analyze ADRs

1. Read the full content of each selected ADR.
2. Extract the structured sections:
   - **Status** — only proceed with `Accepted`, `Proposed`, or `Planned` ADRs.
     Warn if status is `Deprecated` or `Superseded`.
   - **Context** — understand the forces, constraints, and tensions.
   - **Decision** — identify the concrete commitments and design choices.
   - **Consequences** — note positive outcomes to preserve, negative outcomes
     to mitigate, and neutral observations.
3. If the ADR links to other ADRs, read those too and include them in scope.

### Step 2 — Gap Detection

Before generating a plan, evaluate whether the ADR(s) contain sufficient
detail. For each major component implied by the decision, check:

- Is there a clear architectural direction? (e.g., chosen technology, pattern)
- Are the key interfaces or boundaries defined?
- Are constraints and non-functional requirements stated?

**If gaps are found:**
1. List each gap clearly with a brief explanation of why it blocks planning.
2. Recommend that the user author an additional ADR for each gap using the
   `author-adr` skill.
3. Ask the user whether to proceed with a partial plan or wait for the
   missing ADRs.

Read the [Gap Detection Guide](references/planning-practices.md#gap-detection)
for detailed heuristics.

### Step 3 — Generate the Implementation Plan

Produce a `plan.md` file with the following structure. Use the
[plan template](assets/templates/plan-template.md) as a starting point.

#### 3a. Header and ADR References

Start the plan with:
- A title describing what is being implemented
- A list of all ADR(s) consumed, linked by file path
- The date the plan was generated
- The file location using the versioned naming convention
- The revision number (0 for initial plans)

```markdown
# Implementation Plan: [Title from ADR Decision]

**Source ADRs:**
- [ADR-0002: Add implement-adr companion skill](docs/adr/0002-add-implement-adr-companion-skill.md)

**Generated:** YYYY-MM-DD
**Location:** `docs/plans/0002.0.plan.md`
**Revision:** 0
```

#### 3b. Implementation Stages

Decompose the work into a tree of **stages** and **tasks**.

Read the
[Stage Decomposition Guide](references/planning-practices.md#stage-decomposition)
for principles on how to break work into stages.

**Rules:**
- Each stage represents a logical phase (e.g., "Data Layer", "API Surface").
- Stages are ordered so that earlier stages produce foundations for later ones.
- Each stage contains 2–5 tasks. If a stage has more than 5 tasks, split it.
- Avoid stages with only 1 task unless the task is genuinely standalone.

#### 3c. Task Detail

For each task, include:

| Field | Required | Description |
|-------|----------|-------------|
| **Title** | Yes | Short, descriptive name |
| **Cost Estimate** | Yes | `[small]`, `[medium]`, or `[heavy]` — see [Cost Estimation](references/cost-estimation.md) |
| **Description** | Yes | What needs to be done, scoped to this task only |
| **Implementation Notes** | If code-qualified | Code snippets, interface sketches, or pseudocode |
| **Test & Acceptance Criteria** | Yes | What tests to write, what "done" looks like — see [Testing Guidelines](references/testing-guidelines.md) |
| **Dependencies** | If any | Which other tasks must complete first |
| **ADR Reference** | Yes | Which ADR section(s) drive this task |

**Scoping rule:** A task plan must contain enough detail for an engineer or
agent to execute it without referring to other task plans. Cross-task
dependencies are declared but each task is self-contained.

#### 3d. ADR Status Finalize Stage

Every generated plan **must** end with a final stage that updates each source
ADR's status from `Planned` to `Accepted`. This ensures acceptance is an
explicit, traceable step tied to implementation completion.

```markdown
## Stage N: Finalize

### Task N.1: Update ADR status to Accepted    [small]

**Description:** Update the status of each source ADR from `Planned` to
`Accepted` to reflect that the decision has been fully implemented.

**Files to update:**
- `docs/adr/XXXX-<title>.md` — change `## Status` from `Planned` to `Accepted`

**Test & Acceptance Criteria:**
- [ ] Each source ADR status reads `Accepted`
- [ ] No other ADR content is modified

**Dependencies:** All prior stages

**ADR Reference:** ADR-0003, Decision §2
```

#### 3e. Summary Table

End the plan with a summary table:

```markdown
## Summary

| Stage | Task | Cost | Dependencies |
|-------|------|------|--------------|
| 1. Foundation | 1.1 Project structure | small | — |
| 1. Foundation | 1.2 Core data models | medium | 1.1 |
| ... | ... | ... | ... |

**Total estimated cost:** X small, Y medium, Z heavy
```

### Step 4 — Update ADR Statuses

After generating the plan, update each source ADR whose status is `Proposed`
to `Planned`. This signals that the decision has been analyzed, decomposed into
tasks, and is ready for implementation.

**Guard rails:**
- Only ADRs with status `Proposed` are transitioned to `Planned`.
- ADRs that are already `Accepted`, `Planned`, `Deprecated`, or `Superseded`
  are left unchanged.
- If an ADR has an unexpected status, warn the user and ask whether to proceed.

The status update is performed by editing the ADR file's `## Status` section
in-place, replacing `Proposed` with `Planned`.

### Step 5 — Review and Iterate

1. Present the generated plan to the user.
2. Ask if any stages or tasks need adjustment.
3. If the user identifies additional gaps, go back to Step 2.
4. Once the user approves, write the plan to `docs/plans/` using the versioned
   naming convention:
   - Create `docs/plans/` if it does not exist.
   - File name: `<adr-range>.0.plan.md` (initial plan).
   - Example: `docs/plans/0003-0004.0.plan.md`
5. If the user requests changes to an existing plan:
   a. Increment the revision number.
   b. Create a new file (e.g., `0003-0004.1.plan.md`).
   c. Add a revision header linking to the previous revision:
      ```markdown
      **Revision:** 1 (previous: [0003-0004.0.plan.md](docs/plans/0003-0004.0.plan.md))
      **Changes:** <summary of requested changes>
      ```
   d. Preserve the previous revision file unchanged.

## Plan Structure

An implementation plan follows a strict hierarchy:

```
plan.md
├── Header (title, source ADRs, date)
├── Stage 1: [Phase Name]
│   ├── Task 1.1: [Task Title]    [cost]
│   │   ├── Description
│   │   ├── Implementation Notes (optional)
│   │   ├── Test & Acceptance Criteria
│   │   └── ADR Reference
│   ├── Task 1.2: ...
│   └── ...
├── Stage 2: [Phase Name]
│   └── ...
└── Summary Table
```

**Stages** are sequential phases. **Tasks** within a stage may be parallel or
sequential (dependencies are declared explicitly).

## Cost Estimation

Each task is assigned a t-shirt size estimate:

| Size | Meaning | Typical Scope |
|------|---------|---------------|
| **small** | Well-defined, minimal ambiguity | ~1 agent turn, single-file change |
| **medium** | Moderate complexity, some design choices | ~2–3 agent turns, multi-file change |
| **heavy** | Significant complexity, may need research | ~4+ agent turns, cross-cutting change |

Read the [full cost estimation guide](references/cost-estimation.md) for
calibration examples and edge cases.

## Testing Guidelines

Every task must include appropriate test and acceptance criteria. The type of
testing depends on the code context:

| Code Context | Required Testing |
|--------------|-----------------|
| User input processing | Fuzz testing |
| Hot path / performance-critical | Benchmarking |
| Public APIs | Unit tests with edge cases |
| Internal modules | Unit tests at key boundaries |
| Integration points | Integration / contract tests |

**Overall target:** ~80% code coverage bar.

Read the [full testing guidelines](references/testing-guidelines.md) for
detailed requirements per category.

## Tooling

### Listing ADRs

```bash
# List all ADRs in the repository
make -f <skill-root>/Makefile list-adrs
```

### Showing the Plan Template

```bash
# Display the plan.md template
make -f <skill-root>/Makefile show-template
```

### Escape Hatch

If the Makefile targets are unavailable, list ADRs directly:

```bash
ls docs/adr/*.md
```

## Deep References

For detailed guidance beyond what is covered above, consult these references
on-demand:

- **[Planning Practices](references/planning-practices.md)** — Stage
  decomposition principles, gap detection heuristics, scoping rules.
- **[Testing Guidelines](references/testing-guidelines.md)** — Full testing
  taxonomy with examples for each code context category.
- **[Cost Estimation](references/cost-estimation.md)** — Calibration examples,
  edge cases, and guidance for mixed-size tasks.
- **[Asset Index](assets/index.md)** — Curated index of all available assets
  and templates.
