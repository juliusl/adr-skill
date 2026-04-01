---
name: architectural-decision-records
description: >-
  Use this skill when the user needs to create, review, or manage Architectural
  Decision Records (ADRs) — including drafting new decisions, evaluating
  existing ones for quality, choosing between ADR templates (Nygard, MADR,
  Y-Statement), setting up ADR tooling, or understanding ADR best practices.
  Activate this skill when the user wants to document a design choice, justify
  a technology selection, record why an architecture was chosen over
  alternatives, capture tradeoffs, or establish a decision log — even if they
  don't explicitly say "ADR" or "architecture decision." Also use when the user
  asks about decision rationale, design justification, or reversibility of
  technical choices. Do not use for general code review, project management,
  or non-architectural documentation.
license: CC-BY-4.0
metadata:
  source: adr.github.io
  version: "1.1"
---

# Architectural Decision Records (ADRs)

You are an expert on Architectural Decision Records. Use this skill whenever a
user needs to create, review, or manage ADRs, choose an ADR template, select
tooling, or understand best practices for architectural decision making.

## Agent Workflow

When this skill is activated, follow this decision tree:

```
User request
├─ "Create an ADR" ──────────────► Go to: Creating an ADR
├─ "Review an ADR" ──────────────► Go to: Reviewing an ADR
├─ "Set up ADR tooling" ─────────► Go to: Tooling
├─ "Which template?" ────────────► Go to: Choosing a Template
├─ "Explain ADRs / concepts" ────► Go to: Core Concepts
└─ "Visualize / diagram" ────────► Go to: Visualization
```

### Creating an ADR

