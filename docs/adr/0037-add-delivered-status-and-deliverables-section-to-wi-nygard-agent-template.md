# 37. Add Delivered status and Deliverables section to wi-nygard-agent template

Date: 2026-04-05
Status: Proposed
Last Updated: 2026-04-05
Links:
- [ADR-0017: Nygard-agent template](0017-adopt-nygard-agent-template-as-default-adr-format.md)
- [ADR-0018: Unified format-based scripts](0018-replace-adr-tools-and-madr-tools-with-unified-format-based-scripts.md)
- [ADR-0034: Work-item-referenced naming](0034-adopt-work-item-referenced-naming-for-adr-files.md)
- [ADR-0035: Normalized work item data model](0035-define-normalized-work-item-data-model-for-vendor-agnostic-adr-tooling.md)
- [ADR-0036: Cache work item snapshots](0036-cache-work-item-snapshots-in-adr-var-directory.md)
- [ADR-0038: Work-item-driven lifecycle orchestration](0038-enable-work-item-driven-adr-lifecycle-orchestration.md)

## Context

The nygard-agent template (ADR-0017) defines the following status lifecycle for ADRs:

```
Prototype → Proposed → Accepted → Deprecated | Superseded
```

`Accepted` is the terminal "success" state — it means the decision has been approved and is ready for implementation. The `implement-adr` skill transitions an ADR from `Proposed` to `Accepted` after plan execution. But there is no status that signals **"the decision has been implemented and its outcomes are verified."**

**The gap:** Once an ADR is `Accepted`, there is no structured way to answer:
- What did this decision actually produce? (artifacts, code changes, documentation)
- Is the implementation complete, or is it still in progress?
- What constitutes "done" for this decision?

These questions matter especially in team settings where ADRs are associated with work items (ADR-0034). In a work-item-driven workflow, a project manager or tech lead needs to see at a glance: this decision was made, here's what it delivered, and it's done. Currently, that information is scattered across implementation plans (`docs/plans/`), commit history, and tribal knowledge.

**Why this matters for the wi-nygard-agent format:** The wi-nygard-agent format (ADR-0034) ties ADRs to work items. Work items have completion semantics — issues get closed, user stories get resolved. The ADR lifecycle should mirror this: when the work is delivered, both the work item and the ADR should reflect completion with a clear summary of outcomes.

**Future orchestration:** A `Delivered` status also enables work-item-driven orchestration (deferred to a companion ADR). If the ADR lifecycle maps to work item states, an orchestrator can automate transitions — e.g., when `implement-adr` completes execution, automatically transition the ADR to `Delivered` and update the work item. The `Deliverables` section provides the structured data the orchestrator needs to verify completion.

### Decision Drivers

- **Outcome visibility** — stakeholders should see what a decision produced without reading implementation plans or commit logs
- **Scope definition** — the ADR should define what "done" looks like before implementation begins, enabling verification after
- **Work item alignment** — the ADR lifecycle should have a completion state that aligns with work item closure
- **Orchestration readiness** — the status and deliverables data should be machine-readable enough to support automated lifecycle management
- **Backward compatibility** — existing ADRs without a Deliverables section must continue to work

## Options

### Option 1: Add Delivered status only, no template changes

Add `Delivered` as a valid status value after `Accepted`. No new template section — the status alone signals completion. Deliverables are documented informally in the existing Comments section or in implementation plans.

**Strengths:**
- Minimal change — one new status value, no template modification
- Backward compatible — existing ADRs are unaffected
- No new authoring burden — authors don't need to populate a new section

**Weaknesses:**
- No structured deliverables — "what did this produce?" still requires reading plans and commits
- No scope definition upfront — "done" is undefined until someone declares it
- Not machine-readable — an orchestrator cannot verify completion without structured data
- Comments section is mutable workspace — deliverables mixed with revision dialogue are hard to find

### Option 2: Add Delivered status and a Deliverables section

