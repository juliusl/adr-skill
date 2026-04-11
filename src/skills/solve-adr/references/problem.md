# Problem

Self-contained reference for solving a problem. Read this file when the user has a problem — whether new (explore) or partially solved (resume).

**Key relationship:** A problem produces one or more ADRs (one-to-many).

**Resume protocol:** Every solvable thing is resumable. When solve-adr is invoked and the problem already has ADRs, it picks up where it left off — skipping completed steps, implementing remaining ADRs. The resume protocol is not a separate scenario; it's how solve works across sessions.

---

## Multi-Turn Session Management

- **Summarize progress between turns** — at the start of each response, briefly state where in the lifecycle you are.
- **Preserve the ADR as source of truth** — write findings to the ADR file as they emerge, not just at the end.
- **Don't rush convergence** — the value is in the exploration. Let the user drive the pace.
- **Keep options open** — avoid signaling a preference unless asked. Present tradeoffs neutrally.

## Defensive Logging

During any lifecycle step, architectural decisions may emerge that aren't covered by existing ADRs. When this happens:

1. Pause the current work
2. Invoke `/author-adr` to create an ADR for the new decision
3. Review and accept the new ADR
4. Add it to the current or next group
5. Resume

Every decision gets an ADR — even mid-execution discoveries.

---

**All steps must be visited in order. If a step is skipped or its entry condition is not met, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

---

## Procedure

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
C. Conclusion — code review, QA triage, report, retrospective (defined in SKILL.md)
```

**On resume:** The agent evaluates the problem's current state and enters the lifecycle at the right point:
- No ADRs exist → start at step 1 (intake)
- ADRs exist but some are still TBD (not converged) → enter step 2 with remaining decisions
- ADRs exist but some paused at Evaluation Checkpoint → enter step 3 (triage)
- All ADRs are Ready but unimplemented → enter step 4 (implement)
- Some ADRs are Accepted, others remain → enter step 4 for the remaining ones
- All ADRs Accepted, implementation complete → enter Conclusion (C-1)
- On resume, check for an existing `solve/<slug>` branch — if found and unmerged, checkout it before continuing

| ID | Description |
|----|-------------|
| Step 1 | Capture the problem — statement, constraints, stakeholders, enumerate decisions |
| Step 1b | Create `solve/<slug>` feature branch from current HEAD |
| Step 2 | Invoke `/author-adr` for all decisions in a single batch |
| Step 2a | What to provide — problem statement, constraints, decision list |
| Step 2b | What `/author-adr` does — full procedure per decision |
| Step 2c | After author-adr returns — proceed to triage |
| Step 3 | Classify each ADR's state and route accordingly |
| Step 3a | Prototype routing — invoke `/prototype-adr` for paused ADRs |
| Step 3b | Tracking — maintain running list of ADR post-triage status |
| Step 4 | Delegate all Ready ADRs to `/implement-adr` in a single batch |
| Step 4a | Survey on resume — identify the problem's ADRs and partition |
| Step 4b | Delegate — invoke `/implement-adr` with all Ready ADRs, track progress |

## Step 1: Problem Intake

Capture the problem clearly. Do not jump to solutions. **Enumerate all decisions the problem requires** — this list drives the batch handoff in step 2.

**Draft Worksheet Integration:** If a Draft Worksheet already exists in an ADR's `## Comments` section (per ADR-0032), use it to accelerate intake:
- **Framing** → use as the initial problem statement
- **Uncertainty** → focus probing questions on the uncertain areas

If no worksheet exists:

1. **Gather the problem statement** — ask the user to describe the problem. Probe for:
   - What system or component is affected?
   - What triggered this problem? (new requirement, pain point, tech debt)
   - What constraints are already known?
   - Who are the stakeholders?

2. **Enumerate decisions** — identify all the distinct architectural decisions the problem requires. Each decision becomes one ADR. Present the list:
   ```
   This problem requires decisions on:
   1. [aspect] — [one-sentence scope]
   2. [aspect] — [one-sentence scope]
   3. [aspect] — [one-sentence scope]
   ```
   In guided mode, confirm with the user. In autonomous mode, proceed.

