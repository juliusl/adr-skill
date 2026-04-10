# 67. Add fast-path scenario to solve-adr for pre-decided findings

Date: 2026-04-10
Status: Accepted
Last Updated: 2026-04-10
Links: ADR-0068

## Context

solve-adr's S-1 workflow runs full author-adr exploration: intake, option discovery, evaluation, convergence, review. This is the right process for open problems where the decision is not yet made.

Certain finding types arrive with the decision already understood — retrospective findings, bug bash findings, amendments to existing decisions. Running full S-1 exploration for these is waste: there is no ambiguity to resolve and no options to discover.

Findings also split into two categories:
1. **ADR-worthy findings** — the finding would add value when browsing the ADR list. Seeing it in `make list` gives useful ambient context: orientation, history, or rationale about a decision. An ADR is needed, but not exploration.
2. **Plan-only findings** — the finding is a task, fix, or improvement that would clutter the ADR list without adding navigational value. No ADR is needed; the finding should go directly into the plan with a note referencing its source.

**The classification test:** "Would seeing this in the ADR list be useful to someone orienting themselves in this codebase?" If yes → ADR. If no → plan.

The current skill has no mechanism to route findings this way. Every entry to the solve pipeline goes through S-1 or S-2.

ADR-0068 adds a post-solve retrospective step (C-4) to solve-adr. The two decisions are independent additions — this ADR does not depend on ADR-0068 landing first.

## Options

### Option A: S-3 fast-path scenario in solve-adr

Add a new scenario S-3 to solve-adr. S-3 accepts a list of findings and:

1. Classifies each finding: does it create or change an architectural decision?
   - Yes → create a Y-statement ADR inline (no author-adr exploration), fill quality-strategy, delegate to implement-adr
   - No → add to plan directly with a note referencing the finding source
2. Batches all ADR-needed findings into one implement-adr delegation.
3. Includes non-decision findings in the same plan, tagged with their source.

No author-adr invocation for exploration — the Y-statement is written directly in solve-adr. Author-adr is still invoked for the review phase (A-3 onward) on any Y-statement ADRs created.

### Option B: Skip-exploration flag in S-1

Add a `skip_exploration = true` mode to S-1 that bypasses author-adr's option discovery and jumps straight to the decision step.

This reuses S-1 machinery but requires author-adr to support a skip mode it doesn't currently have, and the routing logic for non-decision findings still has no home.

### Option C: Status quo — use S-1 for everything

Accept that retrospective and bug bash findings go through full S-1. Non-decision findings are handled ad hoc.

## Evaluation Checkpoint (Optional)

**Assessment:** Skipped — options are well-understood; Option A is clearly scoped and doesn't require prototyping.

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None.

## Decision

Add S-3 fast-path scenario to solve-adr.

In the context of retrospective findings, bug bash findings, and amendments where the decision is already understood, facing overhead from full S-1 exploration for pre-decided changes and unnecessary ADR creation for non-decision findings, we chose S-3 fast-path over modifying S-1 or author-adr to achieve direct finding classification, inline Y-statement ADR authoring for decision-changing findings, and direct plan inclusion for non-decision findings with source notes, accepting that S-3 bypasses exploration and option evaluation entirely.

**S-3 procedure (in SKILL.md):**

1. **Classify findings** — for each finding, apply the ADR list test: "Would seeing this in the ADR list give useful ambient context to someone orienting in this codebase?"
   - Yes (ADR-worthy) → route to step 2
   - No (plan-only) → route to step 3
2. **Author Y-statement ADRs** — for each ADR-worthy finding:
   - Create an ADR file using the Makefile
   - Populate standard metadata: Date, Status, Last Updated, Links
   - Write provenance in the Context section: where the finding came from (e.g., "From the Q1 retrospective: …", "From the bug bash on YYYY-MM-DD: …"), what the finding was, and why it warrants an ADR
   - Write the decision as a Y-statement in the Decision section
   - Fill the Quality Strategy section
   - Skip Options and Evaluation Checkpoint — not needed for pre-decided changes
   - Invoke author-adr A-3 onward (review, revise, re-review) — no exploration. The reviewer must be informed that Options and Evaluation Checkpoint are absent by design: R-1 criterion 1 (≥2 alternatives) does not apply to S-3 ADRs.
3. **Build the plan** — collect all non-decision findings and add them to the implement-adr plan with a `[Source: <finding-origin>]` note on each item
4. **Delegate** — invoke implement-adr with all Ready ADRs from step 2 and the non-decision task list from step 3 in a single batch

**Trigger:** S-3 activates when the user provides findings from:
- A retrospective
- A bug bash
- An amendment to an existing decision

**Routing:** Added to the S-0 routing table in SKILL.md alongside S-1 and S-2.

## Consequences

**Positive:**
- Retrospective and bug bash workflows are no longer blocked by full exploration overhead.
- Non-decision findings flow to implementation without creating unnecessary ADRs.
- The plan retains traceability to finding sources via the source note. ADR-worthy findings retain full provenance in their Context section.
- Author-adr's review quality gates still apply to Y-statement ADRs — review rigor is unchanged, though review input is intentionally thinner than fully explored ADRs.

**Negative:**
- S-3 requires the caller to apply the ADR list test correctly. A finding that would add ambient value but is misclassified as plan-only will be missing from the decision log — future readers lose context.
- Y-statement ADRs have less context than fully explored ADRs. Reviewers must accept this is intentional for this scenario.

**Neutral:**
- S-3 does not replace S-1 or S-2. It is additive.
- The author-adr skill is unchanged — S-3 only invokes its review phase, not its exploration phase.

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

- SKILL.md for solve-adr must be updated: S-3 section added, routing table updated, Configuration section updated if any new preference keys are introduced.
- The `[Source: <finding-origin>]` note format should be defined clearly so implement-adr can surface it in the plan.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** Option B was considered but rejected because it pushes routing complexity into author-adr, which is not the right owner for finding classification.

---

## Comments

### Draft Worksheet

**Framing:** Add a fast-path scenario to solve-adr for findings that arrive with decisions already made. Classify findings using the ADR list test ("would this add useful ambient context when browsing the list?") — ADR-worthy findings get Y-statement ADRs (no exploration); plan-only findings go straight to the plan with a source note.

**Tolerance:**
- Risk: Low — this is additive; S-1 and S-2 are unchanged
- Change: Low — S-3 is a new scenario, not a modification to existing scenarios
- Improvisation: Low — the workflow is clearly scoped

**Uncertainty:** None — the trigger types (retro, bug bash, amendments) and classification rule (changes a decision vs doesn't) are clear.

**Options:**
- Target count: 2-3
- [ ] Explore additional options beyond candidates listed above

<!-- Review cycle 1 — 2026-04-10 — Verdict: Revise. 3 addressed (M: consequence P4 reframed, review criteria adaptation noted in S-3 step 2; L: ADR-0068 relationship added to Context). -->
<!-- Review cycle 2 — 2026-04-10 — Verdict: Accept. All 3 cycle-1 findings verified as addressed. No new issues. -->
