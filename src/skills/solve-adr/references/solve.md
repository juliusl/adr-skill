# S-1: Problem Exploration

Self-contained reference for the Problem Exploration scenario. Read this file when the user has a problem to solve but hasn't yet identified a decision or chosen a solution.

## When to Use

Activate S-1 when:
- The user describes a **problem** rather than a **decision** ("I need to figure out how to handle X", "what's the best approach for Y")
- The user wants to **explore options** before committing to a solution
- The user has partial context — some ideas but no clear winner
- The user explicitly says "solve," "help me decide," or "explore options"

**Solve vs. Create:** If the user arrives with a decision already made ("Use PostgreSQL for event storage"), redirect to `/author-adr` (create workflow). If they arrive with a problem ("We need persistent event storage but I'm not sure what to use"), use S-1.

## Workflow Overview

```
S-1.1: Problem intake — solve-adr captures problem, constraints, stakeholders
  ↓
S-1.2: /author-adr — create ADR end-to-end (worksheet → options → convergence)
  ↓
S-1.3: /prototype-adr — if Evaluation Checkpoint needs validation
  ↓
S-1.4: /author-adr — review → revise cycle
  ↓
S-1.5: Handoff — /implement-adr for execution
```

**Ownership model:** Solve-adr owns the problem intake (S-1.1) and orchestration (S-1.3–S-1.5). Option discovery, requirements refinement, evaluation, and convergence happen within `/author-adr`'s create workflow (S-1.2). Author-adr's A-1 (draft worksheet) and A-2 (create) procedure handles these steps using the problem context and draft worksheet from S-1.1.

## S-1.1: Problem Intake

Capture the problem clearly. Do not jump to solutions.

### Draft Worksheet Integration

If a **Draft Worksheet** already exists in an ADR's `## Comments` section (per ADR-0032), use it to accelerate intake:

- **Framing** → use as the initial problem statement instead of starting from scratch. The user has already articulated their core idea.
- **Uncertainty** → distinguish what the user knows for certain from what they're unsure about. Focus probing questions on the uncertain areas.

If no worksheet exists, proceed with the standard intake below.

1. **Gather the problem statement** — ask the user to describe the problem in their own words. Probe for:
   - What system or component is affected?
   - What triggered this problem? (new requirement, pain point, tech debt)
   - What constraints are already known?
   - Who are the stakeholders?

2. **Confirm the problem statement** — present your understanding back to the user: "Does this capture the problem accurately?"

## S-1.2: Decision Loop

A single problem may require multiple decisions. Each iteration through this loop produces one reviewed ADR. The loop repeats until all decisions needed to address the problem are captured.

### Per-decision iteration:

**1. Invoke `/author-adr`** to create the ADR end-to-end. Author-adr's create workflow handles option discovery, requirements refinement, evaluation checkpoint, and convergence internally.

**What to provide /author-adr:**
- The problem statement, constraints, and stakeholders from S-1.1
- The user's candidates and thought process (these seed the Draft Worksheet)
- Direction: "create an ADR to address [specific aspect of the problem]"

**What /author-adr does:** Runs its full A-0 → A-1 → A-2 → A-3 procedure:
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
If no → proceed to S-1.3.

**Tracking:** Keep a running list of all ADRs produced during the loop:
```
[ADR ref]: [first decision title] (Proposed, reviewed)
[ADR ref]: [second decision title] (Proposed, reviewed)
[ADR ref]: [third decision title] (Proposed, reviewed)
```

The ADR reference format depends on the active format — `/author-adr` determines the correct naming convention.

## S-1.3: Implement

After all decisions are made, implement them in dependency order.

1. **Analyze dependencies** — from each ADR's Links section and Context references, determine which ADRs depend on which. Order them so dependencies are implemented before dependents.

2. **Present implementation plan** — show the ordered chain:
   ```
   Implementation order:
   1. [ADR ref] (foundation) — no dependencies
   2. [ADR ref] (builds on #1) — depends on #1
   3. [ADR ref] (builds on #2) — depends on #2
   ```
   In autonomous mode, proceed without confirmation. In guided mode, confirm with the user.

3. **Delegate to `/implement-adr`** for each ADR in order. After each completes:
   - If status is `Accepted` → success, continue to next
   - If implementation failed → stop, report progress
   - If a gap is discovered → invoke `/author-adr` for the new decision, add to chain, resume

4. **Report progress** after each implementation and at the end:
   ```
   ✅ [ADR ref]: Accepted
   ✅ [ADR ref]: Accepted
   🔄 [ADR ref]: Implementing...
   ```

**Single-ADR case:** If the decision loop produced only one ADR, S-1.3 simplifies to a single `/implement-adr` invocation (no dependency analysis needed).

**Session boundaries:** If the session is nearing its limits, stop at the current ADR boundary. Report what was completed and what remains — the user can resume with `/solve-adr continue` in a new session.

## Multi-Turn Session Management

The solve workflow is inherently multi-turn and conversational. Keep these practices in mind:

- **Summarize progress between turns** — at the start of each response, briefly state where in the workflow you are (e.g., "We have 3 options documented. Let's evaluate them against the requirements.").
- **Preserve the ADR as the source of truth** — write findings to the ADR file as they emerge, not just at the end. If context is lost, the ADR file contains the current state.
- **Don't rush convergence** — the value of the solve task is in the exploration. Let the user drive the pace. If they're not ready to decide, stay in option discovery or prototyping.
- **Keep options open** — avoid signaling a preference for one option unless asked. Present tradeoffs neutrally.
