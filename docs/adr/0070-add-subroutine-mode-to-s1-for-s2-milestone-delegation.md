# 70. Add Subroutine Mode to S-1 for S-2 Milestone Delegation

Date: 2026-04-11
Status: Accepted
Last Updated: 2026-04-11
Links:
- Related to [ADR-0068](0068-add-post-solve-retrospective-and-project-preference-feedback-loop-to-solve-adr.md) (C-4 retrospective — added conclusion step that exposed the stale references)

## Context

From UX review of solve-adr (2026-04-11, finding F-06): problem.md unconditionally instructs S-1 to run the full conclusion sequence (C-1 → C-2 → C-3 → C-4) after implementation completes. When S-2 (Roadmap) delegates a milestone to S-1, S-1 runs conclusion after every milestone instead of once at the end. No signal tells S-1 whether it is running standalone or as a sub-routine of S-2. This produces duplicate QA triage, duplicate code reviews, and duplicate reports per milestone — all of which should run once after all milestones complete.

[Source: UX review amendment — solve-adr Redesign verdict, F-06: "Missing standalone vs. sub-routine disambiguation"]

## Options

*Absent by design — S-3 fast-path ADR. The direction is pre-decided.*

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Skipped — S-3 fast-path ADR; decision is pre-decided

## Decision

In the context of S-2 delegating milestones to S-1 as sub-routines, facing the problem that S-1 runs conclusion after every milestone instead of once at the end, we chose to add an operating mode parameter to S-1 that suppresses conclusion when called as a sub-routine, over leaving S-1 unaware of its calling context, to achieve correct conclusion execution (once per solve, not once per milestone), accepting the added complexity of a mode parameter in S-1's entry contract.

**Mechanism:** problem.md Step 4b's conclusion routing instruction becomes conditional:

- **Standalone mode (default):** After implementation completes, proceed to the Conclusion sequence (C-1 → C-2 → C-3 → C-4).
- **Sub-routine mode:** After implementation completes, return control to the caller (S-2). Do not run conclusion — the caller owns conclusion.

S-2's roadmap.md Step 4a must signal sub-routine mode when delegating to S-1. The signal is conveyed in the delegation prompt: "Run S-1 in sub-routine mode — skip conclusion, return control after implementation."

## Consequences

**Positive**
- Conclusion runs once per solve, not once per milestone — eliminates duplicate QA triage, code review, and reports.
- S-1 and S-2 have a clear contract for delegation.

**Negative**
- S-1's entry contract gains a mode parameter. An incorrect or missing signal produces the old behavior (conclusion per milestone), which is fail-safe but wasteful.

**Neutral**
- Standalone S-1 invocations are unaffected — the default is standalone mode.
- S-3 (Fast-Path) is unaffected — it runs conclusion unconditionally and is never called as a sub-routine.

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

Changes are procedure text only — no code. Tooling: `make check-refs` must pass. User documentation: SKILL.md, problem.md, and roadmap.md procedure text updated. roadmap.md: in addition to Step 4a changes, update stale conclusion references at Step 5a and the comparison table to include C-4.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

S-3 fast-path ADR — Options and Evaluation Checkpoint absent by design.

---

## Comments
