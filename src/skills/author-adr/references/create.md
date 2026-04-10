# Creating an ADR

Self-contained reference for the ADR creation workflow. Read this file when the user asks to create, draft, or write an ADR.

**If a step is skipped, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

## Good Practices

1. **Select by priority and architectural significance** — use the ASR Test
2. **Don't defer high-impact, hard-to-reverse decisions**
3. **Prioritize meta-qualities** (observability, reactivity) over presumed long-term goals
4. **Root decisions in actual requirements** and personal experience
5. **Invest in editorial quality** — write clearly and concisely
6. **Split complex decisions into stages** (short-term → mid-term → long-term)
7. **Disclose confidence level** and acknowledge biases

## Anti-Patterns to Avoid

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

> An ADR is an executive summary, not a novel. It should answer "why this option?" with honest tradeoffs, not sell a predetermined conclusion.

## Decision Criteria

When defining evaluation criteria, normalize DOWN to the same abstraction level by decomposing high-level criteria into 3–4 specific sub-criteria. Avoid weighted scoring systems — they create false precision and maintenance burden.

**Example:** "Maintainability" (too abstract) → API stability, major release frequency, developer familiarity (concrete, comparable).

## Writing Justifications

**Y-Statement Template:**
> In the context of **{use case}**, facing **{concern}**, we decided for we chose **{option}** over **{alternatives}** to achieve **{benefits}**, accepting **{drawbacks}**.

**Good justifications (grounded in):**
- Prior successful project experience in a similar context
- PoC/PoT with convincing results
- Available market skills for the chosen technology

**Bad justifications (pseudo-rationale):**
- "Everybody does it" — bandwagon fallacy
- "We've always done it this way" — inertia
- "It'll look good on my resume" — personal interest over project needs

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

## Procedure

| ID | Description |
|----|-------------|
| Step 1 | Assess architectural significance using the ASR Test |
| Step 2 | Evaluate decision readiness — timing, urgency, reversibility |
| Step 2a | Identifying big/early decisions |
| Step 2b | Most responsible moment |
| Step 2c | ADMM — 5 logical steps for decision-making |
| Step 3 | Draft the ADR — populate from template |
| Step 3a | Draft Worksheet integration |
| Step 3b | Tech-Writer Dispatch (conditional) — delegate writing to configured agent |
| Step 4 | Evaluate Readiness — Evaluation Checkpoint gate |
| Step 4a | UX/DX Option Review (conditional) — dispatch reviewers on Options |
| Step 4b | TPM Decision Quality Assessment (conditional) — dispatch TPM with findings |
| Step 5 | Validate Completion — implementability criteria |
| Step 5a | Quick self-test |
| Step 6 | Conclusion Checkpoint — verify before requesting review |

```
Step 1 — Assess architectural significance
  ↓
Step 2 — Evaluate decision readiness
  ↓
Step 3 — Draft the ADR
  ├─ Step 3a — Draft Worksheet integration
  └─ Step 3b — Tech-Writer Dispatch (conditional: tech_writer configured?)
  ↓
Step 4 — Evaluation Checkpoint (conditional: Proceed / Pause / Skip)
  ├─ Step 4a — UX/DX Option Review (conditional: ux_review or dx_review configured?)
  └─ Step 4b — TPM Decision Quality Assessment (conditional: tpm configured?)
  ↓
Step 5 — Validate Completion (implementability criteria)
  ↓
Step 6 — Conclusion Checkpoint (conditional: Ready / Needs work / Skip)
```

**Conditional steps:** Step 4 and Step 6 are checkpoints with three possible assessments. If the assessment is "Skipped", record the rationale and proceed to the next step. Step 3b is conditional on `tech_writer` being configured. Step 4a is conditional on `ux_review` or `dx_review` being configured. Step 4b is conditional on `tpm` being configured.

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

### Step 2a: Identifying Big/Early Decisions

Look for 7 markers: high significance, financial cost, long execution time, many dependencies, long decision-making time, high abstraction level, unusual problem/solution space.

### Step 2b: Most Responsible Moment

Don't decide too early (reduces flexibility) or too late (causes costly rework). The MRM is the point where waiting costs more than deciding now.

### Step 2c: ADMM — 5 Logical Steps

1. Identification of design issue & options
2. Criteria collection & option analysis
3. Decision making
4. Decision capturing (ADR documentation)
5. Decision enforcement

## Step 3: Draft the ADR

### Step 3a: Draft Worksheet Integration

Before drafting, check if a **Draft Worksheet** exists in the ADR's `## Comments` section (per ADR-0032). If present, use it to ground the drafting process:

