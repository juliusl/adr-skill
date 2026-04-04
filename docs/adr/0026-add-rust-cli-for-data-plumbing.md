# 26. Add Rust CLI for data plumbing

Date: 2026-04-04
Status: Proposed
Last Updated: 2026-04-04
Links:
- Builds on [ADR-0020](0020-establish-adr-directory-as-project-scoped-convention.md) (`.adr/` project-scoped directory for database file)
- Extends [ADR-0021](0021-append-implementation-summary-to-plan-after-execution.md) (JSONL extraction pattern — structured database replaces JSONL for cross-cutting queries)

## Context

The adr-skills project currently relies on shell scripts (Bash) for all tooling — ADR creation, listing, status updates, format dispatching, and JSONL extraction (per ADR-0021). These scripts serve the individual skill workflows well but produce data that has nowhere to land. `extract-summary.awk`, for example, emits JSONL to stdout — but there is no tooling to persist that output for downstream consumption.

**The problem:** Skills produce structured JSONL data (plan summaries, ADR metadata, review findings), but there is no bridge between that transient JSONL output and a persistent data store. Without plumbing to persist this data, a downstream project (porcelain) cannot aggregate or present it.

**Why this matters:**
- `implement-adr` already has `extract-summary.awk`, which produces JSONL records of task execution (task_id, status, cost, commit, description). This data is emitted to stdout and lost.
- Future skills and scripts will produce additional JSONL streams. Each producer should remain independent — it emits JSONL and doesn't care whether anything persists it.
- A separate downstream project will consume persisted data for management and presentation (porcelain). This project needs to provide the plumbing: a JSONL-to-persistence bridge that lives close to the producers.

**Architecture — the git philosophy:**
Following git's plumbing/porcelain separation, this toolset is plumbing. Skills are JSONL producers. The downstream project is porcelain. The plumbing bridges the two by ingesting JSONL into a persistent store:

```
Skills (producers)         Plumbing (this ADR)        Porcelain (downstream)
extract-summary.awk  ──►  ingest stdin → database  ──►  management, views
future producers...       init (schema setup)          dashboards, reporting
```

Skills do not depend on the plumbing — they work without it. The plumbing is additive.

**Anticipated data layer:** SQLite is the expected persistence backend (to be formalized in a separate ADR). This anticipation shapes the tool's character but is not the decision being made here.

**Scope boundaries (what this ADR does NOT decide):**
- Database library or ORM choice (e.g., Diesel vs rusqlite) — separate ADR.
- SQLite schema design or data model — separate ADR.
- The downstream consumer/management project's architecture.
- Migration of existing shell scripts — existing scripts continue to work unchanged.
- Porcelain commands for skills to *read* persisted data — that's a future layer.

**Constraints:**
- **Language: Rust** — preferred by the maintainer; Python, Go, and JavaScript are excluded.
- **Minimal footprint** — the toolset should be small and focused, not a framework.
- **Plumbing, not porcelain** — a JSONL ingestion bridge, not a query interface or UI.
- **No skill coupling** — skills produce JSONL independently; the plumbing consumes it but skills do not depend on it.
- **Co-located with producers** — the plumbing lives in this repo because it needs to stay close to the scripts that emit JSONL.

### Decision Drivers

- **Composability** — the tool must work in shell pipelines (`extract-summary.awk plan.md | adr-db ingest`)
- **Skill-independent** — skills should never require the plumbing to function; it's additive
- **User-invocable** — users should be able to run the tool directly from the command line
- **Minimal build complexity** — adding Rust to a shell-script project should not burden contributors with heavy build infrastructure
- **Start from what's concrete** — the initial command set should serve existing JSONL producers, not speculate about future needs

## Options

### Option 1: Cargo workspace with multiple small CLI binaries

Add a `tools/` directory at the repo root containing a Cargo workspace with multiple small Rust binaries. Each binary does one thing — collect, merge, filter, or emit structured data. Binaries are designed for shell pipelines (`stdin` → `stdout`) and follow Unix conventions: silent on success, structured output on stdout, errors on stderr.

