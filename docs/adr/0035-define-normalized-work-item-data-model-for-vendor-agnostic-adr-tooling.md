# 35. Define normalized work item data model for vendor-agnostic ADR tooling

Date: 2026-04-05
Status: Proposed
Last Updated: 2026-04-05
Links:
- [ADR-0034: Adopt work-item-referenced naming for ADR files](0034-adopt-work-item-referenced-naming-for-adr-files.md)
- [ADR-0020: .adr/ project-scoped convention](0020-establish-adr-directory-as-project-scoped-convention.md)
- [ADR-0026: Rust CLI for data plumbing](0026-add-rust-cli-for-data-plumbing.md)

## Context

ADR-0034 introduces work-item-referenced naming (`{vendor}-{id}-{slug}.md`), enabling ADRs to structurally reference the work items that motivate them. This requires the ADR tooling to interact with multiple work item systems — each with different concepts, APIs, and data models:

**GitHub Issues:** Flat issue model. Issues have a number, title, body, labels, assignees, milestone, and state (open/closed). Pull requests are a special type of issue. The GitHub MCP server provides access via `github-mcp-server-issue_read`, `github-mcp-server-list_issues`, and `github-mcp-server-search_issues`.

**Azure DevOps Work Items:** Hierarchical model with typed work items — Bug, User Story, Task, Feature, Epic. Each has an ID, title, description, state (New/Active/Resolved/Closed), area path, iteration path, and assigned-to. The ADO MCP server provides access via `ado-wit_get_work_item`, `ado-wit_my_work_items`, and `ado-search_workitem`.

**Local/Offline:** No external system. Developers may want to create ADRs without any tracker configured — either because the project doesn't use one, the developer is offline, or the work item will be created later.

**The problem:** The `wi-nygard-agent` format script (ADR-0034) and any downstream tooling (caching, adr-db ingestion) need a common data model to work with. Without normalization, every tool must handle GitHub-specific, ADO-specific, and local-specific data structures independently — tripling the integration surface and making the system fragile to vendor API changes.

**Why normalization matters:**
- The format script needs a title to suggest ADR titles and populate context
- Cached work items (ADR-0036) need a common schema for consistent storage
- Future adr-db ingestion needs a stable schema to build queries against
- Testing becomes simpler when all vendors produce the same shape of data
- Mirroring to local git servers (like Gitea) requires a portable format

### Decision Drivers

- **Vendor agnosticism** — the data model must not leak vendor-specific concepts that force downstream tools to specialize per vendor
- **Minimal viable model** — capture only what ADR tooling actually needs, not the full richness of each vendor's API
- **Extensible** — vendors may add fields; the model must tolerate unknown fields without breaking
- **Offline capable** — must work without network access when using the `local` vendor
- **Machine readable** — must support structured querying (not just human-readable text)
- **MCP server compatibility** — the adapter layer must work with existing GitHub and ADO MCP servers available in the Copilot CLI environment

## Options

### Option 1: Vendor-specific adapters with no common model

Each vendor produces its own data structure. The format script has `if vendor == gh ... elif vendor == ado ...` branches. Tools downstream consume vendor-specific shapes. No normalization layer.

**Strengths:**
- No abstraction to design — directly use each API's native response
- Full fidelity — no information loss from normalization

**Weaknesses:**
- Every tool must implement per-vendor logic — N tools × M vendors = N×M integration points
- Vendor API changes propagate to all tools
- Testing requires mocking each vendor separately
- Caching requires vendor-specific schemas
- No portability between vendors

### Option 2: Normalized data model with vendor adapters

Define a common work item data model that captures the fields ADR tooling needs. Each vendor has an adapter (a shell function or script) that transforms vendor-specific data into the normalized model. The normalized model is what gets cached, queried, and displayed.

The model is a flat JSON object with fields:

```json
{
  "vendor": "gh",
  "id": "42",
  "title": "Evaluate PostgreSQL for event storage",
  "type": "issue",
  "state": "open",
  "url": "https://github.com/org/repo/issues/42",
  "description": "Brief summary (first 500 chars of body)",
  "labels": ["adr", "architecture"],
  "created": "2026-04-01T10:00:00Z",
  "updated": "2026-04-05T07:00:00Z"
}
```

**Strengths:**
- Single data shape for all downstream tools — N tools × 1 model
- Vendor changes are isolated to adapters — tools never touch vendor-specific data
- Caching and adr-db ingestion get a stable, queryable schema
- Testing requires only one mock shape (plus adapter tests per vendor)
- Portable between vendors — a work item cached from GitHub looks the same as one from ADO

