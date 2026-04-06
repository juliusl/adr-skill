# 46. solve-adr Creates Feature Branches for End-to-End Workflows

Date: 2026-04-06
Status: Planned
Last Updated: 2026-04-06
Links: ADR-0010 (auto-commit), ADR-0044 (solve-adr skill)

## Context

solve-adr orchestrates multi-phase workflows: author → triage → implement. These workflows produce ADR files (documentation) and code changes (implementation). All output currently lands on whatever branch is checked out.

This creates two problems:
1. solve-adr's changes mix with the user's in-progress work.
2. There is no natural PR boundary for reviewing the complete problem-solving output.

implement-adr is branch-agnostic by design — it commits to the current branch (per ADR-0010). Branch management belongs to the orchestrator (solve-adr) because it knows the workflow boundaries: when a problem starts, when it ends, and what artifacts it produces.

## Options

### Option A: Single feature branch per problem

solve-adr creates one branch from current HEAD after problem intake (S-1 Step 1). All subsequent phases — authoring, triage, implementation — work on this branch. The branch name is derived from the problem: `solve/<problem-slug>`.

**Flow:**
1. S-1 Step 1: Intake (on current branch)
2. Create branch `solve/<problem-slug>` from HEAD
3. S-1 Steps 2–5: Author, triage, implement, report (on feature branch)
4. User reviews via PR and merges

**Strengths:**
- Simple mental model: one problem = one branch = one PR
- Compatible with implement-adr's auto_commit — commits land on the feature branch
- Clean isolation from user's working branch

**Weaknesses:**
- Large PRs for multi-ADR problems
- Branch contains both draft ADRs and implementation code

### Option B: Branch per implementation group

solve-adr creates separate branches for each group in S-1 Step 4. Authoring happens on the current branch. Implementation groups each get their own branch and PR.

**Strengths:**
- Smaller, focused PRs per implementation group
- ADR documentation lands on the primary branch immediately

**Weaknesses:**
- More complex branch management (multiple branches to track)
- ADR files committed before implementation — "Proposed" ADRs on main with no implementation yet
- Requires merge coordination between groups

### Option C: No branch management (status quo)

User manages branches manually before invoking solve-adr.

**Strengths:**
- No new complexity
- User has full control

**Weaknesses:**
- Doesn't solve the isolation problem
- User must remember to create a branch before every solve invocation

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed — all options evaluated at comparable depth, decision drivers are clear.

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — branching is well-understood Git workflow.

## Decision

We will create a single feature branch per problem in solve-adr.

In the context of orchestrating multi-phase workflows that produce both documentation and code changes, facing the need to isolate solve-adr's output from the user's working branch, we chose a single feature branch per problem over per-group branching or no management, to achieve a clean one-problem-one-PR workflow with minimal orchestration complexity, accepting that some PRs may be large for multi-ADR problems.

**Branch lifecycle:**

1. **Create** — after S-1 Step 1 (intake), create `solve/<problem-slug>` from current HEAD. This is intentional — the solve branch inherits the user's current branch context. If the user is on a feature branch, the solve branch includes that work.
2. **Switch** — checkout the new branch. All subsequent work happens there.
3. **Work** — authoring, triage, and implementation all commit to this branch.
4. **Complete** — after S-1 Step 5 (report), stay on the branch. The user reviews and merges via PR.
5. **Resume** — on resume, if the branch exists and is unmerged, checkout and continue. If the branch has been merged or deleted, treat the solve as completed — warn the user and do not recreate the branch.

**Branch naming:** `solve/<problem-slug>` where the slug is derived from the problem statement (lowercase, hyphenated, max 50 chars). Example: `solve/branch-management-user-mode-guidance`. The branch name is recorded in solve session state at creation time. Resume retrieves the stored name rather than re-deriving, so slug generation does not need to be deterministic across sessions.

**Dirty working tree:** If the working tree has uncommitted changes when branch creation is attempted, warn the user and ask them to commit or stash before proceeding. solve-adr does not stash automatically — that risks losing user work.

**No new preference key.** Branching is default behavior for solve-adr's e2e workflow. Users who want manual branch control can use author-adr and implement-adr directly.

## Consequences

- **Positive:** Clean isolation — solve-adr's output is on its own branch, not mixed with user's work.
- **Positive:** Natural PR boundary — one problem = one PR for review.
- **Positive:** Compatible with implement-adr's auto_commit — commits land on the feature branch.
- **Positive:** Resume-friendly — branch name is recorded in session state and retrievable on resume.
- **Negative:** Large PRs for complex problems with many ADRs and implementation groups.
- **Negative:** Branch creation adds a git operation that can fail (permissions, hooks, dirty tree).
- **Neutral:** Users who want smaller PRs can split the branch after completion, though this requires non-trivial Git expertise (cherry-picking, interactive rebase).
- **Neutral:** implement-adr remains branch-agnostic — no changes needed there.

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

- solve-adr's SKILL.md and references/problem.md need updates to describe branching behavior at each lifecycle step
- README should mention that solve-adr creates feature branches

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
solve-adr should create Git branches when orchestrating the full e2e workflow. implement-adr stays branch-agnostic — it works on whatever branch it's on. solve-adr is the orchestrator, so it owns the branch lifecycle.

**Tolerance:**
- Risk: Low — branching is well-understood, no experimental approaches needed
- Change: Low — adds behavior to solve-adr only, no changes to other skills
- Improvisation: Low — the pattern is standard Git workflow

**Uncertainty:**
- Certain: branches provide isolation and PR boundaries
- Certain: implement-adr should remain branch-agnostic
- Uncertain: whether branching should be opt-in or default (resolved: default)

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Single branch per problem
- Branch per implementation group
- No management (status quo)
