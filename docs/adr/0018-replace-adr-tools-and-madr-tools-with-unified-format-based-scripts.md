# 18. Replace adr-tools and madr-tools with unified format-based scripts

Date: 2026-04-03

## Status

Accepted

## Context

The author-adr skill currently bundles two third-party/custom script suites:

- **adr-tools 3.0.0** — 22 scripts (14 internal helpers + 8 commands) for
  Nygard format. Bundled third-party code with its own test suite (22 tests),
  license files, CI config, and documentation.
- **madr-tools** — 8 custom scripts for MADR format. Built to mirror
  adr-tools' command structure with a custom test suite (9 tests).

An audit of actual usage across the skill's SKILL.md, references, and Makefile
shows that the overwhelming majority of interactions use a single command:
`new`. The commands `init`, `list`, and `status` see occasional use. The
remaining commands (`link`, `generate`, `config`, `upgrade-repository`, `help`)
and all 14 internal `_adr_*` helpers are referenced only in documentation, not
in practice.

This is ~30 scripts supporting what amounts to 4 operations. The maintenance
burden is disproportionate — any format change (like the inline metadata in
ADR-0017's nygard-agent-template) requires understanding and modifying code
across two independent script suites with different conventions.

Additionally, ADR-0017 adopts a new default template (`nygard-agent-template`)
with inline `Status:` metadata instead of a `## Status` heading. The existing
scripts' status parsing (`grep -m1 "^## Status"`) is incompatible with this
format.

### Decision Drivers

- **Simplicity** — maintain only the scripts that are actually used.
- **Format extensibility** — adding a new ADR format should mean adding one
  file, not building an entire tool suite.
- **Self-contained formats** — each format should own its own template and
  parsing logic, not depend on external template files or path resolution
  chains.

## Options

### Option 1: Patch existing scripts for the new format

Update adr-tools and madr-tools to handle inline metadata. This preserves the
existing architecture but means maintaining two script suites for a problem
that only needs one. The internal helpers and unused commands remain as dead
weight.

### Option 2: Write new unified scripts from scratch

Replace both suites with a small set of purpose-built scripts. A thin
orchestrator (`new.sh`) handles common concerns (ADR directory, numbering,
slugification), then delegates to a format-specific script
(`<format>-format.sh`) that owns the template, metadata parsing, and document
generation via `case` subcommands.

### Option 3: Eliminate scripts entirely, let the agent do everything inline

Remove all scripts and have the agent generate ADR files directly using the
edit/create tools. This eliminates all script maintenance but loses the ability
to use ADR tooling outside of an agent session (e.g., from the command line).

## Decision

Replace adr-tools and madr-tools with a unified, minimal script architecture.
Write new scripts from scratch — do not copy-paste from the existing suites.

### Architecture

**Two-level dispatch:**

```
new.sh <format> <title> [options]
  └─► <format>-format.sh new <number> <title> <dir>
```

**Orchestrator (`new.sh`):**
- Resolves the ADR directory (reads `.adr-dir` or defaults to `docs/adr`)
- Computes the next sequential number
- Slugifies the title into a filename
- Delegates to `<format>-format.sh new <number> <title> <dir>`

**Format script (`nygard-agent-format.sh`):**
- Self-contained: the template is baked into the script, not read from an
  external file
- Subcommands via `case "$1" in`:

  ```bash
  case "$1" in
    new)    # generate ADR document with baked-in template
      ;;
    init)   # bootstrap ADR directory + first ADR
      ;;
    list)   # list ADRs with number, title, status
      ;;
    status) # show or update status (parses inline Status: field)
      ;;
  esac
  ```

**Adding a new format** = adding one file (`<name>-format.sh`). The
orchestrator and Makefile route to it based on the configured format name.

### Commands retained

| Command | Purpose |
|---------|---------|
| `init`  | Bootstrap ADR directory and create first ADR |
| `new`   | Create a new ADR from the format's baked-in template |
| `list`  | List all ADRs with number, title, and status |
| `status`| Show or update an ADR's status |

All other commands from adr-tools and madr-tools are dropped: `link`,
`generate`, `config`, `help`, `upgrade-repository`, and all internal helpers.

### Directory structure

```
scripts/
├── new.sh                      # orchestrator
└── nygard-agent-format.sh      # format: nygard-agent (default)
```

Future formats (e.g., `madr-format.sh`) can be added as single files.

### What is removed

- `scripts/adr-tools-3.0.0/` — entire directory (22 scripts, 22 tests, license,
  CI config, documentation)
- `scripts/madr-tools/` — entire directory (8 scripts, 9 tests)

## Consequences

- **Dramatically reduces script surface area** — from ~30 scripts across two
  suites to 2 files (orchestrator + one format script). Maintenance cost drops
  proportionally.

- **Format scripts are self-contained** — no template file resolution, no
  `$ADR_TEMPLATE` environment variable, no `$dstdir/templates/template.md`
  fallback chain. The template is in the script.

- **Adding a format is one file** — a new format requires writing one
  `<name>-format.sh` with the four subcommands. No changes to the orchestrator
  or Makefile beyond routing.

- **Loses unused but documented commands** — `link`, `generate`, `config`,
  `upgrade-repository` are dropped. If a user needs these, they must implement
  them manually or re-add them to the format script. Based on usage audit, this
  is low risk.

- **Existing test suites are replaced** — the 31 tests across adr-tools and
  madr-tools are no longer applicable. New tests must be written for the
  unified scripts.

- **Third-party adr-tools is fully removed** — the project no longer bundles
  Nat Pryce's adr-tools. This simplifies licensing (removes GPL dependency)
  but means the skill can no longer fall back to upstream adr-tools behavior.

- **Makefile targets are simplified** — the Makefile no longer needs
  format-conditional logic (`ifeq ($(ADR_AGENT_SKILL_FORMAT),madr)`). It calls
  `new.sh` with the format name, and the format script handles the rest.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible

### Additional Quality Concerns

The new scripts must pass tests for init, new, list, and status operations.
Backward compatibility with existing ADRs in standard Nygard format must be
verified — `list` and `status` must handle both inline `Status:` and
`## Status` heading formats during the transition period.

---

## Comments

<!-- No review cycle on record. -->
