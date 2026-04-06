---
name: solve-adr
description: "Use this skill when the user wants to solve a problem through structured exploration — analyzing constraints, discovering options, making decisions, and driving implementation across the ADR skill ecosystem. Activate when the user says things like \"solve this problem,\" \"help me figure out,\" \"explore options for,\" \"I need to decide how to handle,\" or \"what's the best approach for.\" Also activate for multi-ADR orchestration: \"implement these ADRs,\" \"continue solving,\" \"drive this roadmap,\" or \"implement milestones X to Y.\" The skill orchestrates across /author-adr (decisions), /prototype-adr (experiments), and /implement-adr (execution). Do not use for creating a single ADR when the user already has a decision — use author-adr. Do not use for implementing an existing ADR — use implement-adr. Do not use for running a standalone experiment — use prototype-adr."
license: CC-BY-4.0
metadata:
  version: "0.1"
---
# Solve ADR — Scenario-Driven Problem Solving

Orchestrate problem-solving end-to-end by delegating to companion skills: `/author-adr` for decisions, `/prototype-adr` for experiments, `/implement-adr` for execution. Every architectural decision encountered during problem solving is recorded via `/author-adr` for auditability.

## Procedure

| ID | Scenario | Mandatory | Description |
|----|----------|-----------|-------------|
| S-0 | Startup | Yes | Load preferences, check automation config, recommend missing settings |
| S-1 | Problem Exploration | Conditional | User provides problem + constraints → explore options → converge on decision |
| S-2 | Roadmap Execution | Conditional | Implement a chain of ADRs in dependency order |

**Routing:** Exactly one scenario runs per invocation. The agent selects the scenario based on the user's request. If the request doesn't match any scenario, explain what was requested and which scenario would handle it.

**If a mandatory step is skipped, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

```
User request
├─ docs/adr/ exists? ────────────► Load preferences → select scenario
├─ docs/adr/ missing? ──────────► Recommend: run `/author-adr` to bootstrap first
│
│  Scenario routing:
├─ "Solve this problem" ────────► S-0 → S-1: Problem Exploration
├─ "Help me figure out X" ──────► S-0 → S-1: Problem Exploration
├─ "Explore options for Y" ─────► S-0 → S-1: Problem Exploration
├─ "What's the best approach" ──► S-0 → S-1: Problem Exploration
│
├─ "Implement these ADRs" ──────► S-0 → S-2: Roadmap Execution
├─ "Continue solving all ADRs" ─► S-0 → S-2: Roadmap Execution
├─ "Drive this roadmap" ────────► S-0 → S-2: Roadmap Execution
└─ "Implement milestones X to Y" ► S-0 → S-2: Roadmap Execution
```

## Configuration

This skill reads preferences from two scopes:

**User-scoped** (`~/.config/adr-skills/preferences.toml`):
```toml
[solve]
participation = "guided"     # guided | autonomous
auto_delegate = false        # automatically invoke /implement-adr after acceptance
```

**Project-scoped** (`.adr/preferences.toml`):
```toml
[solve]
default_scenario = "problem" # which scenario to use when not specified
```

**Path resolution:**
1. If `$XDG_CONFIG_HOME` is set, use `$XDG_CONFIG_HOME/adr-skills/preferences.toml`.
2. Otherwise, use `$HOME/.config/adr-skills/preferences.toml`.
3. Project-scoped: `.adr/preferences.toml` in the repository root.

Project-scoped values override user-scoped values. If neither file exists, use built-in defaults.

**Create on first write:** When persisting a preference, create the directory with `mkdir -p` before writing. Never assume it already exists.

## Writing Style

All generated content must follow this style:
- **Technical and simple** — write for engineers, not academics
- **No double negatives** — say what things *do*, not what they don't not do
- **Clear logic** — one idea per sentence, explicit cause-and-effect
- **Concise** — cut filler words; if a sentence works without a word, remove it

### S-0: Startup

Run this before every scenario.

1. **Read user preferences** — resolve the config path and read `[solve]` from `preferences.toml`.
   - If set, store `participation` and `auto_delegate` for the session.
   - If absent, proceed with defaults (`participation = "guided"`, `auto_delegate = false`).
2. **Read project preferences** — read `.adr/preferences.toml` for `[solve]` overrides.
   - Project values override user values.
3. **Recommend missing settings** — if no `[solve]` section exists in either scope, recommend defaults:
   > No solve preferences configured. Recommended defaults:
   > - `participation = "guided"` — summarize findings, let you drive decisions
   > - `auto_delegate = false` — ask before invoking /implement-adr
   >
   > Save these defaults?