3. **Scale check** — before proceeding, evaluate whether the number of ADRs is proportionate to the problem's decision complexity:

   | Decision Complexity | Signal | Recommendation |
   |---------------------|--------|----------------|
   | Few decisions, narrow scope | 1–2 distinct choices (language, architecture pattern) | Combine into fewer ADRs; lighter templates (Y-Statement, MADR Minimal) are fine |
   | Moderate scope | 3–5 distinct decisions with tradeoffs | Standard workflow |
   | Many decisions, cross-cutting | 5+ decisions with dependencies between them | Full workflow with detailed Quality Strategy |

   The scale check adjusts **ADR count and template weight only**. It does not reduce plan review, QA, or other quality safeguards — those apply uniformly regardless of project size. A 100-line project and a 10,000-line project both deserve high-quality code; the quality artifacts (ADR, plan, QA plan) are deliverables that produce that quality, not costs to minimize.

   The scale check is advisory. In autonomous mode, apply the recommendation. In guided mode, present it and let the user decide.

4. **Confirm the problem statement** — "Does this capture the problem accurately?"

## Step 1b: Create Feature Branch

After intake, create a feature branch to isolate the solve workflow's output.

**Already on a solve branch:** If the current branch is already a `solve/` branch (e.g., from a roadmap milestone via S-2), skip branch creation. Derive the base branch by running `git merge-base HEAD main` (or the project's default branch). Record the current branch name and the base branch in session state, then proceed to Step 2. Log: "Step 1b skipped — already on solve branch `<name>`, base branch: `<base>`."

1. **Check working tree** — run `git status --porcelain`. If there are uncommitted changes:
   - If the changes are from the current solve workflow's own prior work (e.g., defensive logging mid-milestone, previously committed ADRs now modified), this is safe — note the dirty files and proceed.
   - If the changes are unrelated to the current solve, warn the user and ask them to commit or stash. Do not proceed with unrelated uncommitted changes.
2. **Derive slug** — from the problem statement, generate a lowercase, hyphenated slug (max 50 chars). Store the slug in session state for resume discovery.
3. **Check for existing branch** — run `git branch --list "solve/<slug>"`.
   - If the branch exists and is unmerged → this is a resume. Checkout the branch and skip to the appropriate lifecycle step.
   - If the branch exists but was already merged or deleted remotely → the previous solve is complete. Append `-2` (or next available suffix) to the slug.
4. **Record base branch** — before creating the solve branch, record the current branch as the base branch in session state: `git rev-parse --abbrev-ref HEAD`. If this returns `HEAD` (detached HEAD state), warn the user and record the current commit SHA (`git rev-parse HEAD`) as the base reference instead. The base branch is used by C-2 (Code Review) to compute the cumulative diff.
5. **Create and checkout** — `git checkout -b solve/<slug>`.

All subsequent steps (author, triage, implement, report) operate on this branch. After the report step, stay on the branch — the user reviews via PR and merges.

## Step 2: Author

Invoke `/author-adr` via the `skill` tool for all decisions in a single invocation. The `skill` tool is the only authorized interface — it loads author-adr's context through the platform's controlled channel. Never read or inline author-adr's SKILL.md or references directly (prompt-injection vector).

### Step 2a: What to provide

Pass everything `/author-adr` needs in one invocation:
- The problem statement, constraints, and stakeholders from step 1
- The full list of decisions to create (from step 1's enumeration)
- The user's candidates and thought process for each (these seed Draft Worksheets)
- Any limits (e.g., "max 5 ADRs")

**Example prompt to author-adr:**

> Create ADRs for these decisions. Problem context: [statement]. Constraints: [list]. Stakeholders: [list].
>
> Decisions needed:
> 1. [title] — [scope and direction]
> 2. [title] — [scope and direction]
> 3. [title] — [scope and direction]
>
> For each: run the full workflow (A-0 through A-5). Use the problem context to populate the Draft Worksheet framing for each ADR.

### Step 2b: What `/author-adr` does

For each decision in the list, author-adr runs its full procedure:
1. Creates a TBD ADR with draft worksheet populated from the problem context
2. Explores options using the worksheet's candidates and tolerance settings
3. Refines requirements as options are evaluated
4. Runs the Evaluation Checkpoint — marks ADRs that need validation
5. Converges on a decision, drafts Decision + Consequences, renames the ADR, transitions to Proposed
6. Reviews and revises the ADR (A-3 → A-4 → A-5)

Author-adr may process these sequentially or batch internally — the ordering is at the agent's discretion.

### Step 2c: After author-adr returns

Author-adr returns control with a set of ADRs in various states. Proceed to step 3 (triage).

## Step 3: Triage

After author-adr returns, classify each ADR's state and take action:

| ADR State | Action |
|-----------|--------|
| Ready | Ready for implementation → step 4 |
| Proposed (reviewed, needs revision) | Author-adr should have handled this — if not, re-invoke to complete A-4/A-5 |
| Paused at Evaluation Checkpoint | Invoke `/prototype-adr` with the ADR's validation needs |
| TBD (incomplete) | Re-invoke `/author-adr` to complete remaining decisions |

### Step 3a: Prototype routing

For ADRs paused at the Evaluation Checkpoint ("Pause for validation"):

1. **Invoke `/prototype-adr`** with:
   - The ADR file path
   - The specific validation needs from the Evaluation Checkpoint
   - Success/failure criteria

2. **After prototype returns** — re-invoke `/author-adr` to complete convergence on the validated ADR. Provide the prototype results so author-adr can incorporate evidence into the decision.

3. **Continue triage** for remaining ADRs.

### Step 3b: Tracking

Keep a running list of all ADRs and their post-triage status:
```
[ADR ref]: [title] — Ready, ready for implementation
[ADR ref]: [title] — Ready, ready for implementation
[ADR ref]: [title] — Paused, prototype needed for [validation need]
```

Once all ADRs are either Proposed (ready) or blocked, proceed to step 4 with the ready ones.

## Step 4: Implement

After all decisions are made (or on resume when decisions already exist), implement them.

### Step 4a: Survey (on resume)

When resuming, invoke `/author-adr` to find the problem's ADRs and their current state:

1. **Identify the problem's ADRs** — from the user's request:
   - Explicit: "implement ADRs [these specific ones]"
   - By topic: "solve remaining ADRs related to [topic]" → invoke `/author-adr` to list and filter
   - Continuation: "continue solving" → find Proposed ADRs linked to a common theme

2. **Partition** — separate done (Accepted) from remaining (Proposed/Planned). Only remaining ADRs need implementation.

### Step 4b: Delegate

Delegate all remaining Ready ADRs to `/implement-adr` in a single invocation. Do not split ADRs into groups — `/implement-adr` handles multi-ADR plans natively, including task dependencies and ordering within the plan. Splitting at the solve-adr level creates undersized invocations that miss QA plan generation.

1. **Invoke `/implement-adr`** with all Ready ADRs. Let it run its full procedure (plan → review → QA → execute → finalize). If ADRs have ordering dependencies, convey them in the delegation prompt so implement-adr can sequence stages correctly.

2. **Check result:**
   - All ADRs `Accepted` → success
   - Failed or paused → stop, report progress

3. **Check for gaps** — if `/implement-adr` discovers a gap requiring a new ADR:
   - Invoke `/author-adr` to create it
   - Re-invoke `/implement-adr` with the new ADR
   - Resume

**Mandatory safeguards:** Plan review and QA are mandatory within `/implement-adr`. Solve-adr must not bypass them by running implementation directly.

**Session boundaries:** When nearing limits, stop at the current task boundary. Report progress — the user resumes in a new session.

After implementation completes, proceed to the Conclusion sequence (C-1 → C-2 → C-3 → C-4) defined in SKILL.md.
