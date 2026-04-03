# 14. Define author and implement tables in TOML configuration

Date: 2026-04-03

## Status

Accepted

## Context

ADR-0012 established TOML as the configuration file format for
`~/.config/adr-skills/` and ADR-0011 established the XDG directory convention.
Together they provide the location and format for user-scoped configuration, but
neither defines a schema — the internal structure of the TOML file is unspecified.

Meanwhile, behavioral settings for each skill are scattered across multiple
decision records with no single place to look them up:

- **`author-adr`** has an implicit template preference (Nygard vs. MADR) that is
  detected per-session by reading the first ADR in `docs/adr/`. The format is
  re-detected every time the skill activates, and there is no way for the user
  to set a persistent default independent of a specific project.
- **`implement-adr`** has two behavioral policies — participation mode
  (ADR-0006, refined by ADR-0007's Weighted mode) and auto-commit on task
  completion (ADR-0010) — that were originally stored as meta-ADRs in
  `<adr-dir>/.meta/` (ADR-0008). ADR-0011 superseded the `.meta/` approach
  because it was too complex for agents to utilize, but the replacement
  config file has no defined structure for these settings.

The result is a loose association between settings and the skills that consume
them. Dogfooding has shown that agents struggle to locate and apply these
settings reliably — they must read multiple ADRs to reconstruct what a single
config table lookup could provide.

## Decision

Define two top-level TOML tables — `[author]` and `[implement]` — in the
user-scoped configuration file (`~/.config/adr-skills/preferences.toml`) to
serve as the canonical location for each skill's behavioral settings.

### Schema

```toml
# ~/.config/adr-skills/preferences.toml

[author]
# Default ADR template format: "nygard" or "madr"
# The skill uses this setting directly. Format detection from project
# files is being revisited in a follow-up ADR.
template = "nygard"

[implement]
# Participation mode for plan execution: "full-control", "guided",
# "autonomous", or "weighted"
# See ADR-0006 and ADR-0007 for mode definitions.
participation = "guided"

# Whether to create a git commit after each task's acceptance criteria
# are all satisfied. See ADR-0010 for behavior details.
auto_commit = false
```

### Key design choices

1. **One table per skill** — `[author]` maps to `author-adr`, `[implement]`
   maps to `implement-adr`. Each skill reads only its own table, avoiding
   cross-skill coupling. New skills would add their own top-level tables.

2. **Flat within each table** — all settings are direct key-value pairs under
   the skill table. No nesting beyond the first level. This keeps the schema
   within the "intentionally shallow" constraint from ADR-0012 (revisit
   trigger: three levels of nesting).

3. **Config is authoritative** — `[author].template` is the single source of
   truth for template format. The skill does not re-detect format from project
   files. The current format detection logic (reading the first ADR in
   `docs/adr/`) will be revisited in a follow-up ADR that specializes the
   template format to be more agent-friendly — the existing templates were
   designed for large organizations, and our use case warrants a more tailored
   approach.

4. **Defaults match current behavior** — `template = "nygard"` (the existing
   bootstrap default), `participation = "guided"` (ADR-0006's default),
   `auto_commit = false` (ADR-0010's opt-in default). An absent key means
   "use the default" per ADR-0012's no-null semantics.

### Skill behavior changes

**`author-adr`:** During Format Detection, read `[author].template` from the
config file. If set, use it directly. If not set, default to Nygard. The
current behavior of detecting format from existing project ADRs will be
revisited in a follow-up ADR.

**`implement-adr`:** During the participation check (ADR-0006, Step 5), read
`[implement].participation` and `[implement].auto_commit` from the config file.
If set, apply silently and skip the corresponding prompts. If not set, prompt
as before and offer to save the user's choice to the config file.

### Alternatives considered

| Option | Rejected because |
|--------|-----------------|
| **Separate TOML files per skill** (e.g., `author.toml`, `implement.toml`) | Over-fragments a small config surface. Two files with 1-2 keys each is harder to discover and manage than one file with two tables. TOML tables exist precisely for this kind of namespacing. |
| **Flat keys without tables** (e.g., `author_template = "nygard"`) | Loses the visual grouping that makes the file scannable. As settings grow, flat keys become harder to read and more prone to naming collisions. |
| **Flat keys without tables** is the only genuine structural alternative. The
meta-ADR approach (ADR-0008) was already rejected by ADR-0011 and is not
reconsidered here.

## Consequences

- **Easier setting lookup** — each skill reads one table from one file. No
  scanning multiple meta-ADRs or re-detecting format from project files.
- **Defined setting locations** — template preference, participation mode, and
  auto-commit now have canonical keys in the config file, surviving across
  sessions and repositories without re-prompting.
- **Extensible** — adding a new setting to either skill means adding a key to
  the relevant table. Adding a new skill means adding a new top-level table.
  The schema grows horizontally (more tables) rather than vertically (deeper
  nesting).
- **Precedence rule** — the config file is authoritative for all settings.
  Built-in defaults apply only when a key is absent. Project-level format
  detection (reading `docs/adr/`) is being revisited in a follow-up ADR; until
  then, `[author].template` is the single source of truth for format. For
  `implement` settings, only user-level and built-in defaults apply.
- **Shell parsing limitation** — per ADR-0012, TOML cannot be parsed natively
  by shell scripts. Skills that need these settings must read them via agent
  logic or a lightweight parser, not raw `grep`/`sed`.
- **No schema validation** — there is no schema enforcement. A typo like
  `paticipation = "guided"` would be silently ignored (treated as absent →
  default). This is acceptable while the config is consumed only by agents,
  which associate settings by instruction rather than strict key lookup. If
  the config surface expands to be consumed by tooling (scripts, CI), schema
  validation should be addressed in a dedicated ADR.