```
tools/
├── Cargo.toml          # workspace root
├── Cargo.lock
└── crates/
    ├── adr-collect/     # parse ADR files → JSONL
    ├── adr-merge/       # combine JSONL streams
    └── adr-filter/      # select/transform JSONL records
```

Skills invoke them from Makefile targets or scripts. Users invoke them directly. The root Makefile gains a `build-tools` target that runs `cargo build --release` in `tools/`.

**Strengths:**
- True Unix composability — each tool is a single pipeline stage
- Each binary is tiny and independently testable
- Natural fit with existing shell-script workflows (`adr-collect docs/adr/ | adr-filter --status=Accepted`)
- Adding a new tool doesn't change existing tools

**Weaknesses:**
- Multiple binaries to build, install, and keep on `$PATH`
- Some shared logic (JSONL parsing, ADR metadata extraction) would be duplicated unless factored into an internal library crate
- More Cargo boilerplate (one `Cargo.toml` per binary)

### Option 2: Single binary with subcommands

A single Rust binary (`adr-db`) with subcommands dispatched via `clap`. All aggregation operations are subcommands: `adr-db collect`, `adr-db merge`, `adr-db filter`. One binary to build, one to distribute, one to add to `$PATH`.

```
tools/
├── Cargo.toml
└── src/
    ├── main.rs          # clap dispatch
    ├── collect.rs
    ├── merge.rs
    └── filter.rs
```

Still pipeline-friendly — each subcommand reads stdin and writes stdout by default. Skills invoke via `adr-db <subcommand>` from Makefiles.

**Strengths:**
- Single build artifact — simpler `$PATH` management, one `cargo install`
- Shared code is natural (same crate, module visibility)
- Subcommand discoverability via `adr-db --help`
- Minimal Cargo boilerplate

**Weaknesses:**
- Monolithic binary — all operations ship together even if a user only needs one
- Less composable than separate binaries in complex pipelines (though `adr-db collect | adr-db filter` still works)
- Binary grows with every new operation added

### Option 3: Library crate with thin CLI wrappers

Core aggregation logic in a library crate (`adr-core`), with thin CLI binaries that wrap specific functions. The library crate is also publishable, so the downstream consumer project can depend on it directly as a Rust library — no subprocess spawning needed.

```
tools/
├── Cargo.toml          # workspace root
├── Cargo.lock
└── crates/
    ├── adr-core/        # library: parsing, aggregation, transforms
    ├── adr-collect/     # thin CLI wrapping adr_core::collect
    ├── adr-merge/       # thin CLI wrapping adr_core::merge
    └── adr-filter/      # thin CLI wrapping adr_core::filter
```

**Strengths:**
- Maximum code reuse — library is shared across CLIs and consumable by the downstream project
- Testable at the library level (unit tests) and CLI level (integration tests)
- Clean separation: library handles logic, CLIs handle I/O
- Downstream project gets a Rust API, not just subprocess calls

**Weaknesses:**
- Most upfront design work — library API surface needs careful thought
- May be over-engineered for "minimal" — the downstream project may not need a Rust dependency
- Library versioning adds maintenance burden
- Three layers (library → CLI → shell) where two might suffice

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — the decision is about project structure and tooling shape, not performance or behavior. Validation occurs naturally during implementation.

## Decision

In the context of **needing a persistence bridge for JSONL data produced by adr-skills scripts**, facing **the need to keep tooling minimal and decoupled from skill workflows**, we decided for **a single Rust binary with subcommands (Option 2)** and neglected **multiple small binaries (Option 1, unnecessary distribution complexity) and a library-plus-CLI architecture (Option 3, over-engineered for current needs)**, to achieve **a minimal CLI that ingests JSONL into a persistent data store, following the git philosophy of separating plumbing from porcelain**, accepting that **the binary is monolithic and grows with each new operation, and that adding Rust introduces a build dependency to a previously shell-only project**.

### Initial Command Set

Starting from what's concrete — the JSONL that `extract-summary.awk` already produces — the initial plumbing verbs are:

| Command | Purpose | Example |
|---------|---------|---------|
| `init` | Initialize the data store (create schema, ensure `.adr/var/` exists) | `adr-db init` |
| `ingest` | Read JSONL from stdin and persist each record to the data store | `awk -f extract-summary.awk plan.md \| adr-db ingest` |

