# 6. Implement-adr checks user participation level after planning

Date: 2026-04-01

## Status

Superseded by [ADR-0007](0007-weighted-participation-mode-based-on-task-cost.md)

## Context

The `implement-adr` skill generates a structured implementation plan and
presents it to the user for review (Step 5 — Review and Iterate). However, the
current workflow assumes a single interaction model: present the plan and ask
if any stages or tasks need adjustment. This does not account for the range of
user preferences:

- **Some users want full control** — they want to review every stage, select
  which to start with, and approve each change before proceeding.
- **Some users want to delegate** — they want the agent to execute the plan
  autonomously, checking in only at major milestones.
- **Some users want something in between** — a summary and a chance to redirect,
  but not granular stage-by-stage approval.

Without asking, the skill must guess — and any default will be wrong for some
users. Additionally, the chosen participation level is itself an architectural
decision about the team's workflow with the skill. This decision should be
captured as an ADR so that future sessions respect the same preference.

A secondary concern is how revisions are handled. When the user requests
changes to a plan, the current workflow overwrites the plan in-place. Per
ADR-0005, plans now live in `docs/plans/` with revision numbering. The
revision workflow needs to be integrated with the participation check: when
changes are requested, a new revision file should be created that documents
what changed.

Finally, the skill should be resilient when the `author-adr` skill is not
available. If the user's participation preference cannot be captured as a
formal ADR, the agent should fall back to reasonable defaults and, if
possible, still record the decision using the conventions observed in the
existing ADR directory.

## Decision

We will update the `implement-adr` skill to ask the user about their desired
participation level after the plan has been written, and to capture that
decision as an ADR when possible.

### 1. Participation check after plan creation

After writing the plan (Step 4 — Update ADR Statuses), the skill will ask the
user how much involvement they want during implementation:

> How much participation would you like during implementation?
> 1. **Full control** — I'll review each stage and select what to start
> 2. **Guided** — Summarize the plan, let me pick stages or request changes
> 3. **Autonomous** — Execute the plan, check in at major milestones

### 2. Behavior by participation level

| Level | Behavior |
|-------|----------|
| **Full control** | Present each stage individually. Wait for explicit approval before starting each stage. After each stage, ask which stage to proceed with next. |
| **Guided** (default) | Summarize the full plan. Ask if the user wants changes or which stages to start with. Proceed with approved stages, reporting back at stage boundaries. |
| **Autonomous** | Execute all stages in order. Report progress at stage boundaries but do not wait for approval. Only pause on errors or ambiguity. |

### 3. Capture the decision as an ADR

If the `author-adr` skill is available, the skill will use it to create an ADR
recording the user's participation preference. This ADR ensures that future
sessions respect the same choice without re-asking.

The ADR would capture:
- The chosen participation level
- Any customizations (e.g., "autonomous for small tasks, guided for heavy")
- The rationale provided by the user

### 4. Fallback when author-adr is not available

If the `author-adr` skill is not available:

1. **Default behavior** — use the **Guided** level: summarize the plan and ask
   the user if they would like any changes or which stages they would like to
   start with.
2. **Record the decision** — if a `docs/adr/` directory exists, the agent
   should examine the existing ADRs to determine the format and conventions in
   use (Nygard vs. MADR, numbering scheme, status conventions), then write an
   ADR in that same format to capture the participation decision. The agent
   should follow the patterns it observes (header format, section ordering,
   status values) rather than inventing a new convention.
3. **If no ADR directory exists** — proceed with the Guided default without
   recording the decision.

### 5. Revision workflow on requested changes

When the user requests changes to the plan (at any participation level):

1. Increment the revision number per ADR-0005's naming convention.
2. Create a new plan file at the new revision (e.g., `0003-0004.1.plan.md`).
3. The new revision's header documents:
   - The previous revision it is based on
   - A summary of the requested changes
4. The previous revision is preserved unchanged.

### 6. Capture other behavioral decisions

When the skill encounters a behavioral decision during implementation that is
not yet recorded (e.g., error handling strategy, stage ordering preference),
it should:

1. If `author-adr` is available — use it to draft an ADR capturing the
   decision.
2. If `author-adr` is not available but `docs/adr/` exists — write an ADR
   using the observed conventions in the existing directory.
3. If neither is available — document the decision as a comment in the plan
   and proceed.

## Consequences

**Positive:**

- Users get the interaction model they prefer rather than a one-size-fits-all
  workflow, reducing friction for both hands-on and delegation-oriented users.
- Capturing the participation preference as an ADR means it persists across
  sessions, eliminating repetitive prompts.
- The fallback behavior (Guided default + convention-matching ADR writing)
  ensures the skill remains functional without the `author-adr` dependency.
- The revision workflow integrates naturally with ADR-0005's versioned naming,
  preserving the evolution of plans.

**Negative / Risks:**

- Adding a participation prompt after every plan creation adds one extra
  interaction step. Mitigated by checking for an existing ADR recording this
  preference and skipping the prompt if found.
- The convention-matching fallback (writing ADRs by observing existing format)
  may produce ADRs that differ subtly from those created by `author-adr`.
  Mitigated by the fact that the agent reads and mimics the actual files rather
  than guessing, and that ADRs are reviewed by humans before being acted upon.
- The autonomous participation level carries risk if the plan has errors — the
  agent will proceed without checkpoints. Mitigated by still pausing on errors
  or ambiguity, and by the fact that the user explicitly opted into this mode.

**Neutral:**

- The three participation levels (Full control, Guided, Autonomous) are not
  mutually exclusive across a plan's lifecycle — a user could start with
  Guided and switch to Full control for a specific stage. The skill should
  respect in-session preference changes even if the recorded ADR captures
  the initial choice.

---

## Comments

<!-- No review cycle on record. This ADR predates the formal review process and is superseded. -->
