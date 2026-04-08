# 58. Relocate Cargo workspace from crates/ to src/crates/

Date: 2026-04-07
Status: Planned
Last Updated: 2026-04-07
Links:
- Supersedes [ADR-0028](0028-locate-rust-workspace-for-adr-db-in-the-repository.md) (ADR-0028 chose top-level `crates/` — this ADR relocates it under `src/`)

## Context

[ADR-0028](0028-locate-rust-workspace-for-adr-db-in-the-repository.md) evaluated four locations for the Cargo workspace and chose top-level `crates/` for Rust convention alignment. That decision prioritized discoverability for Rust developers over source code cohesion. After working with this layout, the tradeoff ADR-0028 acknowledged — "source code lives in two places: `src/` for skills, `crates/` for Rust" — has proven to be the dominant ergonomic issue.

**The problem:** The repository has two top-level source directories with no structural relationship between them:

```
adr-skills/
├── crates/           # Rust source code
│   ├── adr-db/
│   ├── adr-db-lib/
│   └── adr-format/
└── src/
    └── skills/       # Shell-script skills
        ├── author-adr/
        └── implement-adr/
```

When navigating the repo, `crates/` feels disconnected from the rest of the source tree. Contributors must remember that source code is split across two unrelated top-level directories. The Cargo workspace has grown to three crates (`adr-db`, `adr-db-lib`, `adr-format`), making the top-level directory more prominent than when ADR-0028 was written for a single crate.

**Why revisit now:** The original concern ADR-0028 flagged about `src/crates/` — deeper nesting and mixed-language `src/` — has turned out to be less problematic than the navigational friction of the current layout. One extra directory level is a minor cost. A mixed-language `src/` directory is actually intuitive: `src/` means "all source code," with subdirectories organizing by kind (`skills/` for scripts, `crates/` for Rust).

### Decision Drivers

- **Source cohesion** — all source code should be reachable from a single top-level directory
- **Navigational ergonomics** — reduce cognitive overhead when exploring the repo
- **Convention alignment** — `crates/` remains recognizable as a Rust workspace, just nested one level deeper
- **Minimal disruption** — the move is a rename operation; internal workspace structure is unchanged

## Options

### Option 1: Keep top-level `crates/` (status quo)

Leave the layout as ADR-0028 decided. Accept the two-directory source split.

**Strengths:** No migration work. Matches standalone Rust project conventions.

**Weaknesses:** The navigational friction persists. Source code remains split across two unrelated top-level directories.

### Option 2: Move to `src/crates/`

Relocate the Cargo workspace under `src/`, creating `src/crates/` as the workspace root.

```
src/
├── agents/              # agent definition files
├── skills/              # shell-script skills
│   ├── author-adr/
│   └── implement-adr/
└── crates/              # Cargo workspace root
    ├── Cargo.toml
    ├── Cargo.lock
    ├── adr-db/
    ├── adr-db-lib/
    └── adr-format/
```

**Strengths:**
- All source code lives under `src/`. One top-level directory for all source.
- `crates/` name is preserved — still recognizable as a Rust workspace.
- Clean organization: `src/skills/` for scripts, `src/crates/` for compiled code.
- Reduces top-level directory count by one.

**Weaknesses:**
- One additional nesting level for Rust paths (e.g., `src/crates/adr-db/src/main.rs`).
- Requires updating Makefile, AGENTS.md, .gitignore, and documentation references.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — this is a directory rename. Validation is running `make build-tools` and `make test` after the move.

## Decision

In the context of **the Cargo workspace location in the repository**, facing **navigational friction from source code split across two unrelated top-level directories**, we decided for **`src/crates/` (Option 2)** and against **keeping top-level `crates/` (Option 1)**, to achieve **a unified source tree where all source code is reachable from `src/`**, accepting that **Rust paths gain one nesting level and references across the repo need updating**.

This supersedes ADR-0028. ADR-0028's analysis of `src/crates/` identified deeper nesting and mixed-language `src/` as weaknesses. After working with the chosen layout, the source code split proved to be the larger ergonomic cost. The nesting concern is marginal (one extra directory level), and a mixed-language `src/` is intuitive rather than confusing.

### Migration

