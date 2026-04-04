# 24. Add checkpoint sections to nygard-agent template

Date: 2026-04-04
Status: Accepted
Last Updated: 2026-04-04
Links:
- Extends [ADR-0017](0017-adopt-nygard-agent-template-as-default-adr-format.md) (template being modified)
- Motivated by [ADR-0022](0022-replace-ecadr-completeness-check-with-implementability-criteria.md) (implementability criteria need structural enforcement)
- Addresses [ADR-0023](0023-add-prototype-adr-skill-for-structured-decision-validation.md) prerequisite (author-adr must produce structured prototype objectives — Evaluation Checkpoint's "Validation needs" provides this)
- Related to [ADR-0019](0019-add-problem-first-solve-task-to-author-adr-workflow.md) (solve workflow's prototyping step lacks structure)

## Context

The nygard-agent template (ADR-0017) flows linearly through six sections: Context → Options → Decision → Consequences → Quality Strategy → Comments. There are no structured pause points where an agent (or human) is expected to stop, assess readiness, and decide whether to proceed or gather more evidence.

**The problem:** Skills operating on ADRs have no machine-readable signals for "is this ready to move from analysis to decision?" or "is this ready for review?" The current process relies on implicit judgment at two critical transitions:

1. **Options → Decision** — after analyzing alternatives, the author (human or agent) jumps directly to a decision. There is no structured moment to ask: "Do we have enough evidence? Should we prototype first? Are all options at comparable depth?" ADR-0022's Experimentation Tolerance criterion flags this gap during review, but only *after* the decision is already written. ADR-0023 introduces a prototype-adr skill, but it has no signal in the ADR telling it *what* to validate.

2. **Quality Strategy → Review** — after completing the ADR, the author hands off to review. There is no structured verification that the ADR is actually ready: links populated, consequences validated, quality strategy filled in. The review process (ADR-0022) catches these gaps, but finding them during review is wasteful — they should be caught during authoring.

**Why checkpoints solve this:**

- **Machine-readable gates** — checkbox sections (`- [ ]` / `- [x]`) give skills a parseable signal for ADR state. A skill can grep for unchecked items to determine what work remains.
- **Procedural workflow** — checkpoints turn implicit judgment calls into explicit checklists. An agent assesses readiness holistically and surfaces concerns rather than relying on ad-hoc judgment.
- **Skill interop** — `author-adr`, `prototype-adr`, and `implement-adr` need to coordinate. Checkpoints provide the handoff contract: "I've verified X, Y, Z — your turn."
- **Progressive quality** — catching gaps during authoring is cheaper than catching them during review. Checkpoints shift quality left.

**Design tensions:**

1. **Static vs. dynamic checklists.** Some checkpoint items apply universally (e.g., "all options at comparable depth"). Others are decision-specific (e.g., "benchmark query latency for Option 2"). A purely static checklist is predictable but may include irrelevant items. A purely dynamic checklist is contextual but unpredictable — skills can't know what items to expect.

2. **Template weight.** Every section added to the template increases the overhead of creating an ADR. The template is already 6 sections; adding 2 more risks making simple decisions feel bureaucratic. The checkpoints need to earn their keep by providing clear value, and they must be lightweight enough that a straightforward decision can pass through them quickly.

3. **Checkpoint naming.** The names should communicate what the checkpoint gates (the transition), not just where it sits in the document. "Intermission" suggests a pause; "Evaluation" suggests assessment; "Gate" suggests pass/fail.

4. **Advisory vs. blocking.** The checkpoints should make the authoring process *procedural* — the agent thinks holistically and surfaces concerns — but not *rigid*. Sometimes a decision is straightforward and the author wants to jump straight to implementation based on intuition. The checkpoints must support a "skip" path where the agent acknowledges the decision is low-risk, or the user overrides the agent's recommendation to pause.

### Decision Drivers

Must-haves:
- **Machine-parseable** — skills must be able to read checkpoint state programmatically (checkbox syntax)
- **Transition-gating** — each checkpoint must correspond to a specific workflow transition (analysis→decision, authoring→review)
- **Lightweight** — checkpoints must not add significant overhead for straightforward decisions
- **Skippable** — the user must be able to skip a checkpoint when a decision is straightforward or when proceeding on intuition

Nice-to-haves:
- **Skill-addressable** — checkpoint items should map to specific skill capabilities (e.g., "run prototype" maps to prototype-adr)
- **Self-documenting** — reading a checkpoint should tell you what was verified and what wasn't, without consulting external docs
- **Extensible** — the checkpoint format should allow agent-generated items alongside static ones

## Options

### Option 1: Fixed checkpoint sections with static checklists

Add two new sections to the template with predetermined checklist items. Every ADR gets the same items in the same order. Items that don't apply are struck through (`~~`) or left unchecked with a note, consistent with how Quality Strategy handles inapplicable items.

**Proposed template addition:**
```markdown
## Evaluation Checkpoint
<!-- Gate: Options → Decision. Verify before writing the Decision section. -->

- [ ] All options evaluated at comparable depth
- [ ] Decision drivers are defined and referenced in option analysis
- [ ] Claims needing validation identified (if any → prototype-adr)
- [ ] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

## Decision
...
## Conclusion Checkpoint
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

- [ ] Decision justified (Y-statement or equivalent)
- [ ] Consequences include positive, negative, and neutral outcomes
- [ ] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [ ] Links to related ADRs populated
- [ ] Ready for review
```

**Strengths:**
- Fully predictable — every ADR has the same items, skills always know what to parse
- Simple to implement — just template text, no logic needed
- Self-documenting — the checklist explains itself
- Consistent with Quality Strategy precedent (static checkbox section)

**Weaknesses:**
- One-size-fits-all — a trivial ADR (e.g., adopting a linter) gets the same checklist as a complex system redesign
- No room for decision-specific items (e.g., "benchmark migration latency")
- May feel like checkbox theater for straightforward decisions
- Static items can't capture what *specifically* needs prototyping — only that *something* might

### Option 2: Checkpoint sections with agent-populated checklists

Checkpoint section headings are fixed in the template, but they contain only a comment prompt. The agent populates the checklist items during authoring based on the ADR's specific content.

**Proposed template addition:**
```markdown
## Evaluation Checkpoint
<!-- Gate: Options → Decision. Agent: populate checklist based on options analysis. -->

## Decision
...
## Conclusion Checkpoint
<!-- Gate: Quality Strategy → Review. Agent: populate checklist based on ADR completeness. -->
```

The agent fills in items contextually, e.g.:
```markdown
## Evaluation Checkpoint

- [ ] Benchmark PostgreSQL vs. SQLite query latency on 10M row dataset
- [ ] Verify PostgreSQL connection pooling works with our async runtime
- [ ] Both options evaluated against the 3 decision drivers
```

**Strengths:**
- Contextual — items reflect the actual decision, not generic boilerplate
- Directly actionable — each item is a concrete task, not a generic question
- Connects naturally to prototype-adr — the evaluation checkpoint *is* the prototype objective list
- Lightweight for simple decisions — agent generates fewer items when fewer are needed

**Weaknesses:**
- Unpredictable — skills can't know what items to expect across different ADRs
- Agent quality dependency — if the agent generates bad items, the checkpoint is useless
- No baseline — there's nothing to fall back on if the agent fails to populate the section
- Harder to compare across ADRs — every checklist is different

### Option 3: Hybrid — static baseline with agent assessment and skip affordance

Each checkpoint section has a fixed set of baseline items (always present, machine-parseable) plus a clearly marked area for agent-generated context-specific items. The section heading is marked **(Optional)** — a first-class signal that the checkpoint can be consciously skipped. When skipped, the author records a rationale, making the skip itself a traceable decision.

The agent evaluates the decision holistically and recommends whether to pause or proceed. The user can always override.

**Proposed template addition:**
```markdown
## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** [Proceed | Pause for validation | Skipped — <rationale>]
<!-- Agent: evaluate decision complexity. Recommend "Proceed" for straightforward
     decisions, "Pause for validation" when experiments would strengthen confidence.
     User may set "Skipped" with a rationale to proceed on intuition. -->

- [ ] All options evaluated at comparable depth
- [ ] Decision drivers are defined and referenced in option analysis
- [ ] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**
<!-- Agent: list specific experiments, prototypes, or evidence that would
     strengthen this decision. Leave empty if assessment is "Proceed." -->

## Decision
...
## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** [Ready for review | Needs work | Skipped — <rationale>]

- [ ] Decision justified (Y-statement or equivalent)
- [ ] Consequences include positive, negative, and neutral outcomes
- [ ] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [ ] Links to related ADRs populated

**Pre-review notes:**
<!-- Agent: list any caveats, open questions, or areas where reviewer attention
     is needed. Leave empty if none. -->
```

**The assessment workflow:**
1. Agent reaches the Evaluation Checkpoint after populating Options.
2. Agent evaluates holistically: Is this decision trivial or complex? Are there unvalidated claims? Would experiments help?
3. Agent writes the assessment ("Proceed" or "Pause for validation") and explains briefly.
4. If "Pause" — agent populates Validation needs. These become inputs for prototype-adr (ADR-0023).
5. User decides: accept the recommendation, skip it (with recorded rationale), or selectively prototype.
6. The user controls what gets prototyped, not the checklist.

**Skip examples:**
- `Skipped — prior experience with this pattern, low risk`
- `Skipped — time-sensitive, will revisit if implementation surfaces issues`
- `Skipped — trivial decision, single clear winner`

**Strengths:**
- Predictable baseline — skills can always parse the fixed items
- Contextual extension — agent-generated items capture decision-specific needs
- Graceful degradation — if the agent doesn't populate the dynamic area, the baseline still works
- **(Optional)** heading makes skip a first-class affordance, not a workaround
- Skip rationale is self-documenting — reviewers see *why* it was skipped
- "Validation needs" is the natural input for prototype-adr (ADR-0023)
- "Pre-review notes" gives the reviewer targeted guidance
- **Advisory, not blocking** — the agent recommends; the user decides

**Weaknesses:**
- Most complex option — two layers per checkpoint (static + dynamic) plus assessment
- The dynamic area may be left empty by habit, reducing its value to boilerplate
- Template is longer than the other options
- Assessment quality depends on the agent's ability to evaluate decision complexity

## Decision

In the context of **needing procedural structure for ADR authoring so that skills can operate within well-defined bounds**, facing **two unstructured transitions in the template where implicit judgment replaces explicit verification**, we decided for **hybrid checkpoint sections with static baselines, agent-populated validation needs, and an explicit (Optional) skip affordance (Option 3)**, and neglected **purely static checklists (too rigid, no contextual adaptation) and purely agent-populated checklists (too unpredictable, no parsing anchor)**, to achieve **a procedural authoring workflow where agents think holistically about decision readiness while users retain full authority to skip or selectively engage**, accepting that **the template grows by two sections and assessment quality depends on agent capability**.

### Template Changes

The nygard-agent template gains two new sections, placed at the transitions they gate:

```
Context
Options
  ┌─────────────────────────────────┐
  │ Evaluation Checkpoint (Optional)│  ← NEW: gates Options → Decision
  └─────────────────────────────────┘
Decision
Consequences
Quality Strategy
  ┌─────────────────────────────────┐
  │ Conclusion Checkpoint (Optional)│  ← NEW: gates Quality Strategy → Review
  └─────────────────────────────────┘
---
Comments
```

### Updated Template

```markdown
# [Number]. [Short Title]

Date: [YYYY-MM-DD]
Status: Accepted
Last Updated: [YYYY-MM-DD]
Links:

## Context

## Options

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** [Proceed | Pause for validation | Skipped — <rationale>]

- [ ] All options evaluated at comparable depth
- [ ] Decision drivers are defined and referenced in option analysis
- [ ] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

## Consequences

## Quality Strategy

- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [ ] Backwards Compatible
- [ ] Integration tests
- [ ] User documentation

### Additional Quality Concerns

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** [Ready for review | Needs work | Skipped — <rationale>]

- [ ] Decision justified (Y-statement or equivalent)
- [ ] Consequences include positive, negative, and neutral outcomes
- [ ] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [ ] Links to related ADRs populated

**Pre-review notes:**

---

## Comments
```

### Skill Behavior

**author-adr (create/solve workflow):**
- After populating the Options section, the agent pauses at the Evaluation Checkpoint.
- The agent assesses decision complexity and writes the assessment value.
- If "Pause for validation" — populates Validation needs and asks the user whether to prototype, selectively validate, or skip.
- If "Proceed" — checks the baseline items and continues to the Decision section.
- If the user says "skip" — writes `Skipped — <user's rationale>` and proceeds.

**prototype-adr (ADR-0023):**
- Reads the Evaluation Checkpoint's "Validation needs" as its prototype objectives.
- If the checkpoint is "Skipped" or "Proceed" with no validation needs, prototype-adr has nothing to do.

**author-adr (review workflow):**
- The review process can check whether checkpoints were completed, skipped, or left blank.
- A blank checkpoint (no assessment written) is a review finding — it means the checkpoint was ignored, not consciously skipped.
- A "Skipped" checkpoint with rationale is acceptable — the reviewer evaluates whether the rationale is sound.

**implement-adr:**
- Reads the Conclusion Checkpoint to verify the ADR has been through authoring quality gates before planning.
- A "Skipped" conclusion checkpoint is informational — implement-adr proceeds but may flag it.

## Consequences

**Positive:**
- The authoring workflow becomes procedural — agents have explicit pause points with defined behavior rather than relying on implicit judgment.
- Skills gain machine-readable interop signals: `author-adr` writes checkpoints, `prototype-adr` reads validation needs, `implement-adr` reads completion state.
- The (Optional) qualifier and skip-with-rationale design respects user authority — straightforward decisions flow through quickly, complex decisions get structured attention.
- "Validation needs" closes the gap identified in ADR-0023: `prototype-adr` now has a well-defined input contract baked into the template itself.
- Blank vs. skipped distinction catches accidental omissions during review without penalizing deliberate fast-tracking.

**Negative:**
- The template grows from 6 sections to 8, increasing visual overhead for every ADR.
- Agent assessment quality is a new dependency — a poorly calibrated agent may recommend "Proceed" when it should "Pause," or vice versa.
- Two new sections means the nygard-agent-format.sh script and related tests need updates.
- Existing ADRs (0001–0023) won't have checkpoint sections — a cosmetic format inconsistency in the decision log, though non-functional since checkpoints are additive and parsing older ADRs is unaffected.

**Neutral:**
- The checkpoint sections are additive — they don't change the semantics of existing sections (Context, Options, Decision, Consequences, Quality Strategy, Comments all work as before).
- The (Optional) qualifier is a template convention, not a tooling enforcement — no script changes are needed to *allow* skipping.
- This ADR introduces a major semantic change to the template format, which should be tracked in Quality Strategy.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing
- [x] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] User documentation

### Additional Quality Concerns

- **Template script update** — `nygard-agent-format.sh` must emit the new checkpoint sections. Existing tests need updating and new tests should verify checkpoint section presence.
- **Backwards compatibility** — existing ADRs (0001–0023) remain valid without checkpoint sections. Scripts should not break when parsing ADRs that lack checkpoints.
- **Skill reference updates** — `create.md`, `solve.md`, and `review.md` references need updates to describe checkpoint behavior at each workflow stage.
- **Template documentation** — the Section Guide in `nygard-agent-template.md` needs entries for both checkpoint sections.

---

## Comments
