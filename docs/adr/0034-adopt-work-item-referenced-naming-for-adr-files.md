# 34. Adopt work-item-referenced naming for ADR files

Date: 2026-04-05
Status: Proposed
Last Updated: 2026-04-05
Links:
- [ADR-0018: Unified format-based scripts](0018-replace-adr-tools-and-madr-tools-with-unified-format-based-scripts.md)
- [ADR-0017: Nygard-agent template](0017-adopt-nygard-agent-template-as-default-adr-format.md)
- [ADR-0020: .adr/ project-scoped convention](0020-establish-adr-directory-as-project-scoped-convention.md)

## Context

The current ADR tooling uses sequential numbering for file naming: `NNNN-title.md` (e.g., `0034-use-postgresql.md`). The orchestrator script (`new.sh`) scans the ADR directory, finds the highest number, and increments it. This creates two problems:

**1. Team collaboration collisions.** When multiple developers create ADRs concurrently on separate branches, they independently compute the same next number. On merge, one branch must be renumbered — and since the number appears in the filename, heading, and all cross-references, this is a disruptive conflict. Git has no directory-level locks; the sequential numbering scheme assumes single-writer semantics that distributed version control does not provide.

**2. No structural link to work items.** ADRs are motivated by work tracked in external systems — GitHub Issues, Azure DevOps work items (Bug, User Story, Task, Feature), or local trackers. Today, the only way to reference a work item is a manual URL in the `Links:` metadata. This link is informational only: tooling cannot derive the work item from the filename, and developers cannot tell which ADR corresponds to which work item without opening the file.

**Why this matters now:** The ADR skill ecosystem is expanding to support team workflows. The `implement-adr` skill already bridges ADRs to code; a similar bridge to work items would enable traceability from requirement → decision → implementation. The format architecture from ADR-0018 (two-level dispatch: orchestrator + format script) was designed for exactly this kind of extensibility — adding a new format means adding one file.

### Decision Drivers

- **Collision-free naming** — multiple developers must be able to create ADRs concurrently without numbering conflicts
- **Work item traceability** — the relationship between an ADR and its motivating work item must be structural, not just a manual link
- **Multi-vendor support** — must work with GitHub Issues, Azure DevOps work items, and local/offline workflows
- **Format extensibility** — must fit the existing format dispatch architecture (ADR-0018)
- **Backward compatibility** — existing sequentially-numbered ADRs must continue to work alongside the new format

## Options

### Option 1: Keep sequential numbering, add work item metadata

Add a `Work-Item:` field to the nygard-agent template's inline metadata block (e.g., `Work-Item: gh#42`). Keep `NNNN-title.md` naming unchanged. The work item reference is informational metadata, not part of the filename or identity.

**Strengths:**
- Zero tooling changes to naming, listing, or status commands
- Fully backward compatible — existing ADRs are unaffected
- Simple to implement — just a template change

**Weaknesses:**
- Does not solve the team collision problem — sequential numbering still requires single-writer semantics
- Work item link is informational only — tooling cannot derive the work item from the filename
- No structural enforcement — the `Work-Item:` field can be left blank or filled inconsistently

### Option 2: Work-item-prefixed naming (wi-nygard-agent format)

Introduce a new format `wi-nygard-agent` that replaces the sequential `NNNN-` prefix with a vendor-qualified work item identifier: `{vendor}-{id}-{slug}.md`. Examples:

- `gh-42-use-postgresql.md` — GitHub Issue #42
- `ado-1234-use-postgresql.md` — Azure DevOps work item #1234
- `local-a1b2c3-use-postgresql.md` — locally generated ID (short hash)

The template content remains the nygard-agent template (ADR-0017). What changes is the naming scheme and the heading format: `# {vendor}-{id}. {title}` instead of `# N. {title}`.

The format script (`wi-nygard-agent-format.sh`) handles:
- Vendor-qualified file naming (no sequential scan needed)
- Heading generation with vendor-id prefix
- Listing that handles both legacy `[0-9]*` and `{vendor}-*` patterns
- A `Work-Item:` metadata field populated automatically from the naming arguments

A `local` vendor provides offline capability: when no external work item system is available, the developer generates a local ID (e.g., short UUID or timestamp hash). This can later be re-linked when the work item is created externally.