**Weaknesses:**
- Information loss — ADO area paths, GitHub milestones, and other vendor-specific fields are not in the common model
- Abstraction design cost — must decide what fields to include and what to drop
- Type normalization is lossy — ADO's "Bug" vs "User Story" vs "Task" collapse to `type: "bug"`, `type: "story"`, `type: "task"`, but these don't map perfectly to GitHub's flat "issue" type
- Requires adapter maintenance — each vendor needs an adapter, and adapters must be updated when vendor APIs change

### Option 3: Schema-on-read with vendor-tagged raw data

Store the raw vendor response as-is, tagged with a `vendor` field. Normalization happens at read time — each consumer applies its own mapping. The stored data is the full vendor response, preserving all fields.

**Strengths:**
- No information loss — raw vendor data is preserved
- No upfront schema design — defer normalization decisions
- Future consumers can extract fields the original adapter didn't anticipate

**Weaknesses:**
- Every consumer must implement normalization — moves complexity downstream instead of eliminating it
- Raw responses vary in structure and size — GitHub issues are ~2KB, ADO work items can be ~10KB with all fields
- Schema-on-read is harder to test — no single shape to validate against
- Caching becomes vendor-specific storage — adr-db must handle multiple schemas
- Breaks the goal of vendor-agnostic tooling

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

In the context of **building vendor-agnostic ADR tooling that interacts with multiple work item systems**, facing **the need to support GitHub Issues, Azure DevOps work items, and local/offline workflows without per-vendor specialization in every tool**, we decided for **a normalized work item data model with vendor-specific adapters** (Option 2), and neglected **vendor-specific adapters with no common model (Option 1, N×M integration surface) and schema-on-read with raw data (Option 3, moves complexity downstream)**, to achieve **a single data shape that all downstream tools consume, isolating vendor-specific logic to adapter boundaries**, accepting that **some vendor-specific fields are lost in normalization and the type mapping between vendors is imperfect**.

### Normalized Work Item Schema

```json
{
  "vendor": "gh | ado | local",
  "id": "string",
  "title": "string",
  "type": "issue | bug | story | task | feature | epic | other",
  "state": "open | active | resolved | closed",
  "url": "string (optional, empty for local)",
  "description": "string (first 500 chars, optional)",
  "labels": ["string"],
  "created": "ISO 8601 datetime",
  "updated": "ISO 8601 datetime"
}
```

**Field semantics:**

| Field | Description |
|-------|-------------|
| `vendor` | Source system identifier. Matches the prefix in ADR filenames (ADR-0034). |
| `id` | Vendor-scoped identifier. GitHub issue number, ADO work item ID, or generated local hash. |
| `title` | Human-readable title. Maps directly from all vendors. |
| `type` | Normalized work item type. See type mapping below. |
| `state` | Normalized lifecycle state. See state mapping below. |
| `url` | Web URL to the work item in its source system. Empty for `local` vendor. |
| `description` | Truncated body/description (max 500 chars). Provides context without storing full content. |
| `labels` | Tags/labels from the source system. ADO uses "Tags" field; GitHub uses labels. |
| `created` | Creation timestamp in ISO 8601. |
| `updated` | Last update timestamp in ISO 8601. |

### Type Mapping

| Normalized Type | GitHub | Azure DevOps |
|----------------|--------|-------------|
| `issue` | Issue | — |
| `bug` | Issue (label: bug) | Bug |
| `story` | — | User Story |
| `task` | — | Task |
| `feature` | — | Feature |
| `epic` | — | Epic |
| `other` | — | Any unlisted type |

GitHub has a flat type model (everything is an "issue"). ADO has a rich hierarchy. The mapping is intentionally lossy — ADR tooling cares about the existence and identity of the work item, not its exact type semantics.

**GitHub label-to-type inference:** The `gh_adapter` defaults to `type: "issue"` for all GitHub issues. If a recognized label is present, the adapter promotes the type — e.g., an issue with the `bug` label maps to `type: "bug"`. The set of recognized labels is minimal by default (`bug` only) and can be extended via adapter configuration. Unrecognized labels are preserved in the `labels` array but do not affect the `type` field.

### State Mapping

| Normalized State | GitHub | Azure DevOps |
|-----------------|--------|-------------|
| `open` | open | New |
| `active` | — | Active |
| `resolved` | — | Resolved |
| `closed` | closed | Closed |

GitHub's binary state model maps directly: `open`→`open`, `closed`→`closed`. The `gh_adapter` emits only these two states. ADO's richer state machine (New/Active/Resolved/Closed) maps to all four normalized states. The `active` and `resolved` states are ADO-only — they exist in the normalized model to preserve ADO's lifecycle semantics without collapsing them.

### Adapter Interface

Each vendor adapter is a function or script that:
1. Accepts vendor-specific input (API response, MCP tool output, or manual input)
2. Produces a JSON object conforming to the normalized schema
3. Writes the result to stdout

