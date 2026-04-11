# 71. Add version-aware install to adr-skills CLI

Date: 2026-04-10
Status: Accepted
Last Updated: 2026-04-10
Links:

## Context

The `adr-skills` CLI binary embeds skill and agent files at compile time via `rust_embed`. The `install` command writes these embedded files to `~/.copilot/skills/` and `~/.copilot/agents/`. Currently, `install_skills()` calls `fs::remove_dir_all(&skills_dir)` and then writes all embedded files unconditionally.

This creates a data-loss risk: a user who has modified installed files (e.g., customized an agent persona, edited a skill reference) will lose those changes on the next install. The embedded files may also be older than the installed files if the user installed a newer version from a different source and then runs an older binary.

There is no `--dry-run` flag to preview what would change, and no version comparison to skip files that are already up-to-date or newer.

## Options

### Option 1: Version-stamped install with dry-run

Embed a version identifier (the Git commit SHA, already available via `GIT_COMMIT_SHA` env var) into a `.version` marker file alongside installed assets. On install:
1. Read the existing `.version` file (if present).
2. If the embedded version matches the installed version, report "already up-to-date" and exit.
3. If different or absent, proceed with install.
4. Add `--dry-run` flag that reports what would be written without writing.
5. Add `--force` flag to bypass version check and overwrite unconditionally (current behavior).

**Strengths:** Simple to implement. Version marker is low-overhead. Dry-run gives users confidence before overwriting.

**Weaknesses:** Per-directory version granularity — cannot detect individual file modifications. Users who edit installed files and then run `--force` still lose changes.

### Option 2: Per-file content hash comparison

Before writing each file, compute a hash (SHA-256) of the embedded content and compare against the target file. Skip files whose content matches. For files that differ, check if the target is newer (by mtime) and warn instead of overwriting.

**Strengths:** Fine-grained — detects exactly which files need updating. Protects user modifications.

**Weaknesses:** More complex. Hash computation on every file adds overhead (though negligible for ~100 files). Requires a decision on mtime-vs-content conflicts.

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps

**Validation needs:**

## Decision

We chose **Option 1: Version-stamped install with dry-run** over per-file hash comparison. The version-stamp approach matches the binary's release model — the binary is versioned as a unit, and all embedded files change together at build time. Per-file granularity solves a problem (individual file modification) that is better addressed by telling users not to edit installed files directly and instead working in the repo.

The `--dry-run` flag addresses the user's need for confidence before overwriting. The `--force` flag preserves current behavior for CI/automation scripts.

## Consequences

**Positive:**
- Users can preview install changes before applying them.
- Re-running `adr-skills install all` on an up-to-date installation is a no-op.
- CI scripts can use `--force` without version-check overhead.

**Negative:**
- Users who modify installed files and re-install (without `--force`) will see "already up-to-date" even though their files differ from embedded — the version check is coarse.
- Adds a `.version` file to the install directory (minor clutter).

**Neutral:**
- Existing install behavior is preserved via `--force` — no breaking change for current users.

## Quality Strategy

- ~~Introduces major semantic changes~~
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- ~~User documentation~~

### Additional Quality Concerns

- Test: verify `--dry-run` produces output but no file changes.
- Test: verify version-match results in early exit.
- Test: verify `--force` bypasses version check.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
The install command blindly overwrites installed files. User reported this as a data-loss risk for modified agent/skill files. Direction: add version checking and dry-run.

**Tolerance:**
- Risk: Low — known solution pattern
- Change: Low — additive flags, backwards compatible
- Improvisation: Low — standard CLI pattern

**Uncertainty:**
Certain: the problem exists (rm -rf then rewrite). Uncertain: whether per-file or per-directory granularity is needed.

**Options:**
- Target count: 2-3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Version-stamp file with dry-run/force flags
- Per-file content hash comparison

**Pre-review notes:**

---

## Comments

