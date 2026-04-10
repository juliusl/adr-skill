# Follow-up Items

Items deferred from solve branches that are not blocking merge but should be tracked.

## From `solve/custom-tech-writer-dispatch` (ADR-0063)

| # | Source | Item | Priority | Triage | Notes |
|---|--------|------|----------|--------|-------|
| 1 | QA Rec #3 (Won't Fix) | Add dispatch observability marker to ADR Comments section (e.g., `<!-- content-author: agent-name -->`) so downstream agents can detect which agent wrote the content | Low | Defer — next branch | Warning mechanism covers runtime. Provenance is quality-of-life, not correctness. |
| ~~2~~ | ~~Untracked file~~ | ~~`src/agents/juliusl-tech-writer-v1.agent.md` is untracked~~ | ~~User decision~~ | ~~Done~~ | ~~Committed in c5308e0~~ |

## From `solve/agent-review-pipeline` (ADR-0064)

| # | Source | Item | Priority | Triage | Notes |
|---|--------|------|----------|--------|-------|
| 3 | QA TG2.2 (Won't Fix) | Define canonical verdict vocabulary for dispatch hooks | Low | Defer — after third-party agents exist | Each agent documents its own output format today. Grounding condition: a third-party agent is being integrated. |
| ~~4~~ | ~~Sweep finding #6~~ | ~~Step 4a/4b identifier reuse across skills~~ | ~~Low~~ | ~~Won't fix~~ | ~~IDs are file-scoped by convention. Cross-file collision requires human confusion in a multi-file context — low real risk.~~ |
| 5 | Sweep findings #1-4 | Standardize "configured/not-configured" phrasing across dispatch conditions | Low | Fix next branch — batch with dispatch refactor | Consistency issue across dispatch conditions. Natural to fix when dispatch machinery is next touched. |
| ~~6~~ | ~~Analytics N-1~~ | ~~Hook table `Instructions` column heading covers two semantics~~ | ~~Low~~ | ~~Won't fix~~ | ~~Functional clarity is adequate. No user-visible failure mode.~~ |
| 7 | C-2e | Code review re-review (C-2e) was not run — original findings addressed in commit 1ec7845 | Medium | Done | Analytics reviewer completed: 1 HIGH (ADR-0031 stale editor ref — fixed), 1 MEDIUM (R-3 anti-pattern count — fixed), 1 nit (count label — fixed). |
| 8 | QA TG3.1 (Won't Fix) | Future plans should use "all tests pass" rather than hardcoded test counts in acceptance criteria | Low | Defer — process improvement | No current correctness gap. Candidate for a qa-planning.md note in a future branch. |

## From `solve/agent-review-pipeline` (ADR-0065 + refactoring + revision pass)

| # | Source | Item | Priority | Triage | Notes |
|---|--------|------|----------|--------|-------|
| ~~9~~ | ~~UX F-05~~ | ~~No shortcut for re-reviewing minor plan edits~~ | ~~Low~~ | ~~Won't fix~~ | ~~Mandatory review is a design property. Review is a deliverable, not overhead.~~ |
| ~~10~~ | ~~UX F-41~~ | ~~Plan template has no explicit revision history section~~ | ~~Low~~ | ~~Won't fix~~ | ~~Comment syntax in header is functional. Adding a section for a feature already covered by comments adds noise.~~ |
| ~~11~~ | ~~UX F-51~~ | ~~assets/index.md not referenced in SKILL.md procedure steps~~ | ~~Low~~ | ~~Won't fix~~ | ~~Index is a convenience artifact. SKILL.md links each reference individually via Deep References.~~ |
| ~~12~~ | ~~TW create.md~~ | ~~Step 4 assessment written before 4a/4b can override — execution order unspecified~~ | ~~Low~~ | ~~Done — resolved by ADR-0066~~ | ~~Parallel dispatch eliminates sequential dependency. Verdict consolidation runs after both return.~~ |
| ~~13~~ | ~~TW create.md~~ | ~~Step 4a Redesign verdict doesn't gate Step 4b — TPM may assess an already-escalated ADR~~ | ~~Low~~ | ~~Done — resolved by ADR-0066~~ | ~~Both run independently in parallel. No gate needed.~~ |
| 14 | Analytics reviewer | `juliusl-code-reviewer-analytics-v5` stalled twice (0 turns, 25+ min) | Medium | Done — intermittent | Third attempt completed successfully (734s, 26 tool calls). Agent definition is sound. Stall pattern correlates with large diffs — likely model/context pressure, not instruction defect. Monitor on next branch. |
