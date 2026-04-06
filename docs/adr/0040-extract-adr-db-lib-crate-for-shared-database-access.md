# 40. Extract adr-db-lib crate for shared database access

Date: 2026-04-06
Status: Accepted
Last Updated: 2026-04-06
Links:
- Revisits [ADR-0026](0026-add-rust-cli-for-data-plumbing.md) (Option 3 was rejected as over-engineered; this ADR revisits that decision now that downstream consumers exist)
- Extends [ADR-0027](0027-use-diesel-with-sqlite-for-adr-db-persistence-layer.md) (library crate owns the Diesel schema, models, and migrations)
- Extends [ADR-0028](0028-locate-rust-workspace-for-adr-db-in-the-repository.md) (new workspace member in `crates/`)

## Context

ADR-0026 chose a single binary with subcommands (Option 2) over a library-plus-CLI architecture (Option 3), describing the latter as "over-engineered for current needs." At that time, `adr-db` had two commands (`init`, `ingest`) and zero downstream consumers.

**The situation has changed.** `adr-db` now has three commands (`init`, `ingest`, `view`), a Diesel schema with migrations, typed models, query helpers, and JSONL serialization logic. The codebase has grown from a two-command skeleton to a substantive data layer — and the maintainer plans to continue building on it:

1. **Build additional tooling crates** — new tools that need to read/write the same SQLite database using the same schema and models.
2. **Add an `examples/` directory** — proof-of-concept crates demonstrating database interactions.
3. **Continue dogfooding the skills** — more crates in the workspace means more implementation plans, more ADRs, and eventually nested `docs/adr/` directories.

Independent of future plans, the current code volume alone justifies extraction: five modules with typed Diesel models, embedded migrations, query helpers, and JSONL serialization are locked inside a binary crate. This code is reusable by construction but inaccessible by packaging.

**The problem:** All reusable database code — schema definitions, Diesel models, migrations, connection helpers, query functions — is locked inside the `adr-db` binary crate. Other crates in the workspace cannot `use adr_db::models::TaskSummary` because `adr-db` is a binary, not a library. Every new tool would need to duplicate the schema, models, and migration logic.

**What needs to be shared:**
- `schema.rs` — Diesel table definitions (the canonical schema)
- `models.rs` — `TaskSummary`, `NewTaskSummary`, `JsonlTaskRecord`, `From` impls
- `init.rs` — embedded migrations and `run_init` logic
- Query helpers from `view.rs` — `list_tables`, table query functions

**What stays CLI-specific:**
- `main.rs` — clap parsing, subcommand dispatch, error printing, exit codes
- Output formatting — TSV/JSONL rendering, `OutputFormat` enum
- stdin reading — `ingest` reads from stdin, which is a CLI concern

**Constraints:**
- **Workspace-internal** — the library is not published to crates.io. It serves this workspace only.
- **Diesel ownership** — the library owns the schema, models, and migrations. The CLI and other crates depend on the library.
- **No behavior change** — `adr-db` CLI behavior remains identical after extraction.

### Decision Drivers

Listed in priority order — clean separation outweighs minimal disruption because the extraction is a one-time cost while dependency clarity is ongoing.

- **Code reuse** — shared types and queries should live in one place, not be duplicated across crates.
- **Workspace ergonomics** — adding a new tool crate should be as simple as `adr-db-lib = { path = "../adr-db-lib" }` in its `Cargo.toml`.
- **Migration ownership** — exactly one crate should own the Diesel migrations. Other crates run them but don't define them.
- **Minimal disruption** — the extraction should be a refactor, not a rewrite. `adr-db` continues to work identically.

## Options

### Option 1: Separate library crate (`adr-db-lib`)

Extract shared code into a new `crates/adr-db-lib/` crate. The `adr-db` binary crate depends on `adr-db-lib` and becomes a thin CLI wrapper. Future tool crates also depend on `adr-db-lib`.

