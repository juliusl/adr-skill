# 36. Cache work item snapshots in .adr/var/

Date: 2026-04-05
Status: Proposed
Last Updated: 2026-04-05
Links:
- [ADR-0034: Adopt work-item-referenced naming for ADR files](0034-adopt-work-item-referenced-naming-for-adr-files.md)
- [ADR-0035: Define normalized work item data model](0035-define-normalized-work-item-data-model-for-vendor-agnostic-adr-tooling.md)
- [ADR-0020: .adr/ project-scoped convention](0020-establish-adr-directory-as-project-scoped-convention.md)
- [ADR-0026: Rust CLI for data plumbing](0026-add-rust-cli-for-data-plumbing.md)
- [ADR-0027: Diesel with SQLite for adr-db](0027-use-diesel-with-sqlite-for-adr-db-persistence-layer.md)

## Context

ADR-0034 introduces work-item-referenced naming (`{vendor}-{id}-{slug}.md`) and ADR-0035 defines a normalized work item data model with vendor-specific adapters. Together, these decisions enable ADRs to structurally reference work items from GitHub, Azure DevOps, and local sources.

**The missing piece: where does the normalized work item data live?**

When the `wi-nygard-agent` format script creates an ADR, it needs the work item's title (for the ADR heading), description (for seeding context), and metadata (for the `Work-Item:` field). This data comes from vendor adapters (ADR-0035) that query MCP servers at creation time. But this creates several problems:

1. **Offline access.** If the developer is offline or the MCP server is unavailable, the format script cannot fetch work item data. Yet the `local` vendor exists precisely for offline scenarios — there should be a way to access previously fetched work items offline.

2. **Testing.** Testing the format script, list command, and future tooling requires mock work item data. Without a local store, tests must either mock MCP server responses (fragile) or skip work item integration entirely (incomplete).

3. **Portability and mirroring.** Moving a project to a different host (e.g., from GitHub to a self-hosted Gitea instance) or mirroring for backup loses the work item references unless they're captured locally.

4. **Future ingestion.** ADR-0026 introduces `adr-db`, a Rust CLI for ingesting JSONL data into SQLite. Work item data in JSONL format would be directly ingestible, enabling porcelain skills to query work items alongside ADR metadata — e.g., "show me all ADRs linked to open bugs" or "which decisions are motivated by this feature request?"

5. **Audit trail.** Work items change over time — titles are edited, states transition, descriptions are updated. A cached snapshot at ADR creation time preserves the work item's state when the decision was made, providing an audit trail that the live system doesn't.

**ADR-0020 provides the home.** The `.adr/var/` directory (established in ADR-0020) is designed for exactly this kind of transient, project-scoped data — gitignored by default, append-friendly, JSONL-compatible.

### Decision Drivers

- **Offline capability** — cached data must enable ADR operations without network access
- **JSONL compatibility** — data format must match the `.adr/var/` convention (ADR-0020) and adr-db ingestion pipeline (ADR-0026)
- **Append-friendly** — writing a new work item snapshot must be a simple append, not a read-modify-write cycle
- **Schema alignment** — cached data must use the normalized schema from ADR-0035
- **Discoverability** — cached work items must be easy to find by vendor and ID
- **Low overhead** — caching must not significantly slow down ADR creation or consume excessive disk space

## Options

### Option 1: Single JSONL file for all work items

Store all cached work items in `.adr/var/work-items.jsonl`. Each line is a normalized work item JSON object (ADR-0035 schema) with an additional `cached_at` timestamp. The format script appends a line when it first fetches a work item; subsequent lookups read the file.

**Strengths:**
- Simplest implementation — one file, append-only
- Directly compatible with adr-db's JSONL ingestion pipeline
- Follows the `.adr/var/` convention (ADR-0020) exactly — one JSONL file per concern
- Easy to back up or inspect — `cat .adr/var/work-items.jsonl | jq .`

**Weaknesses:**
- Lookup by vendor+ID requires scanning the entire file — O(n) per lookup
- File grows unboundedly — no rotation or compaction mechanism
- Duplicate entries (same work item fetched multiple times) accumulate without deduplication
- Concurrent ADR creation could produce interleaved writes (same concern noted in ADR-0020)

