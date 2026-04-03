# Decision Capturing Tools

> This skill supports two ADR formats controlled by `ADR_AGENT_SKILL_FORMAT`:
>
> | Runtime | Template | Scripts | Default Dir |
> |---------|----------|---------|-------------|
> | `nygard` (default) | Nygard ADR | `scripts/adr-tools-3.0.0/` | `doc/adr/` |
> | `madr` | MADR 4.0 | `scripts/madr-tools/` | `docs/decisions/` |
>
> A top-level `Makefile` abstracts both formats behind unified targets.

## Makefile Interface (Recommended)

The Makefile auto-selects the correct scripts based on `ADR_AGENT_SKILL_FORMAT`.

```bash
# Set format (default: nygard)
export ADR_AGENT_SKILL_FORMAT=nygard   # or: madr

# Core targets
make init                               # bootstrap ADR directory
make init DIR=decisions                  # custom directory
make new TITLE="Use PostgreSQL"         # create a new ADR
make new TITLE="Use MySQL" SUPERSEDE=2  # supersede an existing ADR
make new TITLE="Use Redis" TEMPLATE=full  # (madr only) use full template
make list                               # list all ADRs
make status                             # show all ADR statuses (madr only)
make status NUM=2 STATUS=Accepted       # update status (madr only)
make generate                           # generate table of contents
make generate PREFIX="docs/adr/"        # with path prefix
make link SOURCE=12 LINK="Amends" TARGET=10 REVERSE="Amended by"  # (nygard only)
make test                               # run tests for both formats
```

## Runtime: Nygard (adr-tools)

Direct script usage when the Makefile is not available or the agent needs to adapt.

### Quick Start

```bash
# Add to PATH (from skill root)
export PATH="$PWD/scripts/adr-tools-3.0.0/src:$PATH"

# Initialize ADR directory in a project
adr init              # creates doc/adr/ with ADR 0001
adr init decisions    # or use a custom directory name
```

## Commands

### `adr new` — Create a new ADR

```bash
adr new Use PostgreSQL for persistence
# → creates doc/adr/0002-use-postgresql-for-persistence.md and opens editor

# Supercede an existing ADR
adr new -s 2 Use MySQL instead of PostgreSQL

# Link to an existing ADR
adr new -l "3:Complements:Complemented by" Add read replicas
```

Options:
- `-s NUMBER` — marks the referenced ADR as superceded by this one
- `-l TARGET:LINK:REVERSE-LINK` — creates bidirectional links between ADRs

### `adr list` — List all ADRs

```bash
adr list
# doc/adr/0001-record-architecture-decisions.md
# doc/adr/0002-use-postgresql-for-persistence.md
```

### `adr link` — Link two existing ADRs

```bash
adr link 12 Amends 10 "Amended by"
# Creates forward link in ADR 12 and reverse link in ADR 10
```

### `adr generate` — Generate reports

```bash
adr generate toc                        # table of contents
adr generate graph                      # Graphviz dependency graph
adr generate toc -p "docs/adr/"         # with path prefix
adr generate toc -e "included-header"   # with header/footer includes
```

### `adr config` — Show configuration

```bash
adr config
# Outputs adr_bin_dir and adr_template_dir paths
```

### `adr upgrade-repository` — Migrate date format

```bash
adr upgrade-repository
# Converts dates from DD/MM/YYYY to ISO 8601 (YYYY-MM-DD)
```

## Template

adr-tools uses the Nygard ADR format by default:

```markdown
# NUMBER. TITLE

Date: DATE

## Status

STATUS

## Context

The issue motivating this decision, and any context that influences or
constrains the decision.

## Decision

The change that we're proposing or have agreed to implement.

## Consequences

What becomes easier or more difficult to do and any risks introduced by the
change that will need to be mitigated.
```

### Custom Templates

Place a `template.md` in `templates/` within your ADR directory to override the default. The template can use the following placeholders:
- `NUMBER` — auto-incremented ADR number
- `TITLE` — from the command line arguments
- `DATE` — current date (or `ADR_DATE` env var)
- `STATUS` — initial status (typically "Proposed" or "Accepted")

## Workflow

```
1. adr init                          ← bootstrap
2. adr new <title>                   ← capture a decision
3. edit the generated file           ← fill in context, decision, consequences
4. adr link / adr new -s / -l       ← connect related decisions
5. adr generate toc                  ← update documentation
6. commit to version control         ← share with team
```

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `VISUAL` | Editor for new ADRs (preferred over EDITOR) |
| `EDITOR` | Fallback editor if VISUAL is not set |
| `ADR_DATE` | Override the date stamp (useful in CI/tests) |

