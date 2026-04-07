# 56. Consolidate review and revise into single polish reference

Date: 2026-04-07
Status: Ready
Last Updated: 2026-04-07
Links: ADR-0055 (Ready status — clarifies when review/revise applies in lifecycle)

## Context

The author-adr skill has two reference files for the review→revise cycle:

| File | Lines | Content |
|------|-------|---------|
| review.md | 230 | Structured review procedure: implementability, fallacies, anti-patterns, checklist, verdict |
| revise.md | 299 | Interactive revision procedure: load comments, triage, apply changes, re-review recommendation |
| **Total** | **529** | Two files, one workflow |

The user observed "a lot of conflicting directions" when reading these files together. Key issues:

1. **Status guidance is split and inconsistent.** review.md line 183 says "Accept verdict does NOT trigger Accepted status." revise.md is silent on status implications. Neither file mentions the new Ready status (ADR-0055).

2. **The Accept-with-Suggestions path is fragmented.** review.md Step 6a defines an "Accept-with-Suggestions" polish pass. revise.md only covers the "Revise" verdict path. A reader must cross-reference both files to understand what happens after a review.

3. **The files serve a single workflow.** Review produces findings → revise addresses them → re-review checks the result. This is one loop, split across two files. The split made sense when review and revision were separate user actions, but in practice they're always sequential — the review dispatches revision automatically via editor dispatch.

4. **529 lines across two files is significant context.** Each reference is loaded as a prompt for its respective agent. Consolidation could reduce total lines by eliminating shared context (procedure overhead, status references, dispatch mechanics).

**Decision drivers:**
- The user's observation: "review.md and revise.md could be collapsed into a polish.md or even inlined into SKILL.md"
- ADR-0050 requires procedural references to follow procedure-template.md — restructuring both files together is more efficient than restructuring them separately
- The review→revise→re-review loop is a single workflow with a single entry point (A-3)

## Options

### Option A: Consolidate into a single polish.md reference

Merge review.md and revise.md into a single `polish.md` reference that covers the complete review→revise→re-review cycle. The SKILL.md steps A-3 (Review), A-4 (Revise), and A-5 (Re-review) remain as separate steps but reference the same file.

**Structure:**
```
polish.md
├─ Policies (status caps, dispatch compliance)
├─ Procedure index (review → verdict → revise → re-review)
├─ Review phase (implementability, fallacies, anti-patterns, checklist, verdict)
├─ Verdict handling (Accept → Ready status, Revise → revision loop, Rethink → stop)
├─ Revision phase (load findings, triage, apply, summary)
├─ Re-review phase (when to re-review, max cycles)
└─ Prompt templates (review agent prompt, editor agent prompt)
```

**Strengths:**
- Single file for the entire quality loop — no cross-referencing
- Status transitions documented once, consistently
- Duplicated context (dispatch mechanics, status rules) eliminated
- Estimated ~350–450 lines (from 529) — savings come from two sources: deduplication of shared context (status rules, dispatch mechanics, procedure overhead appearing in both files) and editorial tightening during procedure-template.md restructuring. The lower bound assumes aggressive deduplication; the upper bound assumes minimal editorial tightening.
- Cleaner mental model: "polish" is what happens between create and implement

**Weaknesses:**
- Larger single file (~350–450 lines) loaded into agent context
- Review agent and editor agent currently get different files — consolidation means both get the full file (can be mitigated with section markers)
- Rename from established filenames

### Option B: Inline review/revise into SKILL.md

Move all review and revise content directly into author-adr SKILL.md. Eliminate the reference files entirely.

**Strengths:**
- Everything in one place — no reference indirection
- Agent loads the full context naturally

**Weaknesses:**
- author-adr SKILL.md is currently 314 lines. Adding ~400 lines of review/revise content pushes it to ~700+ lines — well over the 500-line recommendation
- SKILL.md becomes the largest file in the system, mixing procedure routing with specialized review logic
- Violates the progressive-disclosure design — SKILL.md is supposed to route, references provide depth

### Option C: Keep review.md and revise.md separate — fix inconsistencies only

Fix the status guidance gap and cross-reference issues without restructuring.

**Strengths:**
- Minimal change
- Preserves existing file structure

