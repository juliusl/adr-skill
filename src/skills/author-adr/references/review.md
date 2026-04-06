# Reviewing an ADR

Self-contained reference for the ADR review workflow. Read this file when the user asks to review, critique, validate, or give feedback on an ADR.

Use this as the review prompt for the configured review agent (per ADR-0031, `[author.dispatch].review`; defaults to general-purpose). Custom agents get the same checks — their persona shapes judgment, not task structure. The review process is structured into six steps executed in order.

## Review Process

### Step 1: Implementability Check

Verify 6 criteria that predict whether `implement-adr` can successfully plan from this ADR:

| # | Criterion | What to Check |
|---|-----------|---------------|
| 1 | **Criteria** | Are ≥2 genuine alternatives identified and compared? |
| 2 | **Documentation** | Is the decision captured in the template and shared? |
| 3 | **Experimentation Tolerance** | Does this decision need more data to support it, or would it benefit from prototyping? Assess on the spectrum below. Flag only when the ADR appears to need data it doesn't have and isn't framed as experimental. |
| 4 | **Scope Clarity** | Are the boundaries of what's in and out of scope clear enough to decompose into tasks? Can the agent identify what files, components, or interfaces are affected? |
| 5 | **Actionable Consequences** | Can test/acceptance criteria be derived from the stated consequences? Are consequences specific enough to verify, or are they vague aspirations? |
| 6 | **Dependency Visibility** | Are links to related ADRs, external systems, or prerequisites explicit? Would `implement-adr` discover missing dependencies during planning? |

Report which criteria are met and which are missing.

#### Checkpoint State Review (ADR-0024)

If the ADR uses the checkpoint template format, also review the state of each checkpoint:

- **Evaluation Checkpoint** — check whether the assessment was `Proceed`, `Pause for validation`, or `Skipped`. If `Skipped`, evaluate whether the rationale is sound. If the checkpoint is **blank** (no assessment written), flag this as a finding — it means the checkpoint was ignored, not consciously skipped.
- **Conclusion Checkpoint** — check whether the assessment was `Ready for review`, `Needs work`, or `Skipped`. A blank conclusion checkpoint suggests the ADR was not self-checked before requesting review.
- **Validation needs** — if populated, check whether the listed experiments were actually run and findings incorporated. If validation needs are listed but not addressed, flag as a gap.

Note: ADRs created before ADR-0024 will not have checkpoint sections. This is expected and not a finding.

#### Experimentation Tolerance Spectrum

Rather than a binary pass/fail, assess on a three-point spectrum:

- **Well-supported** — data, PoC, or prior experience backs the decision. No concern.
- **Needs validation** — the decision makes claims that could be tested but aren't. Recommend a prototype or spike before implementation.
- **Deliberately experimental** — the decision explicitly acknowledges uncertainty and is designed to learn. Acceptable if the ADR frames it honestly (not a Fairy Tale anti-pattern).

The key insight: "flying blind" is fine when it's *intentional* and *acknowledged*. It's a problem when the ADR presents unvalidated claims as fact.

### Step 2: Fallacy Scan

Check the ADR's justification against the seven architectural decision-making fallacies. Flag any that apply:

| # | Fallacy | What to Look For |
|---|---------|------------------|
| 1 | **Blind flight** | Missing context, no NFRs, no measurable quality goals |
| 2 | **Following the crowd** | "Industry standard," "everyone uses it," popularity as justification |
| 3 | **Anecdotal evidence** | Single project success/failure story as entire rationale |
| 4 | **Blending whole and part** | Rejecting an entire style because one element doesn't fit |
| 5 | **Abstraction aversion** | Comparing concepts with products (REST vs. SOAP, style vs. framework) |
| 6 | **Golden hammer** | Only one option considered, or dismissing alternatives without evaluation |
| 7 | **Time irrelevance** | Using outdated benchmarks/evaluations without re-validation |

Also check for the **AI über-confidence** bonus fallacy: AI-generated justifications presented without QA or accountability.

For each detected fallacy, provide:
- The specific text that triggers the concern
- Which fallacy it matches
- A concrete countermeasure recommendation

### Step 3: Anti-Pattern Check

Scan for the 11 ADR creation anti-patterns:

**Subjectivity:**
- **Fairy Tale** — shallow justification without real evidence
- **Sales Pitch** — marketing language instead of technical analysis
- **Free Lunch Coupon** — ignoring negative consequences
- **Dummy Alternative** — including obviously bad options to make the chosen one look good

**Time dimension:**
- **Sprint** — only one option evaluated
- **Tunnel Vision** — ignoring broader context
- **Maze** — topic and content don't match

**Size/nature:**
- **Blueprint/Policy in Disguise** — not actually a decision
- **Mega-ADR** — too many decisions bundled together

**Magic tricks:**
- **Pseudo-accuracy** — false quantitative scoring to disguise subjective judgment

### Step 4: Consequence Validation (Interactive)

