# 2. Add plan-from-adr companion skill

Date: 2026-04-01

## Status

Proposed

## Context

The `author-adr` skill helps users create well-structured Architectural Decision
Records, but the workflow stops at the decision document. Once an ADR is
accepted, teams still need to translate the decision into actionable
implementation work — identifying tasks, ordering dependencies, and scoping
effort. This translation step is manual, error-prone, and often deferred,
leading to ADRs that are accepted but never fully realized.

A companion skill that bridges the gap between "decision made" and "work
planned" would close the loop on the ADR workflow and increase the likelihood
that architectural decisions are actually carried out.

## Decision

Add a new skill called `plan-from-adr` that:

1. **Reads an ADR** produced by `author-adr` (Nygard, MADR, or Y-Statement
   format) and extracts the decision, context, and consequences.
2. **Generates an implementation plan** by breaking the decision into a set of
   discrete, actionable tasks with descriptions.
3. **Identifies dependencies** between tasks and suggests an execution order.
4. **Outputs tasks** in a format consumable by project tracking tools (e.g.,
   markdown checklists, or structured data for work item creation).

The skill will live alongside `author-adr` in the `adr-skills` repository as a
separate skill directory (`plan-from-adr/`), following the same agentskills.io
spec conventions.

## Consequences

- **Easier:** Translating accepted ADRs into work becomes a single agent
  interaction rather than a manual planning session.
- **Easier:** Ensures implementation tasks stay traceable back to the originating
  ADR.
- **Harder:** The skill must handle multiple ADR template formats, adding
  parsing complexity.
- **Risk:** Generated plans may over- or under-decompose work depending on the
  ADR's level of detail — the skill should surface confidence and invite user
  refinement rather than presenting plans as final.
- **Risk:** Coupling between the two skills needs to stay loose — `plan-from-adr`
  should work with any well-formed ADR, not only those created by `author-adr`.
