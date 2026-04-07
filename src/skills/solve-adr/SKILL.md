---
name: solve-adr
description: "Use this skill when the user wants to solve a problem through structured exploration — analyzing constraints, discovering options, making decisions, and driving implementation across the ADR skill ecosystem. Activate when the user says things like \"solve this problem,\" \"help me figure out,\" \"explore options for,\" \"I need to decide how to handle,\" or \"what's the best approach for.\" Also activate for multi-ADR orchestration: \"implement these ADRs,\" \"continue solving,\" \"solve remaining ADRs,\" or \"implement milestones X to Y.\" Also activate for roadmap-driven workflows: \"solve this roadmap,\" \"process roadmap,\" \"continue roadmap,\" \"continue milestone N,\" or \"roadmap progress.\" The skill orchestrates across /author-adr (decisions), /prototype-adr (experiments), and /implement-adr (execution). Do not use for creating a single ADR when the user already has a decision — use author-adr. Do not use for implementing an existing ADR — use implement-adr. Do not use for running a standalone experiment — use prototype-adr."
license: CC-BY-4.0
metadata:
  version: "0.2"
---
# Solve ADR — Scenario-Driven Problem Solving

Orchestrate problem-solving end-to-end by delegating to companion skills: `/author-adr` for decisions, `/prototype-adr` for experiments, `/implement-adr` for execution. Every architectural decision encountered during problem solving is recorded via `/author-adr` for auditability.

## Policies

**Ignoring any of the below policies is a runtime violation ESPECIALLY when the user is away and operating autonomously**

| ID | Policy |
|----|--------|
| P-1 | Every architectural decision gets an ADR — no silent decisions |
| P-2 | Cross-skill invocation must use the `skill` tool exclusively |
| P-3 | Never skip implementation when `auto_delegate = true` |
| P-4 | Triage all deferred QA findings before milestone completion |

### P-1: Every architectural decision gets an ADR

Create an ADR via `/author-adr` for every architectural decision encountered during problem solving. `/author-adr` is capable of authoring more than one ADR at a time — this skill only needs to provide the problem context and any pre-emptive options and let `/author-adr` take over. Use `/author-adr` review workflow for quality assurance on each decision. Never make a decision silently — if a choice affects architecture, it gets an ADR.

The solve-adr skill's primary output is a set of reviewed, accepted decisions — not code. The decisions are the audit trail.

### P-2: Cross-skill invocation must use the `skill` tool exclusively

Never read, load, or inline another skill's SKILL.md, references, or assets directly. The `skill` tool is the only authorized interface — it loads the target skill's context through the platform's controlled channel. Reading skill files directly bypasses platform controls and creates a prompt-injection vector: a compromised or tampered skill document would execute with the current agent's permissions.

### P-3: Never skip implementation when `auto_delegate = true`

When `auto_delegate = true`, implement accepted ADRs via `/implement-adr` — do not skip implementation based on any framing of the output:
- Not the user's framing (e.g., "design", "explore")
- Not the agent's own rationalization (e.g., "these are just documentation files", "simple enough to do directly")
- Skill files (SKILL.md, references/, eval_queries.json) are executable agent instructions, not passive documentation — changes carry the same risk as code changes and require the full `/implement-adr` pipeline

### P-4: Triage all deferred QA findings before milestone completion

After `/implement-adr` completes, scan each QA plan for findings with `Deferred` status. These are findings the executor could not address in scope (per implement-adr P-3).

Triage each finding:
- **Address now** — the finding is addressable with the current codebase. Fix it and update the QA plan status to `Fixed`.
- **Accept** — the finding is a genuine low-risk item that does not need fixing. Update status to `Won't Fix` and document the rationale in the QA plan.
- **Escalate** — the finding reveals a gap that needs a new ADR. Invoke `/author-adr` for the gap.

No finding may remain `Deferred` when step 5 (Report) runs. Every deferred finding exits triage as `Fixed`, `Won't Fix`, or escalated to a new ADR.

