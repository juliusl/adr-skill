# Architectural Decision Records — Agent Skills

An [agentskills.io](https://agentskills.io)-compliant skill suite for AI coding agents to work with **Architectural Decision Records (ADRs)** — from authoring decisions to planning their implementation.

## Skills

### author-adr

Create, review, and manage ADRs.

| Capability | Details |
|---|---|
| **Create** ADRs | Draft decisions using Nygard, MADR, or Y-Statement templates |
| **Review** ADRs | Evaluate existing records for quality and completeness |
| **Manage** ADRs | Supersede, deprecate, link, and generate tables of contents |
| **Tooling** | Bundled `adr-tools` (Nygard) and `madr-tools` (MADR) format scripts |
| **Meta-ADRs** | Bootstrap `.meta/` directory for persistent behavioral policies |

### implement-adr

Turn accepted ADRs into actionable implementation plans.

| Capability | Details |
|---|---|
| **Plan** | Generate structured `plan.md` with staged tasks from ADRs |
| **Decompose** | Break decisions into implementation stages and scoped tasks |
| **Estimate** | Assign effort estimates (small / medium / heavy) per task |
| **Test criteria** | Include test & acceptance criteria per code context |
| **Gap detection** | Identify missing decisions and recommend additional ADRs |
| **Traceability** | Link plan tasks back to source ADR sections |
| **Participation modes** | Full control, Guided, Autonomous, or Weighted (cost-driven) |

## Quick Start

Install the skill by adding it to your agent's skill configuration, then ask your agent to:

> **Note:** Running sessions will need to be restarted for the skill to be picked up.

```
Create an ADR for choosing PostgreSQL as our primary database.
```

The agent will select the appropriate template, scaffold the record, and guide you through filling in the decision context, options, and rationale.

> For more explicit control if you are using copilot you can instead use,
> 
> ```sh
> /author-adr Create an ADR for choosing PostgreSQL as our primary database.
> ```

This will generate a file w/ the format `NNNN-<title>.md`. The skill should ask if you'd like to review/revise the adr (recommended).

After this step, you can use the `implement-adr` skill when you are ready to implement,

```sh
/implement-adr Implement adr 0002

# w/ options
/implement-adr 0002, autonomously w/ auto-commits
```

This will first write a plan under docs/plans in the format `<ADR-RANGE>.<REVISION>.plan.md`. If the plan is quite extensive, and the session is already saturated, you can easily create a new session and use the prompt.

```
/implement-adr Implement adr 0002 using plan 0002.0.plan.md
```

**Installing to copilot user-scoped skills**

```sh
# Clone the repo
git clone github.com/juliusl/adr-skills

# Install the skills to ~/.copilot/skills
make install-user-copilot
```


## Usage Tips

### Elaborate when describing the ADR you wish to write

This skill is mainly used to drive development with an agent so it's important that during the author phase you capture important context needed for the adr. The author-adr skill can also review an ADR which will end up re-inforcing this, but it's better to be in the habit of writing good prompts.

In the above example "Create an ADR for choosing PostgreSQL as our primary database" is missing important context, such as 
alternatives considered, overall reasons for it, any historical detail. So a better prompt might be:

```
/author-adr Create an ADR for choosing PostgreSQL as our primary database. PostgreSQL shows significant performance over other alternatives (MySQL, Sqlite) which is important as our service is called in a hot-path. 
```

This gives the agent much more context when authoring the rest of the ADR.

## Project Structure

```
├── AGENTS.md                         # Contributor guide
├── Makefile                          # Dev targets (test, validate)
├── docs/adr/                         # Project-level ADRs
├── docs/plans/                       # Implementation plans generated from ADRs
├── src/skills/
│   ├── author-adr/                   # Skill: create, review, manage ADRs
│   │   ├── SKILL.md                  # Skill entry point
│   │   ├── Makefile                  # Downstream agent interface
│   │   ├── references/               # On-demand docs (practices, templates, tooling)
│   │   ├── assets/                   # Templates, practice notes, static resources
│   │   └── scripts/                  # Bundled CLI scripts (adr-tools, madr-tools)
│   └── implement-adr/                # Skill: ADR → implementation plans
│       ├── SKILL.md                  # Skill entry point
│       ├── Makefile                  # Downstream agent interface
│       ├── references/               # On-demand docs (planning, testing, cost)
│       └── assets/                   # Plan template, static resources
```

## Development

```bash
# Run all tests (22 adr-tools + 9 madr-tools)
make test

# Run tests for a single format
make test-nygard
make test-madr

# Validate skills against agentskills.io spec
make validate-setup   # one-time
make validate-all     # both skills

# Install bundled custom agents
make install-agents

# Local testing in Copilot CLI
make dogfood-copilot      # installs both skills
```

## License

CC-BY-4.0 — see [SKILL.md](SKILL.md) for details.
