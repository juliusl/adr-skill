# 22. Replace ecADR completeness check with implementability criteria

Date: 2026-04-03
Status: Accepted
Last Updated: 2026-04-03
Links: ADR-0004

## Context

The review process in `author-adr` currently uses the ecADR "Definition of Done" as its completeness check (Step 1). After ~20 review sessions, the ecADR criteria show a mismatch with how the skills actually work.

### Empirical Evidence

**Plan analysis (13 plans):** 11 of 13 implementation plans contained gap-filling tasks — work that clarified scope, added missing interface details, defined bootstrapping mechanics, or specified default behavior that the ADR didn't provide. These gaps cluster around **scope boundaries** and **actionable detail**, not around Evidence, Agreement, or Realization.

**Commit analysis (74 commits):** 12 refactor commits (16%) followed feature implementations, indicating ADRs were often not scoped precisely enough for the agent to get the implementation right on the first pass. The init-data and revise workflows both needed rework shortly after initial implementation.

**Revision addendum analysis (ADRs 0020, 0021):** ecADR findings in practice:
- **Agreement** — never produced useful findings. In a single-developer agent workflow, agreement is implicit in the decision to implement.
- **Realization/Review** — mixed: rejected in ADR-0020 ("revisit conditions will emerge naturally"), accepted in ADR-0021 (concrete trigger was useful). Situational, not predictive.
- **Evidence** — never the real gap. The real issue was whether the ADR provided enough implementation detail, not whether a PoC existed.

**Experimentation chain as counter-evidence:** ADR-0008 (`.meta/` directory) had zero evidence — it was deliberately experimental. But that "flying blind" decision led to ADR-0011 (XDG config), then ADR-0020 (`.adr/` convention), and tangentially ADR-0018 (unified scripts). The experimental chain was valuable *because* it tolerated uncertainty. Demanding evidence would have either blocked the experiment or produced meaningless synthetic validation.

**What works well (keep):**
- **Criteria** (≥2 genuine alternatives compared) — consistently finds real gaps. When the reviewer flags a missing alternative or a dummy option, the revision produces a stronger ADR.
- **Documentation** (captured in template, shared) — always met by construction since the skill creates ADRs from the template. Trivial to check but confirms the baseline.

**What doesn't fit (replace):**
- **Evidence** — the current framing ("Is there confidence the design will work? PoC, spike, peer validation") treats all decisions as needing validation. But in an agent-developer workflow, some decisions are deliberately experimental. ADR-0008 (`.meta/` directory) was "flying blind" — no evidence it would work — but through experimentation it led to ADR-0011, then ADR-0020, and tangentially ADR-0018. That experimental chain was valuable *because* it tolerated uncertainty. The useful question isn't "is there evidence?" but "does this decision need more data before we proceed, or would prototyping be more valuable?" — an experimentation tolerance assessment.
- **Agreement** — irrelevant for `implement-adr` success. The skill plans and executes against the ADR's decision, not against stakeholder consensus. In a single-developer agent workflow, agreement is the developer approving the decision — which is implicit in asking to implement it.
- **Realization/Review** — review data shows mixed results: rejected as irrelevant in ADR-0020 ("revisit conditions will emerge naturally"), addressed in ADR-0021 (where a concrete trigger was useful). The criterion is situational at best and doesn't reliably predict implementability.

**What's missing:**
The ecADR criteria don't assess whether the ADR is *implementable* — whether `implement-adr` can decompose it into a plan with clear tasks, acceptance criteria, and cost estimates. The review should check for properties that directly support planning:
- Is the decision clear enough to derive tasks from?
- Are the consequences actionable enough to generate acceptance criteria?
- Is the scope bounded enough to estimate cost?
- Are dependencies on other decisions or systems identified?
- Does the Quality Strategy section indicate what testing is expected?

