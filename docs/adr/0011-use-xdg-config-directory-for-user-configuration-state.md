# 11. Use XDG config directory for user configuration state

Date: 2026-04-02

## Status

Accepted

Supersedes [ADR-0008](0008-meta-adr-directory-for-skill-behavioral-policies.md)

## Context

Skills and companion tools in this project need a persistent location to store
user-scoped configuration state — such as preferences, cached format selections,
and behavioral policies that apply across repositories. Currently there is no
convention for where this state lives, which forces each skill to either
re-prompt the user every session or store config inside the project tree where
it does not belong.

ADR-0008 introduced `<adr-dir>/.meta/` for project-scoped behavioral policies
stored as meta-ADRs. In practice, this approach proved too complex for agent
skills to utilize effectively — it requires the agent to read and interpret
every meta-ADR in the directory to assemble its configuration. A single
structured config file is clearer and more efficient for describing workflow
behavior.

The [XDG Base Directory Specification](https://specifications.freedesktop.org/basedir-spec/latest/)
defines `$XDG_CONFIG_HOME` (defaulting to `~/.config`) as the standard location
for user-specific configuration files on Unix-like systems.

## Decision

We will use `~/.config/adr-skills/` (i.e., `$XDG_CONFIG_HOME/adr-skills/`) as
the root directory for all user-scoped configuration state.

- **Resolve the base path** using `$XDG_CONFIG_HOME` if set, otherwise fall back
  to `$HOME/.config`. On Windows, use `$HOME/.config` as well — while not a
  native Windows convention, the concept of a home directory is present on
  Windows and `.config` provides a consistent cross-platform location.
- **Namespace under `adr-skills/`** to avoid collisions with other tools.
- **Organize by concern**, for example:
  - `~/.config/adr-skills/preferences.yml` — user defaults (format, template, editor)
  - `~/.config/adr-skills/agents/` — user-scoped custom agent overrides
- **Create the directory on first use** — skills should `mkdir -p` the config
  path when writing state, never assume it already exists.
- **Never store secrets** in this directory; it is for non-sensitive
  configuration only.
- **User-scope only for now** — this ADR establishes user-scoped configuration.
  Project-scoped configuration will be addressed in a future ADR when the need
  becomes concrete, though a single config file per scope is the intended
  direction.

### Alternatives considered

| Option | Rejected because |
|--------|-----------------|
| **`~/.adr-skills/`** (dotfile in `$HOME`) | Adds yet another dotfile to an already bloated home directory. The XDG convention exists specifically to consolidate config files under `~/.config/`. |
| **Environment variables only** | Does not scale for structured configuration. Future needs include embedding additional prompts or instructions in templates at specific workflow hooks — this requires structured data (YAML/etc.), not flat key-value env vars. |
| **Extend `<adr-dir>/.meta/` (ADR-0008)** | Too complex for agent skills in practice. Requires reading and interpreting multiple meta-ADR files to assemble configuration. A single structured config file is clearer, more efficient, and does not require the agent to understand ADR format to read its own settings. |

## Consequences

- **Easier:** Skills can persist user preferences across sessions and
  repositories without re-prompting.
- **Easier:** User-scoped agent overrides get a well-known home, separate from
  project-scoped agents in `.github/agents/` or `agents/`.
- **Easier:** Replaces the `.meta/` meta-ADR approach (ADR-0008) with a simpler
  single-file convention that does not require the agent to parse ADR format
  for its own configuration.
- **Harder:** Skills must handle the case where the config directory does not
  yet exist and create it gracefully.
- **Risk:** On systems where `$HOME` is not set or is non-writable (e.g., some
  CI environments), skills must degrade gracefully — skip config persistence
  rather than fail.
- **Risk:** If multiple skills write to the same config file concurrently,
  data corruption is possible. Mitigate by using separate files per concern.

### Revisit trigger

Revisit this ADR after subsequent ADRs have been added to address specific
aspects of configuration behavior (e.g., structured workflow hooks, project-
scoped config). This decision is foundational and intentionally minimal — it
establishes the location convention; the schema and semantics of config files
will be defined by follow-up decisions.

---

## Comments

<!-- Review cycle 1 — Verdict: Revise. Findings addressed inline. Predates structured Q&A addendum (ADR-0016). -->
