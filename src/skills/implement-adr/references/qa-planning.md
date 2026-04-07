# QA Planning Protocol

Self-contained reference for QA plan generation and execution. Read this file when generating a QA plan after the main implementation plan has been approved, or when executing QA checks at stage boundaries.

## When to Use

After the plan-reviewer approves the implementation plan (Step 4), the main executor spawns a **separate general-purpose QA planner agent** to generate the QA plan. This is Step 4b in the implement-adr workflow.

QA plan generation is **mandatory** — it runs for every plan, regardless of participation mode. There is no opt-out.

## Agent Separation Principle

Three distinct agent roles ensure no agent reviews its own work:

| Role | Agent | Responsibility |
|------|-------|----------------|
| **Main executor** | The orchestrating agent | Generates the dev plan, coordinates execution, remediates QA findings |
| **QA planner** | Separate general-purpose agent | Generates the QA plan (adversarial: "how could this go wrong?") |
| **QA executor** | Separate general-purpose agent (per batch) | Validates completed stage(s) code against QA checks |

The main executor must not write its own QA plan, and the agent that executes stage tasks must not QA its own work.

## Security Checklist

For each stage, verify:

1. No user-supplied strings are interpolated into SQL, shell commands, or file paths (injection)
2. No secrets, credentials, or API keys appear in committed files
3. No deserialization of untrusted input without validation
4. Dependencies are pinned to specific versions (no wildcards)
5. File permissions on created artifacts are not overly permissive
6. Any new external input surface has validation at the boundary

## UX Checklist

For each stage, verify:

1. Every error path produces a human-readable message on stderr (no raw panics, no stack traces)
2. Every user-facing command exits with code 0 on success, non-zero on failure
3. Invalid input is rejected with a helpful message, not a crash
4. Resources (file handles, database connections) are cleaned up on error paths
5. If the stage writes data, there is a way to read it back or verify it was written
6. If the stage creates state (files, schema, config), there is a way to inspect the new state
7. A user who did not write the code can verify the stage's output through the tool's own interface

Items 5–7 are the **observability check**: a stage that produces unverifiable output is a QA finding. The resolution may be a documentation note, a diagnostic command, or a recommendation for a new feature.

## What the QA Plan is NOT

- **Not a replacement for dev acceptance criteria** — dev criteria verify "does it work," QA verifies "can it break." Test-gap findings supplement dev criteria, they don't replace them.
- **Not a comprehensive security audit** — it catches common vulnerability patterns, not sophisticated attacks.
- **Not blocking plan generation** — the QA plan is generated after plan approval, not during.
- **Not limited to checking** — the QA planner can recommend new work (tasks, features, documentation) when test-gap analysis reveals blind spots. Recommendations are surfaced to the main executor for remediation, classified by the finding disposition rules.

## Regeneration on Plan Revision

When the main implementation plan is revised (new revision created per Step 7), the QA plan **must be regenerated**. A stale QA plan that validates against an outdated plan is a silent failure — the QA checks may not cover the revised tasks.

The main executor is responsible for triggering QA plan regeneration when a plan revision occurs.

## Procedure

| ID | Description |
|----|-------------|
| QA-1 | QA Plan Generation — produce per-stage security and UX checks |
| QA-1a | Input — what the QA planner receives |
| QA-1b | Process — 5-step generation procedure |
| QA-1c | Output — qa-plan.md file structure |
| QA-2 | Test-Gap Analysis — find blind spots in dev acceptance criteria |
| QA-2a | Example — illustrates a test gap |
| QA-3 | Finding Disposition — classify findings and determine remediation approach |
| QA-3a | Quality concerns — remediate before finalization |
| QA-3b | Low-severity findings — remediate with minimal implementation |
| QA-3c | Boundary test — classification decision rule |
| QA-4 | QA Execution — stage boundary validation |
| QA-4a | Enforcement — mandatory regardless of participation mode |
| QA-4b | Stage Boundary Hook — spawn QA executor agent per batch |
| QA-4c | Documenting Accepted Findings — rationale for won't-fix items |
| QA-4d | Backwards Compatibility — handle plans without QA plans |
| QA-5 | Prompt Templates — prompts for QA planner and executor agents |
| QA-5a | QA Planner Agent Prompt |
| QA-5b | QA Executor Agent Prompt |

