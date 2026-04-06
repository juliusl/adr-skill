# 42. Add project-scoped preferences to prototype-adr

Date: 2026-04-06
Status: Planned
Last Updated: 2026-04-06
Links:
- Extends [ADR-0020](0020-establish-adr-directory-as-project-scoped-convention.md) (`.adr/` as project-scoped directory)
- Related to [ADR-0011](0011-use-xdg-config-directory-for-user-configuration-state.md) (user-scoped config precedent)
- Related to [ADR-0012](0012-use-toml-as-the-configuration-file-format.md) (TOML format precedent)

## Context

The `author-adr` and `implement-adr` skills read user-scoped preferences from `~/.config/adr-skills/preferences.toml`. The `prototype-adr` skill does the same — it reads `[prototype].isolation`, `[prototype].runtime`, and `[prototype].teardown` from that file.

ADR-0020 established `.adr/` as a project-scoped directory for data and configuration. However, no skill currently reads a project-scoped `.adr/preferences.toml`. Project-scoped preferences would let individual repos configure skill behavior without changing the user's global config.

**The problem:** ADR-0041 proposes adding a `[prototype.persona]` table with experiment protocol settings. This config is project-specific (different repos may have different experiment protocols). Storing it in `~/.config/adr-skills/preferences.toml` (user-scoped) would leak one project's experiment protocol into every project. It belongs in `.adr/preferences.toml` (project-scoped).

But `prototype-adr` does not read `.adr/preferences.toml`. This ADR adds that capability.

**Constraints:**
- Project-scoped preferences override user-scoped preferences for the same key.
- Missing `.adr/preferences.toml` is normal — fall back to user-scoped config.
- The merge is shallow — project-scoped keys override, they don't deep-merge.

### Decision Drivers

- **Consistency with ADR-0020** — `.adr/` is already the project-scoped convention. Preferences should live there.
- **Isolation** — project-specific config should not leak into other projects.
- **Minimal change** — add config reading, not config infrastructure.

## Options

### Option 1: Read `.adr/preferences.toml` with project-over-user precedence

Add a second config read step to prototype-adr's Step 0. After reading `~/.config/adr-skills/preferences.toml`, also read `.adr/preferences.toml` if it exists. Project-scoped keys override user-scoped keys.

```
Resolution order:
1. ~/.config/adr-skills/preferences.toml  (user-scoped, base)
2. .adr/preferences.toml                  (project-scoped, overrides)
3. Built-in defaults                      (fallback for missing keys)
```

**Strengths:**
- Follows the standard layered config pattern (user → project → defaults).
- `.adr/` directory already exists per ADR-0020.
- No new file formats or directories.

**Weaknesses:**
- Shallow override means you cannot partially extend a user-scoped table — the project-scoped table replaces it entirely.

### Option 2: Use environment variables for project-specific overrides

Let projects set `ADR_PROTOTYPE_ISOLATION=container` etc. in their Makefile or shell environment.

**Strengths:**
- No file to manage.
- Works in CI where `.adr/` may not exist.

**Weaknesses:**
- Environment variables don't persist — must be set every session.
- No version control — env vars aren't committed to the repo.
- Doesn't integrate with the existing TOML config pattern.

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — layered config reading is a standard pattern.

## Decision

In the context of **needing project-scoped preferences for prototype-adr experiment protocols**, facing **the choice between file-based config and environment variables**, we chose **reading `.adr/preferences.toml` with project-over-user precedence (Option 1)** over **environment variables (Option 2, not persistent or version-controlled)** to achieve **project-specific experiment configuration that is version-controlled and follows the existing `.adr/` convention**, accepting **that shallow override does not support partial table extension**.

### Config reading order

```
1. Read ~/.config/adr-skills/preferences.toml → base config
2. Read .adr/preferences.toml (if exists)     → override matching keys
3. Apply built-in defaults for missing keys
```

### Skill changes

In prototype-adr's `## Configuration` section, add after the user-scoped config:

```
**Project-scoped overrides:** If `.adr/preferences.toml` exists in the
project root, its keys override user-scoped settings. This allows
per-project experiment configuration without changing global preferences.
```

### Scope

This ADR only adds project-scoped config reading to `prototype-adr`. The `author-adr` and `implement-adr` skills may adopt the same pattern in a future ADR, but that is out of scope here.

## Consequences

**Positive:**
- Projects can configure experiment behavior without changing global user preferences.
- Follows the `.adr/` convention established by ADR-0020.
- Unblocks ADR-0041 (persona experiment protocol needs project-scoped config).

**Negative:**
- Shallow override means a project-scoped `[prototype]` table replaces the entire user-scoped `[prototype]` table, not individual keys within it. Users who set `isolation` globally and want to override only `persona.embed_source` per-project must repeat the `isolation` key in the project file.

**Neutral:**
- `author-adr` and `implement-adr` are not changed. They may adopt the same pattern later.

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
- [ ] User documentation

### Additional Quality Concerns

- **Missing file handling** — if `.adr/preferences.toml` does not exist, behavior is identical to today. No error, no warning.
- **TOML parsing errors** — if the file exists but contains invalid TOML, warn and fall back to user-scoped config only.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** Intentionally scoped to prototype-adr only. Other skills can adopt the pattern later.

---

## Comments
