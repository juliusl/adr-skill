# Architectural Decision Records — Agent Skills

An [agentskills.io](https://agentskills.io)-compliant skill suite for AI coding agents to work with **Architectural Decision Records (ADRs)** — from authoring decisions to planning their implementation.

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

Scenario-driven problem solving orchestrator. Delegates to `/author-adr`, `/prototype-adr`, and `/implement-adr`.

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

## Quick Start

Install the skill by adding it to your agent's skill configuration, then ask your agent to:

> **Note:** Running sessions will need to be restarted for the skill to be picked up.

### When you have a decision ready

```
Create an ADR for choosing PostgreSQL as our primary database.
```

The agent will select the appropriate template, scaffold the record, and guide you through filling in the decision context, options, and rationale.

### When you have a problem but no solution yet

```
I need to figure out how to handle persistent event storage. Help me explore options.
```

The agent will create a TBD ADR, help you discover and evaluate options through dialogue, and converge on a decision — capturing the full exploration as architectural knowledge.

> For more explicit control if you are using Copilot, you can use skill routing:
>
> ```sh
> /author-adr Create an ADR for choosing PostgreSQL as our primary database.
> /solve-adr I have a problem to solve — we need a caching strategy.
> ```

The skill will generate a file with the format `NNNN-<title>.md`. After creation, the skill will offer to review and revise the ADR (recommended).

When you are ready to implement:

```sh
/implement-adr Implement ADR 0002

# w/ options
/implement-adr 0002, autonomously w/ auto-commits
```

This writes a plan under `docs/plans/` in the format `<ADR-RANGE>.<REVISION>.plan.md`. If the plan is extensive and the session is already saturated, you can create a new session:

```
/implement-adr Implement ADR 0002 using plan 0002.0.plan.md
```

**Installing to Copilot user-scoped skills**

```sh
# Clone the repo
git clone github.com/juliusl/adr-skills

# Install the skills to ~/.copilot/skills
make install-skills
```

## Usage Tips

### Use solve-adr for problems, author-adr for decisions

If you know what you want to decide, use **author-adr** and be specific:

```
/author-adr Create an ADR for choosing PostgreSQL as our primary database.
PostgreSQL shows significant performance over other alternatives (MySQL, SQLite)
which is important as our service is called in a hot-path.
```

If you have a problem but haven't picked a solution, use **solve-adr** instead:

```
/solve-adr We need a database for our event storage service. The hot path
requires sub-millisecond reads. Help me figure out the best approach.
```

The solve workflow guides you through option discovery, requirements refinement, and convergence — delegating to `/author-adr` for decisions, `/prototype-adr` for experiments, and `/implement-adr` for execution.

### Elaborate when describing context

Whether creating or solving, more context produces better ADRs. Include:
- **Constraints** — performance requirements, team expertise, existing tech stack
- **Stakeholders** — who cares about this decision and why
- **History** — what you've tried before, what didn't work

### Review and revise every ADR

After authoring, always run the review. The review catches reasoning fallacies, anti-patterns, and missing quality criteria that are easy to overlook during drafting. If the verdict is "Revise," the skill will walk you through each finding interactively.

## Configuration

All skills read preferences from `~/.config/adr-skills/preferences.toml` (user-scoped) and `.adr/preferences.toml` (project-scoped). Project values override user values.

See [docs/PREFERENCES.md](docs/PREFERENCES.md) for the full reference with all keys, defaults, and example configurations.

When using solve-adr for end-to-end workflows, it creates a `solve/<topic>` feature branch to isolate its output from your working branch. The resulting branch is ready for PR review.

## Project Structure

```
├── AGENTS.md                         # Contributor guide
├── Makefile                          # Dev targets (test, validate, build-tools)
├── crates/                           # Cargo workspace — Rust tooling
│   ├── Cargo.toml                    # Workspace root
│   └── adr-db/                       # Plumbing CLI: JSONL → SQLite persistence
│       ├── Cargo.toml
│       ├── diesel.toml               # Diesel schema output config
│       ├── migrations/               # Diesel SQL migrations
│       └── src/                      # Rust source (main, init, ingest, view, models, schema)
├── docs/adr/                         # Project-level ADRs
├── docs/plans/                       # Implementation plans generated from ADRs
├── src/skills/
│   ├── author-adr/                   # Skill: create, review, manage ADRs
│   │   ├── SKILL.md                  # Skill entry point
│   │   ├── Makefile                  # Downstream agent interface
│   │   ├── references/               # On-demand docs (create, review, revise, manage)
│   │   ├── assets/                   # Templates and static resources
│   │   └── scripts/                  # Unified format-based scripts
│   ├── solve-adr/                    # Skill: scenario-driven problem solving orchestrator
│   │   ├── SKILL.md                  # Skill entry point (scenario-based procedures)
│   │   ├── Makefile                  # Minimal — orchestrator has no scripts
│   │   └── references/               # On-demand docs (S-1 Problem Exploration)
│   ├── implement-adr/                # Skill: ADR → implementation plans
│   │   ├── SKILL.md                  # Skill entry point
│   │   ├── Makefile                  # Downstream agent interface
│   │   ├── references/               # On-demand docs (planning, testing, cost, plan-review)
│   │   └── assets/                   # Plan template, static resources
│   └── prototype-adr/                # Skill: validate decisions via prototyping
│       ├── SKILL.md                  # Skill entry point
│       ├── Makefile                  # Downstream agent interface
│       ├── references/               # On-demand docs (profiles, isolation, observation)
│       └── assets/                   # Default profile template
```

## Development

```bash
# Run all tests (10 unified format tests)
make test

# Build Rust tooling (requires Rust toolchain)
make build-tools

# Validate skills against agentskills.io spec
make validate-setup   # one-time
make validate-all     # both skills

# Install bundled custom agents
make install-agents

# Local testing in Copilot CLI
make dogfood-copilot      # installs both skills
```

### adr-db (Rust CLI)

The `crates/adr-db/` directory contains a Rust binary for ingesting JSONL data into a local SQLite database. It serves as plumbing between skill JSONL producers and downstream consumers.

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
cd crates/adr-db && DATABASE_URL="sqlite:///tmp/dev.db" diesel migration generate <name>
```

Contributors who only work on shell scripts do not need Rust or `diesel_cli` installed.

## License

CC-BY-4.0 — see [SKILL.md](SKILL.md) for details.
