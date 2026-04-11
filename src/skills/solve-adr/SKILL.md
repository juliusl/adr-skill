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
| P-3 | Never skip implementation when `auto_delegate = true` — includes session management concerns |
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
- Not session management concerns (e.g., "this will be extensive", "let me check session state", "deferring to a future session"). The plan, commits, and QA checkpoints exist to handle long sessions — the process architecture already solves the context problem
- Skill files (SKILL.md, references/, eval_queries.json) are executable agent instructions, not passive documentation — changes carry the same risk as code changes and require the full `/implement-adr` pipeline

**Enforcement:** When step 4 (Implement) completes and the report is generated, check: did `/implement-adr` actually run for every Ready ADR? If any Ready ADR was not delegated, this is a P-3 violation. Log the violation and invoke `/implement-adr` before proceeding to step 5.

### P-4: Triage all deferred QA findings before milestone completion

After `/implement-adr` completes, scan each QA plan for findings with `Deferred` status. These are findings the executor could not address in scope (per implement-adr P-3).

Triage each finding:
- **Address now** — the finding is addressable with the current codebase. Fix it and update the QA plan status to `Fixed`.
- **Accept** — the finding is a genuine low-risk item that does not need fixing. Update status to `Won't Fix` and document the rationale in the QA plan.
- **Escalate** — the finding reveals a gap that needs a new ADR. Invoke `/author-adr` for the gap.

No finding may remain `Deferred` when C-3 (Report) runs. Every deferred finding exits triage as `Fixed`, `Won't Fix`, or escalated to a new ADR.

In autonomous mode, apply this heuristic: if the minimum fix is a test or validation check, address it. If it requires a design decision, escalate to an ADR.

---

## Procedure

| ID | Scenario | Description |
|----|----------|-------------|
| S-0 | Startup | Load preferences, check automation config, recommend missing settings |
| S-1 | Problem | Solve a problem — explore options, produce ADRs, implement them |
| S-2 | Roadmap | Solve a roadmap — process milestones sequentially, delegating each to S-1 |
| S-3 | Fast-Path | Process pre-decided findings — classify, author Y-statement ADRs, route plan-only items directly |

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
├─ "Roadmap progress" ──────────► S-0 → S-2: Roadmap (survey only)
├─ "Process retro findings" ────► S-0 → S-3: Fast-Path
├─ "Process bug bash findings" ─► S-0 → S-3: Fast-Path
├─ "Apply amendment" ───────────► S-0 → S-3: Fast-Path
└─ "Fast-path [findings]" ──────► S-0 → S-3: Fast-Path
```

## Configuration

This skill reads preferences from two scopes:

**User-scoped** (`~/.config/adr-skills/preferences.toml`):
```toml
[solve]
participation = "guided"     # guided | autonomous
auto_delegate = false        # automatically invoke /implement-adr after acceptance

[solve.dispatch]
# Single reviewer (backward-compatible):
code_review = "juliusl-code-reviewer-v1"

# Multiple reviewers (dispatched in parallel):
# code_review = ["juliusl-code-reviewer-sweep-v5", "juliusl-code-reviewer-analytics-v5"]
```

**`[solve.dispatch]` keys:**

| Key | Default | Description |
|-----|---------|-------------|
| `code_review` | `""` (skip) | Accepts a string (single reviewer) or array of strings (multiple reviewers dispatched in parallel). A bare string is normalized to a single-element list. Empty strings and whitespace-only entries are filtered out. When the resulting list is empty, C-2 is skipped. When non-empty, each agent is dispatched via the `task` tool to review all changes on the solve branch. If a configured agent cannot be resolved at runtime, warn and skip that agent. |

**Project-scoped** (`.adr/preferences.toml`):
```toml
[solve]
default_scenario = "problem" # which scenario to use when not specified
fast_path_sources = ["retro", "bug-bash", "amendment"]  # finding sources that trigger S-3 (requires ADR-0067)
```

**Project-scoped `[solve]` keys:**

| Key | Default | Description |
|-----|---------|-------------|
| `default_scenario` | `"problem"` | Scenario to use when the user's request doesn't clearly match any other routing entry. |
| `fast_path_sources` | `["retro", "bug-bash", "amendment"]` | Finding source labels that trigger S-3 Fast-Path. Requires ADR-0067 (S-3). |

**Project-scoped `[solve.retro]` keys:**

```toml
[solve.retro]
enabled = true                # whether C-4 runs at all
skip_when_no_findings = false # skip C-4 when retrospective produces no actionable findings
```

| Key | Default | Description |
|-----|---------|-------------|
| `[solve.retro] enabled` | `true` | Set to `false` to disable C-4 entirely for this project. |
| `[solve.retro] skip_when_no_findings` | `false` | When `true`, C-4 skips writing a retro record if the retrospective produces no actionable findings. |

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
4. **Load dispatch config** — read `[solve.dispatch]` keys (`code_review`) for optional code review dispatch in C-2. Normalize `code_review` to a list: if it's a string, wrap in a single-element list. Filter out empty or whitespace-only entries. If the resulting list is empty, C-2 will be skipped. Read `[solve] fast_path_sources` for S-3 routing — validate each value against the recognized set (`retro`, `bug-bash`, `amendment`). Log a warning for each unrecognized value: `Warning: fast_path_sources contains unrecognized value "<v>" — ignored`.
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
4. Implement — delegate all Ready ADRs to /implement-adr in a single batch
   ↓
C. Conclusion — code review, QA triage, report, retrospective (defined in SKILL.md)
```

