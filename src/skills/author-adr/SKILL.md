---
name: author-adr
description: >-
  Use this skill when the user needs to create, review, revise, or manage
  Architectural Decision Records (ADRs) — including drafting new decisions,
  evaluating existing ones for quality, addressing review comments
  interactively, solving architectural problems through guided exploration,
  choosing between ADR templates (Nygard, MADR, Y-Statement), setting up ADR
  tooling, or understanding ADR best practices. Activate when the user says
  things like "create an ADR," "new ADR," "draft a decision," "review this
  ADR," "address review comments," "help me decide," or "document this
  decision." Also activate when the user wants to justify a technology
  selection, record why an architecture was chosen over alternatives, or
  capture tradeoffs — even if they don't explicitly say "ADR." Do not use for
  general code review, project management, or non-architectural documentation.
license: CC-BY-4.0
metadata:
  source: adr.github.io
  version: "1.1"
---

# Architectural Decision Records (ADRs)

You are an expert on Architectural Decision Records. Use this skill whenever a user needs to create, review, or manage ADRs, choose an ADR template, select tooling, or understand best practices for architectural decision making.

## Configuration

This skill reads user-scoped preferences from a TOML configuration file at `~/.config/adr-skills/preferences.toml` (per ADR-0011 and ADR-0012).

**Path resolution:**
1. If `$XDG_CONFIG_HOME` is set, use `$XDG_CONFIG_HOME/adr-skills/preferences.toml`.
2. Otherwise, use `$HOME/.config/adr-skills/preferences.toml`.

**Graceful degradation:** If the file or directory does not exist, use built-in defaults. Never fail because config is absent.

**Create on first write:** When persisting a preference, create the directory with `mkdir -p` before writing. Never assume it already exists.

### Project-Scoped Directory (`.adr/`)

Per ADR-0020, projects can opt in to a `.adr/` directory at the project root for project-scoped data (telemetry, intermediate artifacts, project-level preferences). This is separate from `docs/adr/` (decision records) and `~/.config/adr-skills/` (user preferences).

Bootstrap with: `make -f <skill-root>/Makefile init-data`

This creates `.adr/`, `.adr/var/` (gitignored for transient data), and `.adr/.gitignore`. See [references/tooling.md](references/tooling.md) for details.

## Agent Workflow

When this skill is activated, **always start with Format Detection** before proceeding to the relevant task.

### Format Detection

Before any ADR operation, determine which ADR format to use:

