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

## From `solve/agent-review-pipeline` (ADR-0065 + refactoring + revision pass)

| # | Source | Item | Priority | Notes |
|---|--------|------|----------|-------|
| 9 | UX F-05 | No shortcut for re-reviewing minor plan edits — I-7 step 5e always re-runs I-4b after revision | Low | Design choice; review is mandatory. Shortcut would save time on trivial edits |
| 10 | UX F-41 | Plan template has no explicit revision history section — comment syntax in header serves as the only guide | Low | Existing comment syntax is functional; a dedicated section would improve discoverability |
| 11 | UX F-51 | assets/index.md not referenced in SKILL.md procedure steps — only discoverable via Deep References section | Low | Convenience file; SKILL.md links each reference individually |
| 12 | TW create.md | Step 4 assessment written before 4a/4b can override — execution order between checkpoint assessment and dispatch hooks unspecified | Low | Agents currently write assessment after 4a/4b in practice; explicit ordering would remove ambiguity |
| 13 | TW create.md | Step 4a Redesign verdict doesn't gate Step 4b — TPM may assess an already-escalated ADR | Low | Low impact: TPM findings are additive. Explicit gate would prevent redundant work |
| 14 | Analytics reviewer | `juliusl-code-reviewer-analytics-v5` stalled twice (0 turns, 25+ min) — investigate agent definition | Medium | May be a model/context issue; sweep reviewer worked fine both times |