That's it. Two commands. Additional plumbing verbs (e.g., `dump`, `prune`) can be added as concrete needs arise from new JSONL producers. The command set grows from the bottom up — driven by what producers actually emit — not from top-down speculation.

**What is NOT in the initial command set:**
- Query/filter/search — that's porcelain for the downstream project
- ADR parsing or metadata extraction — skills handle their own format-specific parsing
- Any command that skills would depend on — skills remain plumbing-unaware

### Binary Structure

The binary lives in a `tools/` directory at the repo root:

```
tools/
├── Cargo.toml
└── src/
    ├── main.rs          # clap dispatch, connection setup
    ├── init.rs          # schema initialization
    └── ingest.rs        # JSONL stdin → data store
```

The binary is named `adr-db`. The specific database backend and schema are deferred to a follow-up ADR.

### Integration Points

- **Skills** produce JSONL to stdout — they do NOT invoke the plumbing directly. A user or automation pipes JSONL into the tool (e.g., `extract-summary.awk plan.md | adr-db ingest`)
- **Users** invoke the tool directly from the command line
- **Downstream project** reads from the persisted data store — no IPC, no custom protocol, just a shared local database (backend TBD in follow-up ADR)
- **Root Makefile** gains a `build-tools` target that runs `cargo build --release` in `tools/`

### Data Store as Meeting Point

The local database serves as the interface contract between this toolset and any downstream consumer. The plumbing tool writes structured data into the store; the downstream project (porcelain) reads it. This avoids coupling the two projects through Rust library dependencies or IPC — the database file is the API.

The database file location follows the `.adr/` convention (ADR-0020) — likely `.adr/var/` or similar, to be formalized in the data layer ADR.

### Coexistence with Shell Scripts

Existing shell scripts (ADR creation, listing, status updates) continue to work unchanged. The Rust binary is additive — it provides a structured data layer on top of the existing file-based workflows. Over time, skills may optionally call the binary to keep the database in sync after script operations, but this is not required at introduction.

## Consequences

**Positive:**
- JSONL data produced by skills gains a persistence path — data that was previously lost to stdout can now be stored and consumed downstream.
- Skills remain fully independent — they produce JSONL without knowing or caring whether plumbing exists. Zero coupling.
- A local database as the meeting point provides a well-understood interface between this project and downstream consumers. No custom protocols or serialization formats.
- Single binary with subcommands keeps `$PATH` management simple — one binary, one `cargo install`, discoverable via `--help`.
- Starting with just `init` + `ingest` avoids over-engineering — commands grow from concrete JSONL producers, not speculation.
- Rust provides type safety and ships as a single compiled binary with no additional runtime dependencies (no interpreter, VM, or package manager required).
- Additive to the project — existing shell workflows are unaffected.

**Negative:**
- Adds a Rust/Cargo build dependency to a previously shell-only project. Contributors who only work on shell scripts now need Rust installed (or skip the `build-tools` target).
- Monolithic binary means all subcommands ship together — though with only two initial commands this is negligible.
- The specific database backend is deferred, which means this ADR creates a dependency on a follow-up decision before implementation can fully proceed. Implementation of this ADR is blocked on the follow-up database backend ADR — the `init` and `ingest` commands cannot be implemented without a storage layer.

**Neutral:**
- The database backend and data schema are deferred — this ADR decides the structural approach (single Rust binary named `adr-db`, JSONL ingestion plumbing) without prescribing storage implementation details.
- Whether the binary eventually gains porcelain-like commands (query, filter) is a future decision driven by downstream needs, not a goal of this ADR.
- The `tools/` directory is a new top-level directory in the repo; this is a minor structural change.

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

- **Build integration** — the root Makefile must cleanly build the Rust binary without breaking existing `make test` workflows. `cargo` failures in `build-tools` should not block shell-only operations.
- **Cross-platform builds** — the binary should build on macOS and Linux at minimum (the two environments where skills are used).

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR intentionally defers the database backend (Diesel/rusqlite), schema design, and binary naming. A follow-up ADR should formalize the data layer choices.

---

## Comments
