# 28. Locate Rust workspace for adr-db in the repository

Date: 2026-04-04
Status: Accepted
Last Updated: 2026-04-04
Links:
- Amends [ADR-0026](0026-add-rust-cli-for-data-plumbing.md) (ADR-0026 specified `tools/` — this ADR re-evaluates the directory location)
- Related to [ADR-0027](0027-use-diesel-with-sqlite-for-adr-db-persistence-layer.md) (Diesel migrations and `diesel.toml` must live relative to the workspace)

## Context

[ADR-0026](0026-add-rust-cli-for-data-plumbing.md) decided on a single Rust binary (`adr-db`) and placed it in a `tools/` directory at the repo root. That placement was part of the broader decision about the binary's existence and shape — it was not deeply evaluated against alternatives. Before implementation begins, this is the right moment to decide where the Cargo workspace actually lives, since the location affects build integration, contributor navigation, and how future crates are added.

**The problem:** The repository currently has this top-level structure:

```
adr-skills/
├── AGENTS.md
├── Makefile
├── README.md
├── docs/adr/           # decision records
├── scripts/             # legacy/shared scripts
└── src/
    └── skills/          # shell-script based skills
        ├── author-adr/
        └── implement-adr/
```

The Rust binary needs a home. The choice is a Cargo workspace (to accommodate `adr-db` now and future crates later). Where the workspace root sits determines how the project reads, how builds are wired, and whether the Rust code feels native or bolted on.

**Why this matters now:**
- ADR-0026 is ready to implement. The directory structure must be decided first — `cargo init`, `diesel.toml`, migration directories, and Makefile targets all depend on the path.
- The workspace will grow. Future crates (e.g., shared library, additional plumbing tools) should have a natural place to land.
- Contributors should be able to navigate the repo and immediately understand where Rust code lives and how it relates to the shell-script skills.

**Constraints:**
- **Cargo workspace** — the directory must be a workspace root with `adr-db` as the first member crate. Future crates may be added.
- **Coexistence** — shell-script skills in `src/skills/` are unaffected. The Rust workspace must not interfere with existing Make targets or script paths.
- **Build integration** — the root Makefile must be able to invoke `cargo build` in the workspace via a `build-tools` (or equivalent) target.

### Decision Drivers

- **Discoverability** — contributors should find the Rust code intuitively. The directory name and location should signal "this is where compiled code lives."
- **Workspace scalability** — adding a second crate to the workspace should be a natural operation, not a restructuring exercise.
- **Separation of concerns** — compiled Rust code and interpreted shell scripts serve different roles. The directory structure should reflect (or intentionally blur) this distinction.
- **Convention alignment** — preference for patterns that Rust developers will recognize from other projects.
- **Minimal disruption** — avoid unnecessary top-level directory proliferation. The repo should stay navigable.

## Options

### Option 1: `tools/` (ADR-0026 original)

Keep the `tools/` directory as specified in ADR-0026. The workspace root is `tools/Cargo.toml`, with member crates underneath.

```
tools/
├── Cargo.toml          # [workspace] members = ["adr-db"]
├── Cargo.lock
└── adr-db/
    ├── Cargo.toml
    ├── diesel.toml
    ├── migrations/
    └── src/
```

**Strengths:**
- Already specified in ADR-0026 — no amendment needed.
- Clear separation: `tools/` is for compiled developer tooling, `src/` is for skill source. Different roles, different directories.
- Name is language-agnostic — if non-Rust tools are ever added, they fit here too.

**Weaknesses:**
- `tools/` is generic — doesn't signal "Rust workspace" to a contributor scanning the repo.
- Creates a conceptual split: "source code" is in both `src/` and `tools/`, but for different reasons. Contributors must learn the convention.
- Adding `tools/` introduces a new top-level directory when `src/` already exists as the source root.

### Option 2: `src/crates/`

Place the Cargo workspace under `src/crates/`. All source code — both shell skills and Rust crates — lives under `src/`.

