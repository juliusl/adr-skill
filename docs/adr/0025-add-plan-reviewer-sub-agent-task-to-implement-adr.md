# 25. Add plan-reviewer sub-agent task to implement-adr

Date: 2026-04-04
Status: Accepted
Last Updated: 2026-04-03
Links:
- Extends [ADR-0023](0023-add-prototype-adr-skill-for-structured-decision-validation.md) (exposed the plan quality gap — user docs checked but no README task generated)
- Related to [ADR-0024](0024-add-checkpoint-sections-to-nygard-agent-template.md) (Conclusion Checkpoint partially addresses pre-review quality, but for the ADR — not the plan)

## Context

The `implement-adr` skill generates implementation plans (plan.md) from ADRs. During plan execution for ADR-0023 and ADR-0024, a quality gap was observed: **ADR-0023 checked `[x] User documentation` in its Quality Strategy, but the generated plan contained no task to update the project README.md.** The implement-adr SKILL.md explicitly instructs "Add documentation update task or criteria" when User documentation is checked (line 387), but this instruction was silently dropped during plan generation.

**The problem:** There is no verification step that checks whether a generated plan faithfully reflects the source ADR's stated requirements. The plan is generated once by an agent, and then executed — but no one verifies that the plan actually covers:

- All checked Quality Strategy items (the README gap)
- The stated Consequences (positive and negative — are there tasks to realize positives and mitigate negatives?)
- Additional Quality Concerns (specific items the author called out)
- The Evaluation Checkpoint's Validation needs (if populated)
- Decision scope (does the plan cover the full decision, or just part of it?)

**Why this matters:**

1. **Plans are the execution contract.** If the plan misses something from the ADR, it will be silently unimplemented. The ADR says "user documentation" — the plan doesn't have a task for it — the README never gets updated. The gap compounds over time.

2. **Agent plan generation is probabilistic.** LLM-based agents process the ADR as a prompt. They follow instructions most of the time but can miss items — especially when instructions are buried in tables or long reference docs. The current system has no safety net for these misses.

3. **The review step exists for ADRs but not for plans.** The `author-adr` skill has a structured review process (ADR-0022: implementability criteria, fallacies, anti-patterns). But once an ADR is handed to `implement-adr`, the generated plan gets no comparable quality check. The quality boundary drops exactly where the output matters most — at the boundary between "what we decided" and "what we'll actually build."

**What triggered this:** During implement-adr execution of ADR-0023/0024, the user observed that ADR-0023 checked `[x] User documentation` and `[x] Integration tests` in Quality Strategy, and listed 5 specific Additional Quality Concerns. The generated plan included integration test tasks but missed a user documentation task entirely. No agent or process caught this.

### Decision Drivers

Must-haves:
- **ADR-plan alignment** — every checked Quality Strategy item, stated consequence, and AQC item must trace to at least one plan task or criterion
- **Automated verification** — the check must run as part of the planning workflow, not as an afterthought
- **Actionable output** — findings must be specific enough to fix ("ADR checks `User documentation` but no plan task addresses it")
- **Bounded iteration** — the review loop must terminate (not infinite back-and-forth)

Nice-to-haves:
- **User escape hatch** — if the reviewer and planner can't converge, escalate to the user
- **Minimal overhead** — the review should add value without doubling plan generation time
- **Reusable across ADRs** — the reviewer should work on any ADR/plan pair, not be specific to one decision

## Options

### Option 1: Post-generation plan review via general-purpose sub-agent

After `implement-adr` generates a plan, it spawns a general-purpose sub-agent (the "plan reviewer") with a structured prompt. The reviewer reads both the source ADR(s) and the generated plan, then produces a finding report — specific gaps where the plan diverges from the ADR's stated requirements.

**Review checklist for the sub-agent:**

1. **Quality Strategy coverage** — for each checked `[x]` item in the ADR, verify at least one plan task or acceptance criterion addresses it.
2. **Consequence traceability** — for each stated positive consequence, verify there's a task that realizes it. For each negative consequence, verify there's a mitigation or acknowledgment.
3. **AQC coverage** — for each Additional Quality Concern, verify at least one plan task or criterion addresses it.
4. **Evaluation Checkpoint coverage** — if the ADR has Validation needs listed, verify they're addressed in the plan.
5. **Scope completeness** — does the plan cover the full decision, or are there decision sections without corresponding tasks?

