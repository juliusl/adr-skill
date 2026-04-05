# 33. Add defer verb to revise task triage

Date: 2026-04-05
Status: Planned
Last Updated: 2026-04-05
Links:
- Addresses measurement gap in [ADR-0031](0031-add-author-adr-dispatch-hooks-for-custom-agent-delegation.md) (prototype findings, lines 314–320)
- Modifies [ADR-0015](0015-add-interactive-revise-task-to-author-adr-workflow.md) (revise task design)
- Related to [ADR-0016](0016-append-revision-dialogue-as-qa-addendum-to-adr-documents.md) (Q&A addendum format)

## Context

The revise task (ADR-0015) offers two triage verbs for each review finding:

- **Address** — the author provides revised text or agrees with a suggestion
- **Reject** — the author declines the finding

The ADR-0031 prototype experiment discovered a measurement gap: the user's
actual triage pattern includes a third mode — **"reject scope but redirect the
concern."** In this mode the author agrees the finding surfaces a legitimate
concern but asserts it belongs in a different ADR or a future decision. The
author rejects the finding's scope in the current ADR while acknowledging it
should be addressed elsewhere.

This pattern was observed across multiple revision sessions:

- ADR-0020 #3: "Naming overlap — `.adr/` vs `docs/adr/`" → Rejected, but the
  distinction was documented as a consequence.
- ADR-0020 #4: "Include revisit trigger" → Rejected, with note that "revisit
  conditions will emerge naturally when downstream ADRs define specific tools."
- ADR-0016 #1: "Add evidence (PoC/spike)" → Rejected, with redirect:
  "Tooling compatibility will be addressed by a separate ADR."
- ADR-0016 #5: "Add realization plan" → Rejected, with redirect:
  "Implementation is implicit in ADR-0015."

When scored as binary Address/Reject, these "redirect" rejections inflate
disagreement between the editor persona and the user. The persona may Address
(because the concern is valid) while the user Rejects (because the scope is
wrong) — or vice versa. The mismatch isn't about judgment quality; it's about
the triage vocabulary not capturing the user's actual intent.

The ADR-0032 experiment reinforced this: the editor persona's accuracy was
partially limited by interpreting scope-bounded rejections as flat rejections,
when the author's intent was redirection.

### Decision Drivers

- **Measurement accuracy** — persona agreement metrics should reflect actual
  judgment alignment, not vocabulary mismatch
- **Author intent capture** — the Q&A addendum should preserve the redirect
  destination so future readers know where the concern went
- **Minimal vocabulary** — don't over-expand the triage options; each verb
  should correspond to a distinct, commonly observed author behavior
- **Editor persona compatibility** — the editor persona (ADR-0031) should be
  able to use Defer when it recognizes a scope-redirect pattern

## Options

### Option 1: Add Defer as a third triage verb

Add **Defer** alongside Address and Reject. Defer means: "the concern is
valid but belongs elsewhere — redirect it." The author provides a redirect
destination (e.g., "ADR-NNNN," "the implementing ADR," "a future decision").

```
For each comment, offer the user three choices:

- **Address** — revise the ADR to incorporate the finding
- **Reject** — decline the finding; it is not applicable
- **Defer** — the concern is valid but out of scope for this ADR;
  redirect to a specific destination
```

In the Q&A addendum, Defer entries include the redirect:

```markdown
### Q: Should the ADR include a revisit trigger?

**Deferred** — Not in scope for this ADR. Revisit conditions will emerge
when downstream ADRs define specific tools. → Follow-up: tooling ADR
```

**Strengths:**
- Clean three-way vocabulary that maps to observed author behavior
- The redirect destination is an explicit field, preserving traceability
- Editor persona can use Defer directly — no interpretation needed
- Minimal change: one new verb, one new Q&A prefix
- Measurement accuracy improves: persona "Defer" matches user "Defer"
  instead of both sides guessing between Address and Reject

**Weaknesses:**
- Adds decision overhead: the author must choose between three options
  instead of two for every finding
- The boundary between Reject and Defer can be fuzzy — when is a
  rejection "just a rejection" vs. "a redirect"?

### Option 2: Extend Reject with an optional redirect field

Keep the binary Address/Reject vocabulary but add an optional redirect
note to Reject. When the user rejects with a redirect, the Q&A addendum
captures it:

```markdown
### Q: Should the ADR include a revisit trigger?

**Rejected** — Not in scope. → Redirect: tooling ADR
```

**Strengths:**
- No new verb — keeps the triage vocabulary minimal
- Backwards compatible — existing Reject behavior unchanged
- The redirect is optional metadata, not a decision gate

**Weaknesses:**
- Doesn't solve the measurement problem: persona and user still both
  record "Reject," so the redirect intent is captured in text but not
  in the verb. Agreement metrics can't distinguish "flat reject" from
  "scope redirect"
- The editor persona can't signal "I recognize this as a redirect" —
  it just says Reject like any other rejection
- The redirect note is unstructured — no consistent format for tooling
  to parse

### Option 3: Richer verb taxonomy (Address / Reject / Defer / Absorb)

Expand to four verbs: Address, Reject, Defer (redirect elsewhere), and
Absorb (acknowledge as a known limitation without changing text — e.g.,
adding it as a consequence footnote or neutral observation).

**Strengths:**
- Captures the full spectrum of observed author behaviors including
  "acknowledge without fixing" patterns
- Most expressive vocabulary

**Weaknesses:**
- Four options per finding adds significant cognitive load
- Absorb overlaps with Address (adding a consequence IS addressing)
- Diminishing returns: the measurement gap is specifically about
  Defer, not about a fourth category
