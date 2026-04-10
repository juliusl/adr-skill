# 65. Refine dispatch hook behaviors based on dogfood evidence

Date: 2026-04-10
Status: Accepted
Last Updated: 2026-04-10
Links:
- Extends [ADR-0064](0064-add-option-evaluation-dispatch-hooks-for-review-and-decision-arbitration.md) (UX/DX/TPM dispatch hooks)
- Related to [ADR-0031](0031-add-author-adr-dispatch-hooks-for-custom-agent-delegation.md) (dispatch hook pattern)

## Context

ADR-0064 added three dispatch hooks to the `author-adr` workflow:

- **Step 4a** — UX and DX reviewers evaluate each option and return a verdict (Revise or Redesign).
- **Step 4b** — TPM arbitrates conflicting verdicts.
- **R-6a** — Editor agent polishes the final ADR draft.

A dogfood session (session e86db74a-141b-48f9-8ccb-a76a4f2e7eef, ~78 min wall-clock, 12 sub-agents dispatched) produced three empirical findings:

1. **DX reviewer bottleneck** — DX spent 14 min generating 48 findings across 8 criteria × 6 options. Three of the six options were non-viable. The deep analysis on non-viable options produced no usable output.
2. **Verdict tier gap** — The binary Revise/Redesign mapping in Step 4a caused misrouting. DX returned "Redesign" for gaps that were addressable edits to the Options section. "Redesign" routes to `prototype-adr`, which was overweight for the actual scope of change needed.
3. **Editor polish overhead** — R-6a dispatched the editor agent for 2 Low-severity suggestions. Full dispatch took 2 min; inline application would have taken 10 seconds.

TPM sequencing (Step 4b after UX/DX) was validated — no change needed.

### Decision Drivers

- Reduce wasted reviewer cycles on non-viable options
- Correct verdict routing for heavy-but-addressable revisions
- Eliminate dispatch overhead for trivial editor suggestions
- All changes confined to instruction text in `create.md` and `polish.md` — no hook architecture changes

## Options

### Option A: Apply all three retrospective refinements

1. **Viability pre-screen** — Add two-pass guidance to the Step 4a dispatch prompt. DX and UX run a 30-second viability check per option before deep analysis. Non-viable options are flagged and skipped.
2. **"Revise — Major" verdict tier** — Add a third verdict tier to Step 4a/4b verdict mapping. "Revise — Major" routes to inline rework of the Options section without escalating to `prototype-adr`. The full verdict set becomes: Revise / Revise — Major / Redesign.
3. **Editor complexity threshold** — Add a threshold to R-6a: dispatch the editor agent only when there are ≥3 suggestions or any Medium+ severity item. Apply ≤2 Low-severity suggestions inline without dispatch.

**Drivers addressed:** All three.

**Risk:** Low — modifies instruction text only; no hook or agent contract changes.

---

### Option B: Apply pre-screen and verdict tiers only

Apply findings 1 and 2. Defer finding 3 (editor threshold).

**Rationale for deferral:** The editor threshold saves less time per session (~2 min) than the DX bottleneck fix (~14 min). Lower urgency.

**Drivers addressed:** Two of three.

**Risk:** Low. Same scope as Option A minus one instruction change.

---

### Option C: Status quo

No changes. Accept current behavior as-is.

**Drivers addressed:** None.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — all findings are empirically grounded from a completed dogfood run.

Step 4a skipped — no UX/DX review agents configured (and would be circular — these are the hooks being refined).
Step 4b skipped — no TPM agent configured.

## Decision

**Chosen: Option A.**

Apply all three retrospective refinements to `create.md` and `polish.md`.

We choose Option A because all three findings are empirically grounded, low-risk, and precisely scoped. Option B defers a valid improvement with no clear benefit from doing so — the editor threshold is a one-line instruction change with positive expected value. Option C ignores empirical evidence from a completed dogfood run.

**Y-statement:** For the `author-adr` dispatch hook workflow, where efficiency and verdict accuracy matter, the decision to apply all three retrospective refinements was made by the ADR author, accepting slightly more complex verdict mapping logic in `create.md`, to achieve reduced DX reviewer cycle time, correct verdict routing for major revisions, and eliminated overhead for trivial editor suggestions.