In autonomous mode, apply this heuristic: if the minimum fix is a test or validation check, address it. If it requires a design decision, escalate to an ADR.

---

## Procedure

| ID | Scenario | Description |
|----|----------|-------------|
| S-0 | Startup | Load preferences, check automation config, recommend missing settings |
| S-1 | Problem | Solve a problem — explore options, produce ADRs, implement them |
| S-2 | Roadmap | Solve a roadmap — process milestones sequentially, delegating each to S-1 |

**Resume protocol:** Every solvable thing is resumable. When invoked on a problem that already has ADRs, the agent picks up where it left off — skipping completed steps, implementing remaining ADRs. Resume is not a separate scenario; it's how solve works across sessions.

**Routing:** The agent selects the scenario based on the user's request. If the request doesn't match any scenario, explain what was requested and which scenario would handle it.

```
User request
├─ docs/adr/ exists? ────────────► Load preferences → select scenario
├─ docs/adr/ missing? ──────────► Recommend: run `/author-adr` to bootstrap first
│
│  Scenario routing:
├─ "Solve this problem" ────────► S-0 → S-1: Problem
├─ "Help me figure out X" ──────► S-0 → S-1: Problem
├─ "Explore options for Y" ─────► S-0 → S-1: Problem
├─ "What's the best approach" ──► S-0 → S-1: Problem
├─ "Implement these ADRs" ──────► S-0 → S-1: Problem (resume)
├─ "Continue solving" ──────────► S-0 → S-1: Problem (resume)
├─ "Solve remaining ADRs" ──────► S-0 → S-1: Problem (resume)
├─ "Resume solving [topic]" ────► S-0 → S-1: Problem (resume)
├─ "Solve this roadmap" ────────► S-0 → S-2: Roadmap
├─ "Process roadmap [path]" ────► S-0 → S-2: Roadmap
├─ "Continue milestone N" ──────► S-0 → S-2: Roadmap (resume)
├─ "Continue roadmap" ──────────► S-0 → S-2: Roadmap (resume)
└─ "Roadmap progress" ──────────► S-0 → S-2: Roadmap (survey only)
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
5. **Pre-flight check** — before proceeding to any scenario, verify the environment:
   - `git status --porcelain` — warn if the working tree is dirty (branching in S-1 requires a clean tree)
   - `make test` — run the test suite to establish a clean baseline. If tests fail, note pre-existing failures so they aren't mistaken for regressions during implementation.
   - Pre-flight is advisory — log findings and proceed. Do not block on pre-existing issues.

### S-1: Problem

Solve a problem — whether new (explore) or partially solved (resume). A problem produces one or more ADRs (one-to-many).

Read [references/problem.md](references/problem.md) for the full workflow detail.

**Lifecycle:**

```
1. Intake — capture problem, constraints, stakeholders, enumerate decisions needed
   ↓
1b. Branch — create solve/<slug> feature branch from current HEAD
   ↓
2. Author — load /author-adr context and run its procedure for all decisions
   ↓
3. Triage — review returned ADRs, route evaluation-checkpoint-paused ones to /prototype-adr
   ↓
4. Implement — group accepted ADRs, load /implement-adr and run its procedure
   ↓
