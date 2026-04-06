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
S-1.1: Problem intake
  ↓
S-1.2: /author-adr — create TBD ADR with draft worksheet
  ↓
S-1.3: Option discovery
  ↓
S-1.4: Requirements refinement
  ↓
S-1.5: Evaluation checkpoint — /prototype-adr if validation needed
  ↓
S-1.6: Convergence — /author-adr drafts Decision + Consequences
  ↓
S-1.7: /author-adr review → revise cycle
  ↓
S-1.8: Handoff — offer /implement-adr
```

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

Invoke `/author-adr` to create the ADR that will hold this decision.

**What to tell /author-adr:**
- Create a TBD ADR (`TITLE="tbd"`)
- Fill the Draft Worksheet from the problem intake conversation (S-1.1)
- Populate the Context section with the problem statement, constraints, and background

**What /author-adr does:** Runs its A-0 → A-1 → A-2 (create) procedure. The output is an ADR file with `Status: Prototype`, a populated Context section, and a Draft Worksheet.

**After /author-adr returns:** Note the ADR number and file path. Resume the solve scenario at S-1.3.

## S-1.3: Option Discovery

The agent and user collaborate to identify candidate solutions. This is the core exploration phase.

### Draft Worksheet Integration

If a **Draft Worksheet** exists, use it to scope discovery:

- **Target count** → aim for this many options (e.g., "2-3" or "open").
- **Explore additional** → if checked, discover options beyond the pre-identified Candidates. If unchecked, focus on evaluating the Candidates only.
- **Candidates** → start with these as initial options instead of proposing from scratch.
- **Improvisation tolerance** → controls how far the agent diverges from the author's Framing during option discovery. Low = stay close to the framing; High = challenge the framing and explore unconventional approaches.

If no worksheet exists, proceed with the standard discovery below.

1. **Agent proposes options** — based on the problem context and domain knowledge, propose 2–4 candidate solutions. For each option, provide:
   - A short title (noun phrase)
   - A 2–3 sentence description
   - Initial strengths and weaknesses

2. **User collaborates** — the user can:
   - Accept an option as a viable candidate
   - Reject an option (with reason — this is valuable knowledge)
   - Refine an option (modify scope, combine approaches)
   - Add their own options

3. **Document options** — as options emerge, write them into the ADR's Options section:

   ```markdown
   ### Option N: [Title]

   [Description]

   **Strengths:**
   - ...

   **Weaknesses:**
   - ...
   ```

4. **Aim for ≥3 genuine options** — the implementability criteria require at least 2 alternatives compared. Three or more options produce richer analysis.

### Discovery Techniques

- **Analogical reasoning** — what have similar projects done?
- **Constraint relaxation** — what if we removed constraint X?
- **Decomposition** — can the problem be split into sub-problems with different solutions?
- **Inversion** — what would make this problem worse? Avoid those approaches.

## S-1.4: Requirements Refinement

As options are evaluated, new requirements and constraints emerge. This is expected and valuable — it enriches the problem description iteratively.

1. **Surface emergent requirements** — when evaluating an option reveals a constraint that wasn't previously known, call it out explicitly:

   > "Evaluating Option 2 reveals that we need sub-millisecond reads. Adding this to the requirements."

2. **Update the Context section** — fold new requirements back into the Context section. Add them under a `### Decision Drivers` subsection if one doesn't exist:

   ```markdown
   ### Decision Drivers

   - **Sub-millisecond reads** — discovered during Option 2 evaluation
   - **Must support multi-region** — existing requirement
   ```

3. **Re-evaluate options** — after adding new requirements, briefly reassess whether existing options still meet them. An option that was viable before a new requirement may no longer be.

## S-1.5: Evaluation Checkpoint

After documenting options and refining requirements, pause at the ADR's **Evaluation Checkpoint (Optional)** section (per ADR-0024).

1. **Assess the options holistically** — evaluate whether the analysis is sufficient to support a decision. Write the Assessment value:
   - `Proceed` — analysis is sufficient, move to convergence.
   - `Pause for validation` — experiments would strengthen confidence. Populate the **Validation needs** area.
   - `Skipped — <rationale>` — the user chooses to proceed on intuition.

2. **If "Pause for validation"** — invoke `/prototype-adr` to run experiments. The Validation needs become the prototype objectives. Ask the user:
   - Which validation items to pursue now vs. defer?
   - What isolation backend is appropriate (worktree for quick spikes, container for reproducible benchmarks)?

3. **After /prototype-adr completes** — findings are recorded in the ADR's option Strengths/Weaknesses. Update the Assessment to `Proceed` and resume at S-1.6.

## S-1.6: Convergence

When the user selects a preferred option, invoke `/author-adr` to finalize the ADR.

**What to tell /author-adr:**
- Draft the Decision section using a Y-statement justification
- Draft the Consequences section from the explored tradeoffs
- Propose a final title based on the chosen option
- Rename the ADR from "tbd" to the final title
- Transition status from `Prototype` to `Proposed`

**Y-statement template:**
> In the context of **{problem}**, facing **{key concern}**, we decided for **{chosen option}** and neglected **{alternatives}**, to achieve **{benefits}**, accepting that **{drawbacks}**.

## S-1.7: Review and Revise

Invoke `/author-adr` to run the structured review → revise cycle on the completed ADR. The review checks for completeness, reasoning fallacies, and anti-patterns.

**What happens:** /author-adr runs its A-3 (Review) → A-4 (Revise) → A-5 (Re-review) procedure using the configured review and editor agents.

**After review completes:** The ADR is `Proposed` and reviewed. Resume at S-1.8.

## S-1.8: Handoff

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