## Consequences

**Positive:**
- DX reviewer skips deep analysis on non-viable options — estimated 5–10 min saved per session with 3+ non-viable options.
- "Revise — Major" verdict tier eliminates false escalation to `prototype-adr` for addressable Options-section rework.
- Editor dispatch skipped for ≤2 Low-severity suggestions — eliminates ~2 min overhead per occurrence.

**Negative:**
- Step 4a/4b verdict mapping gains a third tier, adding a branch to the routing logic in `create.md`.

**Neutral:**
- Hook architecture unchanged — dispatch mechanism, agent contracts, and hook invocation points are unaffected.
- No changes to `prototype-adr` or downstream workflows.
- Editor dispatch threshold (≤2 Low-severity) is calibrated from a single dogfood session; the cutoff may need tuning as usage broadens.

## Quality Strategy

- [ ] ~~Introduces major semantic changes~~
- [x] Introduces minor semantic changes
- [ ] ~~Fuzz testing~~
- [ ] ~~Unit testing~~
- [ ] ~~Load testing~~
- [ ] ~~Performance testing~~
- [ ] ~~Backwards Compatible~~
- [ ] ~~Integration tests~~
- [ ] ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

`create.md` and `polish.md` are user-facing references consumed by agents and engineers. Changes to verdict tier names and dispatch thresholds must be reflected consistently across both files. Verify that "Revise — Major" appears in the verdict table, routing logic, and any inline examples.

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
The dogfood run of UX/DX/TPM dispatch hooks (ADR-0064) produced a retrospective identifying three actionable refinements. All three address efficiency and calibration gaps discovered through empirical use of the hooks during a real ADR authoring session (~78 min wall-clock, 12 sub-agents dispatched). The refinements improve the dispatch hook workflow without changing the hook architecture.

**Evidence source:** Session e86db74a-141b-48f9-8ccb-a76a4f2e7eef retrospective.

**Findings:**
1. **DX reviewer bottleneck** — DX reviewer spent 14 min analyzing 6 options element-by-element (48 findings across 8 criteria × 6 options). Three options were obviously non-viable. A two-pass approach would eliminate wasted deep analysis: quick viability screen (30s/option), then deep analysis only on viable options.
2. **Verdict tier gap** — Binary Revise/Redesign in Step 4a forced misinterpretation. DX returned "Redesign" but the actual gaps were addressable edits to the Options section. "Redesign" mapped to "Pause for validation" (prototype-adr), which was overweight. A "Revise — Major" tier would distinguish "rework this section" from "rethink the approach."
3. **Editor polish overhead** — Editor dispatch at R-6a took 2 min via full agent dispatch for 2 Low-severity suggestions. These could have been applied inline in 10 seconds. A complexity threshold would skip dispatch for ≤2 Low-severity suggestions.
4. **TPM sequencing validated** — TPM after UX/DX is the correct order. The consumed findings materially improved the assessment. No change needed.

**Tolerance:**
- Risk: Low — these are refinements to proven hooks, not new architecture
- Change: Low — modifying instruction text in create.md and polish.md, not adding new hooks
- Improvisation: Low — all three changes are precisely scoped by retrospective evidence

**Uncertainty:**
- Certain: the three efficiency gaps exist (empirical evidence)
- Certain: the recommended fixes are feasible (they modify instruction text, not hook architecture)
- Uncertain: optimal threshold value for editor dispatch (≤2 Low-severity is the retrospective's recommendation; may need tuning)

**Options:**
- Target count: 2-3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- A: Apply all three refinements — viability pre-screen guidance in Step 4a dispatch, verdict tier addition in Step 4a/4b, editor threshold in R-6a
- B: Apply viability pre-screen and verdict tiers only — defer editor threshold (lower evidence weight)
- C: Status quo — no changes (baseline for comparison)

<!-- Review cycle 1 — 2026-04-10 — Verdict: Accept. 2 Low-priority suggestions: (1) single-session evidence basis is acceptable for scope but noted; (2) consider surfacing editor threshold uncertainty in Consequences section. -->