**Iteration protocol:**
1. Reviewer produces findings (list of gaps).
2. Main agent reviews findings and proposes plan revisions.
3. Revised plan is re-reviewed by the sub-agent.
4. Loop continues until the reviewer reports no issues, or **3 rejection cycles** occur.
5. After 3 rejections → escalate to user review (escape hatch). The user sees the findings and decides what to address.

**Strengths:**
- Directly addresses the observed gap — Quality Strategy items are explicitly verified
- Sub-agent pattern keeps the reviewer's context separate from the planner's
- Bounded iteration (max 3 cycles) prevents infinite loops
- User escape hatch respects human authority
- Reusable — the review prompt works on any ADR/plan pair
- General-purpose agent provides high-quality reasoning for nuanced judgment calls

**Weaknesses:**
- Adds latency — each review cycle requires a full sub-agent invocation
- General-purpose agents are expensive (Sonnet-class model)
- Sub-agent may hallucinate findings (false positives)
- 3-cycle limit is arbitrary — some plans may need more iteration, others may need none
- Relies on the agent runtime supporting sub-agent spawning (task tool)

### Option 2: Inline plan validation via checklist extraction

Instead of a separate reviewer agent, extend implement-adr's planning workflow with a self-check step. After generating the plan, the planner extracts a structured checklist from the ADR (Quality Strategy items, consequences, AQC) and verifies its own plan against it — programmatically where possible, heuristically otherwise.

**Workflow addition to implement-adr:**
1. After generating all tasks, extract from the source ADR:
   - Checked Quality Strategy items → required coverage list
   - Consequences → expected plan outcomes
   - AQC items → specific quality requirements
2. For each extracted item, search the plan for a matching task or criterion.
3. Report unmatched items as warnings.
4. If warnings exist, add placeholder tasks to address them.

**Strengths:**
- No sub-agent overhead — runs in the same context as planning
- Fast — checklist extraction is mechanical
- Lower cost — no additional model invocation
- Deterministic for Quality Strategy items (checkbox → grep for keyword)

**Weaknesses:**
- Self-review is inherently weaker — the same agent that missed the item may also miss it in self-check
- Can't handle nuanced judgment (e.g., "does this task actually address the consequence, or just superficially mention it?")
- Consequence traceability is hard to automate — consequences are freeform text
- Adds complexity to an already long SKILL.md
- Placeholder tasks may be low-quality if auto-generated

### Option 3: Hybrid — lightweight extraction + targeted sub-agent review

