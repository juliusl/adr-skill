# 59. Add optional code review hook to solve-adr branch completion

Date: 2026-04-07
Status: Planned
Last Updated: 2026-04-07
Links: ADR-0046 (feature branches), ADR-0031 (dispatch hooks)

## Context

solve-adr creates feature branches (ADR-0046) to isolate problem-solving output. The current lifecycle is: intake → branch → author → triage → implement → report. After implementation completes (Step 4), the workflow jumps to Step 5 (Report) and stays on the branch. The user then creates a PR for review and merges manually.

During dogfooding of the roadmap workflow on the `solve/adr-atelier/milestone-1` branch, a gap emerged: the accumulated code changes on the solve branch are never reviewed within the solve workflow itself. The PR review happens outside the orchestration — after the solve is declared complete. This means:

1. **No feedback loop.** Code review findings discovered during PR review cannot be addressed within the solve workflow's quality safeguards (QA plans, commit tracking, test validation).
2. **Late discovery.** Issues found during PR review may require re-invoking implement-adr outside the solve context, losing the orchestration benefits.
3. **Inconsistent quality gate.** solve-adr enforces QA via implement-adr (per P-4), but code review — a distinct quality concern from QA — has no equivalent gate.

A code review agent persona (`juliusl-code-reviewer-v1`) has been created and installed. The question is how solve-adr should optionally integrate code review into its lifecycle.

**Decision drivers:**
- The hook must be optional — not all projects use code review agents
- Configuration should follow the existing dispatch pattern (ADR-0031)
- The review scope is the cumulative diff of the solve branch against its base
- Findings must be actionable within the solve workflow, not deferred to PR

## Options

### Option A: Add Step 4d — Code Review between Implement and Report

Insert a new step after implementation completes but before the report. The step dispatches a configured code review agent to review the cumulative diff of the solve branch.

**Flow:**
```
4. Implement — group accepted ADRs, delegate to /implement-adr
   ↓
4d. Code Review — dispatch configured agent to review branch diff
   ↓
5. Report — summarize what was implemented, what remains
```

**Configuration:**
```toml
[solve.dispatch]
code_review = "juliusl-code-reviewer-v1"  # agent reference, or absent to skip
```

When `[solve.dispatch].code_review` is absent or empty, Step 4d is skipped entirely.

**Mechanics:**
1. Compute the cumulative diff: `git diff $(git merge-base HEAD <base-branch>)..HEAD`
2. Dispatch the configured code review agent with the diff
3. The agent produces findings at priority levels (high, medium, nit)
4. High-priority findings trigger remediation before proceeding to Step 5

**Finding handling:**
- In autonomous mode: address high-priority findings inline (fix and commit), log medium as accepted
- In guided mode: present findings to the user for triage

**Strengths:**
- Clean lifecycle separation — code review is a distinct step, not mixed into reporting
- Follows the dispatch pattern from ADR-0031 — same configuration style
- Findings are actionable within the solve workflow — remediation happens on the solve branch with commit tracking
- Optional by default — no behavior change for users who don't configure it

**Weaknesses:**
- Adds another step to an already multi-step workflow
- The code review agent may produce findings that conflict with QA plan decisions from implement-adr

### Option B: Embed code review in Step 5 (Report)

Make the report step optionally include a code review pass before generating the summary.

**Strengths:**
- No new step — reuses existing workflow structure
- Report naturally includes review findings

**Weaknesses:**
- Conflates two responsibilities (summarizing vs. reviewing) in one step
- If review findings require remediation, the report step becomes iterative — it loses its terminal character
- Harder to skip cleanly — the report step is always mandatory

### Option C: No change — rely on PR review

Leave code review to the PR workflow. The user creates a PR and uses their platform's review tools.

**Strengths:**
- No changes to solve-adr
- PR review is a well-understood workflow

**Weaknesses:**
- Does not solve the gap — findings discovered during PR review are outside the solve workflow's quality safeguards
- Late discovery of issues requires re-invoking implement-adr outside the solve context
- Inconsistent with solve-adr's philosophy of orchestrating the full problem-solving lifecycle end-to-end

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed — all options evaluated at comparable depth, decision drivers are clear.

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — dispatch hooks and branch diffs are well-understood patterns already used in this project.

## Decision

We will add Step 4d (Code Review) as an optional step between implementation and reporting in solve-adr's problem lifecycle.

In the context of orchestrating multi-phase problem-solving workflows that produce code changes on a feature branch, facing the gap that accumulated code changes are never reviewed within the solve workflow, we chose to add an optional code review step (4d) dispatched via `[solve.dispatch].code_review` over embedding review in the report step or relying on PR review, to achieve an actionable feedback loop within the solve workflow's quality safeguards, accepting that this adds another step to the lifecycle.

