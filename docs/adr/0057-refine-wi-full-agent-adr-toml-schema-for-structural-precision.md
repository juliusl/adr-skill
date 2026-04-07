# 57. Refine wi-full-agent-adr TOML schema for structural precision

Date: 2026-04-07
Status: Accepted
Last Updated: 2026-04-07
Links: ADR-0051 (original TOML schema; partially supersedes design rule 3), ADR-0052 (Rust tooling), ADR-0053 (Markdown export)

## Context

ADR-0051 defined the wi-full-agent-adr TOML schema as a direct mapping from the nygard-agent Markdown template. That mapping was deliberate — it reduced migration complexity and preserved familiarity. However, now that the format is a full markup language with typed deserialization, several structural improvements are possible that weren't practical in Markdown.

Three areas need refinement:

1. **Checkpoints use flat boolean fields** — `all_options_evaluated = true`, `decision_drivers_referenced = true`, etc. These are hard-coded field names baked into the schema. In TOML, checkpoints could be a structured array of `{ label, checked }` records, making the checklist extensible without schema changes.

2. **Decision restates the choice in prose** — the `decision.body` field contains a paragraph that names the chosen option and explains why. Since options are already an indexed array (`[[options]]`), the decision could reference the chosen option by index and carry a separate `justification` field. This eliminates redundancy and makes the choice machine-readable.

3. **Plan and QA data is embedded in the ADR** — ADR-0051 includes `[plan]` with `[[plan.stages]]` and tasks. In practice, `plan.md` and `qa-plan.md` have been working well as separate files during prototyping (ADR-0051 through ADR-0057) without requiring structural synchronization. Since the project uses a relational database (`adr-db`), plan and QA records can join to the ADR by identifier rather than being embedded. This requires an `identifier` field in `[meta]`.

**Decision drivers:**
- Leverage TOML's type system instead of mapping Markdown conventions 1:1
- Keep the schema extensible — adding a checkpoint item should not require a schema migration
- Machine-readable decision outcomes (which option was chosen)
- Relational joins over embedded documents for plan/QA tracking
- Compatibility with existing ADR-0052 tooling implementation

## Options

### Option 1: Structured arrays for checkpoints, option-index decision, plan removal

Refine three aspects of the ADR-0051 schema:

**Checkpoints as arrays:**
```toml
[evaluation_checkpoint]
assessment = "Proceed"
validation_needs = ""

[[evaluation_checkpoint.items]]
label = "All options evaluated at comparable depth"
checked = true

[[evaluation_checkpoint.items]]
label = "Decision drivers are defined and referenced in option analysis"
checked = true

[[evaluation_checkpoint.items]]
label = "No unacknowledged experimentation gaps"
checked = true

[conclusion_checkpoint]
assessment = "Ready for review"
pre_review_notes = ""

[[conclusion_checkpoint.items]]
label = "Decision justified (Y-statement or equivalent)"
checked = true

[[conclusion_checkpoint.items]]
label = "Consequences include positive, negative, and neutral outcomes"
checked = true

[[conclusion_checkpoint.items]]
label = "Quality Strategy reviewed"
checked = true

[[conclusion_checkpoint.items]]
label = "Links to related ADRs populated"
checked = true
```

**Decision references option index:**
```toml
[decision]
chosen_option = 0          # zero-based index into [[options]] array
justification = """
We chose this option because it provides native TOML handling
with schema validation, and the Rust workspace already exists.
"""
```

**Plan/QA removed, identifier added:**
```toml
[meta]
identifier = "gh-42"       # unique key for relational joins
title = "Use PostgreSQL for event storage"
date = "2026-04-07"
status = "Proposed"
last_updated = "2026-04-07"
work_item = "gh#42"
links = ["ADR-0034", "ADR-0037"]
```

The `[plan]`, `[[plan.stages]]`, and `[deliverables]` tables are removed from the schema. Plan and QA data live in separate files and join to the ADR via `meta.identifier` in the database.

**Sub-decision — index vs. name for `chosen_option`:** The option reference could use a zero-based index or the option's name string. Index was chosen because: (a) option names are not guaranteed unique, (b) string matching is case-sensitive (`"PostgreSQL"` ≠ `"postgresql"`), and (c) renaming an option breaks the reference. The index fragility risk (reordering after decision) is mitigated by the index stability rule (design rule 3). Name-based reference is more human-readable but introduces matching ambiguity that the index approach avoids.

