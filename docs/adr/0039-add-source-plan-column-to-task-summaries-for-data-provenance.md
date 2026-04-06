# 39. Add source plan column to task summaries for data provenance

Date: 2026-04-06
Status: Planned
Last Updated: 2026-04-06
Links:
- Extends [ADR-0026](0026-add-rust-cli-for-data-plumbing.md) (adds provenance tracking to the plumbing data model)
- Extends [ADR-0027](0027-use-diesel-with-sqlite-for-adr-db-persistence-layer.md) (new Diesel migration for schema change)
- Extends [ADR-0029](0029-add-view-subcommand-for-diagnostic-database-inspection.md) (view command must display the new column)

## Context

The `task_summaries` table stores task execution records ingested from implementation plan files via `extract-summary.awk | adr-db ingest`. Each row has `task_id`, `status`, `cost`, `commit_sha`, and `description` — but **no field identifying which plan the task belongs to**.

**The problem:** When multiple plans are ingested, all rows merge into a single flat table with no way to distinguish which plan produced them. Task IDs are only unique within a plan (e.g., every plan has a "1.1"), so without a plan identifier, the data is ambiguous and queries like "show me the tasks from plan 0029" are impossible.

**Why this matters now:**
- The `view` command (ADR-0029) exposes this gap — `adr-db view task_summaries` shows all rows with no plan context.
- As more plans are implemented and ingested, the table becomes increasingly useless without provenance.
- The JSONL producer (`extract-summary.awk`) already receives the plan filename as its argument (`awk -f extract-summary.awk docs/plans/0029.0.plan.md`) — the provenance data is available at the call site but is discarded.

**Breaking changes:** Not a concern. `adr-db` is unreleased development tooling with no external users. The instability contract (ADR-0029) applies to the `view` command, and the entire tool has no published API contract.

**Constraints:**
- **Diesel migration** — schema changes must go through the Diesel migration workflow (ADR-0027).
- **JSONL compatibility** — the `ingest` command must continue to accept existing JSONL format (records without a `source_plan` field should still work).
- **Pipeline composability** — the solution must work in the existing `awk | adr-db ingest` pipeline pattern.

### Decision Drivers

- **Data integrity** — every row should be traceable to its source plan without ambiguity.
- **Pipeline ergonomics** — the solution should require minimal changes to existing ingestion workflows.
- **Backward compatibility of JSONL format** — existing JSONL records (without `source_plan`) should remain ingestible.
- **Query utility** — users should be able to filter and group by plan in both TSV and JSONL output.

## Options

### Option 1: Add `source_plan` to the JSONL record

Modify `extract-summary.awk` to emit a `source_plan` field in each JSONL line, derived from the input filename. The `ingest` command deserializes it and stores it in a new `source_plan` column.

```
$ awk -f extract-summary.awk docs/plans/0029.0.plan.md
{"task_id":"1.1","status":"done","cost":"small","commit":"abc1234","description":"...","source_plan":"0029.0.plan.md"}
```

**Strengths:**
- Each JSONL record is self-contained — provenance travels with the data.
- No changes to `ingest` CLI interface — it's just another field.
- Follows the existing pattern: structured data flows through JSONL.

**Weaknesses:**
- Requires modifying the awk script to extract the filename.
- Every JSONL line repeats the same `source_plan` value — redundant for records from the same file.
- If the user pipes JSONL from a non-file source, `source_plan` may be empty or wrong.

### Option 2: Add `--source` flag to `adr-db ingest`

Add a `--source <plan>` CLI flag to the `ingest` command. The flag value is stored in the `source_plan` column for every record in that ingestion batch.

```
$ awk -f extract-summary.awk docs/plans/0029.0.plan.md | adr-db ingest --source 0029.0.plan.md
```

**Strengths:**
- Clean separation: JSONL carries data, CLI flag carries metadata.
- No awk changes needed — the plan name is provided at the call site.
- No redundancy — the source is stated once, applied to all records.

**Weaknesses:**
- Changes the `ingest` CLI interface — existing invocations without `--source` need updating or a default.
- The provenance is outside the JSONL — if someone stores/replays JSONL records, the source is lost.
- Users must remember to pass `--source` — easy to forget.

### Option 3: Derive source from filename automatically in `ingest`

Have `ingest` accept a file argument instead of (or in addition to) stdin. When a file is provided, derive `source_plan` from the filename automatically.

