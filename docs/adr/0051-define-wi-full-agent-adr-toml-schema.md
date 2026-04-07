# 51. Define wi-full-agent-adr TOML schema

Date: 2026-04-06
Status: Accepted
Last Updated: 2026-04-06
Links: ADR-0017 (nygard-agent template), ADR-0034 (work-item naming), ADR-0037 (deliverables section), ADR-0038 (lifecycle orchestration), ADR-0052 (tooling implementation), ADR-0053 (Markdown export)

## Context

The wi-nygard-agent format uses Markdown for ADR files. After several rounds of implementation, key gaps emerged:

1. **Fuzzy parsing** — agents drift when writing procedural reports because Markdown structure is implicit. Extracting fields like Status, Decision, or Quality Strategy requires regex/awk parsing that is fragile and error-prone.
2. **Optional checkpoints** — the current template marks Evaluation and Conclusion Checkpoints as "(Optional)". Agents consistently skip steps when given the choice, reducing audit quality.
3. **Scattered state** — plan status, deliverables tracking, and ADR content live in separate files (plan.md, qa-plan.md, the ADR itself). This creates synchronization overhead and makes it hard to see a decision's full state in one place.
4. **No typed deserialization** — Markdown has no schema. Every tool that reads an ADR must implement its own parser. TOML has strongly-typed deserialization libraries in Rust (toml crate), Python (tomllib), and most other languages.

The adr-atelier roadmap calls for a new "wi-full-agent-adr" format that uses TOML to address these gaps. The format must be compatible with the existing work-item naming convention (`{remote}-{id}-{slug}.toml`) and the lifecycle orchestration model (ADR-0038).

**Decision drivers:**
- Strongly-typed deserialization for tooling (adr-db-lib, adr-atelier)
- Mandatory checkpoints to enforce procedural rigor
- Single-file state for plan and status tracking
- Compatibility with existing script dispatch (`new.sh`, `ADR_AGENT_SKILL_FORMAT`)
- Convertible to Markdown for human-readable document storage (ADR-0053)

## Options

### Option 1: Flat TOML with section tables

Map each Markdown section to a TOML table. Use arrays of tables for lists (options, consequences). Keep the structure flat — one level of nesting.

```toml
[meta]
title = "Use PostgreSQL for event storage"
date = "2026-04-06"
status = "Proposed"
last_updated = "2026-04-06"
work_item = "gh#42"
links = ["ADR-0034", "ADR-0037"]

[context]
body = """
We need a database for storing audit events...
"""

[[options]]
name = "PostgreSQL"
body = """
Mature relational database with strong JSONB support...
"""

[[options]]
name = "SQLite"
body = """
Embedded database, zero deployment overhead...
"""

[evaluation_checkpoint]
assessment = "Proceed"
all_options_evaluated = true
decision_drivers_referenced = true
no_experimentation_gaps = true
validation_needs = ""

[decision]
body = """
We chose PostgreSQL for event storage because...
"""

[[consequences]]
type = "positive"
body = "Strong JSONB support enables flexible event schemas"

[[consequences]]
type = "negative"
body = "Requires a running database process"

[[consequences]]
type = "neutral"
body = "Team already has PostgreSQL operational expertise"

[deliverables]
items = [
  { description = "Schema migration for events table", done = false },
  { description = "Connection pool configuration", done = true, artifact = "src/db/pool.rs" },
]

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
decision_justified = true
consequences_complete = true
quality_strategy_reviewed = true
links_populated = true
pre_review_notes = ""

[plan]
[[plan.stages]]
name = "schema-definition"
status = "done"
tasks = [
  { description = "Define section tables", done = true },
  { description = "Validate round-trip serialization", done = false },
]

[comments]
draft_worksheet = """
**Framing:** ...
"""
[[comments.revision_entries]]
id = "R1"
date = "2026-04-06"
body = "Addressed review findings on schema completeness."
```

**Strengths:** Direct mapping from existing template. Readable. Easy to implement incrementally.

**Weaknesses:** Deeply nested inline tables can become awkward for complex plans. `body` fields holding multi-paragraph text in TOML strings may be less ergonomic than Markdown.

### Option 2: Hierarchical TOML with typed enums

Introduce stronger typing — status as an enum, consequence types as separate tables, checkpoints as typed records. More structure, more schema enforcement.