1. `git mv crates src/crates`
2. Update `Makefile` — change `build-tools` target path from `crates/` to `src/crates/`
3. Update `AGENTS.md` — all P-13 references from `crates/` to `src/crates/`
4. Update `.gitignore` — adjust any `crates/`-specific patterns
5. Update `src/skills/author-adr/scripts/wi-full-agent-adr-format.sh` — fix relative path to `adr-format` binary (line 15: `../../../../crates/target/release/adr-format` → `../../../crates/target/release/adr-format` — one fewer parent traversal since `crates/` moves from repo root to `src/`)
6. Update documentation references in `docs/` — historical plans and ADRs that reference `crates/` path (informational updates only; no semantic changes to past decisions)

### Resulting repo layout

```
adr-skills/
├── AGENTS.md
├── Makefile              # build-tools target → cargo build in src/crates/
├── README.md
├── docs/
│   └── adr/
└── src/
    ├── agents/           # agent definition files
    ├── skills/           # shell-script skills
    │   ├── author-adr/
    │   └── implement-adr/
    └── crates/           # Cargo workspace
        ├── Cargo.toml
        ├── Cargo.lock
        ├── adr-db/
        ├── adr-db-lib/
        └── adr-format/
```

## Consequences

**Positive:**
- All source code lives under `src/`. Contributors explore one directory, not two.
- Reduces top-level directory count — the repo root becomes cleaner.
- `src/crates/` is still recognizable as a Rust workspace. The `crates/` convention survives the nesting.

**Negative:**
- Rust file paths are one level deeper (e.g., `src/crates/adr-db/src/main.rs` vs `crates/adr-db/src/main.rs`).
- Migration requires updating references in Makefile, AGENTS.md, .gitignore, and documentation files.
- Historical ADRs and plans that reference `crates/` become slightly stale (though the decisions they record remain valid).

**Neutral:**
- Internal workspace structure (`Cargo.toml`, member crates, `Cargo.lock`) is unchanged. Only the workspace root path changes.
- Build commands change path but not semantics: `cargo build --manifest-path src/crates/Cargo.toml`.

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

- **Build verification** — `make build-tools` must pass after the move.
- **Test verification** — `make test` and `cd src/crates && cargo test` must pass.
- **Reference sweep** — all `.md` files referencing `crates/` in actionable paths (Makefile, AGENTS.md) must be updated. Historical references in `docs/adr/` and `docs/plans/` are informational and can be updated opportunistically.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This is a simple directory relocation driven by lived experience with the current layout. ADR-0028 should be marked Superseded with a link to this ADR once accepted.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
After working with top-level `crates/` for a while, the split between `src/` and `crates/` feels awkward to navigate. Moving `crates/` under `src/` consolidates all source code in one place. The user explicitly requested this change and asked to supersede ADR-0028 formally.

**Tolerance:**
- Risk: Low — directory rename with known impact surface
- Change: Low — same workspace structure, different parent directory
- Improvisation: Low — the destination is specified (`src/crates/`)

**Uncertainty:**
- Certain: the move improves navigation ergonomics for the maintainer
- Certain: the migration is mechanical (rename + reference updates)
- No open questions

**Options:**
- Target count: 2 (keep vs move)
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
1. Keep top-level `crates/` (status quo)
2. Move to `src/crates/` (user's request)

### Q&A Addendum — Review Round 1

**Finding 1 (M): Migration step 5 shows identical from/to paths — Address**
Step 5 had a copy-paste error showing `../../crates/` → `../../crates/` (identical paths). The actual script (`wi-full-agent-adr-format.sh` line 15) uses `../../../../crates/target/release/adr-format`. After the move, `crates/` becomes a sibling of `skills/` under `src/`, so the path drops one parent traversal: `../../../crates/target/release/adr-format`. Fixed the step to reflect the real change with full paths.

**Finding 2 (L): ADR-0028 prematurely marked Superseded — Defer**
Valid observation. ADR-0028 was marked `Superseded by ADR-0058` while ADR-0058 is still `Proposed`. The pre-review notes in this ADR already say "once accepted," which is the correct sequencing. However, P-2 (Cross-ADR Modification Guardrail) prevents modifying ADR-0028 in this session. The status correction should happen as a separate action — either revert ADR-0028 to its prior status now and re-apply when ADR-0058 is accepted, or apply the supersession as part of acceptance.

**Finding 3 (L): Rubber Stamp signal from Draft Worksheet — Reject**
Reviewer noted the user-requested decision for transparency but recommended no action. The analysis is genuine and balanced — both options were evaluated with strengths and weaknesses, and the decision drivers are grounded in lived experience with the current layout. No change needed.
