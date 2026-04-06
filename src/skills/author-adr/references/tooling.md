# Decision Capturing Tools

> This skill uses a unified script architecture (per ADR-0018) with a single
> default format: `nygard-agent`.
>
> | Format | Template | Scripts | Default Dir |
> |--------|----------|---------|-------------|
> | `nygard-agent` (default) | Nygard Agent | `scripts/new.sh` + `scripts/nygard-agent-format.sh` | `docs/adr/` |
>
> Adding a new format = adding one `<name>-format.sh` file.

## Makefile Interface (Recommended)

The Makefile routes to the correct scripts based on `ADR_AGENT_SKILL_FORMAT`.

```bash
# Set format (default: nygard-agent)
export ADR_AGENT_SKILL_FORMAT=nygard-agent

# Core targets
make init                               # bootstrap ADR directory
make init DIR=decisions                  # custom directory
make init-data                          # bootstrap .adr/ project-scoped directory
make new TITLE="Use PostgreSQL"         # create a new ADR
make list                               # list all ADRs
make status                             # show all ADR statuses
make status NUM=2 STATUS=Proposed       # update status
```

### Available Targets

| Target | Description |
|--------|-------------|
| `init` | Bootstrap ADR directory and create first ADR |
| `init-data` | Bootstrap `.adr/` project-scoped directory (opt-in) |
| `new` | Create a new ADR from the format's baked-in template |
| `list` | List all ADRs with number, title, and status |
| `status` | Show or update an ADR's status |
| `install-agents` | Install custom agent files |

## Script Architecture

### Two-Level Dispatch

```
new.sh <format> <title...>
  └─► <format>-format.sh new <number> <title> <dir>
```

**Orchestrator (`new.sh`):**
- Resolves the ADR directory (reads `.adr/adr-dir`, then `.adr-dir`, or defaults to `docs/adr`)
- Computes the next sequential number (4-digit zero-padded)
- Slugifies the title into a filename
- Delegates to `<format>-format.sh new <number> <title> <dir>`

**Format script (`nygard-agent-format.sh`):**
- Self-contained: the template is baked into the script, no external files
- Subcommands via `case "$1" in`:

```bash
case "$1" in
  new)    # generate ADR document with baked-in template
  init)   # bootstrap ADR directory + first ADR
  list)   # list ADRs with number, title, status
  status) # show or update status (parses inline Status: field)
esac
```

### Direct Script Usage

When the Makefile is not available:

```bash
# Add scripts to PATH (from skill root)
export PATH="$PWD/scripts:$PATH"

# Initialize ADR directory
nygard-agent-format.sh init              # creates docs/adr/ with ADR 0001
nygard-agent-format.sh init decisions    # custom directory name

# Create a new ADR
new.sh nygard-agent "Use PostgreSQL for persistence"

# List all ADRs
nygard-agent-format.sh list

# Show status
nygard-agent-format.sh status 2          # show status of ADR 2
nygard-agent-format.sh status 2 Proposed # update status
```

### Path Discovery

The scripts resolve the ADR directory using a priority chain:

1. `.adr/adr-dir` — new convention (per ADR-0020)
2. `.adr-dir` — legacy adr-tools convention (backwards compatible)
3. `docs/adr` — default

This means projects that adopt `.adr/` can migrate their directory pointer from
the root-level `.adr-dir` file into `.adr/adr-dir`. Legacy projects continue to
work unchanged.

### Backward Compatibility

The `list` and `status` commands handle both:
- **Inline `Status:` metadata** (nygard-agent format)
- **`## Status` heading** (standard Nygard format)

This ensures existing ADRs written in standard Nygard format work correctly
during the transition period.

### Adding a New Format

To add support for a new format (e.g., MADR):

1. Create `scripts/<name>-format.sh` with the four subcommands
2. Set `ADR_AGENT_SKILL_FORMAT=<name>` when using the Makefile
3. No changes needed to `new.sh` or the Makefile

## Environment Variables

| Variable | Purpose |
|----------|---------|
| `ADR_DATE` | Override the date stamp (useful in CI/tests) |
| `ADR_AGENT_SKILL_FORMAT` | Select the format (default: `nygard-agent`) |

## Visualization

### When to Use Diagrams

Use diagrams when they clarify structure that prose or tables cannot. Skip them otherwise — too many diagrams overload context.

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

When presenting decision justification, use markdown tables to condense the analysis. Tables are easier to scan and keep the ADR focused:

```markdown
| Criteria   | Option A  | Option B  | Option C  |
|------------|:---------:|:---------:|:---------:|
| Latency    | ✅ Low    | ⚠️ Med    | ❌ High   |
| Cost       | ❌ High   | ✅ Low    | ✅ Low    |
| Maturity   | ✅ Proven | ⚠️ New    | ✅ Proven |
```