```
crates/
├── Cargo.toml              # workspace: members = ["adr-db", "adr-db-lib"]
├── adr-db-lib/              # library crate (new)
│   ├── Cargo.toml
│   ├── diesel.toml
│   ├── migrations/          # moved from adr-db
│   └── src/
│       ├── lib.rs           # public API surface
│       ├── schema.rs        # moved from adr-db
│       ├── models.rs        # moved from adr-db
│       └── db.rs            # connection + migration helpers
└── adr-db/                  # binary crate (slimmed)
    ├── Cargo.toml           # depends on adr-db-lib
    └── src/
        ├── main.rs
        ├── ingest.rs        # CLI-specific stdin handling
        └── view.rs          # CLI-specific output formatting
```

**Strengths:**
- Clean separation — library has no CLI dependencies (no `clap`), binary has no schema ownership.
- Future crates depend on `adr-db-lib` without pulling in clap, serde_json, or CLI formatting.
- Diesel migrations live in one place — the library crate.
- Standard Rust pattern — `foo-lib` + `foo` is a well-established workspace convention.

**Weaknesses:**
- Two crates to maintain where one existed before.
- The library API surface needs design thought — what's `pub`, what's `pub(crate)`.
- Moving migrations requires updating `diesel.toml` and the `embed_migrations!()` macro paths.

### Option 2: Dual-purpose crate (lib + bin in `adr-db`)

Keep everything in `adr-db` but add a `src/lib.rs` alongside `src/main.rs`. Rust natively supports this — `cargo build` produces both a library and a binary from the same crate.

```
crates/
├── Cargo.toml
└── adr-db/
    ├── Cargo.toml
    └── src/
        ├── lib.rs           # re-exports schema, models, db helpers
        ├── main.rs          # CLI dispatch (uses adr_db::*)
        ├── schema.rs
        ├── models.rs
        ├── init.rs
        ├── ingest.rs
        └── view.rs
```

Other crates depend on `adr-db = { path = "../adr-db" }` and access `adr_db::models::TaskSummary`.

**Strengths:**
- Minimal change — add `lib.rs`, make modules `pub`, done.
- No file moves — schema, models, migrations stay where they are.
- One crate to maintain.

**Weaknesses:**
- Downstream crates that depend on `adr-db` pull in all its dependencies (clap, serde_json) even if they only need schema access. Cargo feature flags could mitigate this but add complexity.
- The library API is entangled with the binary's internal structure — refactoring the CLI may break library consumers.
- The crate name `adr-db` implies a binary tool, not a library. Using it as a dependency reads oddly: `use adr_db::models::TaskSummary`.
- Rust's lib+bin pattern can cause confusing compilation issues when the binary and library have naming conflicts.

### Option 3: Keep `adr-db` as-is, use code generation

Instead of extracting a library, generate shared code (schema, models) from the Diesel migrations at build time. Each downstream crate runs its own `diesel print-schema` and derives its own model structs.

**Strengths:**
- No shared crate dependency — each crate is self-contained.
- No API surface to design or maintain.

**Weaknesses:**
- Code duplication by design — every crate has its own copy of schema and models.
- Schema drift — if crates generate at different times, they may be inconsistent.
- No shared query helpers — each crate writes its own query functions.
- Violates the "schema as single source of truth" principle from ADR-0027 — effectively disqualified by existing project architecture.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — all options use well-understood Rust workspace patterns. The lib+bin split (Option 2) and separate library crate (Option 1) are standard Cargo conventions with no unknowns.

## Decision

In the context of **needing to share database schema, models, and query helpers across multiple crates in the workspace**, facing **the tension between minimal disruption (keep one crate) and clean separation (separate library)**, we decided for **a separate library crate `adr-db-lib` (Option 1)** and neglected **a dual-purpose lib+bin crate (Option 2, dependency bloat and naming confusion) and code generation (Option 3, schema drift and duplication)**, to achieve **a clean library crate that owns the Diesel schema, models, and migrations without CLI dependencies**, accepting that **this adds a second crate to maintain and requires moving migrations and schema files**.

### Library crate structure

```
crates/adr-db-lib/
├── Cargo.toml            # diesel, diesel_migrations, serde
├── diesel.toml           # moved from adr-db
├── migrations/           # moved from adr-db
│   ├── 00000000000000_create_task_summaries/
│   └── 20260406000000_add_source_plan/
└── src/
    ├── lib.rs            # pub mod schema, models, db;
    ├── schema.rs         # moved from adr-db (unchanged)
    ├── models.rs         # moved from adr-db (unchanged)
    └── db.rs             # connection helper + embedded migrations
```

