# 61. Absorb adr-format into adr-db-lib

Date: 2026-04-08
Status: Planned
Last Updated: 2026-04-08
Links:
- Related to [ADR-0028](0028-locate-rust-workspace-for-adr-db-in-the-repository.md) (original Rust workspace setup)
- Related to [ADR-0051](0051-define-wi-full-agent-adr-toml-schema.md) (defined the TOML schema adr-format implements)
- Related to [ADR-0052](0052-implement-wi-full-agent-adr-format-tooling-in-rust.md) (created adr-format as a separate crate)
- Related to [ADR-0058](0058-relocate-cargo-workspace-from-crates-to-src-crates.md) (relocated workspace to src/crates/)

## Context

The Cargo workspace contains three crates with overlapping concerns:

| Crate | Role | Dependencies |
|-------|------|-------------|
| `adr-format` | CLI + lib for TOML ADR documents (parse, serialize, validate, export) | standalone — no workspace deps |
| `adr-db-lib` | Database layer (SQLite via Diesel) + remote adapters (Gitea) | standalone |
| `adr-db` | CLI for JSONL→SQLite ingestion | depends on adr-db-lib |

`adr-format` is completely decoupled from the rest of the workspace. It was created as a separate crate (ADR-0052) because at the time, the focus was on implementing the TOML schema (ADR-0051) without coupling it to the database layer.

**The problem:** `adr-format` and `adr-db-lib` both model ADR-related data but cannot share types. The TOML document model (`Adr`, `Meta`, `Decision`, `Consequence`, etc.) lives in `adr-format`. The database models (`TaskSummary`, `NormalizedWorkItem`) live in `adr-db-lib`. These two type systems will need to interoperate as the tooling evolves — for example, ingesting TOML ADRs into the database, or querying the database to generate reports that reference ADR content.

The current separation means any future integration requires adapter code between two independently maintained type systems. Absorbing `adr-format` into `adr-db-lib` consolidates the ADR data model in one place, making `adr-db` the unified tool for all ADR operations.

### Decision Drivers

- **Type unification** — one crate should own the ADR data model so downstream consumers have a single dependency
- **Evolutionary coherence** — `adr-db` should evolve into the comprehensive ADR management tool, not remain limited to JSONL ingestion
- **Reduced workspace complexity** — fewer crates means fewer dependency boundaries to maintain
- **Shell script simplification** — `wi-full-agent-adr-format.sh` currently resolves a separate binary; absorbing the CLI into `adr-db` eliminates this indirection

## Options

### Option 1: Keep adr-format separate (status quo)

Leave the three-crate structure. Accept that type sharing requires adapter code.

**Strengths:** No migration work. Each crate has a clear single responsibility.

**Weaknesses:** Type duplication as the tooling evolves. Two CLIs (`adr-format` and `adr-db`) for related operations. Shell wrapper resolves a separate binary path.

### Option 2: Move adr-format types into adr-db-lib, CLI into adr-db

Merge `adr-format`'s library code (`schema.rs`, `template.rs`) into `adr-db-lib` as a new module (e.g., `adr_db_lib::format`). Merge `adr-format`'s CLI commands (`new`, `list`, `rename`, `status`, `lifecycle`, `export`) into `adr-db` as subcommands. Delete the `adr-format` crate.

```
Before:                          After:
src/crates/                      src/crates/
├── adr-format/  (standalone)    ├── adr-db-lib/
│   ├── schema.rs                │   ├── format/        ← types from adr-format
│   ├── template.rs              │   │   ├── schema.rs
│   └── main.rs (CLI)            │   │   └── template.rs
├── adr-db-lib/                  │   ├── db.rs
│   ├── db.rs                    │   ├── models.rs
│   ├── models.rs                │   ├── remote/
│   └── remote/                  │   └── schema.rs (diesel)
└── adr-db/                      └── adr-db/
    └── main.rs                      └── main.rs  ← absorbs adr-format CLI commands
```

**Strengths:**
- Single ADR data model in `adr-db-lib` — downstream consumers get types and database in one dependency.
- `adr-db` becomes the one CLI for all ADR operations (ingestion, format management, querying).
- `wi-full-agent-adr-format.sh` can call `adr-db format <subcommand>` instead of resolving a separate binary.
- Reduces workspace from 3 crates to 2.

**Weaknesses:**
- `adr-db-lib` grows in scope — it becomes "ADR library" rather than "database library."
- Migration work: move code, update imports, update Cargo.toml, update shell scripts, update tests.
- `adr-db-lib` gains the `toml` and `serde` dependencies from `adr-format` (it already has `serde`).

### Option 3: Make adr-format depend on adr-db-lib (soft coupling)

Keep `adr-format` as a separate crate but add a dependency on `adr-db-lib`. Share types by re-exporting from `adr-db-lib`.

**Strengths:** Minimal code movement. Keeps separation.

**Weaknesses:** Adds a dependency chain where `adr-format` now pulls in SQLite/Diesel even when only doing TOML operations. Does not reduce the number of crates or CLIs. The shell script indirection remains.

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — this is a code reorganization. Validation is running `cargo test` and `make test` after the merge.

## Decision

In the context of **the Cargo workspace crate structure**, facing **a standalone adr-format crate that cannot share types with the database layer**, we decided for **absorbing adr-format into adr-db-lib and adr-db (Option 2)** and against **keeping them separate (Option 1) or soft-coupling via dependency (Option 3)**, to achieve **a unified ADR data model in adr-db-lib and a single CLI in adr-db for all ADR operations**, accepting that **adr-db-lib grows in scope and a code migration is required**.