4. **Load dispatch config** — read `[author.dispatch]` keys (`review`, `editor`) for downstream `/author-adr` calls. These control which agents handle review and revision cycles.

### S-1: Problem Exploration

The user provides a problem, background context, current thought process, and constraints. The agent explores the problem space, produces one or more ADRs via `/author-adr`, and drives implementation via `/implement-adr`. A single problem may require multiple decisions — each gets its own ADR.

Read [references/solve.md](references/solve.md) for the full S-1 workflow detail.

**Workflow summary:**

```
S-1.1: Problem intake — capture problem, constraints, stakeholders
  ↓
S-1.2: Decision loop — for each decision the problem requires:
  │  ├─ /author-adr — create ADR (worksheet → options → convergence)
  │  ├─ /prototype-adr — if Evaluation Checkpoint needs validation
  │  └─ /author-adr — review → revise cycle
  │  (repeat if the problem requires additional decisions)
  ↓
S-1.3: Implement — dependency-order the produced ADRs, /implement-adr for each
```

**How it works:** Solve-adr owns the problem intake (S-1.1), decision orchestration (S-1.2), and implementation sequencing (S-1.3). Each individual ADR is created and reviewed by `/author-adr`. Implementation of each ADR is delegated to `/implement-adr`.

**S-1.2 decision loop:** A single problem may decompose into multiple decisions. The agent identifies when the current ADR's scope leaves gaps that need separate decisions (e.g., "the naming convention is one decision, but the data model is another"). Each iteration through the loop produces one reviewed ADR.

**S-1.3 implementation:** After all decisions are made, the agent:
1. Analyzes dependencies between the produced ADRs
2. Orders them for implementation (dependencies first)
3. Presents the implementation plan to the user (or proceeds in autonomous mode)
4. Delegates to `/implement-adr` for each ADR in order
5. Reports progress after each implementation completes

If a gap is discovered during implementation that requires a new decision, the agent pauses implementation, invokes `/author-adr` to create the new ADR, and resumes.

**Cross-skill delegation points:**
- **S-1.2** — invoke `/author-adr` to create each ADR (worksheet → options → decision → review)
- **S-1.2** — invoke `/prototype-adr` if an Evaluation Checkpoint says "Pause for validation"
- **S-1.3** — invoke `/implement-adr` for each ADR in dependency order

### S-2: Roadmap Execution

The user has existing Proposed ADRs that need implementation. No exploration phase — the decisions are already made.

Read [references/roadmap.md](references/roadmap.md) for the full S-2 workflow detail.

**Workflow summary:**

```
S-2.1: Survey — identify which ADRs are in scope, read their status
  ↓
S-2.2: Dependency analysis — determine implementation order
  ↓
S-2.3: Execute — /implement-adr for each ADR in order
  ↓
S-2.4: Progress report — summarize what was implemented, what remains
```

**How it works:** Solve-adr owns the survey, ordering, and progress tracking. Implementation of each ADR is delegated to `/implement-adr`. If a gap is discovered during implementation that requires a new decision, the agent invokes `/author-adr`, adds the new ADR to the chain, and resumes.

**Cross-skill delegation points:**
- **S-2.3** — invoke `/implement-adr` for each ADR in the chain
- **S-2.3** — invoke `/author-adr` if a gap needs a new decision

## Cross-Skill Invocation

The solve-adr agent delegates to companion skills by invoking them via the `skill` tool:

```
skill: "author-adr"    — when a decision needs to be recorded, reviewed, or revised
skill: "prototype-adr"  — when an assumption needs experimental validation
skill: "implement-adr"  — when an accepted decision needs execution
```

Each invocation loads the target skill's SKILL.md into the conversation context. The target skill runs its full procedure. When the target skill completes, the solve-adr agent resumes its scenario from the conversation history.

The platform constraint "do not invoke a skill that is already running" permits this pattern: solve-adr and the target skill are different skills. The agent's orchestration state (scenario step, problem context, ADRs created) is maintained in the conversation — not in skill-scoped storage.

## Defensive Logging

In all scenarios, the agent must:
- Create an ADR via `/author-adr` for every architectural decision encountered during problem solving
- Use `/author-adr` review workflow for quality assurance on each decision
- Never make a decision silently — if a choice affects architecture, it gets an ADR

The solve-adr skill's primary output is a set of reviewed, accepted decisions — not code. The decisions are the audit trail.

## Deep References

- **[references/solve.md](references/solve.md)** — Full S-1 Problem Exploration workflow: problem intake, option discovery, requirements refinement, evaluation checkpoint, convergence, and handoff.
- **[references/roadmap.md](references/roadmap.md)** — Full S-2 Roadmap Execution workflow: survey, dependency analysis, sequential implementation delegation, progress tracking.