- **Framing** → seed the Context section. Use the author's stated direction as the starting point for the problem description.
- **Tolerance** → calibrate option depth. Low risk/change tolerance suggests fewer, more conservative options. High tolerance opens the door to experimental approaches.
- **Candidates** → pre-populate the Options section with the author's identified candidates, expanding each into a full option structure.
- **Uncertainty** → inform which areas need more analysis in Context and which constraints are firm vs. tentative.

If no worksheet exists, proceed with the standard drafting workflow below — A-1 should have created one, but the workflow must not block if it's absent.

### Step 3b: Tech-Writer Dispatch (Conditional)

**Condition:** `[author.dispatch].tech_writer` is configured with a non-empty value.

When a tech-writer agent is configured, delegate the ADR body writing to it instead of writing inline.

1. **Build dispatch context** — assemble the payload for the tech-writer agent:
   - The ADR file path (with draft worksheet from A-1 and any ASR/readiness context from Steps 1–2)
   - The problem statement, constraints, and stakeholder context from the calling workflow
   - The template structure selected in Step 3 (use the active template, not a hardcoded reference)
   - Writing style instructions (from SKILL.md Writing Style section)
   - Section expectations: Context, Options, Decision, Consequences, Quality Strategy

2. **Dispatch via `task` tool** — invoke the configured tech-writer agent. The agent writes the ADR body content and returns control.

3. **Validate output** — after the tech-writer returns, verify:
   - All required sections are populated (Context, Options, Decision, Consequences)
   - Quality Strategy checkboxes reflect the decision's actual quality concerns
   - The content aligns with the draft worksheet's framing and constraints
   If validation fails (missing sections, malformed output, or partial content from a mid-execution failure), the inline agent completes or corrects the remaining sections and warns the user that tech-writer output was supplemented.

4. **Fallback** — if the configured agent cannot be resolved at runtime, fall back to inline writing (the inline agent writes the content itself) and warn the user.

**When `tech_writer` is absent or empty:** Skip this subsection — the inline agent writes content directly as the default behavior.

## Step 4: Evaluate Readiness (Evaluation Checkpoint)

After documenting options, pause at the **Evaluation Checkpoint (Optional)** section. This is an advisory gate between analysis and decision-making (per ADR-0024).

1. **Assess the options holistically** — evaluate whether the options are sufficiently analyzed to support a decision:
   - Are all options at comparable depth?
   - Are decision drivers defined and referenced?
   - Are there unvalidated claims that would benefit from experimentation?

2. **Write the Assessment value:**
   - `Proceed` — the analysis is sufficient, move to the Decision section.
   - `Pause for validation` — experiments would strengthen confidence. Populate the **Validation needs** area with specific prototypes or evidence needed. These become inputs for the `prototype-adr` skill (ADR-0023).
   - `Skipped — <rationale>` — the user chooses to skip (e.g., intuition, time pressure, trivial decision). Record the rationale.

3. **Check the baseline items** — mark each checkbox as satisfied or note why it's not applicable.

4. **If "Pause for validation"** — ask the user whether to prototype, selectively validate, or skip. The user controls what gets prototyped, not the checklist.

## Step 5: Validate Completion (Implementability Criteria)

Verify these six criteria before considering the ADR complete:

1. **Criteria** — ≥2 alternatives identified and compared systematically
2. **Documentation** — decision captured in a template and shared
3. **Experimentation Tolerance** — is the decision well-supported by data, does it need validation, or is it deliberately experimental? Flag only if unvalidated claims are presented as fact.
4. **Scope Clarity** — boundaries clear enough to decompose into implementation tasks
5. **Actionable Consequences** — consequences specific enough to derive test/acceptance criteria
6. **Dependency Visibility** — links to related ADRs, systems, or prerequisites are explicit

### Step 5a: Quick Self-Test

1. Have we decided between ≥2 genuine options?
2. Have we captured and shared the decision?
3. Is this well-supported, or honestly framed as experimental?
4. Are the scope boundaries clear enough to plan implementation?
5. Can we derive acceptance criteria from the stated consequences?
6. Are all dependencies and related decisions linked?

## Step 6: Conclusion Checkpoint

After completing the Quality Strategy section, pause at the **Conclusion Checkpoint (Optional)** section. This is an advisory gate between authoring and review (per ADR-0024).

1. **Write the Assessment value:**
   - `Ready for review` — the ADR is complete and ready for the review workflow.
   - `Needs work` — specific items need attention before review.
   - `Skipped — <rationale>` — the user chooses to skip (record why).

2. **Check the baseline items:**
   - [ ] Decision justified (Y-statement or equivalent)
   - [ ] Consequences include positive, negative, and neutral outcomes
   - [ ] Quality Strategy reviewed — relevant items checked, irrelevant struck through
   - [ ] Links to related ADRs populated

3. **Populate Pre-review notes** if there are caveats, open questions, or areas needing reviewer attention. Leave empty if none.

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