```toml
[meta]
title = "Use PostgreSQL for event storage"
date = 2026-04-06
status = "Proposed"  # Prototype | Proposed | Accepted | Delivered | Deprecated | Superseded
last_updated = 2026-04-06
work_item = { remote = "gh", id = "42" }

[meta.links]
related = ["ADR-0034"]
supersedes = []
superseded_by = []

[context]
body = "..."

[[options]]
name = "PostgreSQL"
body = "..."

[evaluation_checkpoint]
assessment = "Proceed"
checklist = { all_options_evaluated = true, decision_drivers_referenced = true, no_experimentation_gaps = true }

[decision]
body = "..."

[consequences]
positive = ["Strong JSONB support"]
negative = ["Requires running database"]
neutral = ["Team has expertise"]

[quality_strategy]
semantic_changes = "minor"
testing = ["unit", "integration"]
backwards_compatible = true
```

**Strengths:** Strongest typing. Enum validation catches invalid states at parse time. Consequences are pre-categorized.

**Weaknesses:** Opinionated structure makes it harder to evolve the schema. The typed enums (status, semantic_changes) require code changes to extend. Less readable for humans unfamiliar with the schema.

### Option 3: TOML header + Markdown body (hybrid)

Use TOML for structured metadata (status, links, checkpoints, quality strategy) but keep the prose sections (Context, Options, Decision, Consequences) as Markdown in string fields.

This is effectively Option 1 but with an explicit design principle: **TOML for data, Markdown for prose.** The boundary is clear — anything a tool needs to read/write programmatically is a typed TOML field. Anything that's primarily for human consumption is a Markdown string.

**Strengths:** Best of both worlds — tools get typed data, humans get readable prose. Markdown export is simpler because prose sections are already Markdown.

**Weaknesses:** Two syntaxes in one file. The boundary between "data" and "prose" is a judgment call that could drift over time.

## Evaluation Checkpoint
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — the schema is a structural mapping from the existing Markdown template (`nygard-agent-template.md`). The differences between options concern nesting depth and type strictness, which are well-understood trade-offs in configuration format design.

## Decision

In the context of defining a structured ADR format for automated workflows, facing the need for strongly-typed deserialization and mandatory checkpoints, we chose **Option 1 (flat TOML with section tables)** over Option 2 (hierarchical with typed enums) and Option 3 (hybrid TOML+Markdown) to achieve a direct mapping from the existing template that is easy to implement and evolve, accepting that multi-paragraph prose in TOML strings is slightly less ergonomic than raw Markdown.

**Key design rules:**

1. **File extension:** `.toml` — the format uses TOML exclusively. Files are named `{remote}-{id}-{slug}.toml` (e.g., `gh-42-use-postgresql.toml`).
2. **Checkpoints are mandatory** — no "(Optional)" markers. The `assessment` field must be populated; an empty assessment is a validation error.
3. **Plan section** — the `[plan]` table enables storing implementation plan data inline, replacing the need for separate plan.md files. Stages and tasks live in the ADR.
4. **Comments section** — `[comments]` holds the Draft Worksheet and revision entries, preserving the mutable workspace below the semantic boundary.
5. **Status values** — `Prototype`, `Proposed`, `Accepted`, `Delivered`, `Deprecated`, `Superseded`. Same as wi-nygard-agent but now validated by the parser.
6. **Multi-line strings** — use TOML multi-line basic strings (`"""..."""`) for prose fields (context.body, decision.body, option bodies). This preserves Markdown formatting within the TOML structure.
7. **Plan schema** — `[plan]` holds `[[plan.stages]]`, each with `name` (string), `status` (string), and `tasks` (array of `{ description, done }` records). This is a minimal structure; ADR-0052 may refine field names during implementation.
8. **Revision entries** — `[[comments.revision_entries]]` uses `{ id, date, body }` records. The `id` is a sequential label (R1, R2, …) for cross-referencing.
9. **Work-item reference** — `work_item` uses the `{remote}#{id}` string format (e.g., `"gh#42"`). Tooling splits on `#` — this is a single-character delimiter split, not regex. The `#` separates remote from ID within the field value, paralleling the `-` separator in the `{remote}-{id}-{slug}` filename convention (ADR-0034).
10. **Semantic change flags** — `major_semantic_changes` and `minor_semantic_changes` are separate booleans. Implementations must enforce mutual exclusivity (both `true` is invalid). Separate booleans were chosen over a single enum field to keep the quality strategy section as a flat checklist, consistent with the other boolean flags in that table.

## Consequences

**Positive:**
- Tools can deserialize ADR files with `toml::from_str()` — no regex parsing, no heading extraction
- Mandatory checkpoints enforce procedural discipline by default
- Single-file state reduces synchronization overhead between ADR, plan, and QA artifacts
- The flat structure preserves the existing Markdown template's section layout, reducing migration complexity. `[plan]`, `[deliverables]`, and `[comments]` are structural additions that consolidate data previously spread across separate files (plan.md, qa-plan.md).

