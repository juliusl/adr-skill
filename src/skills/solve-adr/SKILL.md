---
name: solve-adr
description: "Use this skill when the user wants to solve a problem through structured exploration — analyzing constraints, discovering options, making decisions, and driving implementation across the ADR skill ecosystem. Activate when the user says things like \"solve this problem,\" \"help me figure out,\" \"explore options for,\" \"I need to decide how to handle,\" or \"what's the best approach for.\" The skill orchestrates across /author-adr (decisions), /prototype-adr (experiments), and /implement-adr (execution). Do not use for creating a single ADR when the user already has a decision — use author-adr. Do not use for implementing an existing ADR — use implement-adr. Do not use for running a standalone experiment — use prototype-adr."
license: CC-BY-4.0
metadata:
  version: "0.1"
---
# Solve ADR — Scenario-Driven Problem Solving

Orchestrate problem-solving end-to-end by delegating to companion skills: `/author-adr` for decisions, `/prototype-adr` for experiments, `/implement-adr` for execution. Every architectural decision encountered during problem solving is recorded via `/author-adr` for auditability.

## Procedure

| ID | Scenario | Mandatory | Description |
|----|----------|-----------|-------------|
| S-0 | Startup | Yes | Load preferences, check automation config, recommend missing settings |
| S-1 | Problem Exploration | Yes | User provides problem + constraints → explore options → converge on decision |

**If a mandatory step is skipped, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

```
User request
├─ docs/adr/ exists? ────────────► Load preferences → select scenario
├─ docs/adr/ missing? ──────────► Recommend: run `/author-adr` to bootstrap first
│
├─ "Solve this problem" ────────► S-0 → S-1: Problem Exploration
├─ "Help me figure out X" ──────► S-0 → S-1: Problem Exploration
├─ "Explore options for Y" ─────► S-0 → S-1: Problem Exploration
└─ "What's the best approach" ──► S-0 → S-1: Problem Exploration
```

## Configuration

This skill reads preferences from two scopes:

**User-scoped** (`~/.config/adr-skills/preferences.toml`):
```toml
[solve]
participation = "guided"     # guided | autonomous
auto_delegate = false        # automatically invoke /implement-adr after acceptance
```

**Project-scoped** (`.adr/preferences.toml`):
```toml
[solve]
default_scenario = "problem" # which scenario to use when not specified
```

**Path resolution:**
1. If `$XDG_CONFIG_HOME` is set, use `$XDG_CONFIG_HOME/adr-skills/preferences.toml`.
2. Otherwise, use `$HOME/.config/adr-skills/preferences.toml`.
3. Project-scoped: `.adr/preferences.toml` in the repository root.

Project-scoped values override user-scoped values. If neither file exists, use built-in defaults.

**Create on first write:** When persisting a preference, create the directory with `mkdir -p` before writing. Never assume it already exists.

## Writing Style

All generated content must follow this style:
- **Technical and simple** — write for engineers, not academics
- **No double negatives** — say what things *do*, not what they don't not do
- **Clear logic** — one idea per sentence, explicit cause-and-effect
- **Concise** — cut filler words; if a sentence works without a word, remove it

### S-0: Startup

Run this before every scenario.

1. **Read user preferences** — resolve the config path and read `[solve]` from `preferences.toml`.
   - If set, store `participation` and `auto_delegate` for the session.
   - If absent, proceed with defaults (`participation = "guided"`, `auto_delegate = false`).
2. **Read project preferences** — read `.adr/preferences.toml` for `[solve]` overrides.
   - Project values override user values.
3. **Recommend missing settings** — if no `[solve]` section exists in either scope, recommend defaults:
   > No solve preferences configured. Recommended defaults:
   > - `participation = "guided"` — summarize findings, let you drive decisions
   > - `auto_delegate = false` — ask before invoking /implement-adr
   >
   > Save these defaults?
4. **Load dispatch config** — read `[author.dispatch]` keys (`review`, `editor`) for downstream `/author-adr` calls. These control which agents handle review and revision cycles.

### S-1: Problem Exploration

The user provides a problem, background context, current thought process, and constraints. The agent explores the problem space, discovers options, and converges on a decision — recording everything via `/author-adr`.

Read [references/solve.md](references/solve.md) for the full S-1 workflow detail.

**Workflow summary:**

```
S-1.1: Problem intake — capture problem, constraints, stakeholders
  ↓
S-1.2: /author-adr — create ADR (draft worksheet → options → convergence)
  ↓
S-1.3: /prototype-adr — if Evaluation Checkpoint needs validation
  ↓
S-1.4: /author-adr — review → revise cycle
  ↓
S-1.5: Handoff — /implement-adr for execution (if auto_delegate or user agrees)
```

**How it works:** Solve-adr owns the problem intake (S-1.1) and orchestration (S-1.3–S-1.5). The option discovery, requirements refinement, and convergence happen within `/author-adr`'s create workflow (S-1.2) — author-adr's A-1 (draft worksheet) and A-2 (create) procedure handles these steps internally using the problem context from S-1.1.

**Cross-skill delegation points:**
- **S-1.2** — invoke `/author-adr` with the problem context to create the ADR end-to-end (worksheet → options → decision)
- **S-1.3** — invoke `/prototype-adr` if the Evaluation Checkpoint says "Pause for validation"
- **S-1.4** — invoke `/author-adr` to run the review → revise cycle
- **S-1.5** — invoke `/implement-adr` if auto_delegate is enabled or the user agrees

## Cross-Skill Invocation

The solve-adr agent delegates to companion skills by invoking them via the `skill` tool:

```
skill: "author-adr"    — when a decision needs to be recorded, reviewed, or revised
skill: "prototype-adr"  — when an assumption needs experimental validation
skill: "implement-adr"  — when an accepted decision needs execution
```

Each invocation loads the target skill's SKILL.md into the conversation context. The target skill runs its full procedure. When the target skill completes, the solve-adr agent resumes its scenario from the conversation history.

The platform constraint "do not invoke a skill that is already running" permits this pattern: solve-adr and the target skill are different skills. The agent's orchestration state (scenario step, problem context, ADRs created) is maintained in the conversation — not in skill-scoped storage.

## Defensive Logging

In all scenarios, the agent must:
- Create an ADR via `/author-adr` for every architectural decision encountered during problem solving
- Use `/author-adr` review workflow for quality assurance on each decision
- Never make a decision silently — if a choice affects architecture, it gets an ADR

The solve-adr skill's primary output is a set of reviewed, accepted decisions — not code. The decisions are the audit trail.

## Deep References

- **[references/solve.md](references/solve.md)** — Full S-1 Problem Exploration workflow: problem intake, option discovery, requirements refinement, evaluation checkpoint, convergence, and handoff.
