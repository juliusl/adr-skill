# Architectural Decision Records — Agent Skill

An [agentskills.io](https://agentskills.io)-compliant skill that enables AI coding agents to create, review, and manage **Architectural Decision Records (ADRs)**.

## What It Does

| Capability | Details |
|---|---|
| **Create** ADRs | Draft decisions using Nygard, MADR, or Y-Statement templates |
| **Review** ADRs | Evaluate existing records for quality and completeness |
| **Manage** ADRs | Supersede, deprecate, link, and generate tables of contents |
| **Tooling** | Bundled `adr-tools` (Nygard) and `madr-tools` (MADR) runtimes |

## Quick Start

Install the skill by adding it to your agent's skill configuration, then ask your agent to:

```
Create an ADR for choosing PostgreSQL as our primary database.
```

The agent will select the appropriate template, scaffold the record, and guide you through filling in the decision context, options, and rationale.

## Project Structure

```
├── AGENTS.md                         # Contributor guide
├── Makefile                          # Dev targets (test, install-agents)
└── author-adr/   # Skill root (copy to install)
    ├── SKILL.md                      # Skill entry point (frontmatter + instructions)
    ├── Makefile                      # Downstream agent interface
    ├── references/                   # On-demand docs (practices, templates, tooling)
    ├── assets/                       # Templates, practice notes, static resources
    └── scripts/                      # Bundled CLI runtimes (adr-tools, madr-tools)
```

## Development

```bash
# Run all tests (22 adr-tools + 9 madr-tools)
make test

# Run tests for a single runtime
make test-nygard
make test-madr

# Install the bundled reviewer agent
make install-agents
```

## License

CC-BY-4.0 — see [SKILL.md](SKILL.md) for details.
