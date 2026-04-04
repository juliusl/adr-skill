# 27. Use Diesel with SQLite for adr-db persistence layer

Date: 2026-04-04
Status: Accepted
Last Updated: 2026-04-04
Links:
- Implements [ADR-0026](0026-add-rust-cli-for-data-plumbing.md) (unblocks `adr-db` implementation — ADR-0026 deferred database backend to this decision)
- Builds on [ADR-0020](0020-establish-adr-directory-as-project-scoped-convention.md) (`.adr/var/` as location for the database file — note: ADR-0020 rejected SQLite as a replacement for the directory convention; this ADR introduces SQLite as an additive persistence layer alongside the JSONL convention that ADR-0020 established)

## Context

[ADR-0026](0026-add-rust-cli-for-data-plumbing.md) decided on a single Rust binary (`adr-db`) with subcommands for JSONL data plumbing. That ADR explicitly deferred the database backend and library choice — and stated that implementation is blocked until this decision is made. The initial command set (`init` and `ingest`) both require a storage layer to function.

**The problem:** `adr-db` needs a SQLite persistence layer, and the Rust ecosystem offers multiple libraries with meaningfully different philosophies. The choice affects how schema is authored, how it evolves, how queries are validated, and what developer tooling is required.

**Why the schema matters:** The `adr-db` tool is fundamentally a schema-driven tool. Its job is to ingest structured JSONL records into well-defined tables. As new JSONL producers are added (beyond `extract-summary.awk`), the schema will grow. The library choice determines whether schema evolution is a first-class concern or a manual exercise.

**Current JSONL shape:** The existing `extract-summary.awk` producer emits records with these fields:

```json
{"task_id": "...", "status": "...", "cost": "...", "commit": "...", "description": "..."}
```

Future producers will emit different record shapes. The database library must make it straightforward to add new tables for new record types.

**Constraints (inherited from ADR-0026):**
- **Language: Rust** — non-negotiable.
- **SQLite** — embedded, file-based, zero-config. The database file lives in `.adr/var/` (per ADR-0020). No network database.
- **Sync** — `adr-db` is a CLI tool invoked in shell pipelines. An async runtime adds unnecessary complexity.
- **Embeddable migrations** — the `init` command must create the schema without external migration files. Migrations should be embeddable in the binary.

### Decision Drivers

- **Schema as primary artifact** — the schema is the interface contract between plumbing and porcelain (per ADR-0026). The library should make schema definition and evolution a first-class workflow.
- **Migration support** — as new JSONL producers are added, the schema must evolve. Built-in migration tooling is strongly preferred over hand-rolled solutions.
- **Compile-time safety** — schema/query mismatches should be caught at compile time, not runtime.
- **Sync-first** — no async runtime overhead for a pipeline CLI tool.
- **Developer ergonomics** — adding a new table for a new JSONL record type should be a well-paved path, not a bespoke exercise.

## Options

### Option 1: Diesel with SQLite (`diesel` + `diesel_migrations`)

