# 52. Implement wi-full-agent-adr format tooling in Rust

Date: 2026-04-06
Status: Planned
Last Updated: 2026-04-06
Links: ADR-0051 (TOML schema), ADR-0018 (unified format scripts), ADR-0022 (implementability criteria / experimentation tolerance), ADR-0026 (Rust CLI precedent), ADR-0028 (Rust tooling workspace), ADR-0053 (Markdown export)

## Context

The wi-full-agent-adr format (ADR-0051) uses TOML for ADR files. The existing format scripts (`nygard-agent-format.sh`, `wi-nygard-agent-format.sh`) are bash scripts that parse Markdown with `grep`, `awk`, and `sed`. This approach does not extend to TOML — TOML has strict syntax rules (quoting, escaping, table boundaries) that are error-prone to handle in shell.

The project already has a Rust workspace at `crates/` with `adr-db` (plumbing CLI) and `adr-db-lib` (library). The Rust ecosystem has mature TOML support via the `toml` crate, and `serde` enables derive-based serialization that maps directly to the schema defined in ADR-0051.

**Decision drivers:**
- TOML parsing correctness — shell tools cannot reliably read/write TOML
- The Rust workspace already exists and builds with `make build-tools`
- The format script must support the same subcommands: `new`, `init`, `list`, `rename`, `status`, `lifecycle`
- The `new.sh` orchestrator dispatches to `<format>-format.sh` — the new tool must fit this interface
- Contributors who only work on shell scripts should not need Rust installed (existing policy from AGENTS.md)

## Options

### Option 1: Rust binary in the existing crates/ workspace

Add a new crate `adr-format` (or extend `adr-db`) that compiles to a binary providing the same subcommand interface as the shell format scripts. The binary is invoked by `new.sh` as `wi-full-agent-adr-format <subcommand> <args>`.

**Subcommands match the existing interface:**
- `new <remote> <id> <title> <dir>` — create a new `.toml` ADR
- `init [dir]` — bootstrap ADR directory with record-architecture-decisions.toml
- `list` — list ADRs with title and status (sorted by date for WI files)
- `rename <remote> <id> <new-title>` — rename file and update title field
- `status [remote] [id] [new-status]` — show or update status
- `lifecycle <remote> <id> [--auto] [--sync]` — check/execute lifecycle transitions
- `export <remote> <id>` — TOML-to-Markdown conversion (ADR-0053)

**Strengths:**
- TOML parsing is native and correct — `toml::from_str()` / `toml::to_string_pretty()`
- Schema validation happens at deserialization — invalid files are caught immediately
- The binary is already built by `make build-tools` — no new build infrastructure
- `serde` derive macros make the schema definition executable (struct = schema)
- The binary can be cross-compiled for different platforms

**Weaknesses:**
- Contributors need Rust to modify format tooling (but not to use it — pre-built binaries can be distributed)
- New dependency in the `new.sh` dispatch path — if the binary isn't built, the format is unavailable
- Testing moves from shell diff-based tests to Rust `#[test]` — different pattern from existing tests

### Option 2: Bash wrapper with external TOML parser

Keep the format script as bash (`wi-full-agent-adr-format.sh`). Use an external TOML CLI tool like `dasel` or `toml-cli` for parsing/writing.

**Strengths:**
- Consistent with existing format scripts — same language, same test pattern
- No Rust dependency for contributors

**Weaknesses:**
- Adds an external dependency (`dasel` or `toml-cli`) that must be installed separately
- Shell-based TOML manipulation is still fragile — multi-line strings, array-of-tables, and escaping are hard to handle correctly
- No schema validation — the tool can read/write fields but can't enforce the full schema structure
- The external tools have their own bugs and version compatibility issues

### Option 3: Hybrid — Rust library with shell wrapper

Build a Rust library (`adr-format-lib`) that provides TOML operations, but expose it through a thin shell wrapper that calls the Rust binary for specific operations (parse, validate, export) and handles orchestration in bash.

**Strengths:**
- TOML operations are correct (Rust handles parsing)
- Shell wrapper maintains familiarity for contributors
- Can be adopted incrementally — start with shell, move operations to Rust as needed

**Weaknesses:**
- Two-language maintenance overhead for a single tool
- The shell wrapper still needs to understand TOML structure for orchestration
- More complex build and test setup

## Evaluation Checkpoint
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — Rust TOML parsing is well-proven in the existing codebase.

## Decision

In the context of implementing format tooling for the TOML-based wi-full-agent-adr format, facing the need for correct TOML parsing and schema validation, we chose **Option 1 (Rust binary in the existing crates/ workspace)** over Option 2 (bash with external parser) and Option 3 (hybrid) to achieve native TOML handling with schema validation, accepting that contributors modifying format tooling need Rust installed.

**Implementation specifics:**