## Shell Completion

Source the autocomplete script for bash completion:

```bash
source scripts/adr-tools-3.0.0/autocomplete/adr
```

## Runtime: MADR (madr-tools)

Direct script usage for the MADR format.

### Quick Start

```bash
# Add to PATH (from skill root)
export PATH="$PWD/scripts/madr-tools/src:$PATH"

# Initialize MADR directory
madr init                    # creates docs/decisions/ with first MADR
madr init decisions          # or use a custom directory name
```

### Commands

#### `madr new` — Create a new MADR

```bash
madr new Use PostgreSQL for persistence
# → creates docs/decisions/0002-use-postgresql-for-persistence.md

# Use full template (default: minimal)
madr new -t full Use PostgreSQL for persistence

# Supersede an existing ADR
madr new -s 2 Use MySQL instead of PostgreSQL
```

Options:
- `-s NUMBER` — marks the referenced ADR as superseded
- `-t TYPE` — template type: `minimal` (default) or `full`

#### `madr list` — List all MADRs

```bash
madr list
# docs/decisions/0001-record-architecture-decisions-with-madr.md
# docs/decisions/0002-use-postgresql-for-persistence.md
```

#### `madr status` — Show or update status

```bash
madr status                  # show all statuses
madr status 2                # show status of ADR 2
madr status 2 Accepted       # update status of ADR 2
```

Status values: Proposed, Accepted, Deprecated, Superseded

#### `madr generate` — Generate table of contents

```bash
madr generate toc
madr generate toc -p "docs/decisions/"    # with path prefix
```

#### `madr help` — Show help

```bash
madr help          # list commands
madr help new      # help for a specific command
```

### Template

madr-tools defaults to the MADR minimal template. Use `-t full` for the full template with decision drivers, pros/cons analysis, and confirmation sections.

Templates are sourced from `assets/templates/`:
- `madr-minimal-template.md` — Context, Options, Outcome, Consequences
- `madr-full-template.md` — adds Drivers, Pros/Cons, Confirmation, More Info

### Custom Templates

Place a custom `adr-template.md` in your decisions directory during `madr init`. The tool copies both templates there; modify them to suit your project.

### Environment Variables

| Variable | Purpose |
|----------|---------|
| `VISUAL` | Editor for new MADRs (preferred over EDITOR) |
| `EDITOR` | Fallback editor if VISUAL is not set |
| `MADR_DATE` | Override the date stamp (useful in CI/tests) |

## Other Tools (For Reference)

While this skill standardizes on adr-tools, agents may encounter projects using alternative tooling. Key alternatives are documented in [assets/](../assets/index.md) under "Tooling":

- **ADG** (Go) — multi-template CLI supporting Nygard, MADR, and QOC
- **dotnet-adr** (.NET) — cross-platform .NET Global Tool
- **Log4brains** (Node.js) — docs-as-code with rendering
- **VS Code ADR Manager** — GUI extension for MADR
- **Backstage ADR Plugin** — developer portal integration
- **Talo** (dotnet) — multi-format design doc CLI
- **ReflectRally** — collaborative SaaS platform

## Visualization

### When to Use Diagrams

Use diagrams when complex relationships between processes or entities are better understood visually. Humans tend to have limited working memory, and visualizations can compress information into a digestible format. However, by that same token, overuse of visualizations can overload context — so use them sparingly and only when they genuinely clarify what prose or tables cannot.

**Good uses for diagrams:**
- ADR status transitions (state diagram)
- Decision process workflows (flowchart)
- Relationships between ADRs — supercedes, amends, relates-to (graph)
- System interaction during a decision's lifecycle (sequence diagram)

**Prefer tables over diagrams for:**
- Option comparisons and tradeoff analysis
- Criteria scoring across alternatives
- Any side-by-side evaluation of discrete choices

### Mermaid Charts

All diagrams should use **Mermaid** syntax. Mermaid renders natively in GitHub Markdown, most documentation platforms, and many editors.

See [assets/mermaid-chart-examples.md](../assets/mermaid-chart-examples.md) for ADR-specific diagram patterns and general syntax reference.

### Markdown Tables for Comparisons

When presenting decision justification, use markdown tables to condense the analysis. Tables are lower-overhead than diagrams, scannable, and keep the ADR focused on substance:

```markdown
| Criteria   | Option A  | Option B  | Option C  |
|------------|:---------:|:---------:|:---------:|
| Latency    | ✅ Low    | ⚠️ Med    | ❌ High   |
| Cost       | ❌ High   | ✅ Low    | ✅ Low    |
| Maturity   | ✅ Proven | ⚠️ New    | ✅ Proven |
```