### Option 2: One JSONL file per vendor

Organize cached work items by vendor: `.adr/var/wi/gh.jsonl`, `.adr/var/wi/ado.jsonl`, `.adr/var/wi/local.jsonl`. Each file contains normalized work items from that vendor only.

**Strengths:**
- Partitioned by vendor — reduces scan scope for lookups
- Clear organization — vendor files are independently manageable
- Vendor-specific backup or cleanup is straightforward
- Still append-only and JSONL-compatible

**Weaknesses:**
- Slightly more complex than a single file — must know the vendor to find the right file
- Cross-vendor queries require reading multiple files
- Still O(n) per vendor for lookups (though n is smaller per file)
- Introduces a `wi/` subdirectory in `.adr/var/` — adds a nesting level

### Option 3: One JSON file per work item

Store each work item as an individual file: `.adr/var/wi/gh-42.json`, `.adr/var/wi/ado-1234.json`. File naming matches the ADR naming convention (ADR-0034). Each file contains the normalized work item JSON object.

**Strengths:**
- O(1) lookup — file existence check by vendor-id
- No scanning, no deduplication needed
- Natural mapping: one ADR file ↔ one work item file
- Easy to update — overwrite the file with a fresh snapshot
- Git-diffable if tracked — each work item is a separate diff

**Weaknesses:**
- Not JSONL — doesn't match the `.adr/var/` JSONL convention or adr-db ingestion format
- Many small files — could be hundreds in a large project
- Not append-only — requires write-in-place semantics
- Ingestion into adr-db requires concatenating files into JSONL first
- File-per-item doesn't capture history — overwrites lose previous snapshots

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

In the context of **needing a local store for normalized work item data to support offline access, testing, portability, and future adr-db ingestion**, facing **the tradeoff between lookup performance, JSONL compatibility, and implementation simplicity**, we decided for **a single JSONL file at `.adr/var/work-items.jsonl`** (Option 1), and neglected **per-vendor JSONL files (Option 2, marginal benefit for added complexity) and per-item JSON files (Option 3, incompatible with JSONL convention and adr-db pipeline)**, to achieve **the simplest possible caching layer that is directly compatible with the existing `.adr/var/` convention and adr-db ingestion**, accepting that **lookups are O(n) and duplicate entries accumulate without deduplication**.

### Cache File Location

```
.adr/var/work-items.jsonl
```

