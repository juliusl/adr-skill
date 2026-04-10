# QA Plan Template

QA plan document template. Used by the QA planner agent (ADR-0030) alongside the implementation plan template.

## Template

```markdown
# QA Plan: [Title]

**Source Plan:** docs/plans/<range>.<revision>.plan.md
**Generated:** YYYY-MM-DD
**Location:** `docs/plans/<range>.<revision>.qa-plan.md`

---

## Stage 1: [Phase Name]

### Security Checks

- [ ] **S1.1** — No user-supplied strings interpolated into SQL, shell commands, or file paths
- [ ] **S1.2** — No secrets, credentials, or API keys in committed files
- [ ] **S1.3** — No deserialization of untrusted input without validation
- [ ] **S1.4** — Dependencies pinned to specific versions (no wildcards)
- [ ] **S1.5** — File permissions on created artifacts are not overly permissive
- [ ] **S1.6** — Any new external input surface has validation at the boundary

### UX Checks

- [ ] **U1.1** — Every error path produces a human-readable message on stderr
- [ ] **U1.2** — Every user-facing command exits with code 0 on success, non-zero on failure
- [ ] **U1.3** — Invalid input is rejected with a helpful message, not a crash
- [ ] **U1.4** — Resources (file handles, database connections) are cleaned up on error paths
- [ ] **U1.5** — If the stage writes data, there is a way to read it back or verify it
- [ ] **U1.6** — If the stage creates state, there is a way to inspect the new state
- [ ] **U1.7** — A user who did not write the code can verify the stage's output

### Test-Gap Findings

<!-- List any blind spots in the dev plan's acceptance criteria for this stage. -->
<!-- "Are there things the dev tests won't catch?" -->
<!-- Use IDs: TG1.1, TG1.2, etc. The stage number is the prefix. -->

---

## Stage 2: [Phase Name]

### Security Checks

- [ ] **S2.1** — ...

### UX Checks

- [ ] **U2.1** — ...

### Test-Gap Findings

---

<!-- Add more stages as needed, matching the implementation plan's stage structure. -->

## Recommendations

<!-- Flat table of all findings. Use the Classification column to distinguish
     quality concerns from low-severity findings. Do NOT split into separate subsections. -->

| # | Stage | Classification | Finding | Recommendation | Plan Ref | Status |
|---|-------|----------------|---------|----------------|----------|--------|
| 1 | 1 | Quality concern | Description | Recommendation | Task 1.1 criteria 3 | ⚠️ Open / ✅ Resolved / Won't Fix / Deferred |
| 2 | 1 | Low-severity | Description | Reason for deferral | Task 1.2 | — |

<!-- Won't Fix rationale: set Status to "Won't Fix" and append a brief rationale below the table under "### Won't Fix — rationale". Omit the heading if no quality concerns are skipped. -->
```

## Section Guide

| Section | Purpose |
|---------|---------|
| **Header** | Links to source plan, generation date |
| **Security Checks** | 6-item checklist, IDs: `S{stage}.{item}` (e.g., `S1.1`, `S2.3`) |
| **UX Checks** | 7-item checklist, IDs: `U{stage}.{item}` (e.g., `U1.1`, `U2.7`) |
| **Test-Gap Findings** | Per-stage blind spots in dev acceptance criteria, IDs: `TG{stage}.{item}` |
| **Recommendations** | Flat table of all findings. Classification column (quality concern / low-severity); Plan Ref column (links to specific plan task or criterion). Won't Fix rationale follows the table when applicable. |
