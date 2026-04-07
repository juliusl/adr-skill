# Preferences Reference

All adr-skills preferences in one place. Each skill reads its own namespace from `preferences.toml` and ignores other sections.

## Quick Start

**Autonomous workflow with auto-commit** (user-scoped):
```toml
# ~/.config/adr-skills/preferences.toml

[solve]
participation = "autonomous"
auto_delegate = true

[author.dispatch]
editor = "juliusl-editor-v4"

[implement]
participation = "autonomous"
auto_commit = true
```

**Project-scoped overrides** (committed to repo):
```toml
# .adr/preferences.toml

[solve]
participation = "guided"

[prototype]
isolation = "container"
```

## Resolution Order

Preferences resolve in this order — later sources override earlier ones:

1. **Built-in defaults** — hardcoded in each skill's SKILL.md
2. **User-scoped** — `~/.config/adr-skills/preferences.toml` (or `$XDG_CONFIG_HOME/adr-skills/preferences.toml`)
3. **Project-scoped** — `.adr/preferences.toml` in the repository root

If neither file exists, skills use built-in defaults and never fail.

**Create on first write:** When saving a preference, the skill creates the directory with `mkdir -p` before writing.

## Reference Table

### author-adr

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `[author].template` | string | `"nygard-agent"` | ADR format: `nygard-agent`, `wi-nygard-agent` — see [author-adr SKILL.md](../src/skills/author-adr/SKILL.md#a-0-format-detection) |
| `[author].scope` | string | `"project"` | Storage scope: `"project"` (docs/adr/) or `"user"` (.adr/usr/docs/adr/) — see [author-adr SKILL.md](../src/skills/author-adr/SKILL.md#user-mode) |
| `[author].username` | string | `$(whoami)` | Username prefix for user-mode filenames — see [author-adr SKILL.md](../src/skills/author-adr/SKILL.md#user-mode) |
| `[author.dispatch].review` | string | `"general-purpose"` | Agent for structured ADR review — see [author-adr SKILL.md](../src/skills/author-adr/SKILL.md#agent-dispatch-authordispatch) |
| `[author.dispatch].editor` | string | `"interactive"` | Agent for editorial revision; `"interactive"` prompts the user — see [author-adr SKILL.md](../src/skills/author-adr/SKILL.md#agent-dispatch-authordispatch) |

### implement-adr

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `[implement].participation` | string | `"guided"` | Participation mode: `full-control`, `guided`, `autonomous`, `weighted` — see [implement-adr SKILL.md](../src/skills/implement-adr/SKILL.md#i-6-participation-check) |
| `[implement].auto_commit` | bool | `false` | Git commit after each completed task — see [implement-adr SKILL.md](../src/skills/implement-adr/SKILL.md#auto-commit-on-task-completion) |

### prototype-adr

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `[prototype].isolation` | string | `"worktree"` | Isolation backend: `worktree`, `container`, `acp-sandbox` — see [prototype-adr SKILL.md](../src/skills/prototype-adr/SKILL.md#e-1-select-isolation) |
| `[prototype].runtime` | string | `""` | Set to `"acp"` for ACP sandbox support — see [prototype-adr SKILL.md](../src/skills/prototype-adr/SKILL.md#configuration) |
| `[prototype].teardown` | string | `"automatic"` | Teardown behavior: `automatic`, `manual` — see [prototype-adr SKILL.md](../src/skills/prototype-adr/SKILL.md#configuration) |

### solve-adr

| Key | Type | Default | Description |
|-----|------|---------|-------------|
| `[solve].participation` | string | `"guided"` | Participation mode: `guided`, `autonomous` — see [solve-adr SKILL.md](../src/skills/solve-adr/SKILL.md#s-0-startup) |
| `[solve].auto_delegate` | bool | `false` | Auto-invoke /implement-adr after ADR acceptance — see [solve-adr SKILL.md](../src/skills/solve-adr/SKILL.md#s-0-startup) |
| `[solve].default_scenario` | string | `"problem"` | Default scenario when not specified — see [solve-adr SKILL.md](../src/skills/solve-adr/SKILL.md#configuration) |

### Project-Scoped Dynamic Tables

Projects can define experiment-specific procedures under `[prototype.*]` tables in `.adr/preferences.toml`:

| Key | Type | Description |
|-----|------|-------------|
| `[prototype.<name>].procedure` | string | Multi-line experiment instructions followed as-written — see [prototype-adr SKILL.md](../src/skills/prototype-adr/SKILL.md#configuration) |
| `[prototype.<name>].embed_source` | bool | Whether to embed ADR content in experiment prompts — see [prototype-adr SKILL.md](../src/skills/prototype-adr/SKILL.md#configuration) |

## Annotated Example

```toml
# ~/.config/adr-skills/preferences.toml
# User-scoped — applies to all projects unless overridden

[author]
template = "nygard-agent"     # ADR format
# scope = "project"           # uncomment for user-mode: scope = "user"
# username = "myname"         # override $(whoami) for user-mode filenames

[author.dispatch]
review = "general-purpose"    # review agent
editor = "interactive"        # revision agent ("interactive" = prompt user)

[implement]
participation = "guided"      # full-control | guided | autonomous | weighted
auto_commit = false           # git commit per completed task

[prototype]
isolation = "worktree"        # worktree | container | acp-sandbox
runtime = ""                  # set to "acp" for ACP sandbox
teardown = "automatic"        # automatic | manual

[solve]
participation = "guided"      # guided | autonomous
auto_delegate = false         # auto-invoke /implement-adr after acceptance
default_scenario = "problem"  # default scenario selection
```

## Synchronization Convention

When adding or modifying a preference key in a SKILL.md, update this file in the same commit. SKILL.md files are the authoritative source — if PREFERENCES.md diverges, update it to match.
