---
tags:
  create: Fragments relevant to creating ADRs
  review: Fragments relevant to reviewing ADRs
  assess: Fragments relevant to assessing architectural significance
  template: Fragments relevant to choosing or filling out a template
  justify: Fragments relevant to writing decision rationale
  adopt: Fragments relevant to organizational ADR adoption
---

# Practice Notes

> Consolidated practice fragments, each tagged by relevant user intent.
> When preparing context, filter by tag to load only the fragments needed.
>
> **Tags:** `create` · `review` · `assess` · `template` · `justify` · `adopt`

---

## ASR Test & Core Decisions
<!-- tags: assess, create -->

*Archive: [archive/ozimmer-asr-test-ecsa-decisions.md](archive/ozimmer-asr-test-ecsa-decisions.md)*

A seven-criteria test to rapidly assess (1-2 min per item) whether a requirement
or issue is architecturally significant and warrants an ADR.

### The 7 ASR Criteria

1. **Business value/risk** — associated with high business value or risk
2. **Key stakeholder concern** — raised by sponsor, auditor, or key stakeholder
3. **QoS deviation** — runtime quality-of-service characteristics deviate substantially from norms
4. **External dependencies** — depends on systems with unpredictable/uncontrollable behavior
5. **Cross-cutting nature** — has system-wide impact across components
6. **First-of-a-Kind (FOAK)** — team's first experience with this type of problem
7. **Past troublemaker** — previously troublesome on similar projects

- Score each issue Y/N/?/H/M/L against the 7 criteria; more criteria met = higher significance
- Prioritize issues with high business value/risk and cross-cutting impact first
- **ECSA Core Decisions** that typically need early attention: minimal functionality/regulations, architectural style, technology stacks, integration options, governance structure, development environment standards

---

## Definition of Ready (START)
<!-- tags: create, assess -->

*Archive: [archive/ozimmer-ad-definition-of-ready.md](archive/ozimmer-ad-definition-of-ready.md)*

Five criteria determine when an Architectural Decision is ready to be made.

- **S** — Stakeholders known
- **T** — Time/Most Responsible Moment has come
- **A** — Alternatives understood
- **R** — Requirements/criteria documented
- **T** — Template chosen

Identify "big/early ADs" using 7 markers: high significance, financial cost,
long execution time, many dependencies, long decision-making time, high
abstraction level, unusual problem/solution space.

Apply the Most Responsible Moment principle — don't decide too early (reduces
flexibility) or too late (causes costly rework).

### ADMM (5 Logical Steps)

1. Identification of design issue & options
2. Criteria collection & option analysis
3. Decision making
4. Decision capturing (ADR documentation)
5. Decision enforcement

---

## ADR Creation
<!-- tags: create, justify -->

*Archive: [archive/ozimmer-adr-creation.md](archive/ozimmer-adr-creation.md)*

Best practices and anti-patterns for writing valuable ADRs. ADRs function as
executive summaries, verdicts, letters of intent, and action plans.

### 7 Good Practices

1. Select by priority and architectural significance (use the ASR Test)
2. Don't defer high-impact, hard-to-reverse decisions
3. Prioritize meta-qualities (observability, reactivity) over presumed long-term goals
4. Root decisions in actual requirements and personal experience
5. Invest in editorial quality — write clearly and concisely
6. Split complex decisions into stages (short-term → mid-term → long-term)
7. Disclose confidence level and acknowledge biases

### 11 Anti-Patterns (Grouped)

- **Subjectivity**: Fairy Tale, Sales Pitch, Free Lunch Coupon, Dummy Alternative
- **Time dimension**: Sprint, Tunnel Vision, Maze
- **Size/nature**: Blueprint/Policy in Disguise, Mega-ADR, Novel/Epic
- **Magic Tricks**: Non-existent urgency, Problem-solution mismatch, Pseudo-accuracy

> An ADR is an executive summary, not a novel. It should answer "why this
> option?" with honest tradeoffs, not sell a predetermined conclusion.

---

## Definition of Done (ecADR)
<!-- tags: create, review -->

*Archive: [archive/ozimmer-ad-definition-of-done.md](archive/ozimmer-ad-definition-of-done.md)*

Five criteria define when an Architectural Decision is complete and ready for
execution.

1. **Evidence** — confident the design will work (via PoC, spike, trusted peer validation)
2. **Criteria** — ≥2 alternatives identified and compared systematically
3. **Agreement** — peer/team consensus reached with adequate stakeholder involvement
4. **Documentation** — decision captured in a template and shared
5. **Realization/Review** — implementation scheduled; revisit date and expiration defined

### Quick Self-Test

1. Are we confident this design will work?
2. Have we decided between ≥2 genuine options?
3. Have we discussed adequately with stakeholders?
4. Have we captured and shared the decision?
5. Do we know when to revisit?

> Don't treat agreement as a rubber stamp — plan stakeholder involvement early.

---

