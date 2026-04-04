# 29. Add view subcommand for diagnostic database inspection

Date: 2026-04-04
Status: Proposed
Last Updated: 2026-04-04
Links:
- Extends [ADR-0026](0026-add-rust-cli-for-data-plumbing.md) (adds a read command to the plumbing CLI — blurs the plumbing/porcelain boundary intentionally)
- Uses [ADR-0027](0027-use-diesel-with-sqlite-for-adr-db-persistence-layer.md) (Diesel match-per-table approach for SQL injection prevention)

## Context

`adr-db` currently has two commands: `init` (create schema) and `ingest` (JSONL stdin → database). Both are write-only plumbing. Once data is ingested, there is no way to inspect it through `adr-db` itself — the only option is to reach for `sqlite3` directly.

**The problem:** After ingesting JSONL data, there's no built-in way to verify what's stored or inspect the database state. This makes debugging ingestion issues, verifying data integrity, and simply understanding what's in the database unnecessarily friction-heavy.

**Why this matters:**
- Users running `extract-summary.awk plan.md | adr-db ingest` have no feedback — did it work? What got stored? How many records?
- As more tables are added (future JSONL producers), knowing which tables exist and what they contain becomes increasingly valuable.
- The downstream porcelain project doesn't exist yet. Until it does, there's zero visibility into the data store.

**The boundary question:** ADR-0026 explicitly stated that "query/filter/search — that's porcelain for the downstream project." Adding a read command to `adr-db` blurs the plumbing/porcelain line. However, this is intentionally framed as **proto-porcelain** — an unstable, diagnostic inspection capability, not a stable query interface. Analogies:
- Git plumbing has read commands (`cat-file`, `ls-tree`) that serve diagnostic needs without being porcelain.
- This is the "caveman version" of the downstream project — rough, unstable, no contract with external tooling.

**Key characteristic: instability is a feature.** This command carries no stability guarantees. Its output format, flags, and behavior may change at any time. It exists for human inspection, not for scripts or downstream tooling to depend on.

**Constraints:**
- **Unstable** — no output format contract. This is explicitly not a stable interface.
- **Read-only** — must not modify the database.
- **Human-oriented** — output should be human-readable, not necessarily machine-parseable.
- **Same `--db-path` convention** — reuses the existing database path resolution.

### Decision Drivers

- **Unix composability** — default output should be friendly to `awk`, `grep`, `sort`, `cut`. Following Linux principles.
- **Plumbing identity** — the command should feel like plumbing (diagnostic), not porcelain (product). No stability contract.
- **Discoverability** — users should be able to discover what tables exist without knowing the schema.
- **Round-trip symmetry** — bonus if the output can be piped back into `ingest` (JSONL mode).
- **Minimal implementation complexity** — the view command should be simple to build and maintain.

## Options

### Option 1: `adr-db view` — Structured subcommand with formatted table output

Add a `view` subcommand with sub-operations:
- `adr-db view tables` — list table names
- `adr-db view task_summaries` — show rows in formatted table (columns, borders)

Uses a Rust table-formatting crate (e.g., `comfy-table`, `tabled`) for human-readable output.

**Strengths:**
- Most "proto-porcelain" — visually polished output for humans.
- Discoverability via `adr-db view --help`.

**Weaknesses:**
- Adds a formatting dependency for a diagnostic command.
- Formatted table output is not composable — hard to pipe to `awk`, `grep`, `jq`.
- Output format is coupled to the formatting library's conventions.

### Option 2: `adr-db dump` — Inverse of ingest (database → JSONL stdout)

Add a `dump` subcommand that emits table contents as JSONL to stdout:
- `adr-db dump task_summaries` → one JSONL line per row.

The symmetry with `ingest` (JSONL in → JSONL out) keeps the tool in the plumbing ecosystem.

**Strengths:**
- Pure plumbing — JSONL in, JSONL out. Composable with `jq`, `wc -l`, `head`.
- No formatting dependency.
- Symmetric with `ingest`.

**Weaknesses:**
- JSONL is not great for quick human scanning — need `jq` or `column` for readability.
- Doesn't list tables — only dumps a named table.
- Feels like a separate tool rather than diagnostic inspection.

### Option 3: `adr-db sql` — Raw SQL passthrough

Add a `sql` subcommand that executes arbitrary SQL:
- `adr-db sql "SELECT * FROM task_summaries"`