1. **Crate name:** `adr-format` in the `crates/` workspace. Produces a binary `adr-format`. This is a standalone crate, not a subcommand of `adr-db` — the two binaries have different concerns (format operations vs. data plumbing) and different dependency profiles (`toml`/`clap` vs. `diesel`).
2. **Schema as structs:** Define `serde::Serialize + Deserialize` structs matching the ADR-0051 schema. The struct definitions ARE the schema — no separate schema file.
3. **Subcommand dispatch:** Use `clap` for argument parsing. Subcommands mirror the existing shell script interface exactly.
4. **Integration with new.sh:** The existing `new.sh` dispatch (line 33) resolves `$SCRIPT_DIR/${format}-format.sh` — a shell script at a known path. To preserve this dispatch mechanism without restructuring `new.sh`:
   - Create a thin wrapper `wi-full-agent-adr-format.sh` in the scripts directory that locates and delegates to the `adr-format` binary. The wrapper checks `$PATH` first, then falls back to the workspace build output at `crates/target/release/adr-format`.
   - Add a `wi-full-agent-adr)` case block in `new.sh` (same pattern as the existing `wi-nygard-agent)` case) that parses `remote`, `id`, and `title` args before calling `exec "$format_script" new "$remote" "$id" "$title" "$adr_dir"`.
   - If the `adr-format` binary is not found, the wrapper prints an error directing the user to run `make build-tools`. No silent fallback to shell — TOML requires the Rust tool.
5. **Subcommand invocation beyond `new`:** The `new.sh` orchestrator only handles the `new` subcommand. The remaining subcommands (`list`, `status`, `rename`, `lifecycle`, `export`) are invoked directly via the `adr-format` binary — either by users on the command line or through Makefile targets. This matches the existing pattern: `adr-db` subcommands are invoked directly, not through `new.sh`.
6. **Build integration:** The existing `make build-tools` target builds all crates — `adr-format` is included automatically as a workspace member.

## Consequences

**Positive:**
- TOML operations are correct by construction — the `toml` crate handles serialization edge cases
- Schema validation happens at deserialization — invalid files produce serde deserialization errors with field names and expected types. Custom error formatting (wrapping serde errors with file path and suggested fixes) is needed for end-user-friendly messages.
- The binary can share schema structs with `adr-db-lib` in the future — once TOML ADRs are ingested into the database, the struct definitions could be factored into the shared library. Currently `adr-db-lib` only contains Diesel task-summary models, so code sharing is a future benefit, not immediate.
- The same subcommand interface preserves compatibility with the existing Makefile and SKILL.md

**Negative:**
- Rust is required to build the format tooling — contributors who only want to edit shell tests cannot modify the format binary
- The `new.sh` dispatch path now has a hard dependency on a compiled binary for `wi-full-agent-adr` format
- Rust test patterns (unit tests, integration tests) are different from the shell diff-based tests used by other formats

**Neutral:**
- The existing `nygard-agent` and `wi-nygard-agent` bash scripts remain unchanged — this is additive, not a migration
- The `make build-tools` target already exists and builds all Rust crates

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

Integration tests should verify the full `new.sh → adr-format` dispatch path. The binary's subcommand interface must produce output compatible with the existing Makefile targets (same stdout format for `list`, `status`, etc.).

## Conclusion Checkpoint
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** None — all open questions resolved during drafting.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
The wi-full-agent-adr format uses TOML (ADR-0051). TOML parsing in bash is unreliable. The project already has a Rust workspace with TOML expertise. The question is whether to implement the format tooling in Rust (correct but higher barrier) or bash (familiar but fragile).

**Tolerance:**
- Risk: Low — Rust TOML handling is proven in this codebase
- Change: Medium — moving from shell to Rust for format scripts is a tooling shift
- Improvisation: Low — the interface must match existing dispatch patterns

**Uncertainty:**
- Certain: TOML parsing needs a real parser, Rust workspace exists, subcommand interface must match
- Uncertain: Whether to create a new crate or extend adr-db, naming convention

**Options:**
- Target count: 2-3
- [x] Explore additional options beyond candidates listed below

**Candidates:**
- Rust binary in crates/ workspace
- Bash with external TOML CLI tool
- Hybrid Rust library + shell wrapper

### Revision Q&A — R1 (post-review)

**Review verdict:** Revise (6 findings, all addressed)

**F1 (Major) — `new.sh` integration mechanism underspecified:**
Addressed. Specified the concrete integration: a thin `wi-full-agent-adr-format.sh` wrapper that delegates to the binary, plus a `wi-full-agent-adr)` case block in `new.sh` matching the existing `wi-nygard-agent` pattern. This preserves the dispatch mechanism without restructuring `new.sh`.

**F2 (Minor) — Code-sharing with `adr-db-lib` asserted but unsupported:**
Addressed. Reworded as a future benefit. `adr-db-lib` currently only contains Diesel task-summary models — TOML schema structs would be entirely new code.

**F3 (Minor) — Crate name unresolved:**
Addressed. Committed to `adr-format` as a standalone crate. Added rationale: different concerns (format ops vs. data plumbing) and different dependency profiles (`toml`/`clap` vs. `diesel`). Removed the pre-review note that left this ambiguous.

**F4 (Minor) — Missing links for ADR-0022 and ADR-0026:**
Addressed. Added both to the Links header.

**F5 (Minor) — "Clear error messages" is an unqualified claim:**
Addressed. Qualified the consequence: default serde errors include field names and types but are not end-user-friendly. Noted that custom error formatting is needed to meet the "clear messages" goal.

**F6 (Minor) — Subcommand invocation beyond `new` unaddressed:**
Addressed. Added Implementation specifics #5: remaining subcommands are invoked directly via the `adr-format` binary (CLI or Makefile), matching the existing `adr-db` pattern. `new.sh` only orchestrates `new`.
