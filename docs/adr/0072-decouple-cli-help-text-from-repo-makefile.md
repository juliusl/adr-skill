# 72. Decouple CLI help text from repo Makefile

Date: 2026-04-10
Status: Accepted
Last Updated: 2026-04-10
Links: ADR-0071

## Context

The `adr-skills` CLI binary is distributed as a standalone binary (via `cargo install` or release artifacts). However, the `Setup` subcommand's help text says "Full setup: install all + init (replaces make install-user)" — referencing a Makefile target that only exists in the source repository.

A user who installs the binary via `cargo install adr-skills` may never see the repo Makefile. The help text creates a false dependency: it implies the binary is a wrapper around `make` targets, when in fact it is a self-contained replacement.

## Options

### Option 1: Remove Makefile references from help text

Update all `#[doc]` and `about` strings in `main.rs` to describe the CLI's own behavior without referencing Makefile targets. The CLI describes what it does, not what it replaces.

**Strengths:** Simple text change. Makes the CLI self-documenting. No functional change.

**Weaknesses:** Users migrating from the Makefile lose the mapping hint. Could add a migration note in README instead.

### Option 2: Add a `--help-migrate` flag

Keep the current help text but add a separate `--help-migrate` subcommand that shows the Makefile-to-CLI mapping.

**Strengths:** Preserves migration context. Clean separation.

**Weaknesses:** Over-engineering for a text fix. Adds a subcommand that will be obsolete once all users migrate.

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps

**Validation needs:**

## Decision

We chose **Option 1: Remove Makefile references from help text**. The CLI is standalone. Its help text should describe its own behavior. Migration context belongs in the README or release notes, not in `--help` output that every user sees on every invocation.

## Consequences

**Positive:**
- CLI is self-documenting for users who never see the repo.
- Help text is shorter and clearer.

**Negative:**
- Users migrating from the Makefile lose the inline mapping hint.

**Neutral:**
- A Makefile-to-CLI migration table can be added to the README for discoverability.

## Quality Strategy

- ~~Introduces major semantic changes~~
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

- Update README with Makefile-to-CLI migration table.

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

**Framing:**
The `Setup` help text says "replaces make install-user" which couples CLI identity to the repo. Direction: make help text self-contained.

**Tolerance:**
- Risk: Low
- Change: Low — text-only change
- Improvisation: Low

**Uncertainty:**
None — the problem is a string literal.

**Options:**
- Target count: 2-3
- [ ] Explore additional options

**Candidates:**
- Remove Makefile references
- Add migration subcommand

**Pre-review notes:**

---

## Comments