Extract the checklist mechanically (Option 2's approach), then only invoke a sub-agent reviewer when the self-check finds gaps or when the ADR has high complexity indicators (many consequences, AQC items, or checked Quality Strategy boxes).

**Triggering criteria for sub-agent review:**
- Self-check finds ≥1 unmatched Quality Strategy item
- ADR has ≥3 AQC items
- ADR has negative consequences without mitigation tasks
- User explicitly requests review

**Strengths:**
- Lightweight by default — simple ADRs get self-check only
- Sub-agent is targeted — only invoked when needed, reducing cost
- Mechanical extraction catches the easy misses (Quality Strategy checkboxes)
- Sub-agent handles nuanced judgment for complex cases
- Cost-proportional to ADR complexity

**Weaknesses:**
- Two review paths (self-check vs. sub-agent) increase implementation complexity
- Triggering heuristics may be wrong — a "simple" ADR might still have subtle gaps
- The self-check step may give false confidence ("no gaps found") when the sub-agent would have found issues
- More code paths to test and maintain

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Pause for validation

This decision would benefit from prototyping to validate two key uncertainties:

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

The experimentation gap has been addressed by prototyping — see findings below.

**Validation needs:**

- ~~Run the plan-reviewer concept against the existing ADR-0023/0024 plan to verify it catches the known `User documentation` gap~~ — **Validated.** Sub-agent reviewer caught it; self-check missed it (false FOUND on keyword match).
- ~~Test the 3-cycle iteration protocol: does the reviewer converge, or does it oscillate between findings?~~ — **Not tested yet.** Only ran one review cycle. Convergence behavior requires testing during implementation.
- ~~Measure false positive rate: does the reviewer flag items that are actually covered but described differently?~~ — **Validated.** Sub-agent: ~1 false positive (flagged solve.md as non-existent, but it does exist). Self-check: ~3 false positives/negatives due to keyword mismatch.
- ~~Compare Option 1 (full sub-agent) vs. Option 2 (self-check) on the same ADR/plan pair — which catches more real gaps?~~ — **Validated.** Sub-agent found 12 real findings across 49 checks (37 pass, 12 fail). Self-check found 9 unmatched but with significant false positive/negative rate — critically, it missed the original motivating bug.

**Prototype observations:**

```json
{"objective": "Sub-agent catches User documentation gap", "result": "pass", "notes": "Found [FAIL] Integration tests for ADR-0023, found all 5 AQC items unaddressed, caught missing implement-adr behavioral update"}
{"objective": "Self-check catches User documentation gap", "result": "fail", "notes": "Falsely reported FOUND — matched template docs keywords instead of README task"}
{"objective": "Sub-agent false positive rate", "result": "data", "value": {"total_checks": 49, "true_positives": 11, "false_positives": 1, "true_negatives": 37}}
{"objective": "Self-check false positive rate", "result": "data", "value": {"total_checks": 17, "keyword_matches": 8, "false_positives_negatives": 3}}
```

**Conclusion:** Option 1 (sub-agent reviewer) is clearly superior for catching plan-ADR divergence. Option 2 (self-check) fails at the core task — it missed the exact bug that motivated this ADR. Option 3 (hybrid) loses its value proposition because the self-check layer provides false confidence rather than useful filtering.

## Decision

In the context of **plan-ADR alignment gaps where implement-adr silently drops Quality Strategy, consequence, and AQC requirements during plan generation**, facing **the need for verification without blocking the planning workflow or requiring constant user oversight**, we decided for **a post-generation plan review via general-purpose sub-agent with bounded iteration and user escape hatch (Option 1)**, and neglected **inline self-check (missed the motivating bug in prototype) and hybrid approach (self-check layer provides false confidence)**, to achieve **reliable ADR-plan alignment verification where every checked quality item and stated consequence traces to plan coverage**, accepting that **each review cycle adds ~3 minutes of latency and requires a Sonnet-class model invocation**.

### Plan-Reviewer Sub-Agent Protocol

After `implement-adr` generates a plan, it spawns a general-purpose sub-agent with the following structured review prompt:

**Inputs:**
- Source ADR(s) — full file content
- Generated plan — full file content

**Review checklist:**

1. **Quality Strategy coverage** — for each checked `[x]` item in the ADR, verify at least one plan task or acceptance criterion addresses it. Cross-reference each checkbox against the template's Quality Strategy Items documentation (nygard-agent-template.md §Quality Strategy Items) to understand what each checkbox means and what plan coverage it implies. Report PASS/FAIL per item.
2. **Consequence traceability** — for each positive consequence, verify a task realizes it. For each negative consequence, verify a mitigation or acknowledgment. Report PASS/FAIL per consequence.
3. **AQC coverage** — for each Additional Quality Concern, verify at least one plan task or criterion addresses it. Report PASS/FAIL per item.
4. **Evaluation Checkpoint coverage** — if the ADR has Validation needs, verify they're addressed.
5. **Scope completeness** — verify the plan covers the full decision, not just part of it.
6. **Project integration** — if the plan creates new directories, scripts, skills, or artifacts, verify the project's build/install/test infrastructure (Makefile, CI, etc.) is updated to include them.

**Output format:** Structured finding report with per-item PASS/FAIL, quoted evidence, and a verdict (Plan Approved / Plan Needs Revision).

### Iteration Protocol

```
implement-adr generates plan
        │
        ▼
plan-reviewer sub-agent reviews
        │
    ┌───┴───┐
    │ Pass  │──────────► Plan Approved → proceed to execution
    └───┬───┘
        │ Fail
        ▼
main agent revises plan based on findings
        │
        ▼
re-review (cycle 2)
        │
    ┌───┴───┐
    │ Pass  │──────────► Plan Approved
    └───┬───┘
        │ Fail
        ▼
revise again (cycle 3)
        │
        ▼
re-review (cycle 3)
        │
    ┌───┴───┐
    │ Pass  │──────────► Plan Approved
    └───┬───┘
        │ Fail
        ▼
User Escape Hatch — present findings to user,
user decides what to address or override
```

**Cycle limit: 3.** Chosen to balance thoroughness with latency. If the reviewer and planner can't converge in 3 cycles, the remaining findings likely require human judgment — an architectural concern the agent can't resolve mechanically.

**User escape hatch behavior:**
1. Present all remaining findings to the user with context.
2. For each finding, the user can: address it, reject it (with rationale), or defer it.
3. The plan is updated with whatever the user approves, and execution proceeds.

### Integration Point

The plan-reviewer is a new step in implement-adr's planning workflow, inserted **after** plan generation and **before** plan execution:

```
Read ADR → Generate plan → Plan review (sub-agent) → [Iterate] → Execute plan
```

The reviewer is not optional — it always runs. This is a deliberate design choice: the prototype showed that even "simple" plans have gaps (the ADR-0023/0024 plan scored 37/49, missing 12 items). The cost (~3 minutes per cycle) is small relative to the cost of silently unimplemented requirements.

## Consequences

**Positive:**
- Every checked Quality Strategy item, consequence, and AQC item is verified against plan coverage — the exact class of bug that triggered this ADR (missing README task) is now caught.
- The structured finding report (per-item PASS/FAIL) provides actionable, specific feedback — not vague "the plan looks incomplete."
- Bounded iteration (max 3 cycles + user escape hatch) prevents infinite loops while respecting human authority.
- The review protocol is reusable — it works on any ADR/plan pair, not just specific decisions.
- The prototype validated the approach: 49 checks, 12 real findings, ~1 false positive on the ADR-0023/0024 plan.

**Negative:**
- Each review cycle adds ~3 minutes and a Sonnet-class model invocation — for a 3-cycle plan, this is ~9 minutes of additional latency.
- The reviewer may produce false positives (~2% rate observed in prototype) that require the planner to waste a cycle justifying existing coverage.
- The 3-cycle limit is a heuristic — some edge cases may need more iteration (though the user escape hatch handles this).
- Adding a mandatory review step increases the complexity of implement-adr's workflow from 4 steps to 5.

**Neutral:**
- The plan-reviewer sub-agent requires a general-purpose agent runtime (task tool with mode="background"). This is available in the current environment but is a platform dependency.
- The review prompt template and checklist become part of implement-adr's reference documentation, adding to its already long SKILL.md.
- The iteration protocol's convergence behavior was not fully tested in the prototype (only 1 cycle run). This should be validated during implementation.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing
- [x] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [x] Integration tests
- [x] User documentation

### Additional Quality Concerns

- **Review prompt quality** — the structured prompt for the plan-reviewer sub-agent must be tested against multiple ADR/plan pairs to verify consistent finding quality (not just ADR-0023/0024).
- **False positive rate** — track false positives across uses; if the rate exceeds ~5%, the review prompt needs refinement.
- **Iteration convergence** — verify the 3-cycle protocol converges (findings decrease per cycle, not oscillate) on at least 3 different ADR/plan pairs.
- **User escape hatch UX** — the escape hatch presentation (findings + user choices) must be clear enough that the user can make informed decisions without re-reading the full ADR.
- **Backwards compatibility** — existing plans and plan-generation workflows must work unchanged; the reviewer is additive.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

- The iteration protocol's convergence was not tested in prototype (only 1 cycle). This is acknowledged in Consequences (neutral) and AQC.
- The false positive rate (~2%) is based on a single test run. AQC calls for tracking across multiple uses.

---

## Comments

**2026-04-04 — Prototype finding: Makefile gap not caught by reviewer.**
During implementation, the root Makefile's `install-skills`, `check-refs`, and `validate-all` targets were not updated for the new `prototype-adr` skill. The plan-reviewer prototype did not catch this because the Makefile isn't referenced in the ADR. This suggests a 6th review check is needed: **Project integration** — if the plan creates new directories, scripts, or skills, verify the project's build/install/test infrastructure is updated to include them. This is a *convention-based* check, not an ADR-content check.

<!-- No review cycle on record. -->
