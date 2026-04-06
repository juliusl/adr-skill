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

## S-1.2: Create ADR via /author-adr

Invoke `/author-adr` to create the ADR end-to-end. Author-adr's create workflow handles option discovery, requirements refinement, evaluation checkpoint, and convergence internally — solve-adr does not need to manage these steps individually.

**What to provide /author-adr:**
- The problem statement, constraints, and stakeholders from S-1.1
- The user's candidates and thought process (these seed the Draft Worksheet)
- Direction: "create an ADR to address this problem" — author-adr's A-1 (worksheet) + A-2 (create) handles the rest

**What /author-adr does:** Runs its full A-0 → A-1 → A-2 procedure:
1. Creates a TBD ADR with draft worksheet populated from the problem context
2. Explores options using the worksheet's candidates and tolerance settings
3. Refines requirements as options are evaluated
4. Runs the Evaluation Checkpoint — if it says "Pause for validation," solve-adr resumes at S-1.3
5. Converges on a decision, drafts Decision + Consequences, renames the ADR, transitions to Proposed

**After /author-adr returns:** Note the ADR number. Check the Evaluation Checkpoint — if "Pause for validation," proceed to S-1.3. Otherwise, skip to S-1.4.

## S-1.3: Validate via /prototype-adr (conditional)

If the Evaluation Checkpoint says "Pause for validation," invoke `/prototype-adr` with the validation needs from the checkpoint.

**After /prototype-adr returns:** Findings are recorded in the ADR. Return to `/author-adr` to update the checkpoint to "Proceed" and complete convergence if needed.

## S-1.4: Review via /author-adr

## S-1.4: Review via /author-adr

Invoke `/author-adr` to run the structured review → revise cycle on the completed ADR. The review checks for completeness, reasoning fallacies, and anti-patterns.

**What happens:** /author-adr runs its A-3 (Review) → A-4 (Revise) → A-5 (Re-review) procedure using the configured review and editor agents.

**After review completes:** The ADR is `Proposed` and reviewed. Resume at S-1.5.

## S-1.5: Handoff

The solve scenario produces a `Proposed`, reviewed ADR.

1. **Present the completed ADR** — summarize the decision and its key tradeoffs.

2. **Offer implementation** — if `auto_delegate` is true (from preferences), invoke `/implement-adr` automatically. Otherwise, ask:

   > This ADR is reviewed and `Proposed`. Would you like to implement it?

   If the user agrees, invoke `/implement-adr` with the ADR number.

3. **Full lifecycle:**

   ```
   solve (S-1) → author (create + review) → implement → accept
   ```

## Multi-Turn Session Management

The solve workflow is inherently multi-turn and conversational. Keep these practices in mind:

- **Summarize progress between turns** — at the start of each response, briefly state where in the workflow you are (e.g., "We have 3 options documented. Let's evaluate them against the requirements.").
- **Preserve the ADR as the source of truth** — write findings to the ADR file as they emerge, not just at the end. If context is lost, the ADR file contains the current state.
- **Don't rush convergence** — the value of the solve task is in the exploration. Let the user drive the pace. If they're not ready to decide, stay in option discovery or prototyping.
- **Keep options open** — avoid signaling a preference for one option unless asked. Present tradeoffs neutrally.