1. **Assess significance** — read the
   [ASR Test](assets/PRACTICES_NOTES.md#asr-test--core-decisions) and score
   the decision against the 7 criteria. If it's not architecturally significant,
   suggest informal documentation instead.
2. **Check readiness** — verify the START criteria from the
   [Definition of Ready](assets/PRACTICES_NOTES.md#definition-of-ready-start):
   Stakeholders, Time/MRM, Alternatives, Requirements, Template.
3. **Pick a template** — default to Nygard (adr-tools default). Use MADR if the
   user needs structured tradeoff analysis. See [Choosing a Template](#choosing-a-template).
4. **Draft the ADR** — populate from the template in
   [assets/templates/](assets/templates/). Refer to
   [ADR Creation](assets/PRACTICES_NOTES.md#adr-creation) for good practices
   and anti-patterns to avoid.
5. **Use adr-tools** — run `adr new TITLE` to create the file. Use `-s` to
   supercede and `-l` to link. See [references/tooling.md](references/tooling.md).
6. **Validate completion** — check the ecADR criteria from the
   [Definition of Done](assets/PRACTICES_NOTES.md#definition-of-done-ecadr):
   Evidence, Criteria, Agreement, Documentation, Realization/Review.

### Reviewing an ADR

For reviews, this skill bundles a custom **adr-reviewer** agent
(`assets/adr-reviewer.agent.md`) that performs structured ecADR
completeness checks, fallacy scans, and anti-pattern detection.

**To install the custom agent:**

```bash
make -C <skill-root> install-agents
# Default: installs to <PARENT>/agents/ (two levels up from skill root)
# Override: make -C <skill-root> install-agents ADR_AGENTS_DIR=.github/agents
```

The agent file format (`.agent.md` with YAML frontmatter) is compatible with
both GitHub Copilot CLI and Claude Code. Install to the appropriate location:

| Platform | Project-scoped | User-scoped |
|----------|---------------|-------------|
| GitHub Copilot CLI | `.github/agents/` | `~/.copilot/agents/` |
| Claude Code | `agents/` (plugin root) | `~/.claude/agents/` |

Use `ADR_AGENTS_DIR` to target the correct location for your environment.

**Using the agent once installed:**

```bash
# Slash command (interactive mode)
/agent → select adr-reviewer → enter prompt

# Explicit instruction
Use the adr-reviewer agent to review docs/decisions/0003-use-redis.md

# By inference (auto-detected from description)
Review this ADR for completeness and fallacies: docs/decisions/0003-use-redis.md

# Programmatic
copilot --agent adr-reviewer --prompt "Review docs/decisions/0003-use-redis.md"
```

**If the custom agent is not installed**, follow the manual review process:

1. Read the [ADR Review](assets/PRACTICES_NOTES.md#adr-review) fragment.
2. Apply the **7-point checklist**: problem significant? options solve it?
   criteria valid? prioritized? solution addresses problem? consequences
   objective? actionable and traceable?
3. Avoid the **7 anti-patterns**: Pass Through, Copy Edit, Siding, Self
   Promotion, Power Game, Offended Reaction, Groundhog Day.
4. Check for **fallacies** using the
   [Seven AD Fallacies](assets/PRACTICES_NOTES.md#seven-ad-making-fallacies) fragment.

### Choosing a Template

| Situation | Template | File |
|-----------|----------|------|
| Using adr-tools (default) | Nygard | [nygard-template.md](assets/templates/nygard-template.md) |
| Structured tradeoff analysis needed | MADR Full | [madr-full-template.md](assets/templates/madr-full-template.md) |
| Quick capture, low ceremony | MADR Minimal | [madr-minimal-template.md](assets/templates/madr-minimal-template.md) |
| Inline / single-sentence capture | Y-Statement | [y-statement-template.md](assets/templates/y-statement-template.md) |

See [references/templates.md](references/templates.md) for full template details
and guidance.

## Core Concepts

An **Architectural Decision (AD)** is a justified design choice that addresses a
functional or non-functional requirement that is architecturally significant.

An **Architecturally Significant Requirement (ASR)** is a requirement that has a
measurable effect on the architecture and quality of a software/hardware system.

An **Architectural Decision Record (ADR)** captures a single AD and its
rationale. The collection of ADRs in a project is its **decision log**.

All of this falls under **Architectural Knowledge Management (AKM)**, but ADR
usage can be extended to design and other decisions ("any decision record").

## Tooling

This skill supports two runtimes via `ADR_AGENT_SKILL_RUNTIME`:

| Runtime | Template | When to Use |
|---------|----------|-------------|
| `nygard` (default) | Nygard ADR | Simple decisions, existing adr-tools workflows |
| `madr` | MADR 4.0 | Structured tradeoff analysis with options/pros/cons |

### Makefile Targets (Preferred)

```bash
# Set runtime (default: nygard)
export ADR_AGENT_SKILL_RUNTIME=nygard   # or: madr

make init                               # bootstrap ADR directory
make new TITLE="Use PostgreSQL"         # create a new ADR
make new TITLE="Use MySQL" SUPERSEDE=2  # supersede an existing ADR
make list                               # list all ADRs
make generate                           # generate table of contents
make install-agents                     # install custom agents (e.g., adr-reviewer)
make test                               # run tests for both runtimes
```

### Escape Hatch: Direct Script Usage

When the Makefile is unavailable or the agent needs to adapt, use scripts
directly. See [references/tooling.md](references/tooling.md) for full command
docs for both runtimes:

```bash
# Nygard runtime
export PATH="$PWD/scripts/adr-tools-3.0.0/src:$PATH"
adr init && adr new Use PostgreSQL

# MADR runtime
export PATH="$PWD/scripts/madr-tools/src:$PATH"
madr init && madr new -t full Use PostgreSQL
```

### Visualization

Use **Mermaid** for all diagrams. Diagrams are valuable when complex
relationships between processes or entities benefit from visual compression, but
overuse can overload context — use sparingly. When comparing options, prefer
**markdown tables** over diagrams. See
[references/tooling.md](references/tooling.md) for guidelines and
[assets/mermaid-chart-examples.md](assets/mermaid-chart-examples.md)
for syntax patterns.

## Deep References

For detailed practice guidance, see:
- [references/practices.md](references/practices.md) — full practices guide with inline summaries
- [references/templates.md](references/templates.md) — template details and selection guide
- [references/tooling.md](references/tooling.md) — dual-runtime command reference and visualization
- [assets/index.md](assets/index.md) — curated asset index with summaries

## Key References

- [MADR project](assets/adr-github-io-madr.md) — Markdown ADR template and official ADR homepage
- [Sustainable Architectural Decisions](assets/zdun-sustainable-architectural-decisions.md) — foundational Y-statement paper
- [Documenting Architecture Decisions](assets/nygard-documenting-architecture-decisions.md) — Nygard's original 2011 post
