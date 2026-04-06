# 47. User-Mode ADR Storage in .adr/usr/

Date: 2026-04-06
Status: Planned
Last Updated: 2026-04-06
Links: ADR-0020 (.adr/ convention), ADR-0017 (nygard-agent template), ADR-0048 (centralized preferences reference — `[author].scope` and `[author].username` keys)

## Context

ADRs are project-scoped — stored in `docs/adr/` and committed to git. This makes every ADR immediately visible to the team.

Users sometimes want to draft ADRs privately before proposing them:
- Personal exploration of an idea before team discussion
- Early-stage drafting that isn't ready for review
- Experimentation with decision options privately

The `.adr/` directory exists as the project-scoped data directory (per ADR-0020). `.adr/var/` is already gitignored for transient data. Extending `.adr/` with a `usr/` subdirectory for user-scoped content follows the established convention.

The existing sequential numbering (`NNNN`) would collide between users working in the same gitignored space. A username prefix (`<username>-<NNNN>-<slug>.md`) avoids collisions and maintains traceability when an ADR is promoted from user to project scope.

## Options

### Option A: .adr/usr/ with username prefix

Store user-mode ADRs in `.adr/usr/docs/adr/` and plans in `.adr/usr/docs/plans/`. Files named `<username>-<NNNN>-<slug>.md`. Activate via preference key `[author].scope = "user"` or natural language gesture.

**Directory layout:**
```
.adr/
├── .gitignore          # Already ignores var/, add usr/
├── usr/
│   └── docs/
│       ├── adr/        # User-scoped ADR records
│       │   ├── juliusl-0001-explore-caching-strategy.md
│       │   └── juliusl-0002-evaluate-new-framework.md
│       └── plans/      # User-scoped implementation plans
│           └── juliusl-0001.0.plan.md
```

**Activation:**
- Persistent: `[author].scope = "user"` in preferences.toml
- Per-invocation: natural language ("create a personal ADR", "draft in user mode")
- Override: `[author].scope = "project"` to explicitly use project mode

**Username resolution:** `$(whoami)` by default. Override via `[author].username` in preferences.toml.

**Strengths:**
- Mirrors project structure — familiar layout
- Username prefix prevents collision between users
- Gitignored → private by default
- Promotion path: move file from `.adr/usr/docs/adr/` to `docs/adr/`, strip prefix, renumber

**Weaknesses:**
- Requires `.adr/` directory to exist (bootstrap with `make init-data`)
- Promoting from user to project scope requires manual rename and renumber
- Sequential numbering scoped to user directory — gaps appear when ADRs are promoted

### Option B: User-home directory

Store in `~/.local/share/adr-skills/<project-name>/docs/adr/`. Completely separate from the project.

**Strengths:**
- Fully project-independent — no `.adr/` dependency
- Works even without a project repository

**Weaknesses:**
- Disconnected from project context
- Harder to promote to project scope (copy across filesystems)
- Not discoverable from the project directory
- Project identity is ambiguous (name collisions across orgs)

### Option C: Natural language gesture only, no persistent mode

User says "create a personal ADR" to store in `.adr/usr/` for that invocation only. No preference key.

**Strengths:**
- No configuration to manage
- Flexible per-invocation

**Weaknesses:**
- No persistent mode — user must remember to say "personal" every time
- Easy to forget and accidentally create a project-scoped ADR

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** No validation needed for directory convention — directory structures and gitignore patterns are well-understood. Script integration (`new.sh`, `nygard-agent-format.sh`) verified during implementation.

## Decision

We will store user-mode ADRs in `.adr/usr/docs/adr/` with username-prefixed filenames, activated by preference or natural language gesture.

In the context of users wanting to draft ADRs privately before proposing them to the team, facing the need for isolated, gitignored storage with multi-user collision avoidance, we chose `.adr/usr/` with username prefix over user-home storage or gesture-only activation, to achieve a familiar directory layout with persistent mode support and a clear promotion path, accepting the requirement for `.adr/` bootstrapping and manual rename on promotion.

**Convention:**
- ADR files: `<username>-<NNNN>-<slug>.md` (e.g., `juliusl-0001-explore-caching.md`)
- Plan files: `<username>-<NNNN>.0.plan.md` (e.g., `juliusl-0001.0.plan.md`)
- NNNN resolution: scan `.adr/usr/docs/adr/` for files matching `<username>-*.md`, extract the highest NNNN, increment. If no files exist, start at 0001.
- Directory: `.adr/usr/docs/adr/` and `.adr/usr/docs/plans/`
- Gitignore: add `usr/` to `.adr/.gitignore`

**Activation gestures:**
- Preference: `[author].scope = "user"` (persistent default)
- Natural language: "create a personal ADR", "draft in user mode", "personal draft"
- Override: `[author].scope = "project"` (explicit project mode)

**Username:**
- Default: `$(whoami)`
- Override: `[author].username` in preferences.toml

**Promotion:** To promote a user ADR to project scope:
1. Move file from `.adr/usr/docs/adr/` to `docs/adr/`
2. Strip username prefix and renumber to next available project NNNN
3. Update cross-references: links from other user-mode ADRs (in `.adr/usr/docs/adr/`), links from implementation plans (in `.adr/usr/docs/plans/`), and any inline references in the promoted ADR itself that point to user-mode paths

## Consequences

- **Positive:** Users can draft ADRs privately without team visibility.
- **Positive:** Username prefix prevents collision between multiple users.
- **Positive:** Gitignored by default — no accidental commits of drafts.
- **Positive:** Familiar directory layout mirrors project structure.
- **Negative:** Requires `.adr/` directory bootstrapping (one-time setup).
- **Negative:** Promoting from user to project scope is a manual process.
- **Neutral:** Sequential numbering is scoped to the user's directory.
- **Negative:** Scripts (`new.sh`, `nygard-agent-format.sh`) need to support an alternative ADR directory and username-prefixed filenames for user mode — the largest implementation cost.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

- author-adr scripts (`new.sh`, `nygard-agent-format.sh`) need to support the alternative ADR directory for user mode
- `.adr/.gitignore` needs `usr/` entry
- author-adr SKILL.md needs documentation of scope preference and activation gestures

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** The user mentioned uncertainty about the activation gesture. This ADR proposes both a preference key and natural language detection. The preference provides persistence; natural language provides flexibility.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Users want a "user mode" for personal/draft ADRs stored in a gitignored folder (`.adr/usr/docs/adr`, `.adr/usr/docs/plans`) with username-prefixed filenames (`juliusl-<ID>-<SLUG>.md`). The activation gesture is undecided.

**Tolerance:**
- Risk: Low — directory conventions are well-understood
- Change: Medium — new directory structure and naming convention
- Improvisation: Medium — activation gesture needs creative design

**Uncertainty:**
- Certain: `.adr/` directory convention exists (ADR-0020)
- Certain: gitignore provides privacy
- Uncertain: best activation gesture (resolved: preference + natural language)
- Uncertain: how promotion from user to project scope should work (resolved: manual rename)

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- `.adr/usr/` with username prefix
- User-home directory
- Natural language gesture only