**Configuration:**

```toml
[solve.dispatch]
code_review = "juliusl-code-reviewer-v1"  # or any agent reference
```

The key lives under `[solve.dispatch]` following the dispatch pattern established by ADR-0031 for `[author.dispatch]`. When absent, Step 4d is skipped — no behavior change for existing users.

**Step 4d mechanics:**

1. **Determine base branch** — retrieve the branch that the solve branch was created from. Use `git merge-base HEAD <base-branch>` to find the common ancestor. The base branch is recorded in session state when the solve branch is created (Step 1b).
2. **Dispatch code review agent** — invoke the configured agent via the `task` tool with agent type matching the configured agent reference. Provide the cumulative diff scope: all changes from merge-base to HEAD.
3. **Collect findings** — the agent produces findings at priority levels per its persona.
4. **Triage findings:**
   - **Autonomous mode:** Address high-priority findings (fix and commit on the solve branch). Accept medium-priority findings with inline rationale. Log nit findings.
   - **Guided mode:** Present findings to the user for triage. The user decides which to address, accept, or defer.
5. **Gate:** If high-priority findings remain unaddressed after triage, do not proceed to Step 5. Log the block and present to the user.

**Base branch tracking:**

Step 1b (branch creation) already records the branch name in session state. This decision extends that: also record the base branch (the branch checked out when `solve/<slug>` was created). On resume, both are retrievable from session context.

## Consequences

- **Positive:** Code review findings are actionable within the solve workflow — remediation commits land on the solve branch with commit-level tracking via git history.
- **Positive:** Optional by default — no behavior change for users who don't configure a code review agent.
- **Positive:** Follows the established dispatch pattern (ADR-0031) — consistent configuration UX.
- **Positive:** The code review agent's persona shapes the review priorities, making it customizable per project or user.
- **Negative:** Adds another step to the lifecycle. For large solve branches, the code review pass may produce many findings that extend the workflow.
- **Negative:** Code review findings may overlap or conflict with QA plan findings from implement-adr. During triage, duplicate or conflicting findings are resolved by the triage actor (user in guided mode, agent in autonomous mode) — no automated deduplication is in scope for this decision.
- **Neutral:** The code review agent is a custom agent persona — solve-adr does not prescribe what the agent checks. The quality of the review depends on the configured persona.
- **Neutral:** Step 4d runs after all implementation groups complete. Per-group code review is not supported — the review covers the full branch diff. This is intentional: per-group review could miss cross-group interactions.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

- solve-adr SKILL.md needs lifecycle diagram update (add Step 4d)
- references/problem.md needs Step 4d detail and finding-triage procedure
- Configuration section in SKILL.md needs `[solve.dispatch]` documentation
- The code review agent dispatch must handle agent resolution failure gracefully (warn and skip, per ADR-0031 fallback pattern)

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** None.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
solve-adr's branch lifecycle (ADR-0046) has no code review step before the branch is RI'ed into its base. During dogfooding on the `solve/adr-atelier/milestone-1` branch, the user discovered that code changes accumulate without review within the solve workflow. A code review agent persona (`juliusl-code-reviewer-v1`) exists and is installed. The user wants solve-adr to optionally dispatch it before branch completion.

**Tolerance:**
- Risk: Low — dispatch hooks and branch diffs are existing patterns
- Change: Low — adds an optional step, no existing behavior changes
- Improvisation: Low — follows established dispatch pattern from ADR-0031

**Uncertainty:**
- Certain: the hook should be optional and configurable
- Certain: it should review the cumulative branch diff
- Certain: findings must be actionable within the solve workflow
- Uncertain: how to handle overlap between code review findings and QA findings (resolved: triage handles it)

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Step 4d between implement and report (recommended)
- Embed in report step
- No change (status quo)

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: How should the triage step handle overlap between code review and QA findings?
**Addressed** — Replaced "the triage step must handle this gracefully" with an explicit mechanism: duplicate or conflicting findings are resolved by the triage actor (user or agent), no automated deduplication in scope.

### Q: What does "full tracking" mean for remediation commits?
**Addressed** — Replaced "full tracking" with "commit-level tracking via git history" to precisely describe the tracking mechanism (git commit history, which is inherent to commits on the solve branch).

### Q: Should "would miss" be softened to "could miss" for per-group review?
**Addressed** — Changed "would miss" to "could miss" for precision. Per-group review wouldn't necessarily miss all cross-group interactions, but could miss those that span groups.

<!-- Review cycle 1 — 2026-04-07 — Verdict: Accept. 3 addressed, 0 deferred, 0 rejected. -->
