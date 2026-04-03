# 19. Add problem-first solve task to author-adr workflow

Date: 2026-04-03
Status: Accepted
Last Updated: 2026-04-03
Links:
- [ADR-0015: Add interactive revise task](0015-add-interactive-revise-task-to-author-adr-workflow.md)
- [ADR-0017: Adopt nygard-agent-template as default](0017-adopt-nygard-agent-template-as-default-adr-format.md)

## Context

The author-adr skill's current "create" workflow assumes the user arrives with
a decision already in mind. The typical interaction pattern is:

1. User provides the decision, some background, and alternatives they considered
2. Agent assesses significance, checks readiness, and drafts the ADR
3. Agent creates the ADR file with a definitive title and populated sections

This is a **bottom-up** approach — the user has already done the mental work of
identifying the problem, exploring options, and converging on a solution before
engaging the skill. The skill's role is documentation, not discovery.

This works well for decisions that are already mature, but it leaves a gap for
decisions that start as problems without clear solutions:

1. **No problem-first entry point.** A developer who knows they have an
   architectural problem but hasn't yet identified options has no structured
   way to engage the skill. They must do the exploration work outside the
   skill, then come back to create the ADR after the decision is already
   made — losing the opportunity to capture the exploration process itself.

2. **The Prototype→Proposed gap.** The nygard-agent template (ADR-0017)
   defines status values `Prototype | Proposed | Accepted | ...`, but there
   is no workflow that naturally transitions from Prototype to Proposed. The
   create task produces a Prototype ADR, and the user manually promotes it.
   A solve task could bridge this gap: after the agent derives options and the
   user converges on a decision, the ADR naturally transitions to Proposed.

3. **Requirements emerge during exploration.** In a problem-first workflow,
   requirements are often discovered as options are evaluated — an option
   reveals a constraint that wasn't previously known. The current create
   workflow has no mechanism for iteratively gathering requirements because
   it assumes requirements are known upfront (the START "R" criterion).

4. **Options discovered through dialogue.** When exploring a problem space
   with an agent, new alternatives surface through conversation. The current
   workflow captures only the alternatives the user brings to the table.
   A solve task would let the agent propose options and document them as part
   of the ADR, enriching the Options section with alternatives the user may
   not have considered.

5. **Title is unknown at creation time.** The current create workflow requires
   a title to generate the file (e.g., `0019-use-postgresql.md`). In a
   problem-first workflow, the decision title isn't known until the
   exploration converges. The ADR needs an interim title (e.g.,
   `0019-tbd.md`) that is later renamed when the decision crystallizes.

### Decision Drivers

- **Top-down workflow** — developers should be able to start from a problem
  and let the workflow drive toward a solution, not just document decisions
  they've already made.
- **Capture the exploration** — the process of discovering options and
  evaluating tradeoffs is valuable architectural knowledge that is currently
  lost.
- **Bridge Prototype→Proposed** — provide a natural workflow that transitions
  an ADR from exploratory to ready-for-review.
- **Iterative requirements** — support requirements discovery during option
  evaluation, not just upfront.

## Options

### Option 1: Add a dedicated "solve" task to the author-adr workflow

Add a new top-level task alongside create, review, revise, and manage. The
solve task starts from a problem statement and drives through a structured
exploration:

1. **Problem intake** — user describes the problem (not the decision). The
   agent creates an ADR with an interim title (`NNNN-tbd.md`) and populates
   the Context section with the problem statement.

2. **Option discovery** — the agent proposes candidate solutions based on the
   problem context. The user can accept, reject, or add their own options.
   Each option is documented in the Options section as it emerges.

3. **Requirements refinement** — as options are evaluated, the agent surfaces
   constraints and requirements that emerge. These are folded back into the
   Context section, enriching the problem description iteratively.

4. **Optional prototyping** — for options that need validation, the agent can
   create lightweight prototypes (code spikes, config examples) to test
   feasibility before committing to a decision. This fills the Prototype
   status gap naturally.

5. **Convergence** — once the user selects a preferred option, the agent:
   - Populates the Decision section with a Y-statement justification
   - Populates Consequences based on the explored tradeoffs
   - Renames the ADR file from `NNNN-tbd.md` to `NNNN-<decision-title>.md`
   - Transitions the status from Prototype to Proposed

**Strengths:**
- Clean separation of concerns — solve is distinct from create
- Full lifecycle coverage: solve → review → revise → accept
- Interim title mechanism keeps the file system tidy

**Weaknesses:**
- New task to implement, test, and document
- Requires scripting changes to support interim titles and renames
- Overlaps with the "create" task for users who arrive with partial context

### Option 2: Extend the "create" task with an optional problem-first mode

Add a mode flag to the existing create workflow. When invoked without a
decision title (e.g., "create an ADR for a problem I have"), the create task
enters problem-first mode and follows a similar exploration flow.

**Strengths:**
- No new task to learn — reuses existing entry point
- Simpler mental model (one task, two modes)

**Weaknesses:**
- Muddies the create task's responsibility — it becomes both documentation
  and exploration
- The create reference (`references/create.md`) grows significantly, mixing
  two different workflows