**Strengths:**
- Checkpoint items are data, not schema — adding or removing items does not require a struct change
- `chosen_option` makes the decision machine-readable — tooling can resolve the chosen option's name and body without parsing prose
- Plan/QA separation matches observed usage — during prototyping (ADR-0051 through ADR-0057), separate files have maintained consistent structure without requiring synchronization
- `identifier` enables relational joins without embedding documents
- Smaller schema surface — fewer structs to maintain in `adr-format`

**Weaknesses:**
- `chosen_option` as a zero-based index is fragile if options are reordered after the decision is made
- Removing `[plan]` means the single-file-state benefit from ADR-0051 is lost — plan data is back to being a separate file
- Checkpoint items lose compile-time field validation — a typo in a label string is not caught by serde

### Option 2: Named checkpoint table, option-index decision, plan stays optional

Keep checkpoints as named fields in a sub-table (preserving compile-time validation) but restructure them for clarity. Change the decision to reference the chosen option. Keep `[plan]` in the schema but mark it optional.

**Checkpoints as named sub-table:**
```toml
[evaluation_checkpoint]
assessment = "Proceed"
validation_needs = ""

[evaluation_checkpoint.checklist]
all_options_evaluated = true
decision_drivers_referenced = true
no_experimentation_gaps = true

[conclusion_checkpoint]
assessment = "Ready for review"
pre_review_notes = ""

[conclusion_checkpoint.checklist]
decision_justified = true
consequences_complete = true
quality_strategy_reviewed = true
links_populated = true
```

**Decision and plan:** Same `chosen_option` as Option 1. Plan stays as an optional `[plan]` table.

**Strengths:**
- Checkpoint items are still typed fields — serde catches unknown keys
- Cleaner nesting (`checklist` sub-table) without losing validation
- Plan remains available for projects that want single-file state

**Weaknesses:**
- Checkpoint items are still hard-coded — adding an item requires a schema change
- Optional plan creates ambiguity — is the source of truth the embedded plan or the external file?
- The named sub-table is effectively what ADR-0051 already has, just reorganized

## Evaluation Checkpoint
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — these are structural refinements to an existing schema. The trade-offs (extensibility vs. compile-time validation, index vs. name reference) are well-understood configuration design choices.

## Decision

In the context of refining the wi-full-agent-adr TOML schema, facing the need for extensible checkpoints, machine-readable decisions, and proper relational separation of plan/QA data, we chose **Option 1 (structured arrays for checkpoints, option-index decision, plan removal)** over Option 2 (named sub-table with optional plan) to achieve an extensible schema that leverages the relational database for cross-file joins, accepting that checkpoint items lose compile-time field validation and the chosen option index is fragile under reordering.

**Key design rules:**

1. **Checkpoint items are arrays** — `[[evaluation_checkpoint.items]]` and `[[conclusion_checkpoint.items]]` use `{ label, checked }` records. The default items match the nygard-agent template's checklist, but additional items can be added without schema changes.
2. **Decision references option index** — `decision.chosen_option` is a zero-based integer index into the `[[options]]` array. Tooling resolves the index to the option's `name` and `body` fields. A `justification` field carries the rationale.
3. **Index stability rule** — once a decision is made (`status` ≠ `Prototype`), the `[[options]]` array order must not change. Tooling must enforce this by rejecting reordering operations on decided ADRs; enforcement is scoped to the `adr-format` binary (ADR-0052) and is not yet implemented.
4. **Plan and QA removed from schema** — the `[plan]`, `[[plan.stages]]`, and `[deliverables]` tables defined in ADR-0051 are removed. Plan and QA data live in separate files (`plan.md`, `qa-plan.md`) managed by implement-adr.
5. **Identifier field in meta** — `meta.identifier` is the ADR's join key for relational lookups. For work-item formats, it matches the `{remote}-{id}` pattern (e.g., `"gh-42"`), derived from the filename's `{remote}-{id}` prefix at creation time and immutable thereafter. For numbered formats, it is the zero-padded number (e.g., `"0057"`). Note: `identifier` and `work_item` reference the same work item but serve different purposes — `identifier` is the database join key (uses `-` delimiter, matching the filename convention from ADR-0034), while `work_item` is the human-readable reference (uses `#` delimiter, per ADR-0051 design rule 9). Multiple ADRs may share the same identifier if they address the same work item; uniqueness is enforced at the filename level (the slug differentiates).
6. **Comments section retained** — `[comments]` with `draft_worksheet` (string) remains in the schema. The Draft Worksheet is part of the ADR's authoring record, not plan data. Revision entries (`[[comments.revision_entries]]`) also remain — they are authoring history, not implementation tracking.
7. **Quality strategy unchanged** — the `[quality_strategy]` table retains its flat boolean checklist from ADR-0051. These are compile-time-validated fields (unlike checkpoint items) because they drive downstream tooling behavior (test generation, documentation flags).

