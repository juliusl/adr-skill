---
name: implement-adr
description: >-
  Use this skill when the user wants to implement an ADR — turning an
  architectural decision into code by decomposing it into staged work,
  generating plan.md files with scoped tasks, estimating effort
  (small/medium/heavy), identifying gaps, or understanding how to go from
  "we decided X" to "X is built." Activate when the user says things like
  "implement ADR 0013," "implement this ADR," "let's implement," "start
  implementing," "execute this decision," "plan the work for this decision,"
  "break this decision into tasks," "what's missing before I can build this,"
  or "generate an implementation plan." Also activate when the user references
  an ADR by number and wants to build, execute, or plan work for it. Also use
  for ADR-to-code traceability, task decomposition, or test and acceptance
  criteria for planned work. Do not use for authoring, reviewing, or managing
  ADRs — use author-adr for that. Do not use for general project management
  or sprint planning.
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

## Configuration

This skill reads user-scoped preferences from a TOML configuration file at
`~/.config/adr-skills/preferences.toml` (per ADR-0011 and ADR-0012).

**Path resolution:**
1. If `$XDG_CONFIG_HOME` is set, use `$XDG_CONFIG_HOME/adr-skills/preferences.toml`.
2. Otherwise, use `$HOME/.config/adr-skills/preferences.toml`.

**Graceful degradation:** If the file or directory does not exist, use built-in
defaults. Never fail because config is absent.

**Create on first write:** When persisting a preference, create the directory
with `mkdir -p` before writing. Never assume it already exists.

## Agent Workflow

```
User request
├─ docs/adr/ exists? ────────────► List ADRs → check config → user selects scope
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
4. **Check for saved preferences:** Read the config file (see
   [Configuration](#configuration)) and look for `[implement].participation`
   and `[implement].auto_commit`.
   - If set, store for use in Step 5 (applied silently, skip corresponding
     prompts).
   - If absent, proceed silently — preferences will be established
     interactively in Step 5 and optionally saved.
5. Ask the user which ADR(s) to implement. Accept one or more by number.

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

### Step 5 — Participation Check

After updating ADR statuses, determine how the user wants to participate during
implementation.

1. **Check for existing preference:** If `[implement].participation` was loaded
   from the config file in Step 0, apply it and inform the user:
   > Using participation mode: **Guided** (from preferences.toml).
   > Say "change mode" at any time to switch.
   Skip to the auto-commit check (item 5).

2. **If no preference exists**, prompt:
   > How much participation would you like during implementation?
   > 1. **Full control** — I'll review each stage and select what to start
   > 2. **Guided** — Summarize the plan, let me pick stages or request changes
   > 3. **Autonomous** — Execute the plan, check in at major milestones
   > 4. **Weighted** — Automatically adjust based on task complexity

3. **Apply the chosen mode** for the remainder of the session. See the
   behavior table below.

4. **Offer to persist** — ask the user whether to save their choice to the
   config file:
   > Save this as your default participation mode?
   If yes, write `participation = "<mode>"` under the `[implement]` table in
   `preferences.toml` (creating the file and directory with `mkdir -p` if
   needed).

5. **Auto-commit preference:** After determining participation mode, check
   whether to enable automatic git commits at task boundaries.
   - **If `[implement].auto_commit` was set** in the config file, apply
     silently:
     > Auto-commit on task completion: **enabled** (from preferences.toml).
   - **If not set**, prompt:
     > Would you like to create a git commit each time a task completes?
     > 1. **Yes** — Commit after each task's acceptance criteria are all satisfied
     > 2. **No** (default) — I'll manage commits myself
   - **Offer to persist** — same pattern as participation mode.

#### Participation Mode Behaviors

| Mode | Granularity | Behavior |
|------|-------------|----------|
| **Full control** | Per-stage | Present each stage individually. Wait for explicit approval before starting. After each stage, ask which to proceed with next. |
| **Guided** (default) | Plan-level | Summarize the full plan. Ask if changes are needed or which stages to start. Proceed with approved stages, reporting at stage boundaries. |
| **Autonomous** | Milestone-only | Execute all stages in order. Report at stage boundaries but do not wait for approval. Pause only on errors or ambiguity. |
| **Weighted** | Per-task, cost-driven | Evaluate each task by cost estimate. `[small]` → autonomous. `[medium]`/`[heavy]` → sentinel (pause for approval). Report at stage boundaries. |

**Mode switching:** The user may change modes at any time during a session.
In-session changes override the loaded preference but do not update the config
file.

#### Persisting Preferences

When the user opts to save a preference, resolve the config path (see
[Configuration](#configuration)), create the directory with `mkdir -p` if
needed, read the existing `preferences.toml` to preserve other keys, and write
the relevant key under `[implement]`. Confirm:
> Saved to ~/.config/adr-skills/preferences.toml

#### Weighted Mode — Per-Task Evaluation

In Weighted mode, the skill evaluates each task independently as it proceeds
through a stage, using the task's cost estimate:

| Task Cost | Behavior | Agent Action |
|-----------|----------|-------------|
| `[small]` | Autonomous | Execute immediately, no approval needed |
| `[medium]` | Sentinel | Pause, summarize what's next, wait for approval |
| `[heavy]` | Sentinel | Pause, summarize what's next, wait for approval |

**Example:**

```
Stage 2: Authentication
  Task 2.1: Add auth config constants        [small]  → autonomous ✓
  Task 2.2: Create user model from schema    [small]  → autonomous ✓
  Task 2.3: Implement JWT middleware          [medium] → sentinel ⏸
  Task 2.4: Add rate limiting to auth routes  [small]  → autonomous ✓
  Task 2.5: Write integration tests           [medium] → sentinel ⏸