- Harder to optimize each mode independently

### Option 3: Keep the status quo — explore externally, create after convergence

Users continue to explore options outside the skill (whiteboards, chat,
prototyping) and engage the create task only when the decision is made.

**Strengths:**
- No changes needed
- Users retain full control of exploration process

**Weaknesses:**
- Exploration knowledge is lost — the ADR captures the outcome but not the
  journey
- The Prototype→Proposed gap remains unfilled
- No agent assistance during the most valuable phase of decision-making

## Decision

Add a dedicated **solve** task to the author-adr skill workflow. The solve task
is a new top-level entry point alongside create, review, revise, and manage.

### Workflow

The solve task follows this flow:

1. **Problem intake** — the user describes the problem they need to solve. The
   agent creates an ADR file with an interim title:

   ```
   docs/adr/0019-tbd.md
   ```

   The Context section is populated with the problem statement. Status is set
   to `Prototype`.

2. **Option discovery** — the agent analyzes the problem and proposes candidate
   solutions. The user and agent collaborate through dialogue:
   - Agent proposes options based on problem context and domain knowledge
   - User can accept, reject, refine, or add their own options
   - Each viable option is documented in the Options section with strengths
     and weaknesses
   - Additional options discovered during dialogue are appended

3. **Requirements refinement** — as options are evaluated, new requirements
   and constraints emerge. The agent folds these back into the Context section,
   building a progressively richer problem description. This addresses the
   START "R" gap — requirements are gathered iteratively rather than demanded
   upfront.

4. **Optional prototyping** — for options that need validation, the user can
   ask the agent to create lightweight prototypes. The ADR remains in
   `Prototype` status during this phase. Prototype artifacts are managed
   outside the ADR itself (e.g., in a branch or spike directory).

5. **Convergence** — when the user selects a preferred option, the agent:
   - Drafts the Decision section with a Y-statement justification
   - Drafts the Consequences section based on explored tradeoffs
   - Proposes a final title for the decision
   - Renames the ADR file from `NNNN-tbd.md` to `NNNN-<title>.md`
   - Transitions the status from `Prototype` to `Proposed`

6. **Handoff** — the solve task produces a `Proposed` ADR ready for the
   existing review workflow. The full lifecycle becomes:

   ```
   solve → review → revise → accept → implement
   ```

### Interim Title Convention

ADRs created by the solve task use `-tbd` as the slug until convergence:

```
0019-tbd.md  →  0019-use-postgresql-for-event-storage.md
```

The Makefile needs a new target or parameter to support creating ADRs with
interim titles and renaming them at convergence.

### Integration Point

The solve task slots into SKILL.md's Agent Workflow routing:

```
├─ "I have a problem to solve" ──► Go to: Solving a Problem
├─ "Create an ADR" ──────────────► Go to: Creating an ADR  (existing)
```

### Reference File

The solve workflow should be documented in a new `references/solve.md` file
following the pattern of `references/create.md` and `references/revise.md`.

## Consequences

- **Enables top-down decision-making** — developers can start from a problem
  and let the workflow drive to a solution, capturing the full exploration
  process as architectural knowledge.

- **Bridges the Prototype→Proposed gap** — the solve task provides a natural
  lifecycle that transitions from Prototype (exploring options, optionally
  prototyping) to Proposed (decision made, ready for review), filling the
  status gap identified since ADR-0017.

- **Iterative requirements gathering** — requirements discovered during option
  evaluation are folded back into Context, producing richer problem
  descriptions than the current upfront-only approach.

- **Richer Options sections** — agent-proposed alternatives supplement user-
  provided alternatives, improving the ecADR "Criteria" quality (≥2 genuine
  alternatives compared).

- **Interim title mechanism adds scripting complexity** — the file rename
  from `NNNN-tbd.md` to `NNNN-<title>.md` requires new Makefile targets and
  shell script support. TBD files are transient working state and unlikely to
  be referenced externally, so the rename is low-risk in practice.

- **New reference file** — adds `references/solve.md` to the skill, following
  the pattern established by ADR-0013 (split references into task-specific
  documents).

- **Dialogue-driven ADR authoring is novel** — unlike the create task (which
  is essentially a one-shot draft), the solve task is inherently multi-turn
  and conversational. This requires careful UX design to avoid unbounded
  sessions or context loss in long explorations.

- **Solve and create may overlap** — users with partial context (some options
  but no decision yet) could use either task. The routing guidance in SKILL.md
  must clearly distinguish when to use solve vs. create.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [ ] Backwards Compatible

### Additional Quality Concerns

The solve task introduces a multi-turn conversational workflow that is
fundamentally different from the single-shot create task. Validation will
rely on dogfooding and iterative refinement of the workflow steps. The
interim title rename mechanism needs testing for edge cases (concurrent
ADRs, special characters in titles).

---

## Comments

## Revision Addendum

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Does the consequence about interim title renames overstate the link-breakage risk?

**Addressed** — Softened the wording to remove the external link-breakage concern. TBD files are transient working state; the real complexity is scripting support for the rename, not reference integrity.

### Q: Should the ADR include a revisit date for realization planning?

**Rejected** — Not needed at this stage.