A thin wrapper around SQLite that knows the database path.

**Strengths:**
- Maximum flexibility — any query, any table.
- Zero opinionation — output mirrors SQLite's native format.
- No new abstractions to maintain.

**Weaknesses:**
- Users must know SQL and table/column names.
- No discoverability — `adr-db sql` doesn't tell you what tables exist.
- Feels redundant with `sqlite3 .adr/var/adr.db` — adds little value over direct SQLite access.
- SQL injection surface if ever used programmatically (unlikely for diagnostic tooling).

### Option 4: `adr-db view` with output format control

Hybrid of Options 1 and 2. A `view` subcommand with two modes:

- `adr-db view` (no args) — list all table names
- `adr-db view task_summaries` — show rows in awk-friendly delimited format (default)
- `adr-db view task_summaries --output jsonl` — emit rows as JSONL to stdout
- `adr-db view task_summaries --limit 10` — limit output rows

**Default output is awk-friendly** — tab-separated or pipe-separated columnar format with a header row. Follows the Linux principle of producing output that composes with `awk`, `grep`, `sort`, `cut`. No fancy table formatting.

**JSONL mode** via `--output jsonl` provides machine-readable output for programmatic consumption — the same JSONL format that `ingest` consumes, closing the read/write loop.

```
$ adr-db view
task_summaries

$ adr-db view task_summaries
task_id	status	cost	commit_sha	description
1.1	done	small	abc1234	Create config file
1.2	done	medium	def5678	Add validation logic

$ adr-db view task_summaries --output jsonl
{"task_id":"1.1","status":"done","cost":"small","commit":"abc1234","description":"Create config file"}
{"task_id":"1.2","status":"done","cost":"medium","commit":"def5678","description":"Add validation logic"}
```

**Strengths:**
- Default output is composable with standard Unix tools (`awk`, `cut`, `sort`, `grep`).
- JSONL mode closes the read/write loop with `ingest` — data can round-trip.
- `adr-db view` (no args) provides table discoverability.
- No formatting dependencies — tab-separated output is trivial to produce.
- Clean separation: human-default, machine-flag.

**Weaknesses:**
- Two output modes means two code paths to maintain (though both are simple).
- Tab-separated output can break if field values contain tabs (unlikely for ADR data but possible).
- The `--output` flag pattern may invite scope creep (CSV, table, markdown...).

### Decision Driver: Dynamic queries vs. Diesel's static type system

All options require **dynamic queries** — listing tables (`SELECT name FROM sqlite_master`), selecting all columns from an arbitrary table (`SELECT * FROM <table>`). Diesel's type-safe query builder is designed for **static schemas** — it compiles queries against known `schema.rs` types.

For the `view` command, this can be handled via a **Diesel match-per-table approach**: the table name from user input is matched against known table names in a Rust `match` statement, and each arm executes a typed Diesel query. Unknown tables are rejected at the match level.

**This approach doubles as a security boundary.** The match arms are a structural whitelist — only known tables produce queries, and user-supplied strings never appear in SQL. SQL injection is impossible by construction. An alternative like rusqlite would require `format!("SELECT * FROM {table_name}")` — a textbook injection vector requiring runtime validation.

The tradeoff: every new table requires adding a match arm to the view command. This is acceptable because tables are added infrequently (each gets an ADR + migration + Insertable struct), and the match update is a single-line change.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — all options use well-understood patterns (TSV output, JSONL serialization, Diesel queries). An independent evaluator agent confirmed Option 4 as the winner against all 5 decision drivers.

## Decision

In the context of **needing diagnostic inspection of the `adr-db` database after ingestion**, facing **the tension between plumbing identity (no stability contract) and human usability (readable output)**, we decided for **an `adr-db view` subcommand with awk-friendly TSV default and `--output jsonl` flag (Option 4)** and neglected **formatted table output (Option 1, not composable), JSONL-only dump (Option 2, poor discoverability), and raw SQL passthrough (Option 3, redundant with sqlite3)**, to achieve **a diagnostic inspection capability that is composable with Unix tools by default, provides table discoverability, and supports round-trip JSONL symmetry with ingest**, accepting that **this is explicitly proto-porcelain with no stability contract, and each new table requires a match arm update in the view command**.

### Subcommand design

