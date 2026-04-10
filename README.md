# Architectural Decision Records — Agent Skills

[agentskills.io](https://agentskills.io)-compliant skills for AI coding agents to author, review, implement, and validate **Architectural Decision Records (ADRs)**.

## Skills

### author-adr

Create, review, revise, and manage ADRs.

| Capability | Details |
|---|---|
| **Create** ADRs | Draft decisions using Nygard Agent, MADR, or Y-Statement templates |
| **Review** ADRs | Evaluate existing records for quality, fallacies, and anti-patterns |
| **Revise** ADRs | Interactively address review comments after a Revise verdict |
| **Manage** ADRs | Supersede, deprecate, link, and track status transitions |
| **Tooling** | Unified format-based scripts via `ADR_AGENT_SKILL_FORMAT` |

### solve-adr

Problem-solving orchestrator. Delegates to `/author-adr`, `/prototype-adr`, and `/implement-adr`.

| Capability | Details |
|---|---|
| **Problem exploration** | Start from a problem, explore options, converge on a decision |
| **Cross-skill orchestration** | Invoke companion skills for decisions, experiments, and implementation |
| **Defensive logging** | Every decision recorded via `/author-adr` for auditability |
| **Preference management** | User and project-scoped automation preferences |

### implement-adr

Turn accepted ADRs into actionable implementation plans.

| Capability | Details |
|---|---|
| **Plan** | Generate structured `plan.md` with staged tasks from ADRs |
| **Plan review** | Verify plan-ADR alignment via sub-agent reviewer (per ADR-0025) |
| **Decompose** | Break decisions into implementation stages and scoped tasks |
| **Estimate** | Assign effort estimates (small / medium / heavy) per task |
| **Test criteria** | Include test & acceptance criteria per code context |
| **Gap detection** | Identify missing decisions and recommend additional ADRs |
| **Traceability** | Link plan tasks back to source ADR sections |
| **Participation modes** | Full control, Guided, Autonomous, or Weighted (cost-driven) |

### prototype-adr

Run controlled experiments to validate architectural decisions.

| Capability | Details |
|---|---|
| **Prototype** | Run spikes or PoCs scoped to a specific architectural question |
| **Experiment** | Define hypotheses, collect observations, draw conclusions |
| **Report** | Produce a structured findings report that feeds back into `author-adr` |

## Quick Start

Install the skill (see [Installation](#installation) below), then use any of the examples below.

### Installation

```sh
# Clone the repo
git clone https://github.com/juliusl/adr-skills

# Install the skills to ~/.copilot/skills
make install-skills
```

To install skills, agents, and bootstrap a project with full automation (user-mode scope, all dispatch hooks):

```sh
# From another project:
make -C /path/to/adr-skills install-user PROJECT_DIR=$(pwd)
```

> **Tip:** If installation fails, verify `~/.copilot/skills/` exists and is writable. Re-run `make install-skills`. Running sessions must be restarted after installation.

### When you have a decision ready

```
Create an ADR for choosing PostgreSQL as our primary database.
```

The skill selects a template, scaffolds the record, and prompts for decision context, options, and rationale. Output is `NNNN-<title>.md`. After creation the skill offers a review pass (recommended).

### When you have a problem but no solution yet

```
I need to figure out how to handle persistent event storage. Help me explore options.
```

The skill creates a TBD ADR, explores options, and converges on a decision — recording the full evaluation as structured ADR content.

> For more explicit control in **GitHub Copilot CLI** or **Copilot Chat**, use skill routing:
>
> ```sh
> /author-adr Create an ADR for choosing PostgreSQL as our primary database.
> /solve-adr I have a problem to solve — we need a caching strategy.
> ```

When you are ready to implement:

```sh
/implement-adr Implement ADR 0002

# w/ options
/implement-adr 0002, autonomously w/ auto-commits
```

This writes a plan under `docs/plans/` in the format `<ADR-RANGE>.<REVISION>.plan.md`. If the session is saturated, continue in a new session:

```
/implement-adr Implement ADR 0002 using plan 0002.0.plan.md
```

> Participation modes (full-control, guided, autonomous, weighted) and auto-commit can be configured in `~/.config/adr-skills/preferences.toml` — see [docs/PREFERENCES.md](docs/PREFERENCES.md).

## Usage Tips

### Use solve-adr for problems, author-adr for decisions

Use **author-adr** when the decision is already known. Use **solve-adr** when the problem is known but the solution is not — it delegates to `/author-adr`, `/prototype-adr`, and `/implement-adr` as needed.

### Elaborate when describing context

Whether creating or solving, more context produces better ADRs. Include:
- **Constraints** — performance requirements, team expertise, existing tech stack
- **Stakeholders** — who cares about this decision and why
- **History** — what you've tried before, what didn't work

### Review and revise every ADR

Always run the review after authoring. It catches reasoning fallacies, anti-patterns, and missing quality criteria. A "Revise" verdict returns each finding with guidance for addressing it.

## Configuration

Skills read preferences from `~/.config/adr-skills/preferences.toml` (user-scoped) and `.adr/preferences.toml` (project-scoped). Project values override user values. See [docs/PREFERENCES.md](docs/PREFERENCES.md) for all keys and defaults.

`solve-adr` creates a `solve/<topic>` branch for its output — the resulting branch is ready for PR review.

## Project Structure

```
├── AGENTS.md                         # Contributor guide
├── Makefile                          # Dev targets (test, validate, build-tools)
├── docs/
│   ├── adr/                          # Project-level ADRs
│   └── plans/                        # Implementation plans generated from ADRs
├── src/
│   ├── agents/                       # Agent definition files
│   ├── crates/                       # Cargo workspace — Rust tooling
│   │   ├── Cargo.toml                # Workspace root
│   │   └── adr-db/                   # Plumbing CLI: JSONL → SQLite persistence
│   │       ├── Cargo.toml
│   │       ├── diesel.toml           # Diesel schema output config
│   │       ├── migrations/           # Diesel SQL migrations
│   │       └── src/                  # Rust source (main, init, ingest, view, models, schema)
│   └── skills/
│       ├── author-adr/                   # Skill: create, review, manage ADRs
│       │   ├── SKILL.md                  # Skill entry point
│       │   ├── Makefile                  # Downstream agent interface
│       │   ├── references/               # On-demand docs (create, review, revise, manage)
│       │   ├── assets/                   # Templates and static resources
│       │   └── scripts/                  # Unified format-based scripts
│       ├── solve-adr/                    # Skill: scenario-driven problem solving orchestrator
│       │   ├── SKILL.md                  # Skill entry point (scenario-based procedures)
│       │   ├── Makefile                  # Minimal — orchestrator has no scripts
│       │   └── references/               # On-demand docs (S-1 Problem Exploration)
│       ├── implement-adr/                # Skill: ADR → implementation plans
│       │   ├── SKILL.md                  # Skill entry point
│       │   ├── Makefile                  # Downstream agent interface
│       │   ├── references/               # On-demand docs (planning, testing, cost, plan-review)
│       │   └── assets/                   # Plan template, static resources
│       └── prototype-adr/                # Skill: validate decisions via prototyping
│           ├── SKILL.md                  # Skill entry point
│           ├── Makefile                  # Downstream agent interface
│           ├── references/               # On-demand docs (profiles, isolation, observation)
│           └── assets/                   # Default profile template
```

## Development

```bash
# Run all tests (32 tests)
make test

# Build Rust tooling (requires Rust toolchain)
make build-tools

# Validate skills against agentskills.io spec
make validate-setup   # one-time
make validate-all     # both skills

# Install bundled custom agents
make install-agents        # installs bundled custom agents to ~/.copilot/agents

# Local testing in Copilot CLI
make dogfood-copilot      # installs both skills for local testing in Copilot CLI
```

### adr-db (Rust CLI)

`adr-db` is a Rust CLI that ingests JSONL data into a local SQLite database. It bridges skill JSONL output and downstream consumers.

```bash
# Initialize the database
adr-db init

# Ingest JSONL from a plan's implementation summary
awk -f src/skills/implement-adr/scripts/extract-summary.awk docs/plans/0020.0.plan.md | adr-db ingest

# Inspect database contents (list tables)
adr-db view

# View table data in TSV format (awk/grep-friendly)
adr-db view task_summaries

# View as JSONL (ingest-compatible, round-trips with `adr-db ingest`)
adr-db view task_summaries --output jsonl

# Limit output rows
adr-db view task_summaries --limit 10

# Suppress header row
adr-db view task_summaries --no-header
```

> **⚠️ Instability notice:** The `view` subcommand is proto-porcelain with no stability guarantees. Output format, flags, column order, and behavior may change at any time. Do not depend on this output in scripts or downstream tooling. Use `--output jsonl` for any structured consumption.

**For contributors working on `adr-db`:**

```bash
# Install diesel_cli for migration authoring
cargo install diesel_cli --no-default-features --features sqlite

# Create a new migration
cd src/crates/adr-db && DATABASE_URL="sqlite:///tmp/dev.db" diesel migration generate <name>
```

Contributors who only work on shell scripts do not need Rust or `diesel_cli` installed.

## License

CC-BY-4.0