**On resume:** The agent evaluates the problem's current state and enters the lifecycle at the right point. No ADRs → step 1. ADRs exist but unreviewed → step 2. All ADRs reviewed but unimplemented → step 4. Some Accepted, others remain → step 4 for remaining. All Accepted, implementation complete → Conclusion. On resume, check for an existing `solve/<slug>` branch — if found and unmerged, checkout it and continue.

### Branch Management

solve-adr creates a feature branch to isolate its output from the user's working branch. implement-adr remains branch-agnostic — it commits to whatever branch it's on.

**Already on a solve branch:** If the current branch is already a `solve/` branch (e.g., from a roadmap milestone via S-2), skip branch creation. Record the branch context and proceed to Step 2.

**Branch lifecycle:**
1. **Create** — after Step 1 (intake), derive a slug from the problem statement (lowercase, hyphenated, max 50 chars). Create `solve/<slug>` from current HEAD: `git checkout -b solve/<slug>`.
2. **Switch** — all subsequent work (authoring, triage, implementation) happens on this branch.
3. **Complete** — after Conclusion (C-4), stay on the branch. The user reviews via PR and merges.
4. **Resume** — on resume, if the branch exists and is unmerged, checkout it and continue. If the branch was already merged or deleted, the previous solve is complete — create a new branch with a `-2` suffix if the same slug is reused.

**Branch naming:** `solve/<problem-slug>`. Example: `solve/caching-strategy-for-events`.

**Dirty working tree guard:** Before creating the branch, check `git status --porcelain`. If the working tree has uncommitted changes from the current solve's own prior work (e.g., defensive logging mid-milestone), note them and proceed. If the changes are unrelated, warn the user and ask them to commit or stash. Do not stash automatically — that risks losing user work.

**Base branch:** Branching from current HEAD is intentional. The user controls what base the solve branch starts from by checking out the desired branch before invoking solve-adr.

**Branch name and base branch storage:** The branch name and the base branch (the branch checked out when the solve branch was created) are maintained in the conversation/session state. On resume, the agent retrieves both from session context. The base branch is used by C-2 (Code Review) to compute the cumulative diff.

**Cross-skill invocation points:**
- **Step 2** — invoke `/author-adr` via the `skill` tool with the full list of decisions and problem context. The `skill` tool loads author-adr's context through the platform — do not read skill files directly.
- **Step 3** — invoke `/prototype-adr` for any ADR that paused at its Evaluation Checkpoint
- **Step 3** — re-invoke `/author-adr` to complete convergence on validated ADRs
- **Step 4** — invoke `/implement-adr` with all Ready ADRs in a single batch

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
5. Update — record milestone completion status → more milestones? → loop to 3
   ↓
C. Conclusion — code review, QA triage, report, retrospective (defined in SKILL.md)
```

**On resume:** The agent reads the roadmap file and checks milestone status markers. No markers → step 1. Some milestones complete → step 3 (select next). A milestone in-progress with ADRs → step 4 (solve, resume). All complete → Conclusion.

**Branch naming:** Roadmap-driven branches use `solve/<project-slug>/milestone-<N>` to distinguish from ad-hoc problem branches.

**Composition:** S-2 wraps S-1. All mandatory safeguards (plan review, QA, ADR for every decision) flow through S-1 unchanged. S-2 does not duplicate S-1's logic — it orchestrates milestone selection and progress tracking.

### S-3: Fast-Path

Process pre-decided findings — retrospective findings, bug bash findings, amendments to existing decisions. Each finding is classified using the ADR list test before routing.

Read [references/fast-path.md](references/fast-path.md) for the full workflow detail.

**Lifecycle:**

```
Entry: Receive finding list, identify source (retro / bug-bash / amendment)
   ↓
