# 48. Add Centralized Preferences Reference and User Guidance

Date: 2026-04-06
Status: Planned
Last Updated: 2026-04-06
Links: ADR-0011 (XDG config), ADR-0012 (TOML format), ADR-0014 (author/implement tables)

## Context

The adr-skills project has 4 skills with 14 preference keys spread across 4 separate SKILL.md files. Users discover preferences by accident — either by reading skill internals or when a skill recommends missing settings during startup.

Current state:
- No centralized reference for all preferences
- No example `preferences.toml` showing valid configuration
- No getting-started guide for new users
- README covers quick-start but not configuration depth
- Each skill documents its own keys in its SKILL.md, but users must read all 4 to build a complete picture

This creates friction for users who want to customize their workflow. The information exists but is scattered.

## Options

### Option A: Centralized PREFERENCES.md reference

Create a single `docs/PREFERENCES.md` document that:
1. Lists all preference keys across all skills with types, defaults, and descriptions
2. Provides complete example configurations for both user-scoped and project-scoped files
3. Documents the resolution order (user → project → built-in defaults)
4. Includes a quickstart section for common configurations

Add a brief "Configuration" section to README.md that points to PREFERENCES.md.

**Strengths:**
- Single source of truth for all preferences
- Discoverable from README
- Can include complete annotated example configs
- Users can copy-paste and customize

**Weaknesses:**
- Requires manual updates when skills add new keys
- Partially duplicates what's in each SKILL.md

### Option B: Interactive make target

Add `make show-config` that reads `preferences.toml` and shows current values alongside available settings.

**Strengths:**
- Dynamic — always shows current state
- Interactive and discoverable via `make help`

**Weaknesses:**
- Requires tooling implementation (script or Rust tool)
- Doesn't replace written documentation
- Only works when the repo is checked out

### Option C: README expansion only

Add a full configuration section directly to README.md with preference tables and examples.

**Strengths:**
- Users already read the README
- No new files to discover

**Weaknesses:**
- README becomes long (already 234 lines)
- Mixes development docs with user docs
- Harder to maintain as skills evolve

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — documentation conventions are well-understood.

## Decision

We will create a centralized PREFERENCES.md reference document with a brief pointer from README.md.

In the context of users needing to discover and configure skill preferences across 4 skills, facing scattered documentation that requires reading all SKILL.md files, we chose a centralized PREFERENCES.md over interactive tooling or README expansion, to achieve a single discoverable reference with example configs, accepting the maintenance cost of keeping it synchronized with individual SKILL.md files.

**Preference key inventory** (PREFERENCES.md will index all of these):

| Namespace | Key | Type | Default | Source |
|-----------|-----|------|---------|--------|
| `[author]` | `template` | string | `"nygard-agent"` | author-adr/SKILL.md |
| `[author.dispatch]` | `review` | string | `"general-purpose"` | author-adr/SKILL.md |
| `[author.dispatch]` | `editor` | string | `"interactive"` | author-adr/SKILL.md |
| `[implement]` | `participation` | string | `"guided"` | implement-adr/SKILL.md |
| `[implement]` | `auto_commit` | boolean | `false` | implement-adr/SKILL.md |
| `[prototype]` | `isolation` | string | `"worktree"` | prototype-adr/SKILL.md |
| `[prototype]` | `runtime` | string | `""` | prototype-adr/SKILL.md |
| `[prototype]` | `teardown` | string | `"automatic"` | prototype-adr/SKILL.md |
| `[solve]` | `participation` | string | `"guided"` | solve-adr/SKILL.md |
| `[solve]` | `auto_delegate` | boolean | `false` | solve-adr/SKILL.md |
| `[solve]` | `default_scenario` | string | `"problem"` | solve-adr/SKILL.md (project-scoped) |
| `[prototype.persona]` | `embed_source` | boolean | `false` | ADR-0041 (project-scoped) |
| `[prototype.persona]` | `ground_truth` | string | `"comments"` | ADR-0041 (project-scoped) |
| `[prototype.persona]` | `procedure` | string | `""` | ADR-0041 (project-scoped) |

**Document structure:**
1. **Quick Start** — copy-paste example configs for common setups
2. **Reference Table** — all keys organized by skill section, with brief one-line descriptions and links to the relevant SKILL.md section for full context
3. **Resolution Order** — user-scoped → project-scoped → defaults
4. **Examples** — annotated user-scoped and project-scoped example files
5. **Skill-Specific Notes** — links to each skill's SKILL.md for full context

**README update:** Add a "Configuration" subsection to the README that points to PREFERENCES.md:
> See [docs/PREFERENCES.md](docs/PREFERENCES.md) for the full preferences reference and example configurations.

## Consequences

- **Positive:** Users have a single place to find all preference keys.
- **Positive:** Example configs reduce trial-and-error configuration.
- **Positive:** README stays focused on overview and quick-start.
- **Negative:** PREFERENCES.md must be updated when skills add or change keys. Convention: when adding or modifying a preference key in a SKILL.md, update PREFERENCES.md in the same commit.
- **Negative:** Partial duplication with individual SKILL.md configuration sections. PREFERENCES.md carries brief one-line descriptions with links to the relevant SKILL.md section for full context — not full documentation. When content diverges, SKILL.md always wins; PREFERENCES.md is updated to match.
- **Neutral:** Individual SKILL.md files remain the authoritative source for skill-specific behavior — PREFERENCES.md is a cross-cutting index with summaries and links, not a replacement.

## Quality Strategy

- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

- PREFERENCES.md should be validated against current SKILL.md files before release
- All example configs must be syntactically valid TOML

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** None.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Users lack guidance for subtle features like preferences management. Information exists but is scattered across 4 SKILL.md files. A centralized reference would improve discoverability and reduce friction for new and existing users.

**Tolerance:**
- Risk: Low — documentation is low-risk
- Change: Low — adds documentation, no code changes
- Improvisation: Low — standard reference documentation format

**Uncertainty:**
- Certain: preferences are documented in individual SKILL.md files
- Certain: no centralized reference exists
- Uncertain: optimal document structure (resolved: quickstart + reference table + examples)

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Centralized PREFERENCES.md
- Interactive make target
- README expansion