**Weaknesses:**
- Doesn't address the user's core observation: "a lot of conflicting directions"
- The structural split still forces cross-referencing for one workflow
- Both files still need procedure-template.md restructuring per ADR-0050 — doing that separately doubles the work

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — the evidence is from the user's direct reading of both files and the line counts are verifiable.

## Decision

**Option A: Consolidate into a single polish.md reference.**

In the context of two reference files with conflicting status guidance serving a single review→revise workflow, facing the user's observation that they contain "a lot of conflicting directions" and ADR-0050's pending requirement to restructure both files anyway, we decided to consolidate review.md and revise.md into a single polish.md that covers the complete quality loop, to achieve a single source of truth for the review→revise→re-review cycle, accepting the one-time migration effort.

### Implementation Requirements

1. **Create polish.md** — merge review.md and revise.md following procedure-template.md structure
2. **Delete review.md and revise.md** — replace with polish.md
3. **Update author-adr SKILL.md** — change A-3/A-4/A-5 references from review.md/revise.md to polish.md
4. **Update dispatch mechanics** — review agent and editor agent both receive polish.md with section markers indicating which phase they're executing. The specific scoping mechanism (e.g., prompt-level phase instructions, section-header anchors, or extracted phase snippets) is deferred to implementation.
5. **Update check-refs** — ensure no dangling references to deleted files

## Consequences

**Positive:**
- Single file for the complete quality loop — status transitions documented once
- Reduced total lines (~350–450 from 529) — shared context eliminated, exact count depends on deduplication and editorial tightening
- ADR-0050 compliance addressed for both files in one restructuring pass
- Cleaner SKILL.md references — A-3/A-4/A-5 all point to one file with section anchors

**Negative:**
- Both agents (reviewer, editor) receive the full file. The impact is asymmetric: the editor currently loads revise.md (299 lines), so a ~350–450 line polish.md is a negligible-to-moderate increase (~17–50%). The reviewer currently loads review.md (230 lines), so the increase is notable (~52–96%). Section markers (`## Review Phase`, `## Revision Phase`) aid cognitive focus — each agent knows which section to execute — but do not reduce token cost.
- One-time migration disrupts existing file references

**Neutral:**
- A-3, A-4, A-5 step identifiers in SKILL.md are unchanged
- The review agent's checklist, fallacy list, and anti-pattern list are preserved verbatim
- The editor agent's guardrails and Q&A mechanics are preserved verbatim
- Prompt templates can use `## Review Phase` and `## Revision Phase` anchors to scope agent context
- The "polish" name connotes light refinement, but the procedure includes rigorous quality gates (fallacy checks, anti-pattern scans, Rethink verdict that can reject the ADR). This tension is intentional — the name reflects the workflow's position in the lifecycle (post-authoring refinement), not the rigor of its checks.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

The consolidated polish.md must preserve all specialized content from both files:
- review.md: 7 fallacies, 11 anti-patterns, 6 implementability criteria, 7-point checklist, 3 verdict types
- revise.md: 7 guardrails, Q&A addendum structure, review cycle marker format, editor dispatch mechanics

Verify by comparing section-level content between old files and new file.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This pairs with ADR-0055 (Ready status). The polish.md file will document the `Proposed → Ready` transition as part of the Accept verdict handling, consolidating the currently-fragmented status guidance.

---

## Comments

### Draft Worksheet

**Framing:**
The user reviewed review.md and revise.md and found conflicting directions. Proposed collapsing into a single reference. The two files serve one workflow (review→revise→re-review) and splitting them creates cross-referencing overhead and status inconsistencies.

**Tolerance:**
- Risk: Low — consolidation, not redesign
- Change: Medium — two files become one, all references update
- Improvisation: Low — user has clear direction ("polish.md or inlined")

**Uncertainty:**
- Known: content of both files (230 + 299 lines)
- Known: the inconsistencies (status guidance, Accept-with-Suggestions path)
- Known: Option B (inline) exceeds the 500-line limit
- Uncertain: exact line count of consolidated file (estimated ~350–450)

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Option A: Consolidate into polish.md
- Option B: Inline into SKILL.md
- Option C: Keep separate, fix inconsistencies only