1. Classify — apply ADR list test to each finding
   ↓
2. Author Y-statement ADRs — for ADR-worthy findings (no exploration)
   ↓
3. Build plan list — plan-only findings with [Source: <origin>] notes
   ↓
4. Delegate — /implement-adr with all Ready ADRs + plan-only task list
   ↓
C. Conclusion — C-1 → C-2 → C-3 → C-4
```

## Conclusion

After either S-1 or S-2 completes its implementation steps, run the conclusion sequence. These steps apply to any solve type regardless of which scenario created it.

```
C-1 QA Triage → C-2 Code Review (optional) → C-3 Report → C-4 Retrospective (optional)
```

| ID | Step | Description |
|----|------|-------------|
| C-1 | QA Triage | Triage deferred QA findings per P-4 |
| C-2 | Code Review (optional) | Dispatch configured reviewer(s) to review branch diff |
| C-2a | Entry condition | Check normalized `code_review` list from S-0 |
| C-2b | Base branch detection | Retrieve base branch from session state, compute merge-base |
| C-2c | Agent dispatch | Invoke all configured reviewers in parallel via `task` tool |
| C-2d | Triage findings | Consolidate findings from all reviewers, fix all valid findings |
| C-2e | Re-review | Re-dispatch all reviewers to verify triage results |
| C-2f | Gate | Block C-3 until all reviewers accept or no high-priority findings remain |
| C-3 | Report | Summarize branch, completion status, remaining work |
| C-4 | Retrospective (optional) | Run structured retro, classify findings, write preference updates to `.adr/preferences.toml` |

### C-1: QA Triage

Triage deferred QA findings per P-4. No finding may remain `Deferred` when C-3 runs.

### C-2: Code Review (Optional)

After all implementation and QA triage completes, optionally dispatch configured code review agent(s) to review the cumulative diff of the solve branch against its base. Running code review after QA triage ensures reviewers see the final diff including any QA fixes.

**C-2a: Entry condition** — Check the normalized `code_review` list from S-0. If the list is empty (absent, all entries empty or whitespace-only), skip C-2 and proceed to C-3. Log: "C-2 skipped — no code review agents configured."

**C-2b: Base branch detection** — Retrieve the base branch from session state (recorded in Step 1b of problem.md). Compute the merge-base: `git merge-base HEAD <base-branch>`. If unavailable (e.g., resume from a prior session or the roadmap flow where S-2 created the branch), fall back to deriving it: `git log --oneline --decorate` to find the branch the solve branch diverged from, or use `git merge-base HEAD main` as a last resort.

**C-2c: Agent dispatch** — Invoke each configured reviewer in parallel via separate `task` tool calls. Each reviewer receives the same prompt:

```
Review the cumulative diff of the `<branch>` branch against its base `<base-branch>` in the repository at `<repo-path>`.

## Diff Scope
The merge-base is `<merge-base-sha>`. Use `git --no-pager diff <merge-base>..HEAD` to see the full diff.

## What was implemented
<summary of implemented work — ADR titles, what each group delivered>

## Project conventions
The project's AGENTS.md is at `<repo-path>/AGENTS.md`. Read it for project-specific review conventions before reviewing.

