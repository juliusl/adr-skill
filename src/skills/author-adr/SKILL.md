---
name: author-adr
description: >-
  Use this skill when the user needs to create, review, or manage Architectural
  Decision Records (ADRs) — including drafting new decisions, evaluating
  existing ones for quality, choosing between ADR templates (Nygard, MADR,
  Y-Statement), setting up ADR tooling, or understanding ADR best practices.
  Activate when the user says things like "create an ADR," "write an ADR,"
  "new ADR," "draft a decision," "review this ADR," or "document this
  decision." Also activate
  when the user wants to justify a technology selection, record why an
  architecture was chosen over alternatives, capture tradeoffs, or establish
  a decision log — even if they don't explicitly say "ADR" or "architecture
  decision." Also use when the user asks about decision rationale, design
  justification, or reversibility of technical choices. Do not use for general
  code review, project management, or non-architectural documentation.
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

When this skill is activated, **always start with Format Detection** before
proceeding to the relevant task.

After writing or editing an ADR. Recommend the user to review the adr using the bundled agent.
See step 7 from the "Creating an ADR" section for details on this process.

### Format Detection

Before any ADR operation, determine which ADR format the project uses:

1. **Check for `docs/adr/`** — if the directory exists, read the first ADR
   (e.g., `0001-record-architecture-decisions.md`) to determine the chosen
   format (Nygard or MADR).
2. **If `docs/adr/` does not exist** — prompt the user with a discrete choice:

   > Which ADR format would you like to use?
   > 1. **Nygard** — lightweight, widely adopted (Status / Context / Decision / Consequences)
   > 2. **MADR** — structured tradeoff analysis (adds Options, Pros/Cons, Decision Outcome)

3. **Bootstrap the decision log** — initialize `docs/adr` and create the first
   ADR recording the format choice. Use the Makefile target:

   ```bash
   make -f <skill-root>/Makefile init DIR=docs/adr
   ```

   This first ADR always uses Nygard format (the bootstrap default). If the user
   chose MADR, set `ADR_AGENT_SKILL_FORMAT=madr` for all subsequent ADRs.

4. **Cache the format** — for the rest of the session, pass
   `ADR_AGENT_SKILL_FORMAT=nygard` or `ADR_AGENT_SKILL_FORMAT=madr` to all
   Makefile targets based on what was recorded in the first ADR.

```
User request
├─ docs/adr/ exists? ────────────► Read first ADR → set format
├─ docs/adr/ missing? ──────────► Prompt user → bootstrap → set format
│
├─ "Create an ADR" ──────────────► Go to: Creating an ADR
├─ "Review an ADR" ──────────────► Go to: Reviewing an ADR
├─ "Set up ADR tooling" ─────────► Go to: Tooling
├─ "Which template?" ────────────► Go to: Choosing a Template
├─ "Explain ADRs / concepts" ────► Go to: Core Concepts
└─ "Visualize / diagram" ────────► Go to: Visualization
```

### Creating an ADR

**Guardrail:** When creating or editing an ADR, never modify other existing
ADRs. Cross-references and status updates to other ADRs (e.g., marking one as
superseded) are the user's responsibility — suggest the change but do not apply
it without explicit instruction.

