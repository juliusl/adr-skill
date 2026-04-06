# Implementation Plan: [Title]

**Source ADRs:**
- [ADR-NNNN: Title](docs/adr/NNNN-title.md)

**Generated:** YYYY-MM-DD
**Location:** `docs/plans/<adr-range>.<revision>.plan.md`
**Revision:** 0

<!-- For revisions > 0, replace the Revision line above with: -->
<!-- **Revision:** N (previous: [<adr-range>.<N-1>.plan.md](docs/plans/<adr-range>.<N-1>.plan.md)) -->
<!-- **Changes:** <summary of requested changes> -->

---

## Task Execution Protocol

When executing any task from this plan, follow this protocol:

1. **Before starting a task** — read the task's **Test & Acceptance Criteria** section in full.
2. **As each criterion is satisfied** — immediately edit this plan file to mark the checkbox from `- [ ]` to `- [x]`.
3. **After completing all implementation work** for a task — review the criteria list one final time. Any criterion still unchecked is either incomplete work (go back and finish it) or an oversight (check it off with a brief inline note if it was already satisfied by prior work).
4. **Do not batch checkbox updates** to the end of a stage — update them incrementally so that progress is visible in the file at all times.

---

## Stage 1: [Phase Name]

### Task 1.1: [Task Title] [small|medium|heavy]

**Description:**
<!-- What needs to be done, scoped to this task only. -->

**Implementation Notes:**
<!-- Optional: code snippets, interface sketches, pseudocode. -->
<!-- Include only if the executing agent is qualified to write code. -->

**Test & Acceptance Criteria:**
- [ ] <!-- Specific test type and measurable criteria -->
- [ ] <!-- e.g., "Unit tests for all public methods (happy path + 3 edge cases)" -->
- [ ] <!-- e.g., "Fuzz test with 1000 randomized inputs; no panics" -->

**Dependencies:** None
<!-- Or: Task 1.1 -->

**ADR Reference:** ADR-NNNN, Decision §[section]

---

### Task 1.2: [Task Title] [small|medium|heavy]

**Description:**

**Implementation Notes:**

**Test & Acceptance Criteria:**
- [ ]

**Dependencies:** Task 1.1

**ADR Reference:** ADR-NNNN, Decision §[section]

---

## Stage 2: [Phase Name]

### Task 2.1: [Task Title] [small|medium|heavy]

**Description:**

**Implementation Notes:**

**Test & Acceptance Criteria:**
- [ ]

**Dependencies:** Stage 1

**ADR Reference:** ADR-NNNN, Decision §[section]

---

<!-- Add more stages and tasks as needed. -->
<!-- Rules: -->
<!-- - 2-5 tasks per stage -->
<!-- - Each task is self-contained -->
<!-- - Every task has test criteria and a cost estimate -->
<!-- - Every task references the ADR section that drives it -->

## Stage N: Finalize

### Task N.1: Update ADR status to Accepted    [small]

**Description:** Update the status of each source ADR from `Planned` to `Accepted` to reflect that the decision has been fully implemented.

**Files to update:**
<!-- List each source ADR file: -->
- `docs/adr/XXXX-<title>.md` — change `## Status` from `Planned` to `Accepted`

**Test & Acceptance Criteria:**
- [ ] Each source ADR status reads `Accepted`
- [ ] No other ADR content is modified

**Dependencies:** All prior stages

**ADR Reference:** ADR-0003, Decision §2

---

## Summary

| Stage | Task | Cost | Dependencies |
|-------|------|------|--------------|
| 1. [Phase] | 1.1 [Title] | small | — |
| 1. [Phase] | 1.2 [Title] | medium | 1.1 |
| 2. [Phase] | 2.1 [Title] | medium | Stage 1 |
| N. Finalize | N.1 Update ADR status to Accepted | small | All prior |

**Total estimated cost:** X small, Y medium, Z heavy

---

## Decision Gaps

<!-- If any gaps were detected, list them here: -->
<!-- 1. **[Gap]** — [Explanation]. _Recommend: author ADR on [topic]._ -->
<!-- Remove this section if no gaps were found. -->
