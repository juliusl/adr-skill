# Creating an ADR

Self-contained reference for the ADR creation workflow. Read this file when the user asks to create, draft, or write an ADR.

## Step 1: Assess Architectural Significance (ASR Test)

Before creating an ADR, verify that the decision is architecturally significant. Score the issue against these 7 criteria (takes 1–2 minutes):

| # | Criterion | Ask |
|---|-----------|-----|
| 1 | **Business value/risk** | Is it associated with high business value or risk? |
| 2 | **Key stakeholder concern** | Was it raised by a sponsor, auditor, or key stakeholder? |
| 3 | **QoS deviation** | Do runtime quality-of-service characteristics deviate substantially from norms? |
| 4 | **External dependencies** | Does it depend on systems with unpredictable or uncontrollable behavior? |
| 5 | **Cross-cutting nature** | Does it have system-wide impact across components? |
| 6 | **First-of-a-Kind (FOAK)** | Is this the team's first experience with this type of problem? |
| 7 | **Past troublemaker** | Has this been previously troublesome on similar projects? |

- Score each Y/N/?/H/M/L — more criteria met = higher significance
- Prioritize issues with high business value/risk and cross-cutting impact
- **ECSA core decisions** that typically need early attention: minimal functionality/regulations, architectural style, technology stacks, integration options, governance structure, development environment standards

If the decision is not architecturally significant, suggest informal documentation (e.g., a comment, wiki page, or team channel post) instead of a formal ADR.

## Step 2: Check Readiness (START)

Verify these five criteria before beginning a decision:

- **S** — **Stakeholders** known
- **T** — **Time** / Most Responsible Moment has come
- **A** — **Alternatives** understood
- **R** — **Requirements** / criteria documented
- **T** — **Template** chosen

### Identifying Big/Early Decisions

Look for 7 markers: high significance, financial cost, long execution time, many dependencies, long decision-making time, high abstraction level, unusual problem/solution space.

### Most Responsible Moment

Don't decide too early (reduces flexibility) or too late (causes costly rework). The MRM is the last point at which the cost of deferral begins to exceed the cost of deciding now.

### ADMM — 5 Logical Steps

1. Identification of design issue & options
2. Criteria collection & option analysis
3. Decision making
4. Decision capturing (ADR documentation)
5. Decision enforcement

## Step 3: Draft the ADR

### Good Practices

1. **Select by priority and architectural significance** — use the ASR Test
2. **Don't defer high-impact, hard-to-reverse decisions**
3. **Prioritize meta-qualities** (observability, reactivity) over presumed long-term goals
4. **Root decisions in actual requirements** and personal experience
5. **Invest in editorial quality** — write clearly and concisely
6. **Split complex decisions into stages** (short-term → mid-term → long-term)
7. **Disclose confidence level** and acknowledge biases

### Anti-Patterns to Avoid

**Subjectivity:**
- **Fairy Tale** — shallow justification without real evidence
- **Sales Pitch** — marketing language instead of technical analysis
- **Free Lunch Coupon** — ignoring negative consequences
- **Dummy Alternative** — obviously bad options to make the chosen one look good

**Time dimension:**
- **Sprint** — only one option evaluated
- **Tunnel Vision** — ignoring broader context
- **Maze** — topic and content don't match

**Size/nature:**
- **Blueprint/Policy in Disguise** — not actually a decision
- **Mega-ADR** — too many decisions bundled together
- **Novel/Epic** — far too much detail for an executive summary

**Magic tricks:**
- **Non-existent urgency** — fabricating time pressure
- **Problem-solution mismatch** — solution doesn't address the stated problem
- **Pseudo-accuracy** — false quantitative scoring to disguise subjective judgment

> An ADR is an executive summary, not a novel. It should answer "why this
> option?" with honest tradeoffs, not sell a predetermined conclusion.

### Decision Criteria

When defining evaluation criteria, normalize DOWN to the same abstraction level by decomposing high-level criteria into 3–4 specific sub-criteria. Avoid weighted scoring systems — they create false precision and maintenance burden.

**Example:** "Maintainability" (too abstract) → API stability, major release frequency, developer familiarity (concrete, comparable).

### Writing Justifications

**Y-Statement Template:**
> In the context of **{use case}**, facing **{concern}**, we decided for
> **{option}** and neglected **{alternatives}**, to achieve **{benefits}**,
> accepting that **{drawbacks}**.

**Good justifications (grounded in):**
- Prior successful project experience in a similar context
- PoC/PoT with convincing results
- Available market skills for the chosen technology

**Bad justifications (pseudo-rationale):**
- "Everybody does it" — bandwagon fallacy
- "We've always done it this way" — inertia
- "It'll look good on my resume" — personal interest over project needs

## Step 4: Validate Completion (ecADR Definition of Done)

Verify these five criteria before considering the ADR complete:

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

## MADR Template Primer

If using MADR format, the full template sections are:

1. **Title** — short noun phrase naming the decision
2. **Metadata** — status, date, deciders, consulted, informed
3. **Context and Problem Statement** — 2–3 sentences or user story form
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
- Status values: prototype → proposed → accepted → deprecated → superseded by

## Seven AD Making Fallacies

Watch for these fallacies when drafting and use the countermeasures:

| # | Fallacy | Countermeasure |
|---|---------|----------------|
| 1 | **Blind flight** — skip context/NFR analysis | **Agree on landing zones** — elicit specific, measurable NFRs |
| 2 | **Following the crowd** — what works for others works for us | **Beat the street** — validate fit against your own requirements |
| 3 | **Anecdotal evidence** — one example justifies everything | **Balanced judgments** — use SMART NFRs, make tradeoffs explicit |
| 4 | **Blending whole and part** — if one element is bad, the whole is bad | **Divide and conquer** — separate system-wide from local concerns |
| 5 | **Abstraction aversion** — mix concepts, tech, and products in one ADR | **Navigate abstract↔concrete** — separate conceptual from technological ADRs |
| 6 | **Golden hammer / silver bullet** — one size fits all | **Grow your toolbox** — stay curious, learn from peers |
| 7 | **Time irrelevance** — old evidence never expires | **Look back, think ahead** — set review dates, re-validate measurements |

**Bonus: AI über-confidence** — using AI-generated design advice without QA; baking fallacies into prompts yields fallacious outputs.

> AD making is a team sport — group decisions mitigate fallacy risk.