```
$ adr-db ingest docs/plans/0029.0.plan.md
```

**Strengths:**
- Zero friction — no extra flags, no awk changes.
- Provenance is automatic and correct.

**Weaknesses:**
- Breaks the stdin pipeline pattern established by ADR-0026.
- Conflates two responsibilities: JSONL parsing and plan-name extraction.
- Doesn't work when JSONL comes from a non-file source (e.g., piped from another tool).
- The input file is a JSONL file, not the plan itself — the filename mapping is indirect.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — all options use straightforward patterns (awk filename extraction, Diesel migrations, clap arguments).

## Decision

In the context of **needing to trace task summary records back to their source plan**, facing **the need to maintain pipeline composability and JSONL self-containment**, we decided for **adding `source_plan` to the JSONL record (Option 1)** and neglected **a CLI flag on ingest (Option 2, provenance lost outside JSONL) and automatic filename derivation (Option 3, breaks stdin pipeline pattern)**, to achieve **self-contained JSONL records where provenance travels with the data through any pipeline**, accepting that **the awk script needs modification and each JSONL line redundantly repeats the same source plan value**.

### Schema change

Add a `source_plan` column to `task_summaries`:

```sql
ALTER TABLE task_summaries ADD COLUMN source_plan TEXT NOT NULL DEFAULT '';
```

The `DEFAULT ''` ensures existing rows and JSONL records without a `source_plan` field remain valid — backward compatibility is preserved.

### JSONL format change

`extract-summary.awk` will emit `source_plan` derived from the input filename, stripped to the basename (e.g., `docs/plans/0029.0.plan.md` → `0029.0.plan.md`):

```json
{"task_id":"1.1","status":"done","cost":"small","commit":"abc1234","description":"Create config file","source_plan":"0029.0.plan.md"}
```

### Ingest handling

The `JsonlTaskRecord` struct gains an optional `source_plan` field:

```rust
#[derive(Deserialize)]
pub struct JsonlTaskRecord {
    pub task_id: String,
    pub status: String,
    pub cost: String,
    pub commit: String,
    pub description: String,
    #[serde(default)]
    pub source_plan: String,
}
```

`#[serde(default)]` ensures records without `source_plan` deserialize successfully with an empty string — no breaking change to existing JSONL producers.

### View output

- **TSV mode:** `source_plan` appears as the first column (before `task_id`) to emphasize provenance grouping.
- **JSONL mode:** `source_plan` is included in the JSON output with the same field name used by ingest.

### Re-ingestion semantics

When the same plan is ingested multiple times, duplicate rows accumulate. This is acceptable for now — deduplication is a future concern. The `source_plan` + `task_id` combination uniquely identifies a task within a plan but is not enforced as a unique constraint.

## Consequences

**Positive:**
- Every task summary row is traceable to its source plan — `adr-db view task_summaries` shows which plan each task came from.
- JSONL records are self-contained — provenance travels with the data through pipelines, storage, and replay.
- Backward compatible — existing JSONL without `source_plan` still ingests successfully via `#[serde(default)]` and `DEFAULT ''`.
- Enables plan-scoped queries: `adr-db view task_summaries --no-header | awk -F'\t' '$1 == "0029.0.plan.md"'`.

**Negative:**
- Each JSONL line redundantly includes `source_plan` — for a plan with 9 tasks, the field is repeated 9 times. This is negligible for the expected data volumes.
- `extract-summary.awk` grows slightly more complex to extract and emit the filename.

**Neutral:**
- Deduplication on re-ingestion is deferred — rows accumulate if a plan is ingested multiple times. A future `--replace` flag or upsert behavior could address this.
- The `source_plan` value is the plan filename basename, not a full path. This is sufficient for identification within a project.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [x] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] Tooling
- [ ] User documentation

### Additional Quality Concerns

- **Migration safety** — `ALTER TABLE ADD COLUMN` with a default value is safe in SQLite and does not require rewriting existing rows.
- **Backward JSONL compatibility** — unit tests must verify that records without `source_plan` ingest successfully.
- **View column ordering** — unit tests must verify `source_plan` appears in both TSV and JSONL output.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** User documentation is unchecked because the README was already updated for `view` in ADR-0029 and the column change will be visible automatically. No new user-facing documentation is needed beyond what the `view` command shows.

---

## Comments