This file is gitignored (per ADR-0020's `.adr/.gitignore` convention) and created on first use.

### Record Format

Each line is a normalized work item (ADR-0035 schema) extended with a `cached_at` timestamp:

```json
{"vendor":"gh","id":"42","title":"Evaluate PostgreSQL","type":"issue","state":"open","url":"https://github.com/org/repo/issues/42","description":"...","labels":["adr"],"created":"2026-04-01T10:00:00Z","updated":"2026-04-05T07:00:00Z","cached_at":"2026-04-05T07:13:00Z"}
```

The `cached_at` field records when the snapshot was taken, distinguishing it from the work item's own `created`/`updated` timestamps.

### Write Path

When the format script creates a new ADR (`wi-nygard-agent-format.sh new`):

1. The vendor adapter fetches and normalizes the work item (ADR-0035)
2. The adapter output is appended to `.adr/var/work-items.jsonl` with a `cached_at` timestamp
3. The format script reads the appended record for title, description, and metadata

If `.adr/var/` doesn't exist, create it with `mkdir -p` (the `init-data` Makefile target bootstraps this, but the format script should be resilient to its absence).

### Read Path

To look up a cached work item by vendor and ID:

```bash
grep "\"vendor\":\"gh\"" .adr/var/work-items.jsonl | grep "\"id\":\"42\"" | tail -1
```

The `tail -1` returns the most recent snapshot when duplicates exist. For more sophisticated querying, use `jq`:

```bash
jq -s 'map(select(.vendor == "gh" and .id == "42")) | last' .adr/var/work-items.jsonl
```

### Offline Behavior

When the MCP server is unavailable (offline or error):

1. Check the cache for an existing snapshot of the requested work item
2. If found, use the cached data (warn the user it may be stale: `"Using cached data from {cached_at}"`)
3. If not found, fall back to `local` vendor behavior — prompt for title and generate a local ID

### Deduplication Strategy

Duplicates (same vendor+id, different cached_at) are tolerated by design:
- Read operations use `tail -1` to get the latest snapshot
- Storage cost is minimal (each record is ~200-500 bytes)
- Periodic compaction is a future concern — if the file grows large, a utility can deduplicate by keeping only the latest snapshot per vendor+id

### adr-db Ingestion

The cache file is directly ingestible by `adr-db` (ADR-0026):

```bash
adr-db ingest --table work_items .adr/var/work-items.jsonl
```

Note: the `work_items` table is a future addition to adr-db — ADR-0026 defines the generic JSONL ingestion pipeline, and work items would be a new ingestion target. This enables porcelain skills to query work items via SQL — e.g., "which ADRs are linked to open bugs?" by joining the work items table with ADR metadata.

## Consequences

**Positive:**
- Enables offline ADR creation by providing locally cached work item data — developers can create ADRs without network access if the work item was previously fetched.
- The JSONL format is directly compatible with the `.adr/var/` convention (ADR-0020) and the adr-db ingestion pipeline (ADR-0026) — no format conversion needed.
- Append-only writes are simple and safe for single-writer scenarios — each ADR creation appends one line.
- Cached snapshots create an audit trail: the `cached_at` timestamp preserves the work item's state at ADR creation time, even if the work item is later edited in the source system.
- Testing becomes straightforward — test fixtures can seed `.adr/var/work-items.jsonl` with known data, no MCP server mocking needed.

**Negative:**
- O(n) lookup performance — finding a work item requires scanning the file. This is acceptable for the expected scale (tens to hundreds of work items per project) but would degrade for very large projects.
- Duplicate entries accumulate — fetching the same work item multiple times appends multiple records. This wastes space and makes the file harder to read manually, though `tail -1` semantics handle it correctly.
- No concurrent write safety — two simultaneous ADR creations could interleave JSONL lines. This mirrors the same concern from ADR-0020 and is deferred to a future ADR.
- The cache is local-only (gitignored) — team members don't share cached work items. This is by design (transient data), but means each developer must fetch their own work items at least once.

**Neutral:**
- The `cached_at` field is the only schema extension beyond ADR-0035's normalized model. Future extensions (e.g., `cached_by` for the user who fetched it) can be added as optional fields.
- Compaction/deduplication tooling is explicitly deferred. The current design tolerates unbounded growth because the expected data volume is small (hundreds of records at most in typical projects).
- This ADR completes the three-part series: ADR-0034 (naming) → ADR-0035 (data model) → ADR-0036 (caching). Together they define the full `wi-nygard-agent` format stack.

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
- [ ] User documentation

### Additional Quality Concerns

Unit tests should verify: write path (append to JSONL), read path (lookup by vendor+id, latest-wins semantics), offline fallback behavior, and resilience to missing `.adr/var/` directory. The `init-data` Makefile target should be verified to create the directory structure correctly. User documentation impact is minimal since caching is transparent to end users.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This is the third of three companion ADRs. ADR-0034 (naming convention) and ADR-0035 (data model) provide the upstream decisions. This ADR focuses on the storage layer — where and how normalized work item data is cached locally.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Normalized work item data (ADR-0035) needs a local home for offline access, testing, and future adr-db ingestion. The .adr/var/ directory (ADR-0020) is the natural location. The cache should be append-only JSONL for simplicity and compatibility.

**Tolerance:**
- Risk: Low — caching is well-understood, the .adr/var/ convention is established
- Change: Low — this is an incremental addition to existing infrastructure
- Improvisation: Low — the JSONL approach is straightforward

**Uncertainty:**
- Certain: .adr/var/ is the right location; JSONL is the right format; adr-db can ingest JSONL
- Uncertain: whether deduplication matters in practice; exact scale of work item data per project

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Single JSONL file
- Per-vendor JSONL files
- Per-item JSON files