**Revised schema summary:**

```toml
[meta]
identifier = "gh-42"
title = "Use PostgreSQL for event storage"
date = "2026-04-07"
status = "Proposed"
last_updated = "2026-04-07"
work_item = "gh#42"
links = ["ADR-0034", "ADR-0037"]

[context]
body = """..."""

[[options]]
name = "PostgreSQL"
body = """..."""

[[options]]
name = "SQLite"
body = """..."""

[evaluation_checkpoint]
assessment = "Proceed"
validation_needs = ""

[[evaluation_checkpoint.items]]
label = "All options evaluated at comparable depth"
checked = true

[[evaluation_checkpoint.items]]
label = "Decision drivers are defined and referenced in option analysis"
checked = true

[[evaluation_checkpoint.items]]
label = "No unacknowledged experimentation gaps"
checked = true

[decision]
chosen_option = 0
justification = """..."""

[[consequences]]
type = "positive"
body = "Strong JSONB support enables flexible event schemas"

[[consequences]]
type = "negative"
body = "Requires a running database process"

[[consequences]]
type = "neutral"
body = "Team already has PostgreSQL operational expertise"

[quality_strategy]
major_semantic_changes = false
minor_semantic_changes = true
fuzz_testing = false
unit_testing = true
load_testing = false
performance_testing = false
backwards_compatible = true
integration_tests = true
tooling = true
user_documentation = false
additional_concerns = ""

[conclusion_checkpoint]
assessment = "Ready for review"
pre_review_notes = ""

[[conclusion_checkpoint.items]]
label = "Decision justified (Y-statement or equivalent)"
checked = true

[[conclusion_checkpoint.items]]
label = "Consequences include positive, negative, and neutral outcomes"
checked = true

[[conclusion_checkpoint.items]]
label = "Quality Strategy reviewed"
checked = true

[[conclusion_checkpoint.items]]
label = "Links to related ADRs populated"
checked = true

[comments]
draft_worksheet = """..."""

[[comments.revision_entries]]
id = "R1"
date = "2026-04-07"
body = "Initial revision."
```

## Consequences

**Positive:**
- Checkpoint items are extensible — projects can add domain-specific checklist items without modifying the `adr-format` binary's struct definitions
- `chosen_option` enables tooling to programmatically identify which option was selected, supporting automated reporting and database queries
- Plan/QA separation aligns with observed usage — during prototyping (ADR-0051 through ADR-0057), separate plan.md and qa-plan.md files have maintained consistent structure, and the relational database handles joins naturally
- `meta.identifier` gives the ADR a stable, immutable key that other tables (plan, QA, telemetry) can reference via foreign key
- Smaller schema surface reduces maintenance burden on the `adr-format` crate

**Negative:**
- Checkpoint items lose serde compile-time validation — a misspelled label is a data error, not a deserialization failure. Mitigation: tooling can validate labels against a known-good list at runtime
- Zero-based `chosen_option` index is fragile if options are reordered — mitigated by the index stability rule (design rule 3); enforcement is not yet implemented and is scoped to the `adr-format` binary (ADR-0052)
- Removing `[plan]` from the schema means the single-file-state benefit described in ADR-0051 is no longer available. Projects that wanted all state in one file now have ADR + plan + QA as separate artifacts

**Neutral:**
- The `[comments]` section structure (draft worksheet + revision entries) is unchanged from ADR-0051
- The `[[consequences]]` array-of-tables structure is unchanged
- This ADR partially supersedes ADR-0051. Specifically, it reverses ADR-0051's design rule 3 (plan section for single-file state) because observed usage during prototyping showed separate files work effectively and the relational database (adr-db) provides the join capability that embedding was designed to achieve. ADR-0051's overall approach — flat TOML, section tables, mandatory checkpoints — remains the foundation

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- [x] Integration tests
- [x] Tooling
- [ ] User documentation

### Additional Quality Concerns

