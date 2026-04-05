# Solving a Problem

Self-contained reference for the problem-first solve workflow. Read this file when the user has a problem to solve but hasn't yet identified a decision or chosen a solution.

## When to Use

Activate the solve workflow when:
- The user describes a **problem** rather than a **decision** ("I need to figure out how to handle X", "what's the best approach for Y")
- The user wants to **explore options** before committing to a solution
- The user has partial context — some ideas but no clear winner
- The user explicitly says "solve," "help me decide," or "explore options"

**Solve vs. Create:** If the user arrives with a decision already made ("Use PostgreSQL for event storage"), use the **create** workflow. If they arrive with a problem ("We need persistent event storage but I'm not sure what to use"), use **solve**.

## Workflow Overview

The solve task follows six steps, producing a `Proposed` ADR ready for review:

```
Problem intake → Option discovery → Requirements refinement
→ Optional prototyping → Convergence → Handoff
```

## Step 1: Problem Intake

The user describes the problem they need to solve. The agent's role is to capture the problem clearly, not to jump to solutions.

### Draft Worksheet Integration

If a **Draft Worksheet** exists in the ADR's `## Comments` section (per ADR-0032), use it to accelerate intake:

- **Framing** → use as the initial problem statement instead of starting from scratch. The user has already articulated their core idea.
- **Uncertainty** → distinguish what the user knows for certain from what they're unsure about. Focus probing questions on the uncertain areas.

If no worksheet exists, proceed with the standard intake below.

1. **Gather the problem statement** — ask the user to describe the problem in their own words. Probe for:
   - What system or component is affected?
   - What triggered this problem? (new requirement, pain point, tech debt)
   - What constraints are already known?
   - Who are the stakeholders?

2. **Create a TBD ADR** — use the Makefile to create an ADR with an interim title:

   ```bash
   make -f <skill-root>/Makefile new TITLE="tbd"
   ```

   This creates `NNNN-tbd.md` with `Status: Prototype`.

3. **Populate the Context section** — write the problem statement into the Context section. Include:
   - The problem description (what, why, who)
   - Known constraints and requirements
   - Any relevant background or prior decisions

   Do **not** populate the Options, Decision, or Consequences sections yet.

4. **Confirm the problem statement** — present the Context section to the user and ask: "Does this capture the problem accurately?"

## Step 2: Option Discovery

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

3. **Document options** — as options emerge, write them into the ADR's Options section using the standard structure:

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

## Step 3: Requirements Refinement

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

## Step 4: Evaluation Checkpoint

After documenting options and refining requirements, pause at the **Evaluation Checkpoint (Optional)** section (per ADR-0024). This replaces the unstructured "Optional Prototyping" step with a procedural assessment.

1. **Assess the options holistically** — evaluate whether the analysis is sufficient to support a decision. Write the Assessment value:
   - `Proceed` — analysis is sufficient, move to convergence.
   - `Pause for validation` — experiments would strengthen confidence. Populate the **Validation needs** area with specific prototypes, benchmarks, or evidence needed.
   - `Skipped — <rationale>` — the user chooses to proceed on intuition.

2. **If "Pause for validation"** — the Validation needs become inputs for the `prototype-adr` skill (ADR-0023). Ask the user:
   - Which validation items to pursue now vs. defer?
   - What isolation backend is appropriate (worktree for quick spikes, container for reproducible benchmarks)?

3. **Run targeted experiments** — if the user opts to validate:
   - Create disposable prototypes that answer specific questions.
   - Record findings in the relevant option's Strengths/Weaknesses.
   - The ADR status remains `Prototype` during this phase.

4. **When validation completes** — update the Assessment to `Proceed` and check the baseline items. Move to convergence.

## Step 5: Convergence

When the user selects a preferred option, the agent finalizes the ADR.

1. **Confirm the decision** — ask the user which option they prefer and why. The "why" is essential for the justification.

2. **Draft the Decision section** — write a Y-statement justification:

   > In the context of **{problem}**, facing **{key concern}**, we decided for
   > **{chosen option}** and neglected **{alternatives}**, to achieve
   > **{benefits}**, accepting that **{drawbacks}**.

   Follow with any implementation details or constraints on the decision.

3. **Draft the Consequences section** — based on the explored tradeoffs, write consequences. Include both positive and negative consequences. Draw from the option's documented strengths and weaknesses.

4. **Propose a final title** — suggest a decision title based on the chosen option (e.g., "Use PostgreSQL for event storage"). Confirm with the user.

5. **Rename the ADR** — once the title is confirmed, rename from the interim title:

   ```bash
   make -f <skill-root>/Makefile rename NUM=<n> TITLE="Use PostgreSQL for event storage"
   ```

   This renames the file and updates the heading.

6. **Transition status** — update the status from `Prototype` to `Proposed`:

   ```bash
   make -f <skill-root>/Makefile status NUM=<n> STATUS=Proposed
   ```

## Step 6: Handoff

The solve task produces a `Proposed` ADR ready for the existing review workflow.

1. **Present the completed ADR** — show the user the final ADR content.

2. **Recommend review** — offer to run the structured review:

   > This ADR is now `Proposed`. Would you like to review it for completeness,
   > reasoning fallacies, and anti-patterns?

   If the user agrees, hand off to the [Reviewing an ADR](../SKILL.md#reviewing-an-adr) workflow.

3. **Full lifecycle** — the solve task slots into the overall ADR lifecycle:

   ```
   solve → review → revise → accept → implement
   ```

   The `accept` and `implement` steps are performed by the `implement-adr` skill, not `author-adr`. The author-adr skill caps at `Proposed` status.

## Multi-Turn Session Management

The solve workflow is inherently multi-turn and conversational, unlike the single-shot create task. Keep these practices in mind:

- **Summarize progress between turns** — at the start of each response, briefly state where in the workflow you are (e.g., "We have 3 options documented. Let's evaluate them against the requirements.").
- **Preserve the ADR as the source of truth** — write findings to the ADR file as they emerge, not just at the end. If context is lost, the ADR file contains the current state.
- **Don't rush convergence** — the value of the solve task is in the exploration. Let the user drive the pace. If they're not ready to decide, stay in option discovery or prototyping.
- **Keep options open** — avoid signaling a preference for one option unless asked. Present tradeoffs neutrally.