Add `Delivered` as a valid status value. Add a `## Deliverables` section to the template between Consequences and Quality Strategy. The section captures planned deliverables (pre-implementation) and actual outcomes (post-implementation).

The Deliverables section has two phases:
1. **Planning phase** (during authoring): Define expected deliverables as a checklist — what artifacts, changes, or capabilities will this decision produce?
2. **Delivery phase** (after implementation): Check off completed items and add references to actual artifacts (PRs, commits, documentation).

**Strengths:**
- Scope at a glance — anyone can read the Deliverables section to see what the decision is expected to produce and what it actually produced
- Machine-readable — checklist items can be parsed by an orchestrator to verify completion
- Upfront scope definition — forces the author to think about outcomes during authoring, not just during implementation
- Clear completion criteria — `Delivered` status is backed by verified deliverables, not just a declaration
- Feeds implement-adr — the Deliverables checklist can seed the implementation plan's task decomposition

**Weaknesses:**
- Additional authoring burden — authors must populate the Deliverables section
- Template grows — one more section to fill, increasing the barrier for simple decisions
- Duplicate information risk — deliverables may overlap with Consequences and Quality Strategy
- Section may be left empty — if not enforced, authors skip it and the section adds noise

### Option 3: Extend Consequences section with outcome tracking

Instead of a new section, extend the existing Consequences section with a structured "Outcomes" subsection that gets populated post-implementation. No new status — use `Accepted` with an `Outcomes:` metadata field.

**Strengths:**
- No new template section — builds on existing structure
- Natural fit — consequences describe expected outcomes, this adds actual outcomes
- Less template growth than a standalone section

**Weaknesses:**
- Mixes prediction with actuality — the Consequences section is for expected outcomes; adding actual results blurs the boundary
- No completion status — `Accepted` doesn't signal "done"
- Metadata-only tracking is less visible than a dedicated section
- Harder to parse — consequences are prose, outcomes need structure

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

In the context of **needing outcome visibility and completion semantics for work-item-driven ADRs**, facing **the absence of a structured way to define scope and verify delivery**, we decided for **adding a `Delivered` status and a `## Deliverables` section to the wi-nygard-agent template** (Option 2), and neglected **status-only (Option 1, no structured data) and consequence extension (Option 3, mixes prediction with actuality)**, to achieve **scope visibility at a glance, machine-readable completion criteria, and alignment between ADR lifecycle and work item closure**, accepting that **the template grows by one section and authors have additional work during authoring**.

### Extended Status Lifecycle

```
Prototype → Proposed → Accepted → Delivered
                          ↘           ↘
                   Deprecated | Superseded by ADR-XXXX
```

Both `Accepted` and `Delivered` can transition to `Deprecated` or `Superseded`. An ADR may be deprecated before delivery (e.g., the approach is abandoned after acceptance) or after (e.g., a newer decision supersedes the delivered solution).

`Delivered` means: the decision has been implemented, deliverables have been verified, and the associated work item can be closed.

### Deliverables Section

Placed between Consequences and Quality Strategy:

```markdown
## Deliverables

<!-- Planning phase: define expected deliverables as a checklist. -->
<!-- Delivery phase: check items off and add artifact references. -->

- [ ] [Expected artifact or outcome]
- [ ] [Expected artifact or outcome]
```

**Planning phase example** (during authoring):
```markdown
## Deliverables

- [ ] `wi-nygard-agent-format.sh` added to `scripts/`
- [ ] `new.sh` orchestrator updated for format-controlled naming
- [ ] Unit tests for new, list, rename, status subcommands
- [ ] SKILL.md updated with wi-nygard-agent format documentation
```

**Delivery phase example** (after implementation):
```markdown
## Deliverables

- [x] `wi-nygard-agent-format.sh` added to `scripts/` — PR #47
- [x] `new.sh` orchestrator updated — commit abc1234
- [x] Unit tests (8 tests) — PR #47
- [x] SKILL.md updated — PR #48
```