### Public API surface

```rust
// lib.rs
pub mod schema;
pub mod models;
pub mod db;
```

- `schema` — re-exports Diesel table definitions
- `models` — `TaskSummary`, `NewTaskSummary`, `JsonlTaskRecord`, `From` impl
- `db` — `establish_connection(path)`, `run_migrations(conn)`, `MIGRATIONS` constant

### Binary crate changes

`adr-db/Cargo.toml` drops `diesel` and `diesel_migrations` direct dependencies, replacing them with `adr-db-lib = { path = "../adr-db-lib" }`. It re-exports nothing — it's purely a CLI consumer.

```
crates/adr-db/
├── Cargo.toml            # depends on adr-db-lib, clap, serde_json
└── src/
    ├── main.rs           # clap dispatch (unchanged behavior)
    ├── ingest.rs         # uses adr_db_lib::models, adr_db_lib::schema
    └── view.rs           # uses adr_db_lib::models, adr_db_lib::schema
```

`init.rs` is removed from `adr-db` — its logic moves to `adr-db-lib::db`. The `Init` command in `main.rs` calls `adr_db_lib::db::run_init()`.

### Workspace update

```toml
# crates/Cargo.toml
[workspace]
members = ["adr-db", "adr-db-lib"]
resolver = "2"
```

### Future tool pattern

Any new tool crate follows this pattern:

```toml
# crates/my-new-tool/Cargo.toml
[dependencies]
adr-db-lib = { path = "../adr-db-lib" }
```

```rust
use adr_db_lib::db;
use adr_db_lib::models::TaskSummary;
use adr_db_lib::schema::task_summaries;
```

## Consequences

**Positive:**
- Schema, models, and migrations live in exactly one place — `adr-db-lib` is the single source of truth for the database layer.
- New tool crates can access the database without duplicating schema code or pulling in CLI dependencies.
- `adr-db` becomes a thin CLI wrapper — easier to understand, test, and modify independently of the data layer.
- Future `examples/` crates can depend on `adr-db-lib` for lightweight proof-of-concept work.
- Clean dependency graph: `adr-db-lib` has no CLI dependencies; `adr-db` has no schema ownership.

**Negative:**
- Two crates to maintain where one existed before. Mitigated by the fact that `adr-db-lib` is stable (schema changes are infrequent and go through ADRs + migrations) while `adr-db` is the active development surface.
- Moving migrations requires updating `diesel.toml` path, `embed_migrations!()` macro location, and any hardcoded migration references. This is a one-time cost.
- The library API surface needs deliberate design — what's `pub` today affects what downstream crates can depend on.

**Neutral:**
- The library is workspace-internal and not published to crates.io. Its API stability is a workspace convention, not a semver contract.
- The name `adr-db-lib` follows the `foo-lib` convention. Alternative names like `adr-db-core` or `adr-db-sdk` are equivalent — the choice is cosmetic.
- Separate crates change incremental compilation — when `adr-db-lib` changes, all dependents recompile. Negligible at the current workspace size (2 crates).
- Nested `docs/adr/` directories (one per crate) are a future concern acknowledged by the maintainer. This ADR does not address them — the current `docs/adr/` at the repo root serves the entire workspace.

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

- **Migration path integrity** — after moving migrations from `adr-db` to `adr-db-lib`, all existing tests must continue to pass with the same behavior. The embedded migrations macro must resolve correctly from the new location.
- **`cargo test` coverage** — unit tests in `models.rs` and `view.rs` that use `embed_migrations!()` must work from both the library crate (where migrations now live) and the binary crate (which depends on the library).
- **Cross-crate integration** — `cargo test --workspace` must verify that `adr-db` works correctly through `adr-db-lib` after extraction. The cross-crate dependency boundary is a wiring risk point.
- **Build integration** — `make build-tools` must build both crates. The root Makefile's `cargo build --release` already builds all workspace members, so no Makefile change is needed.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR deliberately revisits ADR-0026's rejection of Option 3 (library + CLI). The circumstances have changed — there are now concrete downstream consumers planned, which was not the case when ADR-0026 was written. User documentation is unchecked because `adr-db-lib` is workspace-internal; the user-facing CLI is unchanged.

---

## Comments