### Migration

1. Create `src/crates/adr-db-lib/src/format/` module with `mod.rs`, `schema.rs`, `template.rs` — move type definitions and logic from `adr-format/src/`. Note: `adr_db_lib::format::schema` (TOML document types) is distinct from `adr_db_lib::schema` (Diesel-generated table definitions) — these serve different purposes and must not be merged.
2. Add `toml` dependency to `adr-db-lib/Cargo.toml` (it already has `serde`)
3. Add `adr-format` CLI commands to `adr-db/src/main.rs` under a `format` subcommand group (e.g., `adr-db format new`, `adr-db format list`, `adr-db format export`)
4. Update `wi-full-agent-adr-format.sh` to call `adr-db format` instead of `adr-format`
5. Move `adr-format/tests/` to `adr-db/tests/` or `adr-db-lib/tests/` as appropriate. Add `tempfile = "3"` to `adr-db` dev-dependencies (used by the 4 export tests).
6. Delete `src/crates/adr-format/` and remove it from workspace `Cargo.toml`
7. Update `description` fields in `adr-db-lib/Cargo.toml` and `adr-db/Cargo.toml` to reflect the expanded scope (e.g., `adr-db` is no longer limited to JSONL ingestion)
8. Update AGENTS.md P-13 references mentioning `adr-format` as a separate crate
9. Run `cargo test`, `make test`, `make check-refs` to verify

## Consequences

**Positive:**
- Single ADR data model — `adr_db_lib::format::Adr` is the canonical type, available to any crate that depends on `adr-db-lib`.
- One CLI — `adr-db` handles ingestion, format operations, and future querying. Users learn one tool.
- Simpler shell wrapper — `wi-full-agent-adr-format.sh` calls `adr-db format` directly, no separate binary resolution.
- Future integration path — TOML ADR content can be ingested into SQLite using shared types without adapter code.

**Negative:**
- `adr-db-lib` grows beyond "database library" into "ADR library." The name becomes slightly misleading.
- Migration requires updating imports, tests, and the shell wrapper. Moderate effort.
- `adr-db-lib` gains the `toml` crate as a dependency (small, no transitive bloat).

**Neutral:**
- The `adr-db` binary name stays the same. Existing scripts that call `adr-db ingest` are unaffected.
- The `adr-format` binary no longer exists after migration. Direct invocations must use `adr-db format <subcommand>` instead; the shell wrapper (`wi-full-agent-adr-format.sh`) handles this transparently.
- The `toml` dependency is well-maintained and lightweight.

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

- **Export test preservation** — the 4 export tests in `adr-format/tests/export_test.rs` must be migrated and pass against `adr-db format export`.
- **Shell script compatibility** — `wi-full-agent-adr-format.sh` must continue to work with the same interface (`adr-format <subcommand>` becomes `adr-db format <subcommand>`).
- **Workspace build** — `make build-tools` must produce a single `adr-db` binary with all commands.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This is an evolutionary step — `adr-db` grows from a narrow ingestion tool into the unified ADR CLI. The name `adr-db-lib` may warrant revisiting later, but renaming is out of scope for this ADR.

---

## Comments

### Draft Worksheet

**Framing:**
The user observed that `adr-format` should be absorbed into `adr-db-lib` as an evolution of `adr-db`. The three-crate structure has a standalone `adr-format` with no dependencies on the rest of the workspace, which prevents type sharing and creates two separate CLIs for related operations.

**Tolerance:**
- Risk: Low — code reorganization with well-defined boundaries
- Change: Medium — merging two crates changes the workspace structure
- Improvisation: Low — the destination is specified (types → adr-db-lib, CLI → adr-db)

**Uncertainty:**
- Certain: the merge reduces workspace complexity
- Certain: shared types enable future integration
- Minor: exact module organization within adr-db-lib (format/ submodule is straightforward)

**Options:**
- Target count: 3 (keep, merge, soft-couple)
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
1. Keep separate (status quo)
2. Absorb into adr-db-lib + adr-db (user's direction)
3. Soft-couple via dependency

### Revision — Review Triage (V-1 through V-6)

**Findings addressed (4 of 5):**

| # | Priority | Finding | Decision | Rationale |
|---|----------|---------|----------|-----------|
| 1 | M | schema.rs naming collision unaddressed | **Address** | Procedural explicitness — two `schema.rs` files with different purposes in the same crate. Added clarifying note to migration step 1 distinguishing Diesel table definitions from TOML document types. |
| 2 | L | Missing Cargo.toml description updates | **Address** | Consistency — migration steps already include granular metadata updates (AGENTS.md P-13). `adr-db` description says "ingesting JSONL data" which becomes inaccurate after absorbing format commands. Added as migration step 7. |
| 3 | L | Missing `tempfile` dev-dependency | **Address** | Consistency with step 2 (which lists `toml`). Step 5 covers moving tests but omitted the dev-dependency they require. Added to step 5. |
| 4 | L | "Backwards Compatible" but binary ceases to exist | **Address** | Accuracy gap — Quality Strategy claims compatibility, but the `adr-format` binary is deleted. Shell wrapper provides the compatibility layer, but the fact should be explicit. Added neutral consequence. |
| 5 | L | `toml` dependency claim verified accurate | **No action** | Verified correct — no change needed. |