| Invocation | Behavior |
|-----------|----------|
| `adr-db view` | List all table names (one per line) |
| `adr-db view task_summaries` | Show rows in TSV format with header row |
| `adr-db view task_summaries --output jsonl` | Emit rows as JSONL to stdout |
| `adr-db view task_summaries --limit 10` | Limit output to N rows |
| `adr-db view task_summaries --no-header` | Suppress header row in TSV mode |

### Output formats (hard-capped at two)

**TSV (default):** Tab-separated values with a header row. Awk-friendly, grep-friendly, human-scannable. The `description` field is truncated to 60 characters in TSV mode to avoid line-wrapping and tab-in-value issues. Full fidelity is available via `--output jsonl`.

```
task_id	status	cost	commit_sha	description
1.1	done	small	abc1234	Create config file
1.2	done	medium	def5678	Add validation logic
```

**JSONL (`--output jsonl`):** One JSON object per line, matching the format that `ingest` consumes. Closes the read/write loop — data can round-trip between `view --output jsonl` and `ingest`.

**No other formats will be added.** TSV and JSONL are the two output modes. Requests for CSV, YAML, markdown tables, etc. are out of scope.

### Query implementation: Diesel match-per-table

The table name from user input is validated via a Rust `match` statement. Each arm executes a typed Diesel query. Unknown table names produce an error without executing any SQL.

```rust
match table_name {
    "task_summaries" => query_task_summaries(&mut conn, limit),
    _ => Err(format!("unknown table: {table_name}")),
}
```

This is a **structural security boundary** — user-supplied strings never appear in SQL. SQL injection is impossible by construction. The cost is one match arm per table, which aligns with the existing migration workflow (each new table already requires a Diesel model).

### Table listing

`adr-db view` (no args) lists table names by querying `sqlite_master`. This is the one dynamic query in the command — it uses `diesel::sql_query()` with a static string (`"SELECT name FROM sqlite_master WHERE type='table' AND name NOT LIKE '__diesel%'"`) containing no user input.

### Instability contract

This command carries **no stability guarantees**:
- TSV column order may change
- Column set may change
- Truncation behavior may change
- Flag names may change

Scripts and downstream tooling must NOT depend on the output format. Use `--output jsonl` for any structured consumption, and even that format may evolve.

## Consequences

**Positive:**
- Users can verify ingestion immediately — `adr-db view task_summaries` shows what's stored without reaching for `sqlite3`.
- Default TSV output is composable with `awk`, `grep`, `sort`, `cut`, `wc -l` — consistent with Linux principles and the plumbing identity.
- `--output jsonl` closes the read/write loop with `ingest` — data can round-trip, enabling debugging workflows like `adr-db view task_summaries --output jsonl | jq '.status'`.
- `adr-db view` (no args) provides table discoverability without knowing the schema.
- Diesel match-per-table approach prevents SQL injection structurally — user input never enters SQL strings.
- No new dependencies — TSV and JSONL formatting are trivial to implement without crates.

**Negative:**
- Blurs the plumbing/porcelain boundary established in ADR-0026. Mitigated by the explicit instability contract and "proto-porcelain" framing.
- Each new table requires a match arm update in the view command. Mitigated by the low frequency of table additions and alignment with the existing model-per-table workflow.
- TSV truncation of `description` means TSV mode is lossy — users must use `--output jsonl` for full fidelity. This is documented but may surprise users who expect exact values.

**Neutral:**
- The instability contract means this command may evolve significantly or be replaced entirely when the real downstream porcelain project materializes.
- Whether `--limit` defaults to showing all rows or a capped number (e.g., 100) is an implementation detail to decide during development.
- The `view` subcommand name may conflict if `adr-db` later adds genuine porcelain commands. Renaming to `inspect` or `peek` is possible given the instability contract.

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
- [x] User documentation

### Additional Quality Concerns

- **Instability documentation** — the README and `--help` text must clearly state that this command has no stability guarantees.
- **Truncation behavior** — unit tests should verify that `description` truncation in TSV mode works correctly and that `--output jsonl` preserves full values.
- **Empty database** — the view command must handle an empty table gracefully (header only, no rows, exit 0).

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR intentionally defers `--limit` default behavior and the exact truncation length for `description` to implementation. The instability contract gives freedom to adjust these without a new ADR.

- [ ] Decision justified (Y-statement or equivalent)
- [ ] Consequences include positive, negative, and neutral outcomes
- [ ] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [ ] Links to related ADRs populated

**Pre-review notes:**

---

## Comments