## Review instructions
Focus on: security, logic errors, consistency between definitions and implementations, code quality, breaking changes.
Do NOT flag: formatting preferences, test coverage gaps already tracked in QA plans.
```

If a configured agent cannot be resolved at runtime, warn and skip that agent. If all agents fail to resolve, skip C-2.

**C-2d: Triage findings** — Consolidate findings from all reviewers into a single list. Deduplicate equivalent findings (same file, same issue, similar recommendation) — when findings overlap, keep the more detailed version. When findings conflict, the triage actor resolves. For each finding, determine if it is valid. Fix all valid findings on the solve branch. If a finding is not valid, document the rejection rationale in the triage response passed to C-2e — the reviewer will evaluate whether the rejection is justified. If a valid finding cannot be fixed in the current scope, create a follow-up item (issue, work item, or roadmap note) so the finding is not lost. Priority levels (high, medium, nit) guide the reviewer's verdict in C-2e, not the triage actor's willingness to fix. All valid findings are fixed regardless of priority. Deduplication is for the triage actor's own work — preserve each reviewer's original unmodified findings for C-2e.

**C-2e: Re-review** — After triage, re-dispatch all configured reviewers in parallel to verify the fixes. The re-review is adversarial — each reviewer checks whether findings were properly addressed, identifies regressions, and produces new findings if warranted. Each reviewer uses priority levels from the original review to determine their verdict. Pass each reviewer their own original unmodified findings alongside the triage results — this preserves provenance so each reviewer can match their findings to the triage response without relying on the deduplicated list.

**C-2f: Gate** — Check all re-review verdicts:
- **All reviewers accepted** → proceed to C-3.
- **Any reviewer says "Wait for Reviewer"** → high-priority findings remain. In autonomous mode, address the remaining findings and re-dispatch C-2e (one retry). If still unresolved, pause for user intervention. In guided mode, present findings to the user.

### C-3: Report

Stay on the feature branch and present the completion report in the format appropriate to the scenario.

**S-1 (Problem) format:**

```markdown
## Problem: [topic]

**Branch:** `solve/<slug>` — ready for PR review

| Group | ADRs | Status | Result |
|-------|------|--------|--------|
| 1 | ADR-NNNN, ADR-NNNN | Accepted | ✅ Completed |
| 2 | ADR-NNNN | Proposed | ⏳ Next up |

**Completed:** N of M ADRs
**Remaining:** [list]
**Blocked:** None
```

**S-2 (Roadmap) format:**

```markdown
## Roadmap: [project name]

**File:** `[roadmap path]`
**Branch:** `solve/[slug]/milestone-[N]` (if applicable)

| # | Milestone | Status | ADRs | Result |
|---|-----------|--------|------|--------|
| 1 | Milestone 1 | Complete | ADR-NNNN, ADR-NNNN | ✅ |
| 2 | Milestone 2 | In-progress | ADR-NNNN | 🔄 Partial |
| 3 | Milestone 3 | Pending | — | ⏳ |

**Progress:** N of M milestones complete
**Current:** Milestone N — [status detail]
**Next:** Milestone N+1 — [first objective]
```

### C-4: Retrospective

After C-3, run a structured retrospective on the completed solve run. C-4 is optional — skip when `[solve.retro] enabled = false` in `.adr/preferences.toml`. Skip also when `skip_when_no_findings = true` and the retrospective produces no actionable findings.

**Entry condition:** C-3 has completed. C-4 does not affect C-3 artifacts.

**Retrospective questions:**
- What slowed things down?
- What quality steps added the most value?
- Were any steps unnecessary for this project's context?
- What would you change for the next solve?

**Classify each finding:**
- **Pipeline preference** — expressible as an existing key in the preference schema. Propose a config update. If no matching key exists, the finding is note-only — do not invent new keys; new keys require a future ADR.
- **Note-only** — useful context but not expressible as a current preference key.

**Apply updates:**
- Autonomous mode: write proposed changes to `.adr/preferences.toml` and log what changed. Log format: `C-4: set [solve.retro].key = value`.
- Guided mode: present proposals, apply on confirmation.

**Preferences write strategy:** Read the existing `.adr/preferences.toml`, modify only the target keys in memory, and rewrite the full file preserving all other sections and keys. Do not overwrite the file with only the modified section. If the file doesn't exist, note that no existing keys are available to update and skip the write step — do not create the file.

**Retro record slug:** Derive the slug from the current UTC timestamp (`YYYYMMDD-HHMMSS`), not from user-supplied text. Example: `.adr/var/retro-20260410-143022.md`. If a file at the derived path already exists, append a counter suffix (`-2`, `-3`, etc.) rather than overwriting.

**Zero-findings case:** When `skip_when_no_findings = false` and the retrospective produces no actionable findings, note "No findings to retrospect" and skip the 4 questions. Writing a retro record with empty answers serves no purpose regardless of the key value.

**Write retro record:** Save findings to `.adr/var/retro-<slug>.md`. Include: run summary, findings, preference changes applied. Create `.adr/var/` if it doesn't exist.

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
- **[references/fast-path.md](references/fast-path.md)** — Full Fast-Path workflow: finding intake, ADR list classification test, Y-statement authoring, plan-only routing, conclusion.
