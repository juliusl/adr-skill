# Reviewing an ADR

Self-contained reference for the ADR review workflow. Read this file when the user asks to review, critique, validate, or give feedback on an ADR.

Use this reference as a prompt for a general-purpose agent to perform the review. The review process is structured into six steps executed in order.

## Review Process

### Step 1: Completeness Check (ecADR)

Verify the five Definition of Done criteria:

1. **Evidence** — Is there confidence the design will work? (PoC, spike, peer validation)
2. **Criteria** — Are ≥2 genuine alternatives identified and compared?
3. **Agreement** — Is there stakeholder consensus? Was involvement planned early?
4. **Documentation** — Is the decision captured in a template and shared?
5. **Realization/Review** — Is implementation scheduled? Is there a revisit date?

Report which criteria are met and which are missing.

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

### Completeness (ecADR)
- Evidence: ...
- Criteria: ...
- Agreement: ...
- Documentation: ...
- Realization/Review: ...

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