**Strengths:**
- Eliminates numbering collisions — IDs come from external systems or local generation, not sequential scan
- Structural traceability — the work item reference is in the filename and heading, not just metadata
- Fits the format dispatch architecture — one new `wi-nygard-agent-format.sh` file, no orchestrator changes
- Template content reuse — same nygard-agent template, only the naming layer changes
- Mixed decision logs — `list` command can handle both sequential and work-item-prefixed files

**Weaknesses:**
- Requires a work item to exist (or a local ID to be generated) before creating an ADR
- Loses chronological ordering in filename sort — must rely on `Date:` metadata for ordering
- Cross-references change: `ADR-0034` becomes `ADR-gh-42` — existing convention broken
- Vendor prefix is a new concept that developers must learn
- `new.sh` orchestrator needs a small change: skip sequential number computation when format handles its own naming

### Option 3: UUID-based naming with work item metadata

Use UUIDs for filenames: `{uuid8}-{slug}.md` (e.g., `a1b2c3d4-use-postgresql.md`). Track work item references in a `Work-Item:` metadata field. UUIDs are globally unique, eliminating collisions without depending on external systems.

**Strengths:**
- Guaranteed unique — no collisions, no external dependencies
- Simple generation — `uuidgen | head -c 8` or equivalent
- Works offline — no work item system needed

**Weaknesses:**
- UUIDs are unreadable — `a1b2c3d4-use-postgresql.md` conveys no context about the work item
- Loses all ordering — not chronological, not by work item
- Work item link is metadata-only — same limitation as Option 1
- No traceability in filename — must open file to find the work item
- Breaks existing `ADR-NNNN` cross-reference convention with no clear replacement

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