The `adr-format` crate's struct definitions must be updated to reflect the new schema: checkpoint items become `Vec<CheckpointItem>` instead of named boolean fields, and `Decision` gains `chosen_option: usize` and `justification: String` replacing `body: String`. Existing TOML files using the ADR-0051 schema need a migration path. The intended approach is backward-compatible deserialization — the `adr-format` tooling (ADR-0052) will accept both ADR-0051 and ADR-0057 shapes during a transition period, which is why Backwards Compatible is checked above. A one-time migration script may be provided as a convenience but is not required for correctness.

## Conclusion Checkpoint
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR partially supersedes ADR-0051's design rule 3 (plan as single-file state) while preserving the overall flat-TOML approach and mandatory checkpoints. The changes are: checkpoint representation, decision field structure, plan/QA separation, and the addition of `meta.identifier`. Backwards Compatible is checked because the tooling will support backward-compatible deserialization, accepting both ADR-0051 and ADR-0057 shapes.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
ADR-0051 defined the TOML schema by mapping the nygard-agent Markdown template 1:1. Now that we're committed to TOML as a full markup language, three improvements are available: (1) checkpoints as structured arrays instead of hard-coded boolean fields, (2) decision referencing the chosen option by index instead of restating it in prose, (3) removing plan/QA from the schema since separate files have worked fine and the relational database can join by identifier.

**Tolerance:**
- Risk: Low — refining an existing schema, not introducing a new format
- Change: Low — structural changes within an established format
- Improvisation: Low — the user has clear direction on all three improvements

**Uncertainty:**
- Certain: checkpoints should be arrays, decision should reference option index, plan/QA should be separate
- Uncertain: whether `chosen_option` should use zero-based index vs. option name, exact identifier format

**Options:**
- Target count: 2-3
- [x] Explore additional options beyond candidates listed below

**Candidates:**
- Structured arrays for checkpoints + option-index decision + plan removal
- Named sub-table checkpoints + optional plan
- Option-name reference instead of index

### Revision Q&A — R1 (Review Response)

**Review verdict:** Revise (7 findings)

**Addressed (7):**

1. **(F1) "Refines, not supersedes" is misleading** — Recharacterized the relationship as partial supersession. Updated Links annotation to note "partially supersedes design rule 3." Rewrote the neutral consequence to acknowledge reversal of ADR-0051's single-file-state goal with rationale (observed usage + relational database makes embedding unnecessary). Updated Pre-review notes to match.

2. **(F2) "No format drift" is unsubstantiated** — Qualified the claim in three locations (Context, Option 1 Strengths, Consequences). Replaced bare "no format drift" with "during prototyping (ADR-0051 through ADR-0057), separate files have maintained consistent structure without requiring synchronization" — grounding the observation in the actual period.

3. **(F3) Option 3 is a near-dummy alternative** — Folded index-vs-name trade-off into Option 1 as an explicit sub-decision paragraph. Removed Option 3 as a standalone option. Updated the Decision Y-statement to reference only Option 1 and Option 2. The sub-decision explains why index was chosen: names aren't unique, matching is case-sensitive, renaming breaks references.

4. **(F4) `meta.identifier` format overlaps with `meta.work_item`** — Expanded design rule 5 to clarify: (a) `identifier` is the database join key (uses `-`, matches filename convention), `work_item` is the human-readable reference (uses `#`, per ADR-0051 design rule 9); (b) `identifier` is derived from the filename prefix at creation time; (c) multiple ADRs may share the same identifier (slug differentiates at filename level). Changed "unique key" to "join key" to reflect that identifier is not necessarily unique across ADRs.

5. **(F5) Backwards Compatible checked but migration needed** — Resolved the tension by making the strategy explicit: backward-compatible deserialization is the intended approach (tooling accepts both ADR-0051 and ADR-0057 shapes during transition). This justifies the checkbox. Updated Additional Quality Concerns and Pre-review notes to state the strategy directly instead of presenting alternatives.

6. **(F6) Date inconsistency** — Fixed Last Updated from 2026-04-06 to 2026-04-07.

7. **(F7) Index stability enforcement deferred with no timeline** — Updated design rule 3: changed "should enforce" to "must enforce" and added "enforcement is scoped to the `adr-format` binary (ADR-0052) and is not yet implemented." Updated the negative consequence to match: "enforcement is not yet implemented and is scoped to the `adr-format` binary (ADR-0052)."

**Rejected (0):** None.

<!-- Review cycle 1 — 2026-04-07 — Verdict: Revise. 7 addressed, 0 deferred, 0 rejected. -->
<!-- Review cycle 2 — 2026-04-07 — Verdict: Accept. 1 finding (M) addressed inline (removed [comments.revision_entries] from removed-tables list). -->
