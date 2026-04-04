# 30. Add QA plan stage to implement-adr for security and UX validation

Date: 2026-04-04
Status: Planned
Last Updated: 2026-04-04
Links:
- Extends [ADR-0025](0025-add-plan-reviewer-sub-agent-task-to-implement-adr.md) (plan-reviewer sub-agent pattern — QA plan follows the same sub-agent delegation model)
- Related to [ADR-0026](0026-add-rust-cli-for-data-plumbing.md) (the implementation of ADR-0026 surfaced the motivating SQL injection example)

## Context

The `implement-adr` skill currently generates implementation plans with per-task test and acceptance criteria. These criteria are authored from a **developer perspective** — they verify that the code works as designed. A plan-reviewer sub-agent (ADR-0025) validates that the plan covers the source ADR's requirements. This pipeline catches gaps in *what gets built* but not in *how it could go wrong*.

**The problem:** Developer-authored acceptance criteria don't reliably catch security vulnerabilities or user-experience hazards. These concerns require a different lens — one that asks "how could this be misused?" and "how could this crash?" rather than "does this work as designed?"

**Motivating example:** During the authoring of ADR-0029 (view subcommand), an independent evaluation agent recommended using `rusqlite` with dynamic SQL queries — `format!("SELECT * FROM {table_name}")`. This is textbook SQL injection. The vulnerability was caught during ADR review because the human maintainer recognized the pattern. If this had been a plan task executed autonomously, the injection-vulnerable code would have been committed.

The existing plan-reviewer catches *missing* requirements (quality strategy items, consequence traceability). It does **not** evaluate the *security or UX implications* of the implementation approach described in each task. This is the gap between a dev code review and a QA validation.

