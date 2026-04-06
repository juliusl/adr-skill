# 3. Implement-adr skill updates ADR status during planning

Date: 2026-04-01

## Status

Accepted

## Context

The `implement-adr` skill consumes ADRs and produces implementation plans
(`plan.md`), but it currently has no mechanism to reflect planning progress back
into the ADR decision log. Once a plan is generated, the source ADRs remain in
their original status (typically `Proposed`), creating a disconnect between the
planning workflow and the decision lifecycle.

This matters because:

- **Status drift** — an ADR can sit at `Proposed` indefinitely even after
  detailed planning work has been completed, giving a misleading picture of
  project progress.
- **No signal that planning occurred** — other team members reading the ADR
  decision log have no way to know that an ADR has been decomposed into tasks
  and is ready for implementation.
- **The ADR lifecycle has well-defined statuses** — Nygard-format ADRs support
  status progression (Proposed → Accepted → Deprecated/Superseded). The
  transition from `Proposed` to a planning-in-progress state is a natural
  extension of this lifecycle.
- **Final acceptance should be explicit** — after implementation tasks are
  complete, the ADR should move to `Accepted` to signal that the decision has
  been fully realized. Including this as the final step in the plan keeps
  acceptance tied to delivery rather than being a forgotten manual step.

## Decision

We will update the `implement-adr` skill to manage ADR status transitions as
part of its planning workflow. Specifically:

### 1. Transition to "Planned" on plan creation

When the `implement-adr` skill generates a `plan.md` for one or more ADRs, it
will update each source ADR's status from `Proposed` to `Planned`. This
transition signals that the decision has been analyzed, decomposed into tasks,
and is ready for implementation.

The status update is performed by editing the ADR file's `## Status` section
in-place:

```
## Status

Planned

```

**Guard rails:**
- Only ADRs with status `Proposed` are transitioned to `Planned`. ADRs that are
  already `Accepted`, `Deprecated`, or `Superseded` are left unchanged.
- The skill will warn the user if an ADR is in an unexpected status and ask
  whether to proceed.

### 2. Include a final plan step to transition to "Accepted"

The generated `plan.md` will include a final task — after all implementation
stages — that updates each relevant ADR's status from `Planned` to `Accepted`.
This task ensures that acceptance is an explicit, traceable step tied to
implementation completion rather than a manual afterthought.

The final task will follow this structure:

```markdown
### Stage N: Finalize

#### Task N.1: Update ADR status to Accepted    [small]

**Description:** Update the status of each source ADR from `Planned` to
`Accepted` to reflect that the decision has been fully implemented.

**Files to update:**
- `docs/adr/XXXX-<title>.md` — change `## Status` from `Planned` to `Accepted`

**Acceptance Criteria:**
- [ ] Each source ADR status reads `Accepted`
- [ ] No other ADR content is modified
```

### 3. Status lifecycle summary

The full status lifecycle managed across both skills:

```
author-adr creates    implement-adr plans    implement-adr final step
    ADR ──────────► Proposed ──────────► Planned ──────────► Accepted
```

## Consequences

**Positive:**

- ADR statuses accurately reflect planning progress, giving the decision log a
  clear signal of which decisions have been acted upon.
- The `Planned` status provides a distinct checkpoint between "decision made"
  and "decision implemented," improving visibility for team members reviewing
  the decision log.
- Including the `Accepted` transition as an explicit plan step ensures it is
  tracked, reviewable, and not forgotten.
- Aligns with the natural Nygard ADR lifecycle and extends it with a useful
  intermediate state.

**Negative / Risks:**

- Introduces a new status value (`Planned`) that is not part of the original
  Nygard specification. Tools that only recognize `Proposed`, `Accepted`,
  `Deprecated`, and `Superseded` may not understand it. Mitigated by the fact
  that ADR statuses are free-text by convention and tooling in this repo already
  handles arbitrary status values.
- The skill now modifies ADR files as a side effect of plan generation, which
  changes its write surface from "only creates `plan.md`" to "creates `plan.md`
  and edits ADR files." Users who want a read-only planning workflow must
  explicitly opt out.

**Neutral:**

- The `implement-adr` skill already reads ADR files during planning; this change
  adds a write step but does not alter the read/analysis workflow.

---

## Comments

<!-- No review cycle on record. This ADR predates the formal review process. -->
