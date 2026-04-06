# Problem

Self-contained reference for solving a problem. Read this file when the user has a problem — whether new (explore) or partially solved (resume).

**Key relationship:** A problem produces one or more ADRs (one-to-many).

**Resume protocol:** Every solvable thing is resumable. When solve-adr is invoked and the problem already has ADRs, it picks up where it left off — skipping completed steps, implementing remaining ADRs. The resume protocol is not a separate scenario; it's how solve works across sessions.

---

## Lifecycle

```
1. Intake — capture problem, constraints, stakeholders, enumerate decisions needed
   ↓
2. Author — load /author-adr context and run its procedure for all decisions
   ↓
3. Triage — review returned ADRs, route evaluation-checkpoint-paused ones to /prototype-adr
   ↓
4. Implement — group accepted ADRs, load /implement-adr and run its procedure
   ↓
5. Report — summarize what was implemented, what remains
```

**On resume:** The agent evaluates the problem's current state and enters the lifecycle at the right point:
- No ADRs exist → start at step 1 (intake)
- ADRs exist but some are still TBD (not converged) → enter step 2 with remaining decisions
- ADRs exist but some paused at Evaluation Checkpoint → enter step 3 (triage)
- All ADRs are Proposed/reviewed but unimplemented → enter step 4 (implement)
- Some ADRs are Accepted, others remain → enter step 4 for the remaining ones

---

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

## Step 2: Author

Load `/author-adr` context and run its procedure for all decisions in a single invocation. The `skill: "author-adr"` call loads author-adr's SKILL.md into the current conversation — the orchestrating agent executes author-adr's procedure itself. There is no separate agent; solve-adr IS author-adr during this phase.

### What to provide

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

### What `/author-adr` does

For each decision in the list, author-adr runs its full procedure:
1. Creates a TBD ADR with draft worksheet populated from the problem context
2. Explores options using the worksheet's candidates and tolerance settings
3. Refines requirements as options are evaluated
4. Runs the Evaluation Checkpoint — marks ADRs that need validation
5. Converges on a decision, drafts Decision + Consequences, renames the ADR, transitions to Proposed
6. Reviews and revises the ADR (A-3 → A-4 → A-5)

Author-adr may process these sequentially or batch internally — the ordering is at the agent's discretion.

### After author-adr returns

Author-adr returns control with a set of ADRs in various states. Proceed to step 3 (triage).

## Step 3: Triage

After author-adr returns, classify each ADR's state and take action:

| ADR State | Action |
|-----------|--------|
| Proposed (reviewed, accepted) | Ready for implementation → step 4 |
| Proposed (reviewed, needs revision) | Author-adr should have handled this — if not, re-invoke to complete A-4/A-5 |
| Paused at Evaluation Checkpoint | Invoke `/prototype-adr` with the ADR's validation needs |
| TBD (incomplete) | Re-invoke `/author-adr` to complete remaining decisions |

### Prototype routing

For ADRs paused at the Evaluation Checkpoint ("Pause for validation"):

1. **Invoke `/prototype-adr`** with:
   - The ADR file path
   - The specific validation needs from the Evaluation Checkpoint
   - Success/failure criteria

2. **After prototype returns** — re-invoke `/author-adr` to complete convergence on the validated ADR. Provide the prototype results so author-adr can incorporate evidence into the decision.

3. **Continue triage** for remaining ADRs.

### Tracking

Keep a running list of all ADRs and their post-triage status:
```
[ADR ref]: [title] — Proposed, ready for implementation
[ADR ref]: [title] — Proposed, ready for implementation
[ADR ref]: [title] — Paused, prototype needed for [validation need]
```

Once all ADRs are either Proposed (ready) or blocked, proceed to step 4 with the ready ones.

## Step 4: Implement

After all decisions are made (or on resume when decisions already exist), implement them.

### Survey (on resume)

When resuming, invoke `/author-adr` to find the problem's ADRs and their current state:

1. **Identify the problem's ADRs** — from the user's request:
   - Explicit: "implement ADRs [these specific ones]"
   - By topic: "solve remaining ADRs related to [topic]" → invoke `/author-adr` to list and filter
   - Continuation: "continue solving" → find Proposed ADRs linked to a common theme

2. **Partition** — separate done (Accepted) from remaining (Proposed/Planned). Only remaining ADRs need implementation.

### Group for delegation

Determine how to delegate remaining ADRs to `/implement-adr`. The goal is to minimize invocations — each creates one plan with one review and one QA cycle.

1. **Build dependency graph** — from each ADR's Links and Context, identify ordering constraints.

2. **Group remaining ADRs:**
   - ADRs with no ordering constraints → same group
   - Hard dependency between groups → separate groups (ordered)
   - Prefer one large group — only split when ordering requires it

3. **Present the groups:**
   ```
   Group 1: ADR-NNNN, ADR-NNNN, ADR-NNNN — [what they cover]
   Group 2: ADR-NNNN, ADR-NNNN (depends on Group 1) — [what they cover]
   Proceed? [Yes / Adjust]
   ```
   In autonomous mode, proceed without confirmation.

**Why grouping matters:** `/implement-adr` handles multi-ADR plans natively — one plan, one review, one QA per group. Delegating individually wastes context and creates pressure to skip safeguards.

### Delegate

Delegate each group to `/implement-adr`. Solve-adr's role is handoff and progress tracking.

For each group in order:

1. **Invoke `/implement-adr`** with all ADRs in the group. Let it run its full procedure (plan → review → QA → execute → finalize).

2. **Check result:**
   - All ADRs `Accepted` → success, continue to next group
   - Failed or paused → stop, report progress

3. **Check for gaps** — if `/implement-adr` discovers a gap requiring a new ADR:
   - Invoke `/author-adr` to create it
   - Add to current or next group
   - Resume

**Mandatory safeguards:** Plan review and QA are mandatory within `/implement-adr`. Solve-adr must not bypass them by running implementation directly.

**Single-ADR case:** If only one ADR remains, step 4 simplifies to a single `/implement-adr` invocation.

**Session boundaries:** When nearing limits, stop at the current group boundary. Report progress — the user resumes in a new session.

## Step 5: Report

After all groups complete (or execution stops):

```markdown
## Problem: [topic]

| Group | ADRs | Status | Result |
|-------|------|--------|--------|
| 1 | ADR-NNNN, ADR-NNNN | Accepted | ✅ Completed |
| 2 | ADR-NNNN | Proposed | ⏳ Next up |

**Completed:** N of M ADRs
**Remaining:** [list]
**Blocked:** None
```

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