```

After the user approves a sentinel task, the skill continues with subsequent
tasks. At stage boundaries, it reports what was completed.

#### Auto-Commit on Task Completion

The skill supports an optional behavior: **create a git commit each time a
task's acceptance criteria are all satisfied**. Opt-in, disabled by default.

**When it triggers:** After all `- [ ]` checkboxes in a task's Test &
Acceptance Criteria are marked `- [x]` (per the Task Execution Protocol).

**Commit steps:**

1. **Stage the plan file** — `git add <plan-file>`.
2. **Stage implementation files** — `git add` any files the agent created,
   edited, or deleted during the task's execution.
3. **Create a commit** with a conventional message:

   ```
   <type>(<scope>): <brief summary>

   Plan: <plan-file-path>
   Task: <N.M> <task title> [<cost>]
   ADR: <adr-reference>
   ```

   Use the canonical [Conventional Commits](https://www.conventionalcommits.org/)
   type and scope that best describes the work (e.g., `feat`, `fix`, `refactor`,
   `docs`, `test`, `chore`). The summary should be a brief sentence describing
   what was done.

4. **Do not push** — commits are local only. The user decides when to push.

**Guard rails:**

- **Unrelated changes:** If the working tree has unstaged changes that the
  agent did not create or modify during this task, warn the user and ask
  whether to include them or commit only task-related files.
  **Autonomous mode fallback:** do not prompt — commit only task-related files
  and log a warning noting the skipped unrelated changes.
- **Merge conflicts / dirty state:** If a task modifies files that have merge
  conflicts or are in a dirty state from prior work, pause and ask the user to
  resolve before committing.
- **Pre-commit hook failures:** If `git commit` fails due to pre-commit hooks
  (linters, formatters, security scanners), **pause and ask the user regardless
  of participation mode**. Hook failures are unexpected and may indicate code
  quality issues. Report the hook's error output and let the user decide
  whether to fix the issue, skip the commit, or retry with `--no-verify`.
- **Git state warning:** Auto-commit modifies the user's staging area and
  commit history. Users who carefully manage their index (e.g., `git add -p`,
  curated staging) should leave this feature disabled.

**Interaction with participation modes:**

| Mode | Auto-commit enabled | Behavior |
|------|-------------------|----------|
| **Full control** | Yes | Commit after each approved-and-completed task |
| **Guided** | Yes | Commit after each completed task within approved stages |
| **Autonomous** | Yes | Commit after each completed task; commit only task-related files without prompting when unrelated changes exist; still pause on hook failures |
| **Weighted** | Yes | Commit after each completed task (autonomous or sentinel); same autonomous fallback for `[small]` tasks |

### Step 6 — Review and Iterate

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

## Behavioral Policies

The skill supports persistent behavioral preferences stored in the user-scoped
config file at `~/.config/adr-skills/preferences.toml` (see
[Configuration](#configuration)).

### Supported Policies

| Policy | Config Key | Default | Effect |
|--------|-----------|---------|--------|
| Participation mode | `[implement].participation` | `"guided"` | Sets the default participation level (full-control, guided, autonomous, weighted) |
| Auto-commit | `[implement].auto_commit` | `false` | Enables/disables git commits at task boundaries |

### Persistence Hierarchy

1. **Config file** — if `preferences.toml` contains the key under the
   `[implement]` table, use it.
2. **Session context** — if the key is absent, store the preference in session
   context (ephemeral, current session only). The user may opt to save it to
   the config file when prompted.