**The broader picture:** The other review steps (fallacy scan, anti-pattern check, consequence validation, 7-point checklist) are working well — they check the agent's quality on problem identification, context elaboration, and solution analysis. The issue is specifically with Step 1, which was designed for traditional team-based ADR review, not for an agent-developer workflow where the next consumer is `implement-adr`.

### Decision Drivers

- **Implementability-oriented** — the completeness check should predict whether `implement-adr` can successfully plan from this ADR
- **Experimentation-tolerant** — the check should distinguish "needs more data" from "would benefit from prototyping" rather than always demanding evidence
- **Agent-workflow-native** — criteria should fit a single-developer + agent workflow, not assume a team review process
- **Preserve what works** — Criteria and Documentation checks should remain; other review steps (2-6) are unchanged

## Options

### Option 1: Replace ecADR with implementability-oriented criteria

Replace the 5 ecADR criteria (Evidence, Criteria, Documentation, Agreement, Realization/Review) with 6 criteria optimized for the agent-developer workflow and `implement-adr` planning success:

1. **Criteria** (retained) — ≥2 genuine alternatives compared
2. **Documentation** (retained) — captured in template and shared
3. **Experimentation Tolerance** (replaces Evidence) — spectrum assessment: does this decision need more data, or would prototyping be more valuable?
4. **Scope Clarity** (new) — are the boundaries clear enough to decompose into tasks?
5. **Actionable Consequences** (new) — can acceptance criteria be derived from stated consequences?
6. **Dependency Visibility** (new) — are links to related ADRs, systems, or prerequisites explicit?

**Strengths:**
- Directly predicts `implement-adr` planning success — each criterion maps to a planning input
- Experimentation Tolerance is a spectrum, not a gate — accommodates both data-driven and exploratory decisions
- Grounded in empirical analysis of 13 plans and 74 commits
- Preserves what works (Criteria, Documentation) while removing what doesn't (Agreement, Realization/Review)

**Weaknesses:**
- Diverges from the academic ecADR framework — may confuse users familiar with the original
- Experimentation Tolerance is subjective — harder to assess consistently than binary Evidence check
- New criteria (Scope Clarity, Actionable Consequences, Dependency Visibility) haven't been validated through review sessions yet

### Option 2: Keep ecADR but make Agreement and Realization/Review conditional

Keep all 5 ecADR criteria but change Agreement and Realization/Review from "always check" to "only check when explicitly relevant" (e.g., multi-stakeholder projects, team reviews).

**Strengths:**
- Minimal change — keeps the established framework
- Still works for traditional team-based ADR review if needed
- No new criteria to validate

**Weaknesses:**
- Doesn't add the implementability criteria that plans consistently need (Scope Clarity, Actionable Consequences, Dependency Visibility)
- Evidence remains a binary gate that penalizes experimentation
- Incremental — doesn't address the root mismatch between review and planning

### Option 3: Two-tier completeness: ecADR for team review, implementability for agent review

Keep ecADR criteria for `Proposed` ADRs going through team/PR review. Add a separate implementability checklist for `Prototype` ADRs going directly to `implement-adr`.

**Strengths:**
- Preserves ecADR for its intended context (team review)
- Adds the missing implementability criteria without removing anything
- Clear separation of concerns

**Weaknesses:**
- Two checklists increase review complexity and cognitive load
- In practice, this project only uses the agent workflow — the team-review path is theoretical overhead
- Dual criteria create ambiguity about which set applies

## Decision