**Negative:**
- Editing TOML by hand is less natural than Markdown for prose-heavy sections
- Existing tools that consume Markdown ADRs (GitHub rendering, docs sites) need the Markdown export (ADR-0053)
- Agent prompts that generate ADR content must now produce valid TOML, which is stricter than Markdown. Multi-line Markdown embedded in TOML `"""` strings is a known LLM failure mode (unescaped quotes, broken delimiters). This is an open risk — mitigation strategies (validation passes, structured generation) belong in the implementing ADR (ADR-0052).

**Neutral:**
- The `.toml` extension clearly distinguishes wi-full-agent-adr files from Markdown ADRs in mixed decision logs
- The schema can evolve by adding optional tables — existing files remain valid as long as required tables are present

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [ ] Backwards Compatible
- [x] Integration tests
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

The schema definition needs validation tests — round-trip serialize/deserialize for every section, edge cases for empty optional fields, and rejection of invalid status values. The Rust `toml` crate provides the serialization; tests should cover the schema struct definitions.

## Conclusion Checkpoint
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR defines the schema only. The implementation approach is covered by ADR-0052, and the Markdown export by ADR-0053.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
The adr-atelier roadmap (Milestone 1) requires a new "wi-full-agent-adr" format using TOML instead of Markdown. The existing wi-nygard-agent format has known gaps: fuzzy parsing, optional checkpoints that agents skip, and scattered state across multiple files. The TOML format should preserve the same sections and workflow but with strongly-typed structure.

**Tolerance:**
- Risk: Low — the schema is a well-understood mapping from existing sections
- Change: Medium — changing file format is a significant shift for the toolchain
- Improvisation: Low — the roadmap specifies the direction clearly

**Uncertainty:**
- Certain: TOML is the target format, checkpoints become mandatory, work-item naming is preserved
- Uncertain: Exact field names and nesting depth for the plan section, how revision entries are structured

**Options:**
- Target count: 2-3
- [x] Explore additional options beyond candidates listed below

**Candidates:**
- Flat TOML mapping from existing Markdown sections
- Hierarchical TOML with typed enums
- TOML header + Markdown body hybrid

### Revision Q&A — R1 (Review Response)

**Review verdict:** Revise (8 findings)

**Addressed (7):**

1. **(F1) `[plan]` schema undefined** — Added `[[plan.stages]]` with `{ name, status, tasks }` structure to the example and design rule 7. The schema is minimal; ADR-0052 may refine field names during implementation since the Draft Worksheet flagged nesting depth as uncertain.

2. **(F2) `revision_entries` structure undefined** — Added `[[comments.revision_entries]]` with `{ id, date, body }` record structure to the example and design rule 8.

3. **(F3) Vague "prior TOML schema design work" in Evaluation Checkpoint** — Replaced with a specific reference: the schema is a structural mapping from `nygard-agent-template.md`, and the option differences concern nesting depth and type strictness — standard configuration format trade-offs.

4. **(F4) TOML generation risk understated** — Expanded the negative consequence to acknowledge multi-line Markdown in TOML `"""` strings as a known LLM failure mode and an open risk, with mitigation deferred to ADR-0052.

5. **(F5) `major_semantic_changes` / `minor_semantic_changes` boolean trade-off** — Added design rule 10 explaining why separate booleans were chosen over a single enum (consistency with the flat checklist pattern) and noting that implementations must enforce mutual exclusivity.

6. **(F6) `work_item` string parsing vs. "no regex" goal** — Added design rule 9 clarifying the `{remote}#{id}` format uses a single-character delimiter split, not regex. The "no regex parsing" consequence refers to the overall ADR structure (no heading extraction), not individual field values.

7. **(F7) "1:1 mapping" overstatement** — Softened the positive consequence to say the flat structure "preserves the section layout" and explicitly calls out `[plan]`, `[deliverables]`, and `[comments]` as structural additions that consolidate previously separate files.

**Rejected (1):**

8. **(F8) Driver-by-driver comparison requested** — Rejected as process overhead. The Y-statement covers the rationale, and the prose strengths/weaknesses address the key trade-offs. A formal driver-by-driver matrix doesn't add decision clarity for a well-understood schema choice — the drivers are about tooling capabilities (typing, mandatory checkpoints, single-file state), and the options differ on structural approach, not on whether they satisfy those drivers.