1. **Assess significance** — read the
   [ASR Test](assets/PRACTICES_NOTES.md#asr-test--core-decisions) and score
   the decision against the 7 criteria. If it's not architecturally significant,
   suggest informal documentation instead.
2. **Check readiness** — verify the START criteria from the
   [Definition of Ready](assets/PRACTICES_NOTES.md#definition-of-ready-start):
   Stakeholders, Time/MRM, Alternatives, Requirements, Template.
3. **Pick a template** — default to Nygard. Use MADR if the user needs
   structured tradeoff analysis. See [Choosing a Template](#choosing-a-template).
4. **Draft the ADR** — populate from the template in
   [assets/templates/](assets/templates/). Refer to
   [ADR Creation](assets/PRACTICES_NOTES.md#adr-creation) for good practices
   and anti-patterns to avoid.
5. **Create via Makefile** — always use the Makefile target:

   ```bash
   make -f <skill-root>/Makefile new TITLE="Use PostgreSQL"
   make -f <skill-root>/Makefile new TITLE="Use MySQL" SUPERSEDE=2
   ```

   Only fall back to calling scripts directly if the Makefile is unavailable.
   See [Escape Hatch](#escape-hatch-direct-script-usage) for direct usage.

6. **Validate completion** — check the ecADR criteria from the
   [Definition of Done](assets/PRACTICES_NOTES.md#definition-of-done-ecadr):
   Evidence, Criteria, Agreement, Documentation, Realization/Review.

7. **Recommend review** — after creating the ADR, recommend reviewing it with
   the bundled `adr-reviewer` agent:

   > Would you like to review this ADR with the adr-reviewer agent? It will
   > check for completeness, reasoning fallacies, and anti-patterns — and
   > validate your consequences with you directly.

   If the user agrees:

   a. **Check if installed** — look for `adr-reviewer.agent.md` in the
      project's agents directory (e.g., `.github/agents/`, `agents/`).
   b. **If not installed, ask for consent:**

      > The adr-reviewer agent needs to be installed as a custom agent in your
      > project. This will copy `adr-reviewer.agent.md` to your agents
      > directory. Is that okay?

   c. **If user consents** — install via:
      ```bash
      make -C <skill-root> install-agents
      ```
   d. **If user declines installation** — fall back to the manual review
      process: run the inline ecADR checklist, fallacy scan, and anti-pattern
      check as documented in [Reviewing an ADR](#reviewing-an-adr).
   e. **Run the reviewer** on the newly created ADR. The reviewer will
      interactively validate each consequence with the user.

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

This skill supports two ADR formats via `ADR_AGENT_SKILL_FORMAT`:

| Format | Template | When to Use |
|--------|----------|-------------|
| `nygard` (default) | Nygard ADR | Simple decisions, existing adr-tools workflows |
| `madr` | MADR 4.0 | Structured tradeoff analysis with options/pros/cons |

### Makefile Targets (Required)

**Always use Makefile targets.** Only fall back to direct script usage if the
Makefile is genuinely unavailable (e.g., not on `PATH`, broken environment).

```bash
# Set format (default: nygard)
export ADR_AGENT_SKILL_FORMAT=nygard   # or: madr

make -f <skill-root>/Makefile init DIR=docs/adr     # bootstrap ADR directory
make -f <skill-root>/Makefile new TITLE="Use PostgreSQL"
make -f <skill-root>/Makefile new TITLE="Use MySQL" SUPERSEDE=2
make -f <skill-root>/Makefile list                   # list all ADRs
make -f <skill-root>/Makefile generate               # generate table of contents
make -f <skill-root>/Makefile install-agents         # install custom agents
```

### Escape Hatch: Direct Script Usage

Only use direct scripts when the Makefile is unavailable. See
[references/tooling.md](references/tooling.md) for full command docs:

```bash
# Nygard format
export PATH="$PWD/scripts/adr-tools-3.0.0/src:$PATH"
adr init docs/adr && adr new Use PostgreSQL

# MADR format
export PATH="$PWD/scripts/madr-tools/src:$PATH"
madr init docs/adr && madr new -t full Use PostgreSQL
```

#### Meta-ADR Directory

The `init` target also creates `<adr-dir>/.meta/` — a subdirectory for storing
behavioral policies consumed by companion skills (e.g., `implement-adr`). Meta-ADRs
use the same Nygard format as project ADRs but are numbered independently.

To create the `.meta` directory for an existing ADR setup:

```bash
make -f <skill-root>/Makefile init-meta
```

See [ADR-0008](docs/adr/0008-meta-adr-directory-for-skill-behavioral-policies.md)
for the full convention.

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
- [references/tooling.md](references/tooling.md) — dual-format command reference and visualization
- [assets/index.md](assets/index.md) — curated asset index with summaries

## Key References

- [MADR project](assets/adr-github-io-madr.md) — Markdown ADR template and official ADR homepage
- [Sustainable Architectural Decisions](assets/zdun-sustainable-architectural-decisions.md) — foundational Y-statement paper
- [Documenting Architecture Decisions](assets/nygard-documenting-architecture-decisions.md) — Nygard's original 2011 post
