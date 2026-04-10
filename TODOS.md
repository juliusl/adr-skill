# Follow-up Items

Items deferred from solve branches that are not blocking merge but should be tracked.

## From `solve/custom-tech-writer-dispatch` (ADR-0063)

| # | Source | Item | Priority | Notes |
|---|--------|------|----------|-------|
| 1 | QA Rec #3 (Won't Fix) | Add dispatch observability marker to ADR Comments section (e.g., `<!-- content-author: agent-name -->`) so downstream agents can detect which agent wrote the content | Low | Warning mechanism exists for runtime; this adds machine-readable provenance |
| 2 | Untracked file | `src/agents/juliusl-tech-writer-v1.agent.md` is untracked — decide whether to commit it with this branch or separately | User decision | The agent file exists but was not part of the ADR-0063 implementation scope |

## From `solve/agent-review-pipeline` (ADR-0064)

| # | Source | Item | Priority | Notes |
|---|--------|------|----------|-------|
| 3 | QA TG2.2 (Won't Fix) | Define canonical verdict vocabulary for dispatch hooks — Step 4a/4b reference agent-specific Appendix A for verdict labels; a shared vocabulary would decouple create.md from specific agent implementations | Low | Acceptable today because each agent documents its own output format; becomes higher priority if third-party agents are used |
| 4 | Sweep finding #6 | Step 4a/4b identifier reuse across skills — same IDs appear in solve-adr (problem.md, roadmap.md), implement-adr, and code-reviewer. Different meanings per file. Low collision risk because IDs are file-scoped, but worth tracking | Low | Pre-existing pattern, not introduced by ADR-0064 |
| 5 | Sweep findings #1-4 | Standardize "configured/not-configured" phrasing across dispatch conditions — currently mixes "non-empty value", "absent or empty", "not configured", "neither...nor" | Low | Consistency improvement; existing `tech_writer` uses the same "non-empty value" pattern, so changing it should be done holistically |
| 6 | Analytics N-1 | Hook table `Instructions` column heading covers two semantics — old hooks describe artifact + procedure phase, new hooks describe dispatch timing + verb phrase | Low | Nit; functional clarity is adequate |
| 7 | C-2e | Code review re-review (C-2e) was not run — user requested stop before re-dispatch. Original findings were addressed in commit 1ec7845. Risk is low (documentation-only changes, all medium findings fixed) | Medium | Recommend visual review during PR |
| 8 | QA TG3.1 (Won't Fix) | Future plans should use "all tests pass" rather than hardcoded test counts in acceptance criteria | Low | Process improvement for future plan generation |