```
QA-1 — QA Plan Generation
  ↓
QA-2 — Test-Gap Analysis
  ↓
QA-3 — Finding Eligibility Gate
  ↓
QA-4 — QA Execution
  ↓
QA-5 — Prompt Templates
```

## QA-1: QA Plan Generation

### QA-1a: Input

The QA planner receives:
1. The approved implementation plan (full content)
2. The source ADR(s) (full content)
3. This reference document (procedural checklists and protocols)

### QA-1b: Process

**All five steps must be executed in order. If a step is skipped, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

1. **Read the approved plan** — extract the stage structure and task descriptions.
2. **Test-gap analysis** — before generating per-stage checks, review the dev plan's acceptance criteria for blind spots (see [QA-2: Test-Gap Analysis](#qa-2-test-gap-analysis)).
3. **For each stage, generate checks** in two categories:
   - **Security** — apply the [6-item security checklist](#security-checklist) to the stage's specific tasks, interfaces, and data flows.
   - **UX (crash prevention and observability)** — apply the [7-item UX checklist](#ux-checklist), with particular attention to observability gaps (items 5–7).
4. **Classify findings** using the [QA-3: Finding Disposition](#qa-3-finding-disposition).
5. **Write the QA plan** to `docs/plans/<range>.<revision>.qa-plan.md` alongside the main plan. Use the [QA plan template](../assets/templates/qa-plan-template.md). Present all findings in a single flat Recommendations table with a Classification column — do not split into separate subsections.

### QA-1c: Output

A `qa-plan.md` file with:
- Per-stage security and UX checks (checkboxes)
- Test-gap findings with concrete examples
- A flat Recommendations table classifying each finding as quality concern or low-severity

## QA-2: Test-Gap Analysis

Before generating per-stage checks, the QA planner reviews the dev plan's acceptance criteria for blind spots. For each stage, ask: **"are there things the dev tests won't catch?"**

A test gap is any scenario where all dev acceptance criteria pass but the implementation is still wrong, insecure, or unverifiable.

Test-gap findings may result in the QA planner recommending **new tasks or criteria** to be scheduled — the ADR is an incomplete design, and the implementation plan has leeway to make additions when it makes sense.

### QA-2a: Example

The dev plan for an `ingest` command has acceptance criteria:
- "5 valid JSONL lines ingested, 5 rows in database"
- "Malformed JSON lines produce error on stderr"

Test-gap analysis reveals: these tests verify ingest *works*, but there's no test for *verifying ingested data*. If the data is silently corrupted (wrong columns, truncated values), all dev tests pass. QA recommends a view/inspection capability to close the observability gap.

## QA-3: Finding Disposition

All QA findings must be addressed. The QA planner classifies findings to guide the remediation approach, not to filter out work.

### QA-3a: Quality concerns — remediate before finalization

- Security vulnerabilities — injection, credential exposure, permission issues
- Crash-inducing gaps — unhandled errors, resource leaks, missing validation
- Data integrity gaps — silent data loss, missing validation on write paths, incorrect state transitions
- Observability gaps — no way to verify that a stage's output is correct
- UX violations — output format that prevents the intended audience from using the tool effectively
- Regression risk — missing tests for known failure modes or edge cases the code handles implicitly

Quality concerns must be remediated before the milestone is marked complete. "Eligible for scheduling" means "is scheduled" — not "may be scheduled later."

### QA-3b: Low-severity findings — remediate with minimal implementation

- Cosmetic improvements that don't affect correctness (e.g., output formatting)
- Edge cases handled by dependencies but lacking explicit regression tests
- Hardening measures for unlikely scenarios

Low-severity findings are addressed with the minimum implementation that closes the gap. If a finding cannot be addressed in the current scope, it must be deferred per P-3 in the SKILL.md policies — logged in the QA plan and surfaced to solve-adr for triage.

### QA-3c: Boundary test

When classifying a finding, apply this test: **If this finding were a bug report from a user, would it be closed as "won't fix"?** If no — it's a quality concern. If yes — it's low-severity, but still gets the minimum fix.

## QA-4: QA Execution

### QA-4a: Enforcement

QA execution at stage boundaries is **mandatory regardless of participation mode** — including autonomous mode. Generating a QA plan but skipping execution defeats the purpose of the QA separation principle. If QA execution is skipped for any stage, log the justification inline before proceeding. Skipping without justification is a workflow violation.

In autonomous mode with multiple stages, adjacent small-only stages may be batched for a single QA pass. The batching decision uses the plan's cost estimates (`[small]`/`[medium]`/`[heavy]`):

| Condition | Action |
|-----------|--------|
| Stage contains any `[medium]` or `[heavy]` task | QA immediately |
| Stage contains only `[small]` tasks and next stage is also all `[small]` | Defer — batch with subsequent stages |
| Stage contains only `[small]` tasks and next stage has `[medium]`/`[heavy]` | QA immediately — validate batch before heavier work begins |
| Final stage in the plan | QA immediately |
| 3+ stages accumulated without QA | QA immediately — cap batch size |

Batching consolidates QA execution while preserving coverage. Skipping QA entirely is not an acceptable optimization.

### QA-4b: Stage Boundary Hook

During plan execution, after all tasks in a stage complete, evaluate the batching decision (see QA-4a). When QA triggers:

1. **Spawn a separate general-purpose QA executor agent** with:
   - The QA plan's checks for all stages in the current batch
   - The actual code changes made during the batched stages (combined diff or file list)
   - This reference document for context
2. **The QA executor reviews** the actual implementation against the QA checks. The executor must report **PASS/FAIL per individual check** — summary-level verdicts without per-check evidence are insufficient.
3. **If all checks pass** — mark them `[x]` in the QA plan and update the Recommendations table: set each finding's Status from `Open` to `✅ Resolved` when the corresponding verification passed. Proceed to auto-commit.
4. **If any check fails** — pause execution, report findings to the main executor, and request remediation before committing.

### QA-4c: Documenting Accepted Findings

When QA findings are accepted without remediation (e.g., low-risk gaps deemed acceptable for the current scope), the main executor **must** document the rationale in the QA plan file. Undocumented acceptances are silent gaps — a future reader cannot distinguish "we evaluated this and decided not to fix it" from "we missed this."

For each finding that won't be fixed:

1. Update the finding's Status column in the Recommendations table to `Won't Fix`.
2. Add an entry under a **Won't Fix — rationale** heading below the table explaining why. Include:
   - Why the risk is low enough to accept
   - What existing mechanisms mitigate the gap (e.g., shared error handling, consistent patterns across the codebase)
   - Under what conditions the finding should be revisited

### QA-4d: Backwards Compatibility

If no QA plan exists (plans generated before this feature), the stage boundary hook is a no-op — execution proceeds to auto-commit directly.

## QA-5: Prompt Templates

### QA-5a: QA Planner Agent Prompt

```
You are a QA planner agent. Your role is adversarial: "how could this
implementation go wrong?"

You must generate a QA plan for the following implementation plan.

## Procedural Checklists

[Insert Security Checklist items 1-6 from this reference]
[Insert UX Checklist items 1-7 from this reference]

## Test-Gap Analysis

[Insert Test-Gap Analysis section from this reference]

## Finding Disposition

[Insert Finding Disposition section from this reference]

## Source Material

### Source ADR(s)
[Insert full ADR content]

### Approved Implementation Plan
[Insert full plan content]

## Output

Write a QA plan using the qa-plan-template.md structure. For each stage:
1. Apply the security checklist (6 items)
2. Apply the UX checklist (7 items)
3. Note any test-gap findings
4. Classify all findings using the disposition rules
```

### QA-5b: QA Executor Agent Prompt

```
You are a QA executor agent. Review the actual implementation of
completed stage(s) against the QA plan's checks.

You must not have been the agent that executed the tasks.

## QA Checks for This Batch

[Insert the QA plan's checks for all stages in this batch]

## Code Changes

[Insert combined diff or file list from the completed stage(s)]

## Instructions

1. For each numbered check, verify against the actual code.
2. Report PASS or FAIL per individual check with specific evidence.
3. Do not summarize multiple checks into a single verdict.
4. Mark passing checks [x] in the QA plan.
5. For failing checks, report the specific code/file and the violation.
6. Apply the finding disposition rules to any new findings.

## Output Format

- PASS/FAIL per check with specific file and line evidence
- Any new findings with disposition classification
- Verdict: Batch Approved / Batch Needs Remediation
```