```bash
# Adapter contract (pseudo-shell)
gh_adapter()   { # GitHub MCP response → normalized JSON }
ado_adapter()  { # ADO MCP response → normalized JSON }
local_adapter(){ # User input → normalized JSON with generated ID }
```

Adapters are invoked by the format script or caching layer, not by end users. The adapter interface is internal plumbing.

### Extensibility

The schema is intentionally minimal. Vendor-specific fields that ADR tooling doesn't need (ADO area paths, GitHub milestones, PR associations) are dropped during normalization. If future tooling needs additional fields, the schema can be extended by adding optional fields — existing data remains valid because consumers ignore unknown fields.

## Consequences

**Positive:**
- All downstream tools (format script, cache, adr-db, porcelain commands) consume a single data shape — reducing integration complexity from N×M to N+M.
- Vendor-specific logic is isolated to adapters — a new vendor requires one adapter, not changes to every tool.
- The normalized schema is simple enough to construct manually for the `local` vendor — enabling offline ADR creation and testing without mock servers.
- The schema maps naturally to a SQLite table for adr-db ingestion — most fields map directly, with the `labels` array requiring JSON serialization or a join table.
- Cached work items use a common structure — a project that moves from GitHub to ADO retains historical work item metadata, though URLs and IDs remain vendor-scoped.

**Negative:**
- Type mapping between vendors is lossy — GitHub's flat "issue" type doesn't distinguish bugs from features. Teams using GitHub must rely on labels for type discrimination, which is convention-dependent.
- State mapping is approximate — ADO's "Resolved" vs "Closed" distinction is semantically meaningful but collapsed in the normalization.
- The 500-character description truncation may lose important context. This is a tradeoff between cache size and utility — the URL field provides access to the full work item when needed.
- Adapter maintenance — each vendor adapter must be updated if the MCP server API changes. This is a maintenance burden, but it's isolated to the adapter layer.

**Neutral:**
- The adapter interface is internal plumbing — end users never invoke adapters directly. The format script and caching layer handle adapter invocation transparently.
- This ADR defines the data model and adapter interface. The caching strategy (where and how normalized data is stored) is deferred to ADR-0036.
- The `local` vendor adapter generates IDs locally (e.g., `date +%s | sha256sum | head -c 8`). The specific ID generation algorithm is an implementation detail, not an architectural decision.

## Quality Strategy

- [ ] Introduces major semantic changes
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

Adapter tests should validate that each vendor adapter produces valid normalized JSON. A JSON schema or validation function can enforce the contract. The type and state mappings should be documented for users who need to understand how their vendor's concepts map to the normalized model.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This is the second of three companion ADRs. ADR-0034 (naming convention) provides the naming layer. ADR-0036 (work item caching) addresses the storage layer. This ADR focuses on the data model that bridges naming and storage.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Multiple work item systems (GitHub, ADO, local) have different data models. ADR tooling needs a common interface to avoid per-vendor specialization in every tool. The priority is normalizing the interface to enable multiple vendors.

**Tolerance:**
- Risk: Low — data model design is well-understood; the main risk is over-engineering
- Change: Medium — introducing a new abstraction layer
- Improvisation: Low — stay close to what the tooling actually needs

**Uncertainty:**
- Certain: GitHub and ADO have different concepts; MCP servers provide access; we need a common shape
- Uncertain: exact type mapping semantics; whether 500-char description truncation is sufficient; future field needs

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Normalized model with adapters (schema-on-write)
- No normalization (vendor-specific throughout)
- Schema-on-read with raw vendor data

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Does the state mapping table accurately reflect what `gh_adapter` emits?

**Addressed** — Fixed inconsistency between table and prose. The table previously mapped GitHub `open` to both `open` and `active`, and `closed` to both `resolved` and `closed`, contradicting the prose. Updated the table to show dashes for GitHub in the `active` and `resolved` rows, and expanded the prose to explicitly state that `gh_adapter` emits only `open`/`closed` and that `active`/`resolved` are ADO-only states.

### Q: Does the ADR specify how `gh_adapter` infers work item types from GitHub labels?

**Addressed** — Added a paragraph under the Type Mapping section specifying that `gh_adapter` defaults to `type: "issue"` and promotes to `type: "bug"` when the `bug` label is present. The recognized label set is minimal by default and extensible via adapter configuration. Unrecognized labels are preserved in the `labels` array but don't affect `type`.

### Q: Does the SQLite mapping consequence accurately describe the transformation needed?

**Addressed** — Revised C4 to acknowledge that the `labels` array requires JSON serialization or a join table. Changed "no intermediate transformation needed" to "most fields map directly, with the `labels` array requiring JSON serialization or a join table."

### Q: Does the portability consequence accurately reflect vendor-scoped limitations?

**Addressed** — Revised C5 to qualify the portability claim. Changed "can still read its cached work item history" to "retains historical work item metadata, though URLs and IDs remain vendor-scoped."