1. **Read the config file** — resolve the config path (see [Configuration](#configuration)) and read `[author].template` from `preferences.toml`.
   - If set (e.g., `"nygard-agent"`, `"nygard"`, or `"madr"`), use it directly.
   - If absent, default to `"nygard-agent"`.
2. **If `docs/adr/` does not exist** — bootstrap the decision log using the default nygard-agent format:

   ```bash make -f <skill-root>/Makefile init DIR=docs/adr ```

3. **Cache the format** — for the rest of the session, pass `ADR_AGENT_SKILL_FORMAT=nygard-agent` (or the configured format) to all Makefile targets.

```
User request
├─ docs/adr/ exists? ────────────► Read config → set format
├─ docs/adr/ missing? ──────────► Bootstrap with nygard-agent → set format
│
├─ "Create an ADR" ──────────────► Go to: Creating an ADR
├─ "I have a problem to solve" ─► Go to: Solving a Problem
├─ "Review an ADR" ──────────────► Go to: Reviewing an ADR
├─ "Revise an ADR" ──────────────► Go to: Revising an ADR
├─ "Update/supersede an ADR" ───► Go to: Managing ADRs
├─ "Set up ADR tooling" ─────────► Go to: Tooling
├─ "Which template?" ────────────► Go to: Choosing a Template
├─ "Explain ADRs / concepts" ────► Go to: Core Concepts
└─ "Visualize / diagram" ────────► Go to: Visualization
```

### Creating an ADR

Read [references/create.md](references/create.md) for the full creation workflow including significance assessment, readiness checks, good practices, and anti-patterns.

1. **Assess significance** — score the decision against the 7 ASR criteria. If it's not architecturally significant, suggest informal documentation.
2. **Check readiness** — verify the START criteria: Stakeholders, Time/MRM, Alternatives, Requirements, Template.
3. **Pick a template** — default to Nygard Agent. Use MADR if the user needs structured tradeoff analysis. See [Choosing a Template](#choosing-a-template).
4. **Draft the ADR** — populate from the template in [assets/templates/](assets/templates/).
5. **Create via Makefile** — always use the Makefile target:

   ```bash make -f <skill-root>/Makefile new TITLE="Use PostgreSQL" ```

   Only fall back to calling scripts directly if the Makefile is unavailable. See [Escape Hatch](#escape-hatch-direct-script-usage) for direct usage.

6. **Validate completion** — check the ecADR Definition of Done criteria: Evidence, Criteria, Agreement, Documentation, Realization/Review.

7. **Recommend review** — after creating the ADR, recommend reviewing it:

   > Would you like to review this ADR? It will be checked for completeness, > reasoning fallacies, and anti-patterns.

   If the user agrees, proceed to [Reviewing an ADR](#reviewing-an-adr).

### Solving a Problem

Read [references/solve.md](references/solve.md) for the full problem-first solve workflow. Use this when the user has a problem to solve but hasn't yet identified a decision.

The solve process covers:

1. **Problem intake** — gather the problem statement, create a TBD ADR (`make new TITLE="tbd"`), populate the Context section
2. **Option discovery** — agent proposes candidate solutions, user collaborates to refine and add options
3. **Requirements refinement** — as options are evaluated, new requirements emerge and are folded back into Context
4. **Optional prototyping** — lightweight spikes to validate options; ADR stays in `Prototype` status
5. **Convergence** — user selects an option; agent drafts Decision and Consequences, renames the ADR (`make rename NUM=<n> TITLE="..."`), transitions status to `Proposed`
6. **Handoff** — the `Proposed` ADR is ready for the existing review workflow

**Solve vs. Create:** Use solve when the user describes a problem without a predetermined solution. Use create when the user arrives with a decision already made.

### Reviewing an ADR

Read [references/review.md](references/review.md) for the full structured review process. Use it as a prompt for a general-purpose agent to perform the review.

The review process covers:

1. **ecADR completeness check** — verify the 5 Definition of Done criteria
2. **Fallacy scan** — check against 7 architectural decision-making fallacies
3. **Anti-pattern check** — scan for 11 ADR creation anti-patterns
4. **Consequence validation** — interactively verify stated consequences with the user
5. **7-point checklist** — structured quality assessment
6. **Verdict** — Accept, Revise, or Rethink
7. **Offer revision** — if the verdict is "Revise," offer to interactively address the review comments. If the user agrees, proceed to [Revising an ADR](#revising-an-adr).

### Revising an ADR

Read [references/revise.md](references/revise.md) for the full interactive revision workflow. Use this after a review produces a "Revise" verdict.

The revision process covers:

1. **Load review comments** — parse the structured review output into discrete revision items
2. **Present each comment** — show findings one at a time with context
3. **Collect user response** — for each comment, the user can address it or reject it
4. **Apply revisions** — update the ADR with the user's approved changes
5. **Produce revision summary** — document what was addressed vs. rejected
6. **Recommend re-review** — suggest re-review if substantive changes were made

### Managing ADRs

Read [references/manage.md](references/manage.md) for the full management reference including status transitions, superseding, linking, and splitting.

**Guardrail:** When modifying ADRs, never modify other existing ADRs without explicit instruction. Cross-references and status updates to other ADRs (e.g., marking one as superseded) are the user's responsibility — suggest the change but do not apply it unilaterally.

### Choosing a Template

| Situation | Template | File |
|-----------|----------|------|
| Default (agent-developer workflow) | Nygard Agent | [nygard-agent-template.md](assets/templates/nygard-agent-template.md) |
| Structured tradeoff analysis needed | MADR Full | [madr-full-template.md](assets/templates/madr-full-template.md) |
| Quick capture, low ceremony | MADR Minimal | [madr-minimal-template.md](assets/templates/madr-minimal-template.md) |
| Inline / single-sentence capture | Y-Statement | [y-statement-template.md](assets/templates/y-statement-template.md) |
| Legacy projects using adr-tools | Standard Nygard | [nygard-template.md](assets/templates/nygard-template.md) |

See [references/templates.md](references/templates.md) for full template details and guidance.

## Core Concepts

An **Architectural Decision (AD)** is a justified design choice that addresses a functional or non-functional requirement that is architecturally significant.

An **Architecturally Significant Requirement (ASR)** is a requirement that has a measurable effect on the architecture and quality of a software/hardware system.

An **Architectural Decision Record (ADR)** captures a single AD and its rationale. The collection of ADRs in a project is its **decision log**.

All of this falls under **Architectural Knowledge Management (AKM)**, but ADR usage can be extended to design and other decisions ("any decision record").

## Tooling

This skill uses a unified script architecture via `ADR_AGENT_SKILL_FORMAT`:

| Format | Template | When to Use |
|--------|----------|-------------|
| `nygard-agent` (default) | Nygard Agent | Agent-developer workflows, quality-aware decisions |

### Makefile Targets (Required)

**Always use Makefile targets.** Only fall back to direct script usage if the Makefile is genuinely unavailable (e.g., not on `PATH`, broken environment).

```bash
# Set format (default: nygard-agent)
export ADR_AGENT_SKILL_FORMAT=nygard-agent

make -f <skill-root>/Makefile init DIR=docs/adr     # bootstrap ADR directory
make -f <skill-root>/Makefile init-data              # bootstrap .adr/ project-scoped directory
make -f <skill-root>/Makefile new TITLE="Use PostgreSQL"
make -f <skill-root>/Makefile rename NUM=2 TITLE="Use PostgreSQL"  # rename ADR file and heading
make -f <skill-root>/Makefile list                   # list all ADRs
make -f <skill-root>/Makefile status NUM=2 STATUS=Proposed  # update status
```

### Escape Hatch: Direct Script Usage

Only use direct scripts when the Makefile is unavailable. See [references/tooling.md](references/tooling.md) for full command docs:

```bash
export PATH="$PWD/scripts:$PATH"
nygard-agent-format.sh init docs/adr
nygard-agent-format.sh init-data
new.sh nygard-agent "Use PostgreSQL"
nygard-agent-format.sh rename 2 "Use PostgreSQL"
nygard-agent-format.sh list
nygard-agent-format.sh status 2 Proposed
```

### Visualization

Use **Mermaid** for all diagrams. Diagrams are valuable when complex relationships between processes or entities benefit from visual compression, but overuse can overload context — use sparingly. When comparing options, prefer **markdown tables** over diagrams. See [references/tooling.md](references/tooling.md) for guidelines and syntax patterns.

## Deep References

For detailed guidance beyond what is covered above, consult these references on-demand:

- [references/create.md](references/create.md) — full ADR creation workflow with significance assessment, readiness checks, and anti-patterns
- [references/solve.md](references/solve.md) — problem-first solve workflow with option discovery, requirements refinement, and convergence
- [references/review.md](references/review.md) — structured review process with ecADR checks, fallacy scan, and verdict format
- [references/revise.md](references/revise.md) — interactive revision workflow for addressing review comments after a Revise verdict
- [references/manage.md](references/manage.md) — status transitions, superseding, linking, splitting, and guardrails
- [references/templates.md](references/templates.md) — template details and selection guide
- [references/tooling.md](references/tooling.md) — unified script architecture and command reference

