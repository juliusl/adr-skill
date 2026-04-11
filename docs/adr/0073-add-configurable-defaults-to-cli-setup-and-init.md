# 73. Add configurable defaults to CLI setup and init

Date: 2026-04-10
Status: Planned
Last Updated: 2026-04-10
Links: ADR-0071, ADR-0072

## Context

The `adr-skills init` and `adr-skills setup` commands write a fixed `preferences.toml` with hard-coded values. The dispatch hooks reference specific agent versions (e.g., `tpm = "juliusl-tpm-v1"`) that become stale as agents are updated (e.g., `juliusl-tpm-v2` already exists). The participation mode (`autonomous`), auto-commit (`true`), and all dispatch hook assignments are baked into the binary at compile time.

Users cannot customize the generated preferences without editing the file after generation. There are no CLI flags to select participation mode, enable/disable specific dispatch hooks, or choose agent versions.

## Options

### Option 1: CLI flags for common settings

Add flags to `init` and `setup` for the most common customizations:

```
adr-skills init --participation guided --auto-commit false
adr-skills setup --tpm juliusl-tpm-v2 --no-ux-review
```

Use the embedded defaults for any flag not provided. This keeps the simple `adr-skills setup` path working while allowing customization.

**Strengths:** Discoverable via `--help`. No external files needed. Backwards compatible — existing `setup` behavior unchanged when no flags given.

**Weaknesses:** Adding a flag per dispatch hook creates a long flag list. New hooks require new flags and a binary rebuild.

### Option 2: External defaults template file

Read defaults from a user-scoped template file (e.g., `~/.config/adr-skills/init-template.toml`). If present, use it as the template for generated `preferences.toml`. If absent, fall back to embedded defaults.

**Strengths:** Fully customizable without binary changes. Users set their template once and every `init` uses it.

**Weaknesses:** Requires users to create the template file before first use. Less discoverable than CLI flags. Two-file configuration (template + generated) can confuse.

### Option 3: Flags for mode + template for dispatch

Hybrid: use CLI flags for the high-level settings (participation, auto-commit, auto-delegate) and a template file for dispatch hook configuration. This separates "how to behave" (flags) from "who to dispatch" (template).

**Strengths:** Flags cover the 80% case. Template covers the power-user customization. Each mechanism handles what it's good at.

**Weaknesses:** Two configuration mechanisms to document and maintain.

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps

**Validation needs:**

## Decision

We chose **Option 1: CLI flags for common settings**. The binary is already the source of truth for embedded defaults — extending it with flags is natural. The flag list is bounded (participation, auto-commit, auto-delegate, plus one flag per dispatch hook). Adding a new hook requires a binary update, but adding a new hook already requires a binary update (to embed the new agent files). The one-command `adr-skills setup` experience is preserved as the default path.

Embedded defaults should reference the latest agent versions at build time. When `juliusl-tpm-v2` replaces `v1`, the embedded default updates in the next release.

## Consequences

**Positive:**
- Users can customize setup without post-editing files.
- `--help` documents all available options.
- Simple path (`adr-skills setup`) unchanged.

**Negative:**
- Flag list grows with each new dispatch hook. Bounded by the dispatch hook count (~5 hooks currently).
- Users with highly custom setups still need to edit the generated file after init.

**Neutral:**
- Embedded defaults track the latest agent versions at build time — this is a build-time concern, not a runtime one.

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

- Test: verify `setup` with no flags produces the same output as current behavior.
- Test: verify each flag overrides the corresponding embedded default.
- Test: verify `--help` lists all flags with descriptions.

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
Setup writes hard-coded preferences. Agent versions go stale. No way to customize without post-editing. Direction: add CLI flags.

**Tolerance:**
- Risk: Low
- Change: Low — additive flags
- Improvisation: Low

**Uncertainty:**
Certain: the defaults are hard-coded. Open: whether flags or template file is the right mechanism.

**Options:**
- Target count: 2-3
- [ ] Explore additional options

**Candidates:**
- CLI flags for common settings
- External defaults template
- Hybrid (flags + template)

**Pre-review notes:**

---

## Comments