## ADR Review
<!-- tags: review -->

*Archive: [archive/ozimmer-adr-review.md](archive/ozimmer-adr-review.md)*

Best practices for reviewing ADRs across three perspectives, with a concrete
checklist and anti-patterns to avoid.

### 3 Review Perspectives

1. **Friendly peer/coach** — early feedback for improvement
2. **Official stakeholder** — adequacy confirmation and agreement
3. **Formal design authority** — approval and enforcement

### 7-Point Review Checklist

1. Is the problem significant enough to warrant an ADR?
2. Do the considered options actually solve the stated problem?
3. Are the evaluation criteria valid and relevant?
4. Are criteria appropriately prioritized?
5. Does the chosen solution address the problem and criteria?
6. Are consequences (positive and negative) reported objectively?
7. Is the ADR actionable and traceable?

### 7 Review Anti-Patterns

1. **Pass Through** — few/shallow comments, rubber-stamping
2. **Copy Edit** — grammar/formatting over content substance
3. **Siding** — switching topics to reviewer's pet concerns
4. **Self Promotion** — conflict of interest, pushing own solutions
5. **Power Game** — using hierarchy to override technical merit
6. **Offended Reaction** — defensive response to critique
7. **Groundhog Day** — repeating previously resolved feedback

> Prioritize comments by urgency (H/M/L), provide concrete
> finding-recommendation pairs, and lead with questions rather than demands.

---

## Seven AD Making Fallacies
<!-- tags: create, review, justify -->

*Archive: [ozimmer-seven-ad-fallacies.md](ozimmer-seven-ad-fallacies.md)*

Seven common fallacies that undermine architectural decision quality, each paired
with a countermeasure.

| # | Fallacy | Countermeasure |
|---|---------|----------------|
| 1 | **Blind flight** — skip context/NFR analysis | **Agree on landing zones** — elicit specific, measurable NFRs |
| 2 | **Following the crowd** — what works for others works for us | **Beat the street** — validate fit against your own requirements |
| 3 | **Anecdotal evidence** — one example justifies everything | **Balanced judgments** — use SMART NFRs, make tradeoffs explicit |
| 4 | **Blending whole and part** — if one element is bad, the whole is bad | **Divide and conquer** — separate system-wide from local concerns |
| 5 | **Abstraction aversion** — mix concepts, tech, and products in one ADR | **Navigate abstract↔concrete** — separate conceptual from technological ADRs |
| 6 | **Golden hammer / silver bullet** — one size fits all | **Grow your toolbox** — stay curious, learn from peers |
| 7 | **Time irrelevance** — old evidence never expires | **Look back, think ahead** — set review dates, re-validate measurements |

**Bonus: AI über-confidence** — using AI-generated design advice without QA;
baking fallacies into prompts yields fallacious outputs.

> AD making is a team sport — group decisions mitigate fallacy risk.

---

## Architectural Decision Making — Y-Statement & Justifications
<!-- tags: create, justify, template -->

*Archive: [archive/ozimmer-architecture-decision-making.md](archive/ozimmer-architecture-decision-making.md)*

### Y-Statement Template (6 Parts)

> In the context of **{use case}**, facing **{concern}**, we decided for
> **{option}** and neglected **{alternatives}**, to achieve **{benefits}**,
> accepting that **{drawbacks}**.

### Good vs. Bad Justifications

**Good (grounded in):**
- Prior successful project experience in a similar context
- PoC/PoT with convincing results
- Available market skills for the chosen technology

**Bad (pseudo-rationale):**
- "Everybody does it" — bandwagon fallacy
- "We've always done it this way" — inertia
- "It'll look good on my resume" — personal interest over project needs

- Focus on architecturally significant decisions only; avoid logs with 100+ entries
- Answer "why?" — rationale is the most valuable part of any ADR
- Pick a template and stick to it; don't over-document

---

## MADR Template Primer
<!-- tags: create, template -->

*Archive: [archive/ozimmer-madr-template-primer.md](archive/ozimmer-madr-template-primer.md)*

Section-by-section walkthrough of the MADR template.

### Full MADR Sections

1. **Title** — short noun phrase naming the decision
2. **Metadata** — status, date, deciders, consulted, informed
3. **Context and Problem Statement** — 2-3 sentences or user story form
4. **Decision Drivers** — desired qualities, forces, concerns (bullet list)
5. **Considered Options** — list of genuine alternatives
6. **Decision Outcome** — chosen option with justification
7. **Consequences** — Good/Bad, listed separately
8. **Validation/Confirmation** — how compliance will be verified (optional)
9. **Pros and Cons of Options** — detailed per-option analysis
10. **More Information** — evidence, confidence, realization plan

- Start with **MADR Light** (5 core sections) if templates feel overwhelming
- Evaluate all options at the same abstraction level
- Document "Good, because…" and "Bad, because…" separately for each option
- Status values: proposed → accepted → deprecated → superseded by
