# 12. Use TOML as the configuration file format

Date: 2026-04-02

## Status

Accepted

## Context

ADR-0011 established `~/.config/adr-skills/` as the location for user-scoped
configuration but did not specify the file format. As the configuration surface
grows — user preferences, workflow hook instructions, and eventually project-
scoped overrides within the same file — the format choice has significant
implications for readability, maintainability, and mergeability.

The configuration needs to support:

- **Structured data** — nested sections for targeting specific workflow hooks
  (e.g., embedding additional prompts or instructions in templates at certain
  points in the workflow).
- **Human readability** — users will hand-edit this file, not just machines.
- **Comments** — inline documentation explaining what each setting does,
  especially for project-scoped sections where comments can provide context
  that helps agents remediate stale references (e.g., when a project moves).
- **No null semantics** — absent keys should mean "not configured," not a
  distinct null state that must be handled separately.

## Decision

We will use [TOML 1.0](https://toml.io/en/v1.0.0) as the configuration file
format for all files in `~/.config/adr-skills/`. Version 1.0 is chosen over
the 1.1 draft as it is the current stable specification and provides sufficient
features for our needs.

Configuration files will use the `.toml` extension (e.g.,
`~/.config/adr-skills/preferences.toml`).

### Alternatives considered

| Option | Rejected because |
|--------|-----------------|
| **YAML** | Overly complex for this use case. Significant whitespace sensitivity makes hand-editing error-prone. Supports features (anchors, aliases, multiple documents, custom tags) that add complexity without benefit here. The implicit typing rules (e.g., `no` becoming a boolean) are a common source of bugs. |
| **JSON** | Not human-friendly for hand-editing — no comment support, strict syntax (trailing commas, mandatory quoting). Error-prone for users who need to manually adjust config. |
| **INI** | Simple and shell-parseable, but lacks a standardized spec (many incompatible dialects), has no typed values (everything is a string), no nested sections, and no array support. A full-featured modern markup language is a better long-term choice for structured configuration that will grow over time. |
| **Flat key=value file** (e.g., `.env` style) | Does not scale to structured configuration. Future needs include targeting workflow hooks with nested settings, which flat key-value pairs cannot express without inventing a naming convention (e.g., `hook.pre_create.prompt=...`). |

### Why TOML

- **No null concept** — a missing key is simply absent. Skills check for key
  presence rather than handling null vs. absent vs. empty distinctions.
- **Human-friendly** — clear, readable syntax that non-developers can
  understand. Whitespace-agnostic (no indentation sensitivity).
- **Native comment support** — allows inline documentation explaining settings,
  which is valuable both for users and for agents that read the config file for
  context.
- **Structured enough to scale** — tables and arrays of tables support the
  nested configuration needed for workflow hooks and project-scoped overrides.
- **Simpler project-scoping path** — when project-scoped config is introduced
  (per ADR-0011's deferred scope), merging user and project settings is simpler
  in a single structured file format. TOML's table syntax could support project-
  scoped overrides as namespaced sections, though the specific schema is deferred
  to a future ADR.

## Consequences

- **Easier:** Hand-editing config is straightforward — no whitespace traps, no
  quoting rules to remember, and comments explain what each setting does.
- **Easier:** Skills can check for key presence without null-handling logic.
  "Not set" has exactly one representation: the key is absent.
- **Easier:** When project-scoped overrides are introduced, TOML's table
  structure could support them within the same file, with comments providing
  context for agents. The specific schema is deferred to a future ADR.
- **Harder:** TOML is less ubiquitous than JSON/YAML in some ecosystems. Shell
  scripts cannot parse TOML natively — a lightweight parser or agent-side
  handling is required.
- **Risk:** TOML's table syntax can become verbose for deeply nested
  structures. If config complexity grows significantly, this may need
  revisiting. Mitigated by keeping the config surface intentionally shallow.

### Revisit trigger

Revisit if the config schema exceeds three levels of nesting, or if shell-based
skills need to read config without agent assistance.

---

## Comments

<!-- Review cycle 1 — Verdict: Revise. Findings addressed inline. Predates structured Q&A addendum (ADR-0016). -->
