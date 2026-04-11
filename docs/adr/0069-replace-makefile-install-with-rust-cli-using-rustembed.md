# 69. Replace Makefile Install with Rust CLI Using RustEmbed

Date: 2026-04-10
Status: Planned
Last Updated: 2026-04-11
Links:
- Related to [ADR-0026](0026-add-rust-cli-for-data-plumbing.md) (established Rust binary and Cargo workspace)
- Related to [ADR-0058](0058-relocate-cargo-workspace-from-crates-to-src-crates.md) (current workspace location)
- Related to [ADR-0061](0061-absorb-adr-format-into-adr-db-lib.md) (current workspace shape: adr-db + adr-db-lib)

## Context

Installation is currently driven by four Makefile targets: `make install-skills` (rm + copy skills), `make install-agents` (copy agent files), `make init-project` (create `.adr/` with preferences template), and `make install-user` (composite of all three). Each requires the source repo to be cloned before any install can run.

Three forces drive the change:

1. **Not distributable.** There is no way to hand a user a single artifact that performs installation. The repo clone is the distribution mechanism.
2. **Fragile coupling.** Install behavior is defined in `make`, which is not portable, not testable, and not versionable as a first-class artifact.
3. **No version identity.** The installed files carry no record of which build produced them, making troubleshooting difficult.

RustEmbed compiles directory trees into a binary at build time. A Rust CLI binary can embed `src/skills/` and `src/agents/` and reproduce the exact same file layout as the current `make` targets — without requiring the source repo at install time.

## Options

### Option A: Single `adr-skills` binary with subcommands

Add a new `adr-skills` crate to the existing Cargo workspace. Use RustEmbed to embed `src/skills/` and `src/agents/`. Expose subcommands: `install skills`, `install agents`, `install all`, and `init`.

| | |
|---|---|
| **Pro** | Clean namespace — the binary name matches the project |
| **Pro** | Subcommand hierarchy is discoverable and extensible |
| **Pro** | Fits the existing workspace without changing `adr-db` scope |
| **Pro** | Version info embeddable via compile-time env vars |
| **Con** | Adds a new crate to maintain |

### Option B: Extend existing `adr-db` binary

Add `install` subcommands to `adr-db`, reusing its existing `clap` setup.

| | |
|---|---|
| **Pro** | No new crate; reuses existing build and release pipeline |
| **Con** | `adr-db` is a database tool — install logic is unrelated to its purpose |
| **Con** | Embeds ~850KB of skill/agent files into a binary with a different primary function |
| **Con** | Makes `adr-db` a multi-purpose binary without a clear identity |

### Option C: Separate `adr-install` binary

A minimal crate whose only function is installation. No subcommands beyond install and init.

| | |
|---|---|
| **Pro** | Single-purpose, minimal surface area |
| **Con** | Binary name `adr-install` signals a one-time tool, not an ongoing CLI |
| **Con** | Harder to extend if new install-adjacent commands are needed |
| **Con** | Splits the `adr-*` CLI surface without a clear gain over Option A |

### Not considered: shell install script (`curl | sh`)

A shell script would solve distributability (Force 1) but not version identity (Force 3) without additional tooling. It also cannot embed files — it would need a hosted archive, adding a release infrastructure dependency. Rejected because the Rust workspace already exists and RustEmbed solves embedding natively.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

None. RustEmbed is production-stable. The workspace pattern is already established. Binary naming follows the project name directly.

## Decision

In the context of a make-based install system that requires repo cloning and produces no distributable artifact, facing the need to ship a standalone binary that installs skills and agents with the same file layout as the current `make` targets, we decided to add a new `adr-skills` crate to the Cargo workspace using RustEmbed to achieve a self-contained, versionable install binary, accepting the maintenance cost of one additional crate.

The `adr-skills` binary exposes the following subcommands:

| Subcommand | Replaces |
|---|---|
| `adr-skills install skills` | `make install-skills` |
| `adr-skills install agents` | `make install-agents` |
| `adr-skills install all` | `make install-skills` + `make install-agents` |
| `adr-skills init` | `make init-project` |
| `adr-skills setup` | `make install-user` (runs `install all` + `init`) |

Install destinations default to `~/.copilot/skills/` and `~/.copilot/agents/`. A `--prefix` flag overrides the base path for testing and non-standard environments. Re-running any install subcommand overwrites existing files (rm + write, matching current Makefile behavior). On failure (permission denied, disk full), the binary reports what was written and what remains, enabling partial recovery.

## Consequences

**Positive**
- Installation requires only a binary download — no repo clone, no `make`.
- The binary carries its own version, built from compile-time env vars (`CARGO_PKG_VERSION`, git commit SHA).
- The subcommand structure is extensible for future install-adjacent commands.
- The existing Makefile targets remain available for developer workflows.
- `adr-skills setup` provides a single first-run command replacing `make install-user`.

**Negative**
- A new crate adds surface area to the workspace (build, CI, release).
- Binary size increases by ~850KB (embedded skills and agents), growing as content is added.
- Every skill or agent file change requires a new binary build and release — the rebuild-and-redistribute cycle replaces the current `git pull && make install` workflow.
- Dual-maintenance: the Makefile and the binary must produce identical output. Both paths must stay synchronized as skills and agents change. Revisit Makefile preservation when no active workflow depends on `make install-*` directly.
- Build requires `src/skills/` and `src/agents/` to be present at compile time. A sparse checkout or directory restructure will produce a build failure. A `build.rs` guard should emit a clear compile error when either directory is missing.

**Neutral**
- The installed file layout is unchanged — `~/.copilot/skills/` and `~/.copilot/agents/` receive the same content as before.

## Quality Strategy

- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests — install paths are validated by unit tests with `--prefix`; end-to-end verification against real Copilot runtime is out of scope~~
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

Unit tests cover: embedded file enumeration, path construction matching the make target layout, and idempotent re-install (overwrite behavior). Tests use `--prefix` to write to a temp directory, avoiding real `HOME` writes. User documentation covers the binary's subcommands, the `--prefix` flag, and the migration path from `make install-*`.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

Makefile targets are preserved as a backwards-compatible dev convenience. The binary is the new primary distribution artifact.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
The current installation mechanism (`make install-skills`, `make install-agents`, `make install-user`) requires cloning the repo and running make targets. This couples installation to the dev environment and is not distributable as a standalone tool. A Rust binary using RustEmbed would compile all skill files and agent definitions into a single binary, enabling `adr-skills install` and `adr-skills init` commands without requiring the source repo.

**Tolerance:**
- Risk: Low — the approach (RustEmbed) is proven and well-documented
- Change: Medium — replaces make targets but preserves the same output layout
- Improvisation: Low — user has a clear direction

**Uncertainty:**
- Known: RustEmbed can embed directory trees at compile time; the existing Cargo workspace supports adding a new crate; target directories are `~/.copilot/skills/` and `~/.copilot/agents/`
- Uncertain: Binary naming convention; whether to also embed preferences templates; how to handle version reporting

**Options:**
- Target count: 2-3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Option A: Single `adr-skills` binary with subcommands (`install skills`, `install agents`, `init`)
- Option B: Extend existing `adr-db` binary with install subcommands
- Option C: Separate `adr-install` binary dedicated to installation only