- Over-engineering for the observed problem

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

None. The measurement gap is empirically documented in ADR-0031 with
specific examples. The Defer pattern was observed in 4+ revision sessions
across multiple ADRs.

## Decision

In the context of **the revise task's binary Address/Reject vocabulary
missing the "redirect" pattern**, facing **inflated persona disagreement
on scope-bounded rejections**, we decided for **adding Defer as a third
triage verb (Option 1)**, and neglected **extending Reject with a redirect
field (doesn't fix measurement) and a four-verb taxonomy (over-engineered
for the observed gap)**, to achieve **accurate intent capture where scope
redirections are recorded as Defer with an explicit destination, improving
both Q&A addendum traceability and editor persona agreement metrics**,
accepting that **triage decisions now have three options instead of two,
adding marginal cognitive overhead per finding**.

### Verb Definitions

| Verb | Meaning | Author Action | Q&A Prefix |
|------|---------|---------------|------------|
| **Address** | The finding is valid and in scope; revise the ADR | Provide revised text or agree with suggestion | `**Addressed**` |
| **Reject** | The finding is not applicable or incorrect | Optionally provide reason | `**Rejected**` |
| **Defer** | The concern is valid but out of scope; redirect | Provide redirect destination | `**Deferred**` |

### Defer Mechanics

When the user (or editor persona) selects Defer:

1. **Acknowledge the concern** — confirm the finding surfaces something real
2. **State the scope boundary** — explain why it's out of scope for this ADR
3. **Provide redirect** — name where the concern belongs (specific ADR number,
   "the implementing ADR," "a future decision about X," etc.)

### Q&A Addendum Format

Deferred entries follow the existing Q&A pattern with a new prefix and
redirect arrow:

```markdown
### Q: Should the ADR include a revisit trigger?

**Deferred** — Not in scope for this ADR; revisit conditions will emerge
when downstream ADRs define specific tools. → Follow-up: tooling ADR
```

The `→` arrow and redirect destination are a convention, not enforced
syntax. The revise task should prompt for a destination but accept
freeform text.

### Revision Summary Format

The revision summary table gains a third action value:

```markdown
| # | Finding | Priority | Action | Change |
|---|---------|----------|--------|--------|
| 1 | ... | H | Addressed | [description] |
| 2 | ... | M | Deferred | → tooling ADR |
| 3 | ... | L | Rejected | [reason] |

**X addressed, Y deferred, Z rejected.**
```

### Editor Persona Impact

The editor persona (ADR-0031) gains the ability to express Defer directly.
This maps to the existing v3 principle 1 ("scope discipline overrides
priority") with the IMPORTANT clause ("rejection ≠ ignoring — always note
where the concern DOES belong"). Defer is the verb that implements that
clause.

Expected measurement impact: findings where the persona currently chooses
Address (because the concern is valid) but the user chose Reject (because
the scope is wrong) — or vice versa — should now converge on Defer.

## Consequences

**Positive:**

- Persona agreement metrics become more accurate by distinguishing scope
  redirections from flat rejections. The ADR-0031 measurement gap (which
  inflated apparent disagreement) is closed.
- The Q&A addendum preserves redirect destinations, giving future readers
  a trail of where concerns went — not just that they were rejected.
- The editor persona can express its actual judgment ("valid concern, wrong
  ADR") instead of approximating with Address or Reject.
- The revision summary gains a third category that makes scope management
  visible: "3 addressed, 2 deferred, 1 rejected" tells a richer story than
  "3 addressed, 3 rejected."

**Negative / Risks:**

- Marginal cognitive overhead: three options per finding instead of two.
  Mitigated by Defer being a natural category that authors already use
  informally — the vocabulary catches up to the behavior.
- The boundary between Reject and Defer requires judgment. Mitigated by
  a simple test: "Does the concern belong somewhere else?" If yes, Defer.
  If the concern is simply wrong or irrelevant, Reject.

**Neutral:**

- Existing Q&A addendum entries (using Address/Reject) remain valid. Defer
  is additive — no migration needed for existing ADRs.
- The revise task's guard rails are unchanged: one finding at a time,
  preserve author voice, respect priority ordering.
- The persona experiment (ADR-0031) should be re-run after implementation
  with the Defer verb available to validate the expected measurement
  improvement.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

- **revise.md update** — Step 3 (Collect User Response) must add Defer as a
  third choice with redirect prompt. Step 5b (Q&A Addendum) must handle the
  `**Deferred**` prefix and redirect format.
- **Editor persona update** — the v3 persona's principle 1 IMPORTANT clause
  already describes the Defer behavior. The persona should be updated to use
  the Defer verb explicitly when applying that clause.
- **Revision summary format** — the summary line changes from "X addressed,
  Y rejected" to "X addressed, Y deferred, Z rejected."

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

- This is a direct follow-up from ADR-0031's recommended action item.
- The measurement impact should be validated after implementation by
  re-running the persona experiment with the Defer verb available.

---

## Comments

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: How does the Defer verb interact with priority ordering and quality signal counting?

**Rejected** — The three-way summary format ("X addressed, Y deferred, Z rejected") already distinguishes Defer as its own category; how downstream quality signals interpret those counts is a concern for the revise task, not this verb-definition ADR.

### Q: Should the persona re-run experiment be promoted from pre-review notes to a structural consequence?

**Addressed** — Promoted to a neutral consequence: "The persona experiment (ADR-0031) should be re-run after implementation with the Defer verb available to validate the expected measurement improvement."