Review the Consequences section (Nygard format) or Pros/Cons sections (MADR format) with the user to verify factual accuracy. AI-drafted ADRs are especially prone to plausible-sounding but unverified assertions in this section.

For each stated consequence or pro/con:

1. **Present the assertion** — quote the specific consequence from the ADR.
2. **Assess plausibility** — flag assertions that appear speculative, unsubstantiated, or overly optimistic/pessimistic. Look for:
   - Quantitative claims without cited evidence (e.g., "reduces latency by 50%")
   - Unqualified absolutes ("eliminates all risk," "no downsides")
   - Predictions stated as facts ("this will become the industry standard")
3. **Ask the user directly** — confirm whether the stated consequence matches their understanding of reality. For example:

   > The ADR states: "Reduces deployment time by 50%." > Is this based on measured data, an estimate, or an assumption? Should we > qualify this assertion?

4. **Suggest revisions** — if the user indicates a consequence is inaccurate or unverified, propose revised wording that accurately reflects the level of certainty (e.g., "Expected to reduce deployment time" vs. "Reduces deployment time by 50%").

If the ADR has many consequences, you may group related ones and ask about them together rather than one at a time — but never skip this step entirely.

**Autonomous mode:** When reviewing in autonomous mode (no interactive user), the reviewer assesses consequence plausibility on its own. Flag any assertion scored as speculative with an inline note (e.g., `<!-- Unverified: [reason] -->`). These become items for the user to verify before the ADR reaches Accepted status. The reviewer cannot confirm real-world accuracy — it can only flag assertions that lack cited evidence or use unqualified absolutes.

### Step 5: Review Checklist

Answer each question:

1. Is the problem significant enough to warrant an ADR?
2. Do the considered options actually solve the stated problem?
3. Are the evaluation criteria valid and relevant?
4. Are criteria appropriately prioritized?
5. Does the chosen solution address the problem and criteria?
6. Are consequences (positive and negative) reported objectively?
7. Is the ADR actionable and traceable?

### Step 6: Verdict

Provide a summary verdict:

- **Accept** — ADR is ready. Minor suggestions only.
- **Revise** — ADR has addressable gaps. List specific changes needed.
- **Rethink** — Fundamental issues with the decision or analysis. Explain why.

> **Note:** The "Accept" verdict means the ADR passes review and is ready for implementation. It does **not** trigger a status transition to `Accepted` — that status is set by the `implement-adr` skill after plan execution. The `author-adr` skill caps at `Proposed` status.

When the verdict is **Accept**, append a review cycle marker to the ADR's `## Comments` section to record that the review ran (see `revise.md` Step 5c):

```markdown
<!-- Review cycle 1 — [YYYY-MM-DD] — Verdict: Accept. No findings. -->
```

### Accept-with-Suggestions Polish Pass

When the verdict is **Accept** but includes minor suggestions (e.g., editorial improvements, phrasing refinements, additional context), those suggestions are recorded in the review cycle marker but not acted on by default. To prevent silently dropping feedback:

1. **If an editor agent is configured** (`[author.dispatch].editor`): dispatch the editor agent with the Accept suggestions for a lightweight polish pass. The editor applies non-blocking improvements and logs what it changed. This is not a full revision cycle — no re-review is needed.
2. **If no editor agent is configured** (`editor = "interactive"` or absent): present the suggestions to the user as optional improvements. Let the user decide whether to apply them.

The polish pass is optional — Accept means the ADR is ready. But configured editor agents should apply minor feedback rather than drop it.

## Review Perspectives

Apply three perspectives progressively:

1. **Friendly peer/coach** — early feedback for improvement
2. **Official stakeholder** — adequacy confirmation and agreement
3. **Formal design authority** — approval and enforcement

> Prioritize comments by urgency (H/M/L), provide concrete
> finding-recommendation pairs, and lead with questions rather than demands.

## Review Anti-Patterns (Self-Check)

Do NOT fall into these reviewer anti-patterns:

1. **Pass Through** — few/shallow comments, rubber-stamping
2. **Copy Edit** — commenting on grammar/formatting instead of substance
3. **Siding** — switching topics to reviewer's pet concerns
4. **Self Promotion** — conflict of interest, pushing own solutions
5. **Power Game** — using hierarchy to override technical merit
6. **Offended Reaction** — defensive response to critique
7. **Groundhog Day** — repeating previously resolved feedback

> Lead with questions rather than demands. Review like you want to be reviewed.

## Output Format

Structure the review as:

```markdown
## ADR Review: [title]

### Implementability
- Criteria: ...
- Documentation: ...
- Experimentation Tolerance: ...
- Scope Clarity: ...
- Actionable Consequences: ...
- Dependency Visibility: ...
- Checkpoint State: ... (if applicable — Evaluation/Conclusion checkpoint assessments)

### Fallacies Detected
[list or "None detected"]

### Anti-Patterns Detected
[list or "None detected"]

### Consequence Validation
[list of assertions reviewed with user, any revisions made]

### Checklist
[7 answers]

### Verdict: [Accept/Revise/Rethink]
[summary and specific recommendations]
```