Use [Diesel](https://diesel.rs/) as the ORM and query builder with its SQLite backend. Diesel takes a schema-first approach: the SQL schema is the source of truth, and `diesel print-schema` generates a `schema.rs` file with Rust type definitions that map directly to tables. Queries are built with a type-safe DSL and validated at compile time against `schema.rs`.

**Migration workflow:**
1. `diesel migration generate create_task_summaries` — creates `up.sql` and `down.sql`
2. Write SQL DDL in `up.sql`
3. `diesel migration run` — applies migrations and regenerates `schema.rs`
4. `diesel_migrations::embed_migrations!()` — embeds migration files in the binary for `init` command

**Adding a new JSONL record type:**
1. Generate a migration with `diesel migration generate`
2. Write the `CREATE TABLE` SQL
3. Run the migration to regenerate `schema.rs`
4. Add a Rust struct with `#[derive(Queryable, Insertable)]`
5. Write the ingest handler using Diesel's DSL

**Strengths:**
- Schema-first philosophy — SQL schema is the source of truth, Rust types are derived from it. This matches the project's schema-as-contract architecture.
- Built-in migration system with `diesel_cli` — generate, run, revert, and redo migrations. Well-established workflow.
- Embeddable migrations via `diesel_migrations` — the `init` command can run all migrations from the compiled binary without external files.
- Compile-time query validation — type mismatches between queries and schema are caught during `cargo build`.
- Synchronous by design — no async runtime, natural fit for CLI tools.
- Mature ecosystem — widely used, well-documented, stable API.

**Weaknesses:**
- `diesel_cli` is an additional developer dependency — contributors need it installed for migration authoring (though not for building the binary).
- Proc-macro codegen increases compile times compared to lighter libraries.
- The DSL has a learning curve — simple queries are easy, complex joins or raw SQL escape hatches are less ergonomic.
- Heavier dependency tree than a direct bindings library.

### Option 2: rusqlite (direct SQLite bindings)

Use [rusqlite](https://github.com/rusqlite/rusqlite) for direct access to the SQLite C API. Write raw SQL for all queries. No ORM, no DSL, no codegen. Schema management is manual — embed SQL strings or files for the `init` command.

**Migration workflow:**
- No built-in migration system. Options:
  - Embed SQL strings with a version table and manual `IF NOT EXISTS` guards
  - Use a separate migration crate (e.g., `refinery`) alongside rusqlite
  - Hand-roll a version-based migration runner

**Adding a new JSONL record type:**
1. Write `CREATE TABLE` SQL, add to the init SQL strings
2. Write a Rust struct for the record
3. Implement `INSERT` with raw SQL and parameter binding
4. No compile-time query checking — errors surface at runtime

**Strengths:**
- Minimal dependency — thin wrapper over SQLite C API, fast compile times.
- Full SQLite control — access to virtual tables, FTS5, custom functions, PRAGMA tuning.
- No abstraction to fight — what you write is what executes. No DSL learning curve.
- Small binary footprint.

**Weaknesses:**
- No built-in migrations — schema evolution must be hand-rolled or delegated to a separate crate.
- No compile-time query safety — SQL string typos and schema mismatches are runtime errors.
- More boilerplate for type mapping — manual `FromRow`-style extraction for each record type.
- Schema is scattered across SQL strings rather than being a centralized, inspectable artifact.

### Option 3: SQLx with SQLite (`sqlx`)

Use [SQLx](https://github.com/launchbadge/sqlx) for compile-time checked raw SQL queries. SQLx validates SQL at compile time by checking against an actual database (or a cached `sqlx-data.json` in offline mode). Built-in migration support via `sqlx migrate`.

**Migration workflow:**
1. `sqlx migrate add create_task_summaries` — creates a timestamped SQL file
2. Write DDL in the migration file
3. `sqlx migrate run` — applies migrations
4. Migrations are embeddable via `sqlx::migrate!()` macro

**Adding a new JSONL record type:**
1. Add a migration SQL file
2. Write a Rust struct
3. Use `sqlx::query_as!` with raw SQL — compile-time checked against the database

**Strengths:**
- Compile-time SQL checking without a DSL — write raw SQL, get type safety via macros.
- Built-in migrations with `sqlx migrate` — comparable to Diesel's workflow.
- Embeddable migrations via `sqlx::migrate!()` — same binary-embedded pattern as Diesel.
- No DSL learning curve — plain SQL throughout.

**Weaknesses:**
- Async-first architecture — requires a `tokio` or `async-std` runtime. While `block_on()` works for sync CLIs, it pulls in the async ecosystem as a dependency for no benefit.
- Compile-time checking requires a live database (or offline mode with `sqlx-data.json`). This complicates CI and fresh clones — contributors must either have a database running or regenerate the offline cache.
- Heavier dependency tree than rusqlite due to the async runtime.
- The async runtime adds binary size and startup overhead for a tool that processes stdin line-by-line.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — all three libraries are mature, well-documented, and have established track records. The decision is about philosophy fit (schema-first vs. raw SQL vs. async-checked SQL), not unproven technology.

## Decision

In the context of **needing a SQLite persistence layer for `adr-db` (ADR-0026)**, facing **the need for schema-driven development where schema evolution is a first-class concern**, we decided for **Diesel with SQLite (Option 1)** and neglected **rusqlite (Option 2, no built-in schema management) and SQLx (Option 3, unnecessary async runtime overhead)**, to achieve **a schema-first development workflow where the SQL schema is the source of truth, migrations are built-in and embeddable, and queries are validated at compile time**, accepting that **Diesel adds `diesel_cli` as a developer dependency, increases compile times via proc macros, and has a DSL learning curve for complex queries**.

### Why Diesel over the alternatives

The central argument is **schema as primary artifact**. ADR-0026 established that the local database is the interface contract between plumbing and porcelain. Diesel's schema-first philosophy — where the SQL schema drives Rust type generation — aligns directly with this architecture. The schema is inspectable (`schema.rs`), version-controlled (migration files), and enforced at compile time.

- **vs. rusqlite:** rusqlite would require hand-rolling migration infrastructure and scatter schema across SQL strings. For a tool where "the majority of the toolset will revolve around schema," manual schema management is a poor fit.
- **vs. SQLx:** SQLx offers compile-time SQL checking and migrations, but requires an async runtime (`tokio`) that adds unnecessary complexity and dependency weight to a synchronous CLI tool. The `block_on()` workaround works but is fighting the library's design.

### Integration with `adr-db`

| Command | Diesel usage |
|---------|-------------|
| `adr-db init` | Runs `diesel_migrations::embed_migrations!()` → creates/migrates `.adr/var/adr.db` |
| `adr-db ingest` | Reads JSONL from stdin, deserializes to `#[derive(Insertable)]` structs, batch-inserts via Diesel DSL |

### Schema and migration files

Migrations live alongside the Rust source in `tools/`:

```
tools/
├── Cargo.toml
├── diesel.toml              # points schema.rs output to src/schema.rs
├── migrations/
│   └── 00000000000000_create_task_summaries/
│       ├── up.sql
│       └── down.sql
└── src/
    ├── main.rs
    ├── schema.rs             # auto-generated by diesel print-schema
    ├── models.rs             # Queryable/Insertable structs
    ├── init.rs
    └── ingest.rs
```

### Developer workflow

Contributors working on `adr-db` need:
1. **Rust toolchain** — for `cargo build`
2. **`diesel_cli`** — for migration authoring (`cargo install diesel_cli --no-default-features --features sqlite`)

Contributors who only work on shell scripts do **not** need Diesel or Rust installed — the `build-tools` Makefile target is optional (per ADR-0026).

## Consequences

**Positive:**
- Schema is a first-class, inspectable artifact — `schema.rs` is auto-generated from SQL and serves as living documentation of the data model.
- Migrations are built-in and embeddable — `init` can bootstrap the database from the compiled binary without external migration files.
- Compile-time query safety — mismatches between Rust code and the database schema are caught during `cargo build`, not at runtime.
- Synchronous by design — no async runtime overhead for a CLI tool that processes stdin.
- Adding a new JSONL record type is a well-paved path: generate migration → write DDL → run migration → add Insertable struct → write ingest handler.
- Unblocks ADR-0026 implementation — the `init` and `ingest` commands can now be built.

**Negative:**
- `diesel_cli` is an additional developer dependency for migration authoring. Contributors must install it to create or modify migrations (though not to build the binary).
- Diesel's proc macros increase compile times compared to rusqlite. For a small codebase this is manageable but grows with schema complexity.
- The Diesel DSL has a learning curve — contributors unfamiliar with Diesel need to learn the query builder syntax. Complex queries may require raw SQL escape hatches.
- Larger dependency tree than rusqlite — Diesel pulls in codegen and proc-macro crates.

**Neutral:**
- The initial schema design (table structure for `extract-summary.awk` records) is not decided here — this ADR selects the library, not the data model. The first migration will be authored during implementation.
- Whether Diesel's `schema.rs` is committed to version control or regenerated from migrations is a project convention to establish during implementation.
- The downstream porcelain project reads the same SQLite database but is not coupled to Diesel — it may use any SQLite library (rusqlite, SQLx, or Diesel). The database file is the interface contract (per ADR-0026), not the Rust library.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [x] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [x] Integration tests
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

- **`diesel_cli` install documentation** — the README and contributing guide must document how to install `diesel_cli` with SQLite support. This is a new developer requirement.
- **CI build integration** — the CI pipeline must install `diesel_cli` if migration checks are part of the build. Alternatively, commit `schema.rs` so CI only needs `cargo build`.
- **Database file location** — integration tests should use temporary databases, not `.adr/var/adr.db`, to avoid side effects.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR deliberately defers the initial schema design (table structure, indexes, constraints) to the implementation phase. The first migration will be authored when `adr-db init` is implemented per ADR-0026.

---

## Comments