### Section Semantics

| Aspect | Behavior |
|--------|----------|
| **When populated** | During authoring (planning) and after implementation (delivery) |
| **Granularity** | One checkbox per artifact or verifiable outcome |
| **References** | PR numbers, commit SHAs, or file paths appended after checkbox text |
| **Completion rule** | All items checked → eligible for `Delivered` status |
| **Optional** | The section is present in the template but may be left empty for decisions that don't produce artifacts (e.g., policy decisions). In that case, `Delivered` is set when the team agrees the decision is in effect. |

### Interaction with implement-adr

The `implement-adr` skill can read the Deliverables section to:
1. Seed the implementation plan's task list from the checklist items
2. Update checklist items with PR/commit references as tasks complete
3. Transition status to `Delivered` when all items are checked

This creates a traceable chain: **work item → ADR → deliverables → implementation plan → code**.

## Consequences

**Positive:**
- Stakeholders can see what a decision produced by reading one section — no need to cross-reference implementation plans, PRs, or commit logs.
- Scope is defined upfront during authoring. The Deliverables checklist prompts the author to articulate "what will this decision produce?" before implementation begins, surfacing scope gaps early.
- The `Delivered` status provides a clear completion signal that aligns with work item closure, enabling automated transitions in work-item-driven workflows.
- The checklist format is machine-readable — an orchestrator can parse checkbox state to verify completion programmatically.
- Backward compatible: existing ADRs without a Deliverables section remain valid. The section is optional for decisions that don't produce artifacts.

**Negative:**
- The template grows by one section, increasing authoring overhead. For simple decisions (e.g., "use tabs not spaces"), the Deliverables section may feel like busywork.
- Risk of duplication between Deliverables and Quality Strategy — e.g., "Unit testing" in Quality Strategy overlaps with "Unit tests" in Deliverables. The distinction is intent: Quality Strategy flags *what kind* of testing; Deliverables names *specific test artifacts*.
- Authors may leave the section empty during planning, then never return to fill it after implementation. Without enforcement, the section degrades to noise. The implement-adr skill's integration mitigates this for ADRs that go through formal implementation.

**Neutral:**
- The `Delivered` status is a new addition to the wi-nygard-agent template's status values. Existing nygard-agent format ADRs don't use it, and the status parsing logic (ADR-0018) already handles arbitrary status strings.
- This ADR defines the template change and status lifecycle. Work-item-driven orchestration that uses the Delivered status and Deliverables data is deferred to a companion ADR (ADR-0038).

## Quality Strategy

- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

The template change must update the `generate_template()` function in the format script to include the new section. The status parsing logic already handles arbitrary strings, so no changes needed there. User documentation (SKILL.md, template reference) must describe the Deliverables section and the Delivered status. The implement-adr skill's interaction with Deliverables is a separate implementation concern.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This is the first of two companion ADRs extending the wi-nygard-agent format. ADR-0038 (work-item-driven orchestration) builds on the Delivered status and Deliverables data defined here.

---

## Comments

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Are all referenced ADRs linked in the header for traceability?

**Addressed** — Added ADR-0018 (cited in Quality Strategy and consequences), ADR-0036 (same series), and ADR-0038 (companion ADR referenced in prose) to the Links section.

### Q: Does the lifecycle diagram accurately show which statuses can transition to Deprecated or Superseded?

**Addressed** — Revised the Extended Status Lifecycle diagram to show Deprecated/Superseded reachable from both Accepted and Delivered. Added explanatory text clarifying that an ADR may be deprecated before delivery (approach abandoned) or after (superseded by a newer decision).

### Q: Does the "forces" wording in consequence P2 match the stated optionality of the Deliverables section?

**Addressed** — Changed "forces the author to articulate" to "prompts the author to articulate" to align with the Section Semantics table, which states the section "may be left empty."
