# AD Practices — Detailed Reference

> These practices point at ADR capturing advice but do not necessarily endorse
> all of them.
>
> Practice notes are consolidated in
> [assets/PRACTICES_NOTES.md](../assets/PRACTICES_NOTES.md), organized as
> tagged fragments. Each fragment carries `<!-- tags: ... -->` comments indicating
> which user intent it supports (e.g., `create`, `review`, `assess`, `template`,
> `justify`, `adopt`). When an agent needs to prepare context, it can scan the
> tags to load only the relevant fragments.
>
> Supplemental context (adoption models, criteria tips, alternative tooling) is in
> [assets/SUPPLEMENTAL.md](../assets/SUPPLEMENTAL.md), also tagged.

## AD Making

The [Design Practice Repository](https://socadk.github.io/design-practice-repository/)
and the [Design Practice Reference](https://leanpub.com/dpr) (LeanPub e-Book)
feature AD making and capturing as one of the essential activities (Mirko Stocker
and Olaf Zimmermann, 2021-2024).

**Decision criteria abstraction**
([Jacqui Read, 2024](../assets/SUPPLEMENTAL.md#decision-criteria-abstraction)):
- Normalize criteria DOWN to the same abstraction level (3-4 sub-criteria per parent)
- Avoid weighted scoring — it creates false precision and maintenance burden
- Example: decompose "Maintainability" → API stability, release frequency, developer familiarity

**Seven AD Making Fallacies**
([Olaf Zimmermann, 2025](../assets/PRACTICES_NOTES.md#seven-ad-making-fallacies)):
- Seven fallacies paired with countermeasures — Blind flight, Following the crowd,
  Anecdotal evidence, Blending whole/part, Abstraction aversion, Golden hammer, Time irrelevance
- Bonus: AI über-confidence — using AI-generated design advice without QA
- Core advice: AD making is a team sport; avoiding big mistakes pays off more than
  finding optimal solutions

## Good ADRs — and How to Get to Them

### Core Guidance (by adr.github.io maintainers)

1. **Definition of Ready (START)** —
   [Notes](../assets/PRACTICES_NOTES.md#definition-of-ready-start)
   - Use the **START** checklist before beginning an AD: **S**takeholders known,
     **T**ime/MRM has come, **A**lternatives understood, **R**equirements documented,
     **T**emplate chosen
   - Identify "big/early ADs" via 7 markers (high significance, financial cost,
     long execution time, many dependencies, etc.)
   - Apply the Most Responsible Moment — don't decide too early or too late

2. **Architectural Significance Test** —
   [Notes](../assets/PRACTICES_NOTES.md#asr-test--core-decisions)
   - Score issues against 7 criteria: business value/risk, key stakeholder concern,
     QoS deviation, external dependencies, cross-cutting nature, FOAK, past troublemaker
   - Takes 1-2 minutes per item; more criteria met = higher significance
   - ECSA core decisions: architectural style, technology stacks, integration options,
     governance, dev environment standards

3. **How to Create ADRs — and How Not To** —
   [Notes](../assets/PRACTICES_NOTES.md#adr-creation)
   - 7 good practices: prioritize by significance, don't defer high-impact decisions,
     root in requirements, invest in editorial quality, split complex decisions into stages
   - 11 anti-patterns grouped by type: Fairy Tale, Sales Pitch, Sprint, Tunnel Vision,
     Mega-ADR, Pseudo-accuracy, and more
   - An ADR is an executive summary, not a novel

4. **MADR Template Primer** —
   [Notes](../assets/PRACTICES_NOTES.md#madr-template-primer)
   - Start with MADR Light (5 core sections) if templates feel overwhelming
   - Full template: Title → Metadata → Context → Drivers → Options → Outcome →
     Consequences → Validation → Pros/Cons → More Info
   - Evaluate all options at the same abstraction level; use plain Markdown for
     version control compatibility

5. **Definition of Done (ecADR)** —
   [Notes](../assets/PRACTICES_NOTES.md#definition-of-done-ecadr)
   - **e**vidence (design will work), **c**riteria (≥2 options compared),
     **A**greement (stakeholder consensus), **D**ocumentation (captured & shared),
     **R**ealization/review (implementation scheduled, revisit date set)
   - Quick self-test: Are we confident? ≥2 options? Discussed adequately?
     Captured & shared? Know when to revisit?

6. **Architectural Decision Making History** —
   [Notes](../assets/PRACTICES_NOTES.md#architectural-decision-making--y-statement--justifications)
   - Y-statement: "In context of {X}, facing {Y}, we decided for {Z} and
     neglected {alternatives}, to achieve {benefits}, accepting {drawbacks}"
   - Good justifications: prior experience, PoC results, available skills
   - Bad justifications: "everybody does it," "we've always done it," resume-driven

7. **How to Review ADRs — and How Not To** —
   [Notes](../assets/PRACTICES_NOTES.md#adr-review)
   - 3 perspectives: friendly peer → stakeholder → design authority
   - 7-point checklist: problem significant? options solve it? criteria valid?
     prioritized? solution addresses problem? consequences objective? actionable?
   - 7 anti-patterns: Pass Through, Copy Edit, Siding, Self Promotion,
     Power Game, Offended Reaction, Groundhog Day

8. **Adoption Model** —
   [Notes](../assets/SUPPLEMENTAL.md#ad-adoption-model)
   - 5 levels from Undefined → Ad-hoc → Encouraged → Systematic → Optimized
   - 7 dimensions: usage scenario, scope, structure, process, tools, reviews, learning
   - Level 4 is an ideal practical target for most teams; start simple with
     markdown files before investing in tooling

### Third-party Articles

- **Documenting Architecture Decisions**
  ([Fabian Keller](../assets/SUPPLEMENTAL.md#standardizing-decision-documentation)):
  Standardize documentation using templates (lowers barrier, ensures completeness).
  At the end of every meeting, ask: "Did we just take an architectural decision
  worth documenting?" Compares Nygard and Tyree/Akerman templates.

## ADRs in the Media

- [Sustainable Architectural Design Decisions](../assets/zdun-sustainable-architectural-decisions.md)
  — Zdun et al., InfoQ. Introduces the Y-statement and sustainability criteria.
- [Documenting Architecture Decisions](../assets/nygard-documenting-architecture-decisions.md)
  — Michael Nygard's seminal 2011 blog post defining the ADR format.
- [Architectural Knowledge Management (AKM)](../assets/SUPPLEMENTAL.md#academic-research-akm)
  — OST IFS research pointers, decision templates, and tools.