```
src/
├── skills/              # existing shell-script skills
│   ├── author-adr/
│   └── implement-adr/
└── crates/              # Cargo workspace root
    ├── Cargo.toml       # [workspace] members = ["adr-db"]
    ├── Cargo.lock
    └── adr-db/
        ├── Cargo.toml
        ├── diesel.toml
        ├── migrations/
        └── src/
```

**Strengths:**
- All source code under one roof — `src/skills/` for scripts, `src/crates/` for compiled code. Consistent `src/` root.
- `crates/` is a recognizable Rust convention, even nested under `src/`.
- Clean workspace scalability — adding `src/crates/adr-core/` is natural.

**Weaknesses:**
- Deeper nesting — `src/crates/adr-db/src/main.rs` is four levels deep from the repo root.
- `src/` becomes a mixed-language directory — skills are Bash, crates are Rust. The `src/` prefix suggests homogeneity that doesn't exist.
- Makefile build target needs to reach into `src/crates/` — slightly longer path.

### Option 3: `src/adr-db/`

Place `adr-db` directly under `src/` with the workspace root at `src/Cargo.toml`.

```
src/
├── Cargo.toml           # [workspace] members = ["adr-db"]
├── Cargo.lock
├── skills/              # existing shell-script skills
└── adr-db/
    ├── Cargo.toml
    ├── diesel.toml
    ├── migrations/
    └── src/
```

**Strengths:**
- Flat structure — `adr-db` is a direct sibling of `skills/` under `src/`.
- No new top-level directory — everything stays under `src/`.
- Short path — `src/adr-db/src/main.rs`.

**Weaknesses:**
- Workspace root (`src/Cargo.toml`) sits alongside `skills/`, which is not a Rust crate. Cargo ignores it, but it's conceptually odd — `src/` looks like a Cargo workspace root but contains non-Rust directories.
- Scaling is awkward — adding a second crate means `src/adr-core/` alongside `src/skills/`. The `src/` directory becomes a mix of workspace members and non-Rust directories with no grouping.
- `Cargo.lock` at `src/` level may confuse contributors who expect it at a workspace-specific location.

### Option 4: `crates/adr-db/` (top-level workspace)

Create a top-level `crates/` directory as the Cargo workspace root. This is a common Rust monorepo convention.

```
crates/
├── Cargo.toml           # [workspace] members = ["adr-db"]
├── Cargo.lock
└── adr-db/
    ├── Cargo.toml
    ├── diesel.toml
    ├── migrations/
    └── src/
```

**Strengths:**
- Immediately recognizable Rust convention — `crates/` at the repo root is a well-established pattern in Rust projects (e.g., `bevy`, `nushell`).
- Clean workspace scalability — adding `crates/adr-core/` is the expected operation.
- Clear separation — `crates/` is Rust, `src/` is skills. No ambiguity.
- Short, clean path — `crates/adr-db/src/main.rs`.

**Weaknesses:**
- New top-level directory (same as `tools/`), adding to repo breadth.
- Name is Rust-specific — if non-Rust compiled code were ever added, `crates/` wouldn't be the natural home. (Though this is unlikely given the Rust constraint.)
- Splits "source code" across `src/` and `crates/` — a contributor must know that skills are in `src/` and compiled tools are in `crates/`.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — this is a directory structure decision. Validation occurs by running `cargo init` and `cargo build` during implementation.

## Decision

In the context of **placing the Cargo workspace for `adr-db` and future crates in the repository**, facing **the need for a location that is discoverable, scalable, and convention-aligned**, we decided for **`crates/adr-db/` with a top-level workspace root (Option 4)** and neglected **`tools/` (Option 1, generic naming), `src/crates/` (Option 2, unnecessary nesting), and `src/adr-db/` (Option 3, awkward workspace root placement)**, to achieve **a recognizable Rust workspace layout where contributors immediately understand where compiled code lives and adding future crates is natural**, accepting that **this introduces a new top-level directory and splits source code across `src/` (skills) and `crates/` (Rust)**.