5. Report — summarize what was implemented, what remains
```

**On resume:** The agent evaluates the problem's current state and enters the lifecycle at the right point. No ADRs → step 1. ADRs exist but unreviewed → step 2. All ADRs reviewed but unimplemented → step 4. Some Accepted, others remain → step 4 for remaining. On resume, check for an existing `solve/<slug>` branch — if found and unmerged, checkout it and continue.

### Branch Management

solve-adr creates a feature branch to isolate its output from the user's working branch. implement-adr remains branch-agnostic — it commits to whatever branch it's on.

**Branch lifecycle:**
1. **Create** — after Step 1 (intake), derive a slug from the problem statement (lowercase, hyphenated, max 50 chars). Create `solve/<slug>` from current HEAD: `git checkout -b solve/<slug>`.
2. **Switch** — all subsequent work (authoring, triage, implementation) happens on this branch.
3. **Complete** — after Step 5 (report), stay on the branch. The user reviews via PR and merges.
4. **Resume** — on resume, if the branch exists and is unmerged, checkout it and continue. If the branch was already merged or deleted, the previous solve is complete — create a new branch with a `-2` suffix if the same slug is reused.

**Branch naming:** `solve/<problem-slug>`. Example: `solve/caching-strategy-for-events`.

**Dirty working tree guard:** Before creating the branch, check `git status --porcelain`. If the working tree has uncommitted changes, warn the user and ask them to commit or stash before proceeding. Do not stash automatically — that risks losing user work.

**Base branch:** Branching from current HEAD is intentional. The user controls what base the solve branch starts from by checking out the desired branch before invoking solve-adr.

**Branch name storage:** The branch name is maintained in the conversation/session state. On resume, the agent retrieves the branch name from session context or re-derives it from the problem statement.

**Cross-skill invocation points:**
- **Step 2** — invoke `/author-adr` via the `skill` tool with the full list of decisions and problem context. The `skill` tool loads author-adr's context through the platform — do not read skill files directly.
- **Step 3** — invoke `/prototype-adr` for any ADR that paused at its Evaluation Checkpoint
- **Step 3** — re-invoke `/author-adr` to complete convergence on validated ADRs
- **Step 4** — invoke `/implement-adr` for each group (multi-ADR batch)

### S-2: Roadmap

Process a roadmap document milestone-by-milestone. Each milestone is delegated to S-1 Problem as a structured intake.

Read [references/roadmap.md](references/roadmap.md) for the full workflow detail.

**Lifecycle:**

```
1. Load — read and parse the roadmap document
   ↓
2. Survey — identify milestone progress (complete, in-progress, pending)
   ↓
3. Select — determine which milestone to work on next
   ↓
4. Solve — delegate milestone to S-1 Problem lifecycle
   ↓
5. Update — record milestone completion status
   ↓
6. Report — summarize roadmap progress
```

**On resume:** The agent reads the roadmap file and checks milestone status markers. No markers → step 1. Some milestones complete → step 3 (select next). A milestone in-progress with ADRs → step 4 (solve, resume). All complete → step 6 (report).

**Branch naming:** Roadmap-driven branches use `solve/<project-slug>/milestone-<N>` to distinguish from ad-hoc problem branches.

**Composition:** S-2 wraps S-1. All mandatory safeguards (plan review, QA, ADR for every decision) flow through S-1 unchanged. S-2 does not duplicate S-1's logic — it orchestrates milestone selection and progress tracking.

## Cross-Skill Invocation

The solve-adr agent delegates to companion skills by invoking them via the `skill` tool:

```
skill: "author-adr"    — when a decision needs to be recorded, reviewed, or revised
skill: "prototype-adr"  — when an assumption needs experimental validation
skill: "implement-adr"  — when an accepted decision needs execution
```

Each invocation uses the `skill` tool — the platform's controlled channel. The `skill` tool loads the target skill's context securely. Never read skill files directly (see MANDATORY INSTRUCTIONS). The target skill runs its full procedure — this is intentionally thorough. When the target skill completes, control returns to solve-adr.

**Callback pattern:** When solve-adr delegates to `/implement-adr` and more work remains (additional groups in S-1.4), instruct `/implement-adr` to invoke `/solve-adr` on completion to continue. This creates the continuation chain: solve → implement → solve → implement. Each skill invocation carries its full safeguards (plan review, QA) — this is intentional, not reducible.

The platform constraint "do not invoke a skill that is already running" permits this pattern: solve-adr and the target skill are different skills. The agent's orchestration state (scenario step, problem context, ADRs created) is maintained in the conversation — not in skill-scoped storage.

## Deep References

- **[references/problem.md](references/problem.md)** — Full Problem workflow: intake, batch authoring, triage, implementation delegation, resume protocol, progress tracking.
- **[references/roadmap.md](references/roadmap.md)** — Full Roadmap workflow: document format, milestone parsing, survey, selection, S-1 delegation, progress tracking, resume protocol.
