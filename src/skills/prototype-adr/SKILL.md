---
name: prototype-adr
description: "Use this skill when the user wants to prototype or validate an architectural decision — running controlled experiments, benchmarks, or spikes to gather evidence that supports or refutes a decision before committing to implementation. Activate when the user says things like \"prototype this decision,\" \"validate ADR 0023,\" \"run an experiment,\" \"spike this option,\" \"test this assumption,\" \"gather evidence for this decision,\" or \"I need data before deciding.\" Also activate when an ADR's Evaluation Checkpoint has \"Pause for validation\" with populated Validation needs, or when a review flags \"needs validation\" on the Experimentation Tolerance criterion. Do not use for authoring or reviewing ADRs — use author-adr for that. Do not use for turning finalized decisions into implementation plans — use implement-adr for that."
license: CC-BY-4.0
metadata:
  version: "0.1"
---
# Prototype ADR — From Decisions to Evidence
Design and run controlled experiments to test architectural decisions. Gather evidence that supports or refutes a decision before committing to implementation.
This skill consumes ADRs produced by the `author-adr` skill, reads prototype
objectives from the Evaluation Checkpoint's "Validation needs" section (per
ADR-0024), and executes experiments in isolated environments. Findings feed
back into the ADR lifecycle as empirical evidence.
## Configuration
This skill reads user-scoped preferences from a TOML configuration file at
`~/.config/adr-skills/preferences.toml` (per ADR-0011 and ADR-0012).
**Supported keys under `[prototype]`:**
| Key | Default | Description |
|-----|---------|-------------|
| `isolation` | `"worktree"` | Default isolation backend: `worktree`, `container`, or `acp-sandbox` |
| `runtime` | `""` | Set to `"acp"` to enable ACP sandbox backend |
| `teardown` | `"automatic"` | Default teardown behavior: `automatic` or `manual` |
If the file or directory is missing, use built-in defaults. Do not fail when config is absent.
## Agent Workflow
```
User request
├─ ADR specified? ──────────────► Read ADR → extract prototype objectives
├─ ADR not specified? ──────────► List ADRs → check for Evaluation Checkpoints
│                                  with "Pause for validation"
│
├─ "Prototype this ADR" ───────► Go to: Running Experiments
├─ "What needs validation?" ───► Go to: Scanning for Validation Needs
└─ "Show profile options" ─────► Go to: Profile Management
```
### Step 0 — Locate ADRs and Extract Objectives
1. Check for `docs/adr/` directory. If missing, recommend `author-adr` first.
2. Read the specified ADR(s).
3. Extract **prototype objectives** from:
   - The Evaluation Checkpoint's **Validation needs** section (primary source)
   - The **Additional Quality Concerns** section (secondary source)
   - Per-option **Strengths/Weaknesses** that contain unvalidated claims
4. If no objectives are found, inform the user and offer to help identify what
   needs validation.
### Step 1 — Select Isolation Backend
Choose the environment for running experiments. The backend determines
isolation level and available tooling.
| Backend | Isolation Level | Dependencies | Best For |
|---------|----------------|--------------|----------|
| `worktree` | Git-level (shared filesystem) | `git` | Quick spikes, config experiments, skill changes |
| `container` | OS-level (full isolation) | Container runtime | Database schemas, service interactions, reproducible benchmarks |
| `acp-sandbox` | Agent-level (sub-agent) | ACP-compatible runtime | Parallel experiments, structured observation, A/B comparisons |
**Selection logic:**
1. Check user preference in `preferences.toml` (`[prototype].isolation`)
2. If a profile exists for this ADR in `.adr/profiles/`, use its `isolation` value
3. Otherwise, default to `worktree` (lowest dependency)
4. Verify the selected backend is available (e.g., check for container runtime)
5. Fall back gracefully if unavailable
### Step 2 — Environment Setup
Set up the isolated experiment environment using the selected backend.
**Worktree backend:**
```bash
git worktree add .prototype/<adr-number> HEAD
cd .prototype/<adr-number>
```
**Container backend:**
Read the profile's `image` and `setup` fields. Execute setup commands in the
container.
**Profile-driven setup:** If a TOML profile exists in `.adr/profiles/`, read it
and apply environment configuration. See [Profile Format](references/profiles.md).
**Open-system scenarios:** If the profile declares `requires = "user-intervention"`,
switch to interactive mode — pause at each checkpoint for user action.
### Step 3 — Run Experiments
Execute each prototype objective as a discrete experiment:
1. **State the objective** — what question is this experiment answering?
2. **Describe the method** — what steps will be taken?
3. **Execute** — run the experiment in the isolated environment
4. **Observe** — capture measurements, behavioral notes, and pass/fail results
5. **Record** — log observations as JSONL to stdout
**Observation format (JSONL):**
```json
{"objective": "Validate awk parsing", "result": "pass", "notes": "Parsed 5/5 plan files correctly"}
{"objective": "Measure plan growth", "result": "data", "value": {"5_tasks": "2.1KB", "10_tasks": "4.3KB", "15_tasks": "6.8KB"}}
```
### Step 4 — Report Findings
After all experiments complete:
1. **Summarize findings** — present results for each prototype objective
2. **Assess confidence** — does the evidence support the decision?
   - **Validated** — evidence supports the decision as written
   - **Partially validated** — some aspects confirmed, others need revision
   - **Invalidated** — evidence contradicts the decision; revision recommended
3. **Propose ADR updates** — suggest specific changes to the ADR based on findings:
   - Update option Strengths/Weaknesses with empirical data
   - Add evidence to the Context section
   - Revise consequences if findings contradict stated outcomes
### Step 5 — Teardown
Clean up the experiment environment:
- **Automatic teardown** (default): remove worktree, stop container
- **Manual teardown**: inform the user the environment is still available
```bash
# Worktree cleanup
git worktree remove .prototype/<adr-number>
```
### Step 6 — Feed Back into ADR Lifecycle
1. If findings validate the decision → update Evaluation Checkpoint assessment
   to `Proceed` and offer to transition status from `Prototype` → `Proposed`
2. If findings invalidate → keep status at `Prototype`, recommend revisions
3. Append findings summary to the ADR's Context section as new evidence
## Profile Management
Profiles are declarative TOML files stored in `.adr/profiles/` (per ADR-0020).
They define environment setup, not experiment logic.
See [Profile Format Reference](references/profiles.md) for the full TOML schema
and examples.
## Deep References
- **[Profile Format](references/profiles.md)** — TOML profile schema, examples,
  field reference
- **[Isolation Backends](references/isolation.md)** — Detailed backend comparison,
  setup procedures, fallback logic
- **[Observation Format](references/observation.md)** — JSONL schema, jq
  validation, feedback loop mechanics