### Why `crates/` over the alternatives

The central argument is **convention alignment**. `crates/` at the repo root is a widely recognized Rust workspace pattern. A contributor familiar with Rust who clones this repo will immediately know where to look for compiled code. The alternatives either obscure this (`tools/`), nest it too deep (`src/crates/`), or create an awkward workspace root (`src/adr-db/`).

- **vs. `tools/`:** `tools/` is language-agnostic, which sounds like a strength but is actually a weakness — it doesn't signal that this is a Cargo workspace. A Rust developer would look for `Cargo.toml` or `crates/`, not `tools/`.
- **vs. `src/crates/`:** Adds a nesting level without benefit. `src/` already contains non-Rust code (`skills/`), so the `crates/` subdirectory is doing the same separation work that a top-level `crates/` would do, but one level deeper.
- **vs. `src/adr-db/`:** Putting the workspace root at `src/Cargo.toml` next to `skills/` creates a misleading layout — `src/` would look like a Cargo workspace root but contain non-Rust directories.

### Amendment to ADR-0026

This ADR amends ADR-0026's directory structure. Where ADR-0026 specified:

```
tools/
├── Cargo.toml
└── src/
```

This ADR replaces it with:

```
crates/
├── Cargo.toml           # [workspace] members = ["adr-db"]
├── Cargo.lock
└── adr-db/
    ├── Cargo.toml
    ├── diesel.toml
    ├── migrations/
    └── src/
```

All other aspects of ADR-0026 (binary name `adr-db`, subcommand design, JSONL ingestion, plumbing/porcelain philosophy) remain unchanged. The root Makefile `build-tools` target should point to `crates/` instead of `tools/`.

### Resulting repo layout

```
adr-skills/
├── AGENTS.md
├── Makefile              # build-tools target → cargo build in crates/
├── README.md
├── crates/               # Cargo workspace (NEW)
│   ├── Cargo.toml
│   ├── Cargo.lock
│   └── adr-db/
│       ├── Cargo.toml
│       ├── diesel.toml
│       ├── migrations/
│       └── src/
├── docs/adr/
├── scripts/
└── src/
    └── skills/
        ├── author-adr/
        └── implement-adr/
```

## Consequences

**Positive:**
- Contributors familiar with Rust will immediately recognize the `crates/` directory and understand its purpose.
- Adding a future crate (e.g., `crates/adr-core/`) requires only adding a directory and updating the workspace `members` list — no restructuring.
- Clean separation between skill source code (`src/skills/`) and compiled tooling (`crates/`). Different build systems, different directories.
- Short, descriptive paths — `crates/adr-db/src/main.rs` is clear and shallow.

**Negative:**
- Introduces a new top-level directory, increasing repo breadth. The repo root will have `crates/`, `docs/`, `scripts/`, `src/` — four content directories.
- Source code lives in two places: `src/` for skills, `crates/` for Rust. Contributors must learn this split.
- Amends ADR-0026, which means ADR-0026's directory layout sections are now partially outdated. References to `tools/` in ADR-0026 should be read as `crates/`.

**Neutral:**
- The root Makefile `build-tools` target path changes from `tools/` to `crates/`. This is a one-line change.
- Whether `Cargo.lock` is committed (recommended for binaries) is a project convention to establish during implementation.
- The `crates/` name is Rust-specific. If non-Rust compiled code were ever needed, a separate directory would be appropriate — but this is unlikely given the project's Rust constraint.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

- **ADR-0026 cross-reference** — ADR-0026's directory layout sections reference `tools/`. A note should be added to ADR-0026 indicating that the directory location was amended by this ADR.
- **Makefile integration** — the `build-tools` target must point to `crates/`, not `tools/`.
- **README update** — the repository README should document the `crates/` directory and its relationship to `src/skills/`.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR amends ADR-0026's directory structure only. All other aspects of ADR-0026 (binary name, subcommand design, plumbing philosophy) are unchanged. ADR-0026 should be annotated with a reference to this amendment once accepted.

---

## Comments
