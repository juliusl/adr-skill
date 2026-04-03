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

## Configuration

This skill reads user-scoped preferences from a TOML configuration file at
`~/.config/adr-skills/preferences.toml` (per ADR-0011 and ADR-0012).

**Path resolution:**
1. If `$XDG_CONFIG_HOME` is set, use `$XDG_CONFIG_HOME/adr-skills/preferences.toml`.
2. Otherwise, use `$HOME/.config/adr-skills/preferences.toml`.

**Graceful degradation:** If the file or directory does not exist, use built-in
defaults. Never fail because config is absent.

**Create on first write:** When persisting a preference, create the directory
with `mkdir -p` before writing. Never assume it already exists.

## Agent Workflow

When this skill is activated, **always start with Format Detection** before
proceeding to the relevant task.

### Format Detection

Before any ADR operation, determine which ADR format to use:

1. **Read the config file** — resolve the config path (see [Configuration](#configuration))
   and read `[author].template` from `preferences.toml`.
   - If set to `"nygard"` or `"madr"`, use it directly.
   - If absent, default to `"nygard"`.
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

4. **Offer to persist** — if the user chose a format during bootstrap, offer to
   save it to `[author].template` in `preferences.toml`:

   > Save this format as your default for all projects?

   If yes, write `[author]\ntemplate = "<format>"` to `preferences.toml`
   (creating the file and directory if needed with `mkdir -p`).

5. **Cache the format** — for the rest of the session, pass
   `ADR_AGENT_SKILL_FORMAT=nygard` or `ADR_AGENT_SKILL_FORMAT=madr` to all
   Makefile targets.

```
User request
├─ docs/adr/ exists? ────────────► Read config → set format
├─ docs/adr/ missing? ──────────► Prompt user → bootstrap → persist → set format
│
├─ "Create an ADR" ──────────────► Go to: Creating an ADR
├─ "Review an ADR" ──────────────► Go to: Reviewing an ADR
├─ "Update/supersede an ADR" ───► Go to: Managing ADRs
├─ "Set up ADR tooling" ─────────► Go to: Tooling
├─ "Which template?" ────────────► Go to: Choosing a Template
├─ "Explain ADRs / concepts" ────► Go to: Core Concepts
└─ "Visualize / diagram" ────────► Go to: Visualization
```

### Creating an ADR

Read [references/create.md](references/create.md) for the full creation
workflow including significance assessment, readiness checks, good practices,
and anti-patterns.

1. **Assess significance** — score the decision against the 7 ASR criteria.
   If it's not architecturally significant, suggest informal documentation.
2. **Check readiness** — verify the START criteria: Stakeholders, Time/MRM,
   Alternatives, Requirements, Template.
3. **Pick a template** — default to Nygard. Use MADR if the user needs
   structured tradeoff analysis. See [Choosing a Template](#choosing-a-template).
4. **Draft the ADR** — populate from the template in
   [assets/templates/](assets/templates/).
5. **Create via Makefile** — always use the Makefile target:

   ```bash
   make -f <skill-root>/Makefile new TITLE="Use PostgreSQL"
   make -f <skill-root>/Makefile new TITLE="Use MySQL" SUPERSEDE=2
   ```

   Only fall back to calling scripts directly if the Makefile is unavailable.
   See [Escape Hatch](#escape-hatch-direct-script-usage) for direct usage.

6. **Validate completion** — check the ecADR Definition of Done criteria:
   Evidence, Criteria, Agreement, Documentation, Realization/Review.

7. **Recommend review** — after creating the ADR, recommend reviewing it:

   > Would you like to review this ADR? It will be checked for completeness,
   > reasoning fallacies, and anti-patterns.

   If the user agrees, proceed to [Reviewing an ADR](#reviewing-an-adr).

### Reviewing an ADR

Read [references/review.md](references/review.md) for the full structured
review process. Use it as a prompt for a general-purpose agent to perform
the review.

The review process covers:

1. **ecADR completeness check** — verify the 5 Definition of Done criteria
2. **Fallacy scan** — check against 7 architectural decision-making fallacies
3. **Anti-pattern check** — scan for 11 ADR creation anti-patterns
4. **Consequence validation** — interactively verify stated consequences with
   the user
5. **7-point checklist** — structured quality assessment
6. **Verdict** — Accept, Revise, or Rethink

### Managing ADRs

Read [references/manage.md](references/manage.md) for the full management
reference including status transitions, superseding, linking, and splitting.

**Guardrail:** When modifying ADRs, never modify other existing ADRs without
explicit instruction. Cross-references and status updates to other ADRs (e.g.,
marking one as superseded) are the user's responsibility — suggest the change
but do not apply it unilaterally.

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

### Visualization

Use **Mermaid** for all diagrams. Diagrams are valuable when complex
relationships between processes or entities benefit from visual compression, but
overuse can overload context — use sparingly. When comparing options, prefer
**markdown tables** over diagrams. See
[references/tooling.md](references/tooling.md) for guidelines and syntax
patterns.

## Deep References

For detailed guidance beyond what is covered above, consult these references
on-demand:

- [references/create.md](references/create.md) — full ADR creation workflow with significance assessment, readiness checks, and anti-patterns
- [references/review.md](references/review.md) — structured review process with ecADR checks, fallacy scan, and verdict format
- [references/manage.md](references/manage.md) — status transitions, superseding, linking, splitting, and guardrails
- [references/templates.md](references/templates.md) — template details and selection guide
- [references/tooling.md](references/tooling.md) — dual-format command reference and visualization