In the context of **enabling team-scale ADR authoring with work item traceability**, facing **sequential numbering collisions and the absence of structural work item references**, we decided for **a new `wi-nygard-agent` format that replaces sequential numbering with vendor-qualified work item identifiers in filenames** (Option 2), and neglected **metadata-only work item links (Option 1, doesn't solve collisions) and UUID naming (Option 3, unreadable and no traceability)**, to achieve **collision-free concurrent authoring and structural traceability from ADR to work item**, accepting that **chronological ordering moves from filename to metadata, cross-references adopt a new `ADR-{vendor}-{id}` convention, and a work item (or local ID) must exist before ADR creation**.

### Naming Convention

```
{vendor}-{id}-{slug}.md
```

| Vendor | ID Source | Example Filename |
|--------|----------|-----------------|
| `gh` | GitHub Issue number | `gh-42-use-postgresql.md` |
| `ado` | ADO work item ID | `ado-1234-use-postgresql.md` |
| `local` | Short hash (8 chars) | `local-a1b2c3d4-use-postgresql.md` |

Heading format: `# {vendor}-{id}. {title}` (e.g., `# gh-42. Use PostgreSQL`).

If two developers reference the same work item, the resulting filename collision is intentional — it surfaces a genuine decision conflict (two competing decisions for the same problem), caught during PR review.

### Format Script

A new `wi-nygard-agent-format.sh` added to `scripts/`, following the format dispatch architecture (ADR-0018):

| Subcommand | Behavior |
|-----------|----------|
| `new <vendor> <id> <title> <dir>` | Generate ADR with vendor-qualified filename and heading |
| `init [dir]` | Bootstrap ADR directory (same as nygard-agent) |
| `list` | List ADRs, handling both `[0-9]*` and `{vendor}-*` patterns |
| `rename <vendor> <id> <new-title>` | Rename ADR file and update heading |
| `status [vendor-id] [new-status]` | Show or update status |

### Orchestrator Change

`new.sh` receives a format-level signal that the format handles its own naming. When `format=wi-nygard-agent`, the orchestrator passes vendor and ID instead of a sequential number. The interface change is minimal: the format script's `new` subcommand accepts `<vendor> <id>` instead of `<number>`.

### Cross-Reference Convention

Existing: `ADR-0034`
New: `ADR-gh-42`, `ADR-ado-1234`, `ADR-local-a1b2c3d4`

Both conventions coexist in mixed decision logs.

### Ordering

Chronological ordering is by `Date:` metadata, not filename. The `list` subcommand sorts by date when displaying work-item-prefixed ADRs.

## Consequences

**Positive:**
- Developers can create ADRs concurrently on separate branches without numbering collisions — the ID comes from the work item system, not a directory scan.
- Work item traceability is structural: the filename and heading encode the work item reference, making it discoverable without opening the file.
- The format dispatch architecture (ADR-0018) absorbs this change cleanly — one new format script and a small interface extension to the orchestrator.
- Mixed decision logs work: `list` handles both sequential and work-item-prefixed files, so teams can adopt incrementally.

**Negative:**
- A work item must exist before an ADR can be created. The `local` vendor mitigates this for offline/early-stage work, but adds a concept developers must learn.
- Filename-based chronological ordering is lost. Teams accustomed to `ls docs/adr/` showing ADRs in creation order must use the `list` command (which sorts by date) instead.
- Cross-references change from `ADR-NNNN` to `ADR-{vendor}-{id}`. Both conventions coexist indefinitely — existing `ADR-NNNN` references are not migrated. If the cost of dual conventions becomes a problem, a bulk migration tool would be addressed in a separate ADR.
- The `new.sh` orchestrator requires a change to its interface for non-sequential formats. This is a minor but real modification to shared infrastructure.

**Neutral:**
- The nygard-agent template content is unchanged — only the naming and heading layer changes. Quality Strategy, checkpoints, and Comments sections work identically.
- This ADR defines the naming convention and format script architecture. The normalized work item data model and caching strategy are deferred to companion ADRs.
- The `local` vendor serves as both an offline escape hatch and a testing facility — ADRs can be created without any external system configured.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [ ] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

The format script needs tests for: new (with each vendor), list (mixed formats), rename, and status. The orchestrator change needs a test for format-controlled naming. User documentation in SKILL.md must cover the new format option and the cross-reference convention.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This is the first of three companion ADRs. ADR-35 (normalized work item data model) and ADR-36 (work item caching in .adr/var/) address the data layer. This ADR focuses on naming and format script architecture only.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Traditional ADR sequential numbering doesn't work for teams (no directory locks in git) and prevents structural work item references. A "wi-nygard-agent" format replaces sequential numbers with vendor-qualified work item IDs, enabling collision-free team authoring and traceability to GitHub Issues, ADO work items, and local trackers.

**Tolerance:**
- Risk: Medium — new naming convention is a significant change to developer workflow
- Change: High — willing to break sequential numbering convention
- Improvisation: Medium — open to creative approaches within the format dispatch architecture

**Uncertainty:**
- Certain: sequential numbering causes team collisions; format dispatch architecture supports new formats
- Uncertain: exact vendor prefix scheme; how `new.sh` orchestrator adapts; cross-reference migration path

**Options:**
- Target count: 3
- [x] Explore additional options beyond candidates listed below

**Candidates:**
- Work-item-prefixed naming (`{vendor}-{id}-{slug}.md`)
- Metadata-only work item links (keep sequential naming)
- UUID-based naming

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Does the "no architectural changes" claim in positive consequence 3 align with the orchestrator interface change described in the Decision section?

**Addressed** — Revised positive consequence 3 from "no architectural changes" to "one new format script and a small interface extension to the orchestrator." The original wording contradicted both the Decision section (which describes the orchestrator receiving a format-level signal) and negative consequence 4 (which acknowledges the interface modification).

### Q: Is the long-term strategy for cross-reference coexistence (`ADR-NNNN` vs `ADR-{vendor}-{id}`) explicit?

**Addressed** — Revised the cross-reference negative consequence to state that both conventions coexist indefinitely and existing `ADR-NNNN` references are not migrated. A bulk migration tool is explicitly deferred to a separate ADR if the cost of dual conventions warrants it. The Draft Worksheet had already flagged "cross-reference migration path" as uncertain — this revision makes the current position explicit rather than implicit.

### Q: What happens when two developers create ADRs for the same work item?

**Addressed** — Added a sentence to the Naming Convention section noting that same-work-item filename collisions are intentional: they surface a genuine decision conflict (two competing decisions for the same problem), caught during PR review. This makes the "collision-free" claim more precise — the scheme eliminates *accidental* numbering collisions while preserving *meaningful* conflicts.
