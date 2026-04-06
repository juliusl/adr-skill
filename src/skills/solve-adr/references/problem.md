# Problem

Self-contained reference for solving a problem. Read this file when the user has a problem — whether new (explore) or partially solved (resume).

**Key relationship:** A problem produces one or more ADRs (one-to-many).

**Resume protocol:** Every solvable thing is resumable. When solve-adr is invoked and the problem already has ADRs, it picks up where it left off — skipping completed steps, implementing remaining ADRs. The resume protocol is not a separate scenario; it's how solve works across sessions.

---

## Lifecycle

```
1. Intake — capture problem, constraints, stakeholders
   ↓
2. Decision loop — for each decision the problem requires:
   │  ├─ /author-adr — create ADR (worksheet → options → convergence)
   │  ├─ /prototype-adr — if Evaluation Checkpoint needs validation
   │  └─ /author-adr — review → revise cycle
   │  (repeat if the problem requires additional decisions)
   ↓
3. Implement — group the produced ADRs, delegate to /implement-adr
   ↓
4. Report — summarize what was implemented, what remains
```

**On resume:** The agent evaluates the problem's current state and enters the lifecycle at the right point:
- No ADRs exist → start at step 1 (intake)
- ADRs exist but some are still Proposed (not reviewed) → enter step 2 (decision loop) for unfinished ADRs
- All ADRs are Proposed/reviewed but unimplemented → enter step 3 (implement)
- Some ADRs are Accepted, others remain → enter step 3 for the remaining ones

---

## Step 1: Problem Intake

Capture the problem clearly. Do not jump to solutions.

**Draft Worksheet Integration:** If a Draft Worksheet already exists in an ADR's `## Comments` section (per ADR-0032), use it to accelerate intake:
- **Framing** → use as the initial problem statement
- **Uncertainty** → focus probing questions on the uncertain areas

If no worksheet exists:

1. **Gather the problem statement** — ask the user to describe the problem. Probe for:
   - What system or component is affected?
   - What triggered this problem? (new requirement, pain point, tech debt)
   - What constraints are already known?
   - Who are the stakeholders?

2. **Confirm the problem statement** — "Does this capture the problem accurately?"

## Step 2: Decision Loop

A single problem may require multiple decisions. Each iteration produces one reviewed ADR. The loop repeats until all decisions needed to address the problem are captured.

### Per-decision iteration:

**1. Invoke `/author-adr`** to create the ADR end-to-end.

What to provide:
- The problem statement, constraints, and stakeholders from step 1
- The user's candidates and thought process (these seed the Draft Worksheet)
- Direction: "create an ADR to address [specific aspect of the problem]"

What `/author-adr` does: Runs its full A-0 → A-1 → A-2 → A-3 procedure:
1. Creates a TBD ADR with draft worksheet populated from the problem context
2. Explores options using the worksheet's candidates and tolerance settings
3. Refines requirements as options are evaluated
4. Runs the Evaluation Checkpoint — if "Pause for validation," solve-adr invokes `/prototype-adr`
5. Converges on a decision, drafts Decision + Consequences, renames the ADR, transitions to Proposed
6. Reviews and revises the ADR (A-3 → A-4 → A-5)

**2. If Evaluation Checkpoint says "Pause for validation"** — invoke `/prototype-adr` with the validation needs, then return to `/author-adr` to complete convergence.

**3. After review completes** — check: does the problem require additional decisions?

Signals that another decision is needed:
- The ADR's Consequences mention a deferred decision ("addressed in a separate ADR")
- The problem has aspects not covered by the current ADR's scope
- The user identifies additional decisions during the exploration

If yes → start a new iteration with the next decision's scope.
If no → proceed to step 3.

**Tracking:** Keep a running list of all ADRs produced:
```
[ADR ref]: [first decision title] (Proposed, reviewed)
[ADR ref]: [second decision title] (Proposed, reviewed)
```

## Step 3: Implement

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

**Single-ADR case:** If only one ADR remains, step 3 simplifies to a single `/implement-adr` invocation.

**Session boundaries:** When nearing limits, stop at the current group boundary. Report progress — the user resumes in a new session.

## Step 4: Report

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