In the context of **the review completeness check not predicting implementability**, facing **ecADR criteria designed for team-based review being applied to an agent-developer workflow**, we decided for **replacing the ecADR completeness criteria with 6 implementability-oriented criteria**, and neglected **making ecADR conditional (doesn't add missing criteria) and a two-tier system (unnecessary complexity for a single-workflow project)**, to achieve **a completeness check that directly predicts whether `implement-adr` can successfully plan from the ADR**, accepting that **we diverge from the academic ecADR framework and the new criteria need validation through use**.

### The 6 Implementability Criteria

| # | Criterion | What to Check | Replaces |
|---|-----------|---------------|----------|
| 1 | **Criteria** | Are ≥2 genuine alternatives identified and compared? | ecADR Criteria (retained) |
| 2 | **Documentation** | Is the decision captured in the template? | ecADR Documentation (retained) |
| 3 | **Experimentation Tolerance** | Does this decision need more data to support it, or would it benefit from prototyping? Assess on a spectrum: *well-supported* → *needs validation* → *deliberately experimental*. Flag only when the ADR appears to need data it doesn't have and isn't framed as experimental. | ecADR Evidence |
| 4 | **Scope Clarity** | Are the boundaries of what's in and out of scope clear enough to decompose into tasks? Can the agent identify what files, components, or interfaces are affected? | New |
| 5 | **Actionable Consequences** | Can test/acceptance criteria be derived from the stated consequences? Are consequences specific enough to verify, or are they vague aspirations? | New |
| 6 | **Dependency Visibility** | Are links to related ADRs, external systems, or prerequisites explicit? Would `implement-adr` discover missing dependencies during planning? | New |

### Experimentation Tolerance Spectrum

Rather than a binary pass/fail, Experimentation Tolerance uses a three-point assessment:

- **Well-supported** — data, PoC, or prior experience backs the decision. No concern.
- **Needs validation** — the decision makes claims that could be tested but aren't. Recommend a prototype or spike before implementation.
- **Deliberately experimental** — the decision explicitly acknowledges uncertainty and is designed to learn. Acceptable if the ADR frames it honestly (not a Fairy Tale anti-pattern).

The key insight: "flying blind" is fine when it's *intentional* and *acknowledged*. It's a problem when the ADR presents unvalidated claims as fact.

## Consequences

**Positive:**
- The completeness check directly predicts planning success — each criterion maps to an input that `implement-adr` needs (scope → task decomposition, consequences → acceptance criteria, dependencies → ordering).
- Experimentation Tolerance accommodates the full decision spectrum from data-driven to exploratory, matching how the project actually evolves (ADR-0008 → 0011 → 0020).
- Dropping Agreement and Realization/Review eliminates findings that are consistently irrelevant, reducing revision churn.

**Negative:**
- Diverges from ecADR — users familiar with the academic framework may be confused by the replacement criteria.
- The 3 new criteria (Scope Clarity, Actionable Consequences, Dependency Visibility) are untested — they need validation through real review sessions.
- Experimentation Tolerance is inherently more subjective than binary Evidence — reviewer agents may assess it inconsistently.

**Neutral:**
- Review steps 2-6 (Fallacy Scan, Anti-Pattern Check, Consequence Validation, Checklist, Verdict) are unchanged.
- The 7-point review checklist may need minor wording updates to align with the new criteria but is otherwise unaffected.

## Quality Strategy

- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] User documentation

### Additional Quality Concerns

Validation approach: apply the new criteria to the next 3 ADR reviews and compare findings with what ecADR would have produced. If the new criteria consistently surface implementability gaps that ecADR missed, the replacement is validated. If they produce false positives or miss real issues, revisit.

**Validation mechanics (unresolved — may warrant a separate ADR):**

- **Option A: Dual files** — keep `review.md` (current ecADR) alongside `review-next.md` (new criteria). Run both per ADR and compare outputs. Most practical for live comparison but doubles review time.
- **Option B: Replace and compare historically** — swap `review.md` to the new criteria immediately. Compare findings against the existing Comments sections in ADRs 0020-0021, which have ecADR findings recorded. No extra cost per review, but comparison is retrospective.
- **Option C: Shadow mode** — keep `review.md` as-is, add a shadow section where the reviewer also evaluates the new criteria in a single pass.

A `prototype-adr` skill may be a better home for managing this kind of A/B validation workflow.

---

## Comments