**The analogy:** In software teams, developers write code and unit tests (what today's plan captures). QA engineers validate the code through a different lens — security, edge cases, crash scenarios, injection vectors, resource exhaustion. This ADR adds the QA role to the agent workflow.

**Why per-stage, not per-task:** QA validation benefits from seeing the full context of a completed stage — how tasks interact, what attack surfaces the combined changes create, and whether the stage as a whole is safe. Per-task QA would be too narrow (missing cross-task interactions) and too expensive (spawning an agent per task).

**Why after the main plan review:** The QA plan is a consumer of the approved implementation plan. It needs to know what will be built (tasks, interfaces, data flows) before it can identify what could go wrong. Generating QA checks against a plan that might be revised is wasted work.

### Decision Drivers

- **Security-by-default** — security validation should be structural, not optional. The executor should not be able to skip QA when running autonomously.
- **Crash prevention** — crashes are a bad user experience. QA should verify graceful error handling, not just happy paths.
- **Stage-level granularity** — QA at the stage level balances thoroughness (sees cross-task interactions) with efficiency (one agent per stage, not per task).
- **Non-blocking authoring** — generating the QA plan should not slow down the main planning workflow. It runs after plan approval.
- **Sub-agent delegation** — follows the established pattern from ADR-0025 (plan-reviewer sub-agent).

## Options

### Option 1: QA plan as a separate document

Generate a `qa-plan.md` file alongside the main `plan.md` after plan review passes. The QA plan contains per-stage validation checks focused on security and UX. During execution, after each stage completes, the executor spawns a general-purpose agent to run that stage's QA checks before auto-committing.

**QA plan structure:**
```markdown
# QA Plan: [Title]

**Source Plan:** docs/plans/0026-0028.0.plan.md
**Generated:** YYYY-MM-DD

## Stage 1: Workspace Scaffolding

### Security Checks
- [ ] No secrets, credentials, or API keys in committed files
- [ ] .gitignore covers sensitive files (.env, database files, build artifacts)
- [ ] Dependencies are pinned to specific versions (no wildcards)

### UX Checks
- [ ] Binary exits cleanly with --help (no panics, exit code 0)
- [ ] Invalid subcommands produce helpful error messages, not stack traces
- [ ] Exit codes follow Unix convention (0 success, non-zero failure)
- [ ] Observability: can a user verify the stage's output without reading source code?

## Stage 2: Database Layer

### Security Checks
- [ ] No user-supplied strings interpolated into SQL queries
- [ ] Database file permissions are restrictive (not world-readable)
- [ ] Migration SQL uses parameterized queries where applicable

### UX Checks
- [ ] Init command handles existing database gracefully (no crash on re-run)
- [ ] Missing directory creates parent directories (no crash on fresh system)
- [ ] Database errors produce human-readable messages, not raw error dumps
- [ ] Observability: can a user verify data was written to the database?
```

**Execution model:**
```
Main plan approved → Generate QA plan
                          │
Execute Stage 1 ──────────┤
  (all tasks complete)    │
         │                ▼
         ├──► QA agent runs Stage 1 checks
         │       │
         │    Pass → auto-commit Stage 1
         │    Fail → pause, report findings
         │
Execute Stage 2 ──────────┤
  (all tasks complete)    │
         │                ▼
         ├──► QA agent runs Stage 2 checks
         ...
```

**Strengths:**
- QA plan is inspectable, reviewable, and version-controlled — the user can read and modify QA checks before execution.
- Clear separation of concerns — dev plan says "what to build," QA plan says "what to verify."
- QA checks are persistent — they can be re-run, referenced, and learned from.
- Follows the existing plan-as-artifact pattern.

**Weaknesses:**
- Two plan files to manage and keep in sync.
- QA plan generation adds a planning step (though it runs after plan approval and can be parallelized with ADR status updates).
- The QA plan may become stale if the main plan is revised — synchronization is manual.

### Option 2: QA sections embedded in the main plan

Instead of a separate file, append QA validation sections to each stage in the existing `plan.md`. Each stage gains a `### QA Validation` subsection with security and UX checks.

```markdown
## Stage 2: Database Layer

### Task 2.1: Create initial migration [medium]
...

### Task 2.2: Define Diesel models [small]
...

### QA Validation (Stage 2)
**Security:**
- [ ] No SQL injection vectors in query construction
- [ ] Database file path is validated (no path traversal)

**UX:**
- [ ] Init handles existing database without crash
- [ ] Error messages are human-readable
```

**Strengths:**
- Single source of truth — dev tasks and QA checks live together.
- No synchronization problem — revising the plan naturally includes QA.
- Easier to see the relationship between tasks and their QA checks.

**Weaknesses:**
- Makes the plan larger and more complex — plan files already have tasks, acceptance criteria, implementation notes, and dependencies.
- Blurs the dev/QA separation — the plan-reviewer sub-agent would need to distinguish between dev criteria and QA criteria.
- QA checks are interleaved with dev tasks, making it harder for the QA agent to extract its scope.
- The plan template must change, affecting all future plans.

### Option 3: QA as runtime-only instructions (no document)

Don't generate a QA plan document. Instead, at stage boundaries, the executor constructs a QA prompt dynamically from the completed stage's code changes and spawns a general-purpose agent with instructions to look for security and UX issues.

**Strengths:**
- Zero planning overhead — no document to generate or maintain.
- QA agent sees the actual code changes, not predicted tasks.
- Always fresh — no staleness problem.

**Weaknesses:**
- QA scope is not inspectable or reviewable before execution.
- No persistence — QA checks are ephemeral, not version-controlled.
- User has no ability to customize or pre-approve QA focus areas.
- Harder to reproduce — re-running QA requires re-executing the prompt.
- Inconsistent — different runs may focus on different things.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — the sub-agent delegation pattern is established (ADR-0025), and the motivating example provides concrete evidence of the gap.

## Decision

In the context of **needing security and UX validation beyond developer-authored acceptance criteria**, facing **the risk that autonomously executed plans can introduce vulnerabilities (like SQL injection) that dev-level tests don't catch**, we decided for **a separate QA plan document generated after main plan approval, with per-stage checks executed by a general-purpose sub-agent at stage boundaries (Option 1)** and neglected **embedded QA sections (Option 2, overloads the plan file) and runtime-only QA (Option 3, not inspectable or reproducible)**, to achieve **a persistent, reviewable QA validation layer that catches security vulnerabilities and crash-inducing bugs before code is committed**, accepting that **this adds a second plan file to manage and introduces a QA generation step after plan approval**.

### Workflow integration

The QA plan inserts into the existing implement-adr workflow after plan review:

```
Read ADR → Generate plan → Plan review → Generate QA plan → Update ADR statuses
                                              │
                                              ▼
                                    Execute plan (per stage):
                                      1. Run stage tasks
                                      2. QA agent validates stage
                                      3. If pass → auto-commit
                                      4. If fail → pause, report
```

### QA plan generation (new Step 4b)

After the plan-reviewer approves the plan (Step 4) and before ADR status updates (Step 5), **spawn a separate general-purpose agent** to generate the QA plan. The main executor (which authored the dev plan) must not write its own QA plan — this is the same separation principle that prevents developers from writing their own QA test plans.

1. **Spawn a general-purpose QA planner agent** with the approved plan, the source ADR(s), and the procedural QA checklists (security items 1–6, UX items 1–7). The agent's role is adversarial: "how could this implementation go wrong?"
2. **The QA planner reads the approved plan** — extracts the stage structure and task descriptions.
3. **Test-gap analysis** — before generating per-stage checks, the QA planner reviews the dev plan's acceptance criteria for blind spots. For each stage, it asks: "are there things the dev tests won't catch?" A test gap is any scenario where all dev acceptance criteria pass but the implementation is still wrong, insecure, or unverifiable. Test-gap findings may result in the QA planner recommending **new tasks or criteria** to be scheduled — the ADR is an incomplete design, and the implementation plan has leeway to make additions when it makes sense. For example: the dev plan tests that `ingest` stores 5 rows, but no test verifies the data can be *read back* — QA recommends a view/inspection capability.
4. **For each stage, the QA planner generates checks** in two categories:
   - **Security** — applying the 6-item security checklist to the stage's specific tasks, interfaces, and data flows.
   - **UX (crash prevention and observability)** — applying the 7-item UX checklist, with particular attention to observability gaps (items 5–7).
5. **The QA planner writes the QA plan** to `docs/plans/<range>.<revision>.qa-plan.md` alongside the main plan.
6. The QA plan references the source plan and uses the same stage numbering.

### QA execution (stage boundary hook)

During plan execution, after all tasks in a stage are complete but before auto-commit:

1. **Spawn a separate general-purpose QA executor agent** with the QA plan's checks for the current stage, plus the actual code changes made during the stage (diff or file list). This must be a different agent from the one that executed the stage's tasks — the agent that wrote the code must not QA its own work.
2. **The QA executor reviews** the actual implementation against the QA checks.
3. **If all checks pass** — mark them `[x]` in the QA plan, proceed to auto-commit.
4. **If any check fails** — pause execution, report findings to the main executor, and request remediation before committing.

### QA focus areas

The QA plan concentrates on two domains, each with a procedural checklist the QA agent applies to every stage:

**Security — for each stage, verify:**
1. No user-supplied strings are interpolated into SQL, shell commands, or file paths (injection)
2. No secrets, credentials, or API keys appear in committed files
3. No deserialization of untrusted input without validation
4. Dependencies are pinned to specific versions (no wildcards)
5. File permissions on created artifacts are not overly permissive
6. Any new external input surface has validation at the boundary

**UX (crash prevention and observability) — for each stage, verify:**
1. Every error path produces a human-readable message on stderr (no raw panics, no stack traces)
2. Every user-facing command exits with code 0 on success, non-zero on failure
3. Invalid input is rejected with a helpful message, not a crash
4. Resources (file handles, database connections) are cleaned up on error paths
5. If the stage writes data, there is a way to read it back or verify it was written
6. If the stage creates state (files, schema, config), there is a way to inspect the new state
7. A user who did not write the code can verify the stage's output through the tool's own interface

Items 5–7 are the **observability check**: a stage that produces unverifiable output is a QA finding. The resolution may be a documentation note, a diagnostic command, or (as with the `view` subcommand gap) a recommendation for a new feature.

### What the QA plan is NOT

- **Not a replacement for dev acceptance criteria** — dev criteria verify "does it work," QA verifies "can it break." Test-gap findings supplement dev criteria, they don't replace them.
- **Not a comprehensive security audit** — it catches common vulnerability patterns, not sophisticated attacks.
- **Not blocking plan generation** — the QA plan is generated after plan approval, not during.
- **Not limited to checking** — the QA planner can recommend new work (tasks, features, documentation) when test-gap analysis reveals blind spots in the original plan. These recommendations are surfaced to the main executor for scheduling.

### QA finding eligibility — quality concern vs. preference

Not all QA findings justify scheduling new work. The QA planner and executor must distinguish between quality concerns and preferences:

**Eligible for scheduling (quality concerns):**
- Security vulnerabilities — injection, credential exposure, permission issues
- Crash-inducing gaps — unhandled errors, resource leaks, missing validation
- Observability gaps — no way to verify that a stage's output is correct
- UX violations — output format that prevents the intended audience from using the tool effectively (e.g., a pipeline tool that produces non-pipeable output)

**Not eligible — defer to follow-up iterations (preferences):**
- Aesthetic or ergonomic suggestions — "this should have a fancy table view," "add color output," "use a different flag name"
- Feature requests beyond the minimum needed to close the quality concern
- Opinions about implementation approach that don't affect security or UX

**The boundary case — UX-grounded design feedback:** A QA finding that *looks* like a preference may actually be a quality concern when it affects the intended user's ability to use the tool. Example: if a pipeline tool only supports formatted table output (not pipeable), a QA finding saying "this should be awk-friendly" is a legitimate UX concern — the output format violates the tool's design intent and its users' workflows. The test: **does the finding affect a user's ability to accomplish the task the tool was designed for, or does it just make the tool nicer?** If the former, it's a quality concern. If the latter, it's a preference.

This distinction matters for autonomous execution. When a QA finding recommends new work, the main executor must apply this gate before scheduling it. The minimum implementation that closes the quality concern is what gets scheduled — iteration handles the rest.

## Consequences

**Positive:**
- Security vulnerabilities like SQL injection are flagged for review before code is committed — the procedural checklists catch common vulnerability patterns, though LLM-based review is not infallible.
- Crash-inducing edge cases (missing error handling, resource leaks) are validated per-stage.
- QA plan is persistent and reviewable — the user can inspect, modify, or extend QA checks before execution.
- Follows the established sub-agent pattern (ADR-0025) — consistent with the existing plan-reviewer architecture.
- Autonomous execution mode gains a safety net — even fully autonomous plans pass through QA validation.
- QA checks accumulate project-specific patterns over time — common security checks can be templated.

**Negative:**
- Adds a second plan file (`qa-plan.md`) alongside `plan.md` — two artifacts to manage per implementation.
- QA plan generation adds processing time after plan approval (mitigated by parallelizing with ADR status updates).
- QA agent at stage boundaries adds execution time (~2-3 minutes per stage). For plans with many stages, this accumulates.
- The QA plan may become stale if the main plan is revised after QA generation. The QA plan must be regenerated on plan revision.

**Neutral:**
- QA plan generation is mandatory — it runs for every plan, regardless of participation mode. This matches the plan-reviewer's mandatory status (ADR-0025) and enforces the "security-by-default" decision driver. There is no opt-out configuration.
- The QA plan template and focus areas will evolve as more vulnerability patterns are encountered. The initial focus on SQL injection and crash prevention is a starting point.
- Whether QA failures in autonomous mode should auto-remediate or always pause is a UX decision to make during implementation.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [x] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

- **Backwards compatibility with existing plans** — the QA plan is additive. Existing plans without QA plans should execute normally. The QA stage boundary hook must be a no-op when no QA plan exists.
- **QA plan template** — a reference template should be created in `implement-adr/assets/templates/` alongside the existing plan template.
- **Skill documentation** — the implement-adr SKILL.md must document the new Step 4b and the stage boundary hook.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR focuses on the architectural decision (separate QA plan, stage-boundary execution). The specific QA check taxonomy (which security patterns to check, which UX patterns to verify) will evolve during implementation. The initial implementation should start with the patterns identified in the motivating example (SQL injection, crash handling) and grow from there.

---

## Comments
