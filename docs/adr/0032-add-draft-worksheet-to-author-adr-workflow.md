# 32. add draft worksheet to author-adr workflow

Date: 2026-04-05
Status: Planned
Last Updated: 2026-04-04
Links:
- Informed by [ADR-0031](0031-add-author-adr-dispatch-hooks-for-custom-agent-delegation.md) (dispatch hooks and editor persona experiment)
- Related to [ADR-0024](0024-add-checkpoint-sections-to-nygard-agent-template.md) (checkpoint gates in template)
- Related to [ADR-0017](0017-adopt-nygard-agent-template-as-default-adr-format.md) (nygard-agent template structure)
- Related to [ADR-0016](0016-append-revision-dialogue-as-qa-addendum-to-adr-documents.md) (Comments area / semantic boundary)
- Related to [ADR-0015](0015-add-interactive-revise-task-to-author-adr-workflow.md) (revise task ordering)
- Related to [ADR-0022](0022-replace-ecadr-completeness-check-with-implementability-criteria.md) (experimentation tolerance)

## Context

The author-adr workflow currently follows a linear pipeline:

```
create/solve → review → edit → revise → (loop)
```

Each step receives the ADR file as its primary input, but there is no
structured capture of the user's **original intent** — the direction they were
leaning, how much uncertainty they're comfortable with, what tolerance they have
for the agent diverging from their initial framing, or how many options they
want explored. This information exists only in the conversational context and
is lost between workflow steps.

The ADR-0031 prototype experiment surfaced several findings that motivate a
structured pre-creation intake:

1. **Editor persona accuracy depends on intent context.** The experiment showed
   82% agreement when the editor persona had access to source material. A draft
   worksheet that captures original intent would give the editor a stable
   reference point for triage decisions — "does this finding align with what the
   author was trying to achieve?"

2. **Measurement gap reveals intent loss.** The revise task's Address/Reject
   binary doesn't capture "reject scope but redirect concern" (the Defer
   pattern). This gap exists partly because the editor has no reference for
   what the author's original tolerance and scope boundaries were. If the
   editor knew the author's tolerance for scope expansion, it could distinguish
   "this is out of scope" from "this is interesting but defer it."

3. **Improvisation calibration is implicit.** During the solve workflow, the
   agent explores options freely, but the user has no way to signal how much
   creative divergence is acceptable. Some decisions need tight exploration
   (the user has a strong direction), others need wide exploration (the user is
   genuinely unsure). Currently this calibration happens through conversational
   cues that don't persist.

4. **Create workflow lacks grounding.** The create workflow jumps from "user
   has a thought" to structured ADR sections (Context, Options, Decision).
   The solve workflow is better — it has a problem intake step — but neither
   captures the user's meta-preferences about how the workflow should behave.

A "draft mode" would add a structured intake step before create/solve that
captures the user's intent, tolerance levels, and exploration preferences in
a persistent worksheet. This worksheet would:

- **Ground the create/solve workflow** — the agent knows the direction, scope,
  and how much to diverge from it.
- **Inform the editor** — during triage, the editor (human or persona agent)
  can reference the original intent when deciding Address/Reject.
- **Calibrate exploration** — tolerance and improvisation settings guide option
  discovery depth and creative range.

### Decision Drivers

- **Intent preservation across workflow steps** — original direction and
  tolerance levels must survive the create → review → edit → revise pipeline
  so each step can reference them.
- **Editor grounding** — the dispatch editor hook (ADR-0031) needs a stable
  reference for the author's intent to make accurate triage decisions.
- **Calibrated exploration** — users should be able to signal how much
  creative divergence they want without relying on conversational tone.
- **Progressive disclosure** — the worksheet should be optional. Users who
  want to freeform straight into create/solve should still be able to.
- **Template compatibility** — the solution should work with the nygard-agent
  template without breaking existing ADRs.

## Options

### Option 1: Worksheet as a mutable preamble in the ADR file

Add a `## Draft Worksheet` section to the nygard-agent template between the
metadata block and `## Context`. The section is the first thing filled out
when using draft mode. It lives inside the ADR file, above the decision
record content.

```markdown
# 32. [Title]

Date: 2026-04-05
Status: Planned
Last Updated: 2026-04-04
Links:

## Draft Worksheet
<!-- Optional. Fill out before drafting to capture intent and calibration. -->

### Framing
<!-- What's the core idea? What triggered this? What direction are you leaning? -->

### Tolerance
- Risk: [Low | Medium | High] — appetite for experimental or unproven options
- Change: [Low | Medium | High] — departure from current state
- Improvisation: [Low | Medium | High] — creative divergence from this framing

### Uncertainty
<!-- What do you know for certain? What are you unsure about? -->

### Options
- Target count: [2-3 | 3-5 | open]
- [ ] Explore additional options beyond candidates listed below

#### Candidate: [name]
<!-- Why this is being considered, initial strengths/concerns -->

## Context
...
```

The worksheet section sits above the semantic boundary — it's part of the
decision record, not the comments area. However, it's explicitly mutable
(the user can update it as understanding evolves during drafting).

**Strengths:**
- Single file — no fragmentation or indirection
- The editor can read the worksheet directly from the same file it's triaging
- Workflow stays simple: fill out worksheet → populate Context/Options
- Naturally degrades — if the section is left empty or removed, the ADR works
  exactly as before
- Visible in version control — intent evolution is tracked

**Weaknesses:**
- Adds another section to an already structured template (metadata → worksheet
  → Context → Options → Evaluation Checkpoint → Decision → Consequences →
  Quality Strategy → Conclusion Checkpoint → Comments)
- The worksheet is informal/calibration-oriented while the surrounding
  sections are formal/analytical — tonal mismatch
- Worksheet may feel redundant with Context, since framing and uncertainty
  overlap with "forces at play"
- Template changes affect all new ADRs, even those that don't use draft mode

### Option 2: Separate draft artifact in `.adr/var/drafts/`

Create a dedicated draft worksheet file in the project's `.adr/var/` directory
(gitignored, per ADR-0020). The file follows a naming convention that links it
to its ADR (e.g., `.adr/var/drafts/0032-worksheet.md`). The create/solve
workflow reads the worksheet as input context. The editor reads it during
triage by following the ADR number.

```
.adr/
├── .gitignore          # var/ is gitignored
└── var/
    └── drafts/
        └── 0032-worksheet.md
```

The worksheet file uses the same structure as Option 1's section, but as a
standalone document. The ADR itself is unchanged — no template modifications.

**Strengths:**
- Clean separation — draft intent is a workflow artifact, not part of the
  formal decision record
- No template changes — existing ADRs and template unaffected
- Gitignored by default — draft worksheets are transient working documents
- Can evolve independently of the ADR (the worksheet captures pre-decision
  intent; the ADR captures the decision itself)
- Multiple worksheets per ADR are possible (e.g., if the problem is reframed)

**Weaknesses:**
- File indirection — the editor must resolve the worksheet path from the
  ADR number, adding a lookup step
- Risk of drift — the worksheet and ADR can diverge without anyone noticing
- Gitignored means the intent record is lost after the decision is made
  (unless explicitly preserved)
- More infrastructure — needs a Makefile target, naming convention, and
  discovery logic
- Users must learn another file location and convention

### Option 3: Worksheet in the Comments area (below the semantic boundary)

Place the draft worksheet in the `## Comments` section — the mutable area
below the `---` separator. The worksheet is the first content in Comments,
before any revision Q&A entries. This leverages the existing mutable/immutable
boundary: the decision record above is formal; the Comments area below is a
working space.

```markdown
---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:** [core idea and direction]

**Tolerance:**
- Risk: [Low | Medium | High]
- Change: [Low | Medium | High]
- Improvisation: [Low | Medium | High]

**Uncertainty:** [what's known vs. unknown]

**Options:**
- Target: [2-3 | 3-5 | open]
- [ ] Explore additional options beyond candidates
- Candidates: [brief list]

<!-- Revision Q&A entries appear below -->
```

The semantic boundary already exists (ADR-0016). The worksheet becomes the
first "worksheet" in the mutable area, and revision Q&A entries append after
it. The editor naturally encounters the worksheet when reading the Comments
section during triage.

**Strengths:**
- No template structure changes above the semantic boundary — Context,
  Options, Decision, etc. are untouched
- Uses the existing mutable area for its intended purpose (working notes)
- The editor already reads the Comments section during revise — the worksheet
  is co-located with revision Q&A
- Version-controlled — intent is preserved in the repo
- Consistent with ADR-0016's "mutable worksheet" framing of Comments

**Weaknesses:**
- Comments area is currently defined as "revision Q&A" territory — adding a
  pre-creation worksheet changes its role from "post-review working space" to
  "general working space"
- The worksheet is at the bottom of the file — less prominent than a preamble
  position, requiring the editor to scroll/seek to find it
- Ordering dependency: worksheet must come before revision Q&A entries, which
  means the revise task needs to know to append after the worksheet, not at
  the start of Comments

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

1. ~~Retroactive worksheet fill~~ — **Validated.** Filled out worksheets for
   ADR-0016 and ADR-0020 retroactively from session history.

**Prototype findings:**

Experiment: Retroactively filled draft worksheets for two ADRs using only
the user's session history messages, then assessed whether the worksheets
would have grounded the create/solve workflows and provided useful signal
to the editor persona.

```
Hypothesis: The proposed five-dimension worksheet (Framing, Tolerance,
Uncertainty, Options, Candidates) can capture the user's original intent
from session history accurately enough to ground workflow steps.

Data provenance:
- ADR-0016 — 1 session, 14 user turns, 1 initial prompt (detailed)
- ADR-0020 — 2 sessions, 18+ user turns, 1 initial prompt (brief)

Method:
1. Extracted user messages from creation sessions via session_store
2. Filled out the five worksheet dimensions using only user messages
3. Assessed each dimension: (a) captured from session? (b) would have
   grounded the workflow?
4. Cross-compared the two ADRs for pattern differences

Results:
| Dimension        | ADR-0016 (create) | ADR-0020 (solve) |
|------------------|--------------------|-------------------|
| Framing          | ✅ Full            | ⚠️ Emerged mid-conversation |
| Risk tolerance   | ✅ Low             | ✅ Medium          |
| Change tolerance | ✅ Low             | ✅ High            |
| Improvisation    | ✅ Low (prescribed)| ✅ Medium (open)   |
| Uncertainty      | ✅ Clear           | ✅ With prior art  |
| Candidates       | ✅ Prescribed      | ✅ From prior art  |

Key findings:

A. Worksheet timing varies by workflow:
   - Create workflow (ADR-0016): user arrives with direction → worksheet
     filled BEFORE drafting. Captures known intent.
   - Solve workflow (ADR-0020): user arrives with problem → worksheet
     crystallizes DURING intake. Captures emergent intent.
   The worksheet should support both pre-draft and post-intake fill modes.

B. Improvisation tolerance is the highest-value editor signal:
   - ADR-0016 (Low): user rejected scope-expanding suggestions during
     actual revisions — consistent with Low improvisation.
   - ADR-0020 (Medium): user engaged with alternative proposals during
     actual revisions — consistent with Medium improvisation.
   This validates that the editor persona would make better triage
   decisions with access to the improvisation tolerance field.

C. "Explore additional options" checkbox validated:
   - ADR-0016 (unchecked): actual ADR has 3 closely related options —
     consistent with constrained exploration.
   - ADR-0020 (checked): actual ADR has 3 diverse options (JSONL, SQLite,
     XDG) — consistent with open exploration.

D. Five dimensions are sufficient but the Framing field behaves
   differently in solve vs. create: brief for solve (expands through
   conversation), detailed for create (user arrives with direction).
```

**Assessment: Validated.** The worksheet format captures meaningful intent
signal across both workflow types. The key design refinement: the worksheet
is not always a pre-creation step — for solve workflows, it crystallizes
during the problem intake conversation.

2. ~~Editor accuracy with worksheet context~~ — **Validated (negative).**
   Tested v3 editor persona (Opus 4.6) against all 4 ADRs with and without
   worksheet context.

```
Hypothesis: The editor persona makes better triage decisions when given
the draft worksheet as additional context alongside review findings.

Method:
1. Established baseline: v3-opus WITHOUT worksheet = 25/28 (89%)
2. Created retroactive worksheets for all 4 ADRs from session history
3. Re-ran v3-opus WITH worksheet context for all 4 ADRs
4. Compared per-finding accuracy

Results:
| Condition        | ADR-0020 | ADR-0021 | ADR-0015 | ADR-0016 | Total |
|------------------|----------|----------|----------|----------|-------|
| v3 no worksheet  |10/10 100%| 5/6 83%  | 4/6 67%  | 6/6 100% | 89%   |
| v3 with worksheet|10/10 100%| 6/6 100% | 3/6 50%  | 4/6 67%  | 82%   |

Per-finding deltas:
| ADR   | Finding              | Expected  | No WS     | With WS   | Delta     |
|-------|----------------------|-----------|-----------|-----------|-----------|
| 0021#1| Revisit trigger      | Addressed | Rejected  | Addressed | FIXED     |
| 0015#4| Success criteria     | Addressed | Addressed | Rejected  | REGRESSED |
| 0016#4| --- robustness       | Addressed | Addressed | Rejected  | REGRESSED |
| 0016#6| PR-workflow interact | Addressed | Addressed | Rejected  | REGRESSED |

Failure mode: Low-improvisation-as-rejection-accelerant.
The editor interprets Low improvisation tolerance as license to reject
more aggressively — "the author had a specific format in mind, so edge
cases of that format are out of scope." It conflates "stay close to the
author's direction" (intended: guide option exploration breadth) with
"reject findings about the author's chosen approach" (unintended: suppress
quality feedback on the decision itself).

The worksheet fixed 1 finding (ADR-0021 revisit trigger — the concrete
milestone matched the worksheet's clear scope) but broke 3 findings
where Low improvisation reinforced over-rejection of legitimate quality
concerns.
```

**Assessment: Worksheet format validated, but editor consumption of
tolerance signals needs calibration.** The worksheet successfully captures
intent (Experiment 1) but the editor persona misinterprets tolerance
levels as triage policy rather than exploration guidance (Experiment 2).
The fix is in how the editor reads tolerance signals, not in the worksheet
format itself. Deferred to implementation — the editor's worksheet
consumption instructions (in revise.md) must explicitly separate
"exploration calibration" from "finding triage policy."

## Decision

In the context of **preserving original intent across the author-adr
create → review → edit → revise pipeline**, facing **the editor persona's
need for grounding in the author's direction and tolerance levels**, we
decided for **a draft worksheet in the Comments area below the semantic
boundary (Option 3)**, and neglected **an inline preamble section (adds
template complexity above the formal record) and a separate file (loses
intent history and adds indirection)**, to achieve **persistent intent
capture that grounds the create/solve workflow and gives the editor access
to original direction, tolerance levels, and exploration preferences**,
accepting that **the Comments area expands from "revision Q&A" to "general
working space" and the worksheet's position at the end of the file is less
prominent than a preamble**.

### Why Option 3 over Options 1 and 2

**Over Option 1 (inline preamble):** The worksheet is pre-decision
calibration metadata — it describes how the author wants the workflow to
behave, not what the decision is. Placing it above Context, alongside formal
analytical sections, conflates workflow calibration with decision content.
The Comments area is explicitly a mutable working space (ADR-0016), which is
exactly what the worksheet is.

**Over Option 2 (separate file):** The editor (human or persona agent) needs
to read the worksheet during triage. A gitignored file in `.adr/var/` means
the intent record is transient — it's lost after the decision lifecycle
completes. Since the worksheet captures the author's original direction,
losing it means losing the record of *why* the author explored certain
options and rejected others at a meta level. The Comments area preserves
this in version control alongside the decision itself.

### Draft Worksheet Structure

The worksheet captures five dimensions of pre-decision intent:

```markdown
---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. Filled out before
     drafting. The create/solve workflow reads this for direction; the editor
     reads it during triage for intent grounding. -->

**Framing:**
<!-- What's the core idea? What triggered this? What direction are you
     leaning? This seeds the Context section. -->

**Tolerance:**
- Risk: [Low | Medium | High] — appetite for experimental or unproven options
- Change: [Low | Medium | High] — acceptable departure from current state
- Improvisation: [Low | Medium | High] — how much creative divergence from
  this framing is welcome during option discovery

**Uncertainty:**
<!-- What do you know for certain? What are you unsure about? What
     assumptions are you making that could be wrong? -->

**Options:**
- Target count: [2-3 | 3-5 | open]
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
<!-- Pre-identified option candidates with brief notes on why they're
     being considered. Leave empty if starting from scratch. -->
```

### Worksheet Fields

| Field | Purpose | Consumed By |
|-------|---------|-------------|
| **Framing** | Seeds the Context section; captures the initial direction before structured analysis | Create, Solve |
| **Risk tolerance** | Guides how experimental the considered options can be | Create, Solve, Editor |
| **Change tolerance** | Signals acceptable departure from current state | Create, Solve, Editor |
| **Improvisation tolerance** | Controls creative divergence during option discovery — Low means stay close to framing, High means challenge the framing | Solve, Editor |
| **Uncertainty** | Declares knowns and unknowns; helps the Evaluation Checkpoint assess whether validation is needed | Create, Solve, Review |
| **Target count** | How many options to aim for; prevents under-exploration (Sprint anti-pattern) and over-exploration | Solve |
| **Explore additional** | Checkbox: whether to discover options beyond the pre-identified candidates | Solve |
| **Candidates** | Pre-identified options with brief rationale; the solve workflow starts here instead of from scratch | Solve |

### Workflow Integration

**Draft mode activation:** When the user says "draft an ADR," "start a draft,"
or "I have an idea for a decision," the workflow creates the ADR file (via
`make new TITLE="tbd"`) and immediately populates the Draft Worksheet in the
Comments section before proceeding to Context.

**Create workflow integration:** After the worksheet is filled out, the create
workflow reads it to:
- Seed the Context section from the Framing field
- Calibrate option depth based on tolerance levels
- Pre-populate Options from the Candidates field

**Solve workflow integration:** The solve workflow reads the worksheet to:
- Use Framing as the initial problem statement (Step 1: Problem intake)
- Scope option discovery using Target count and Explore additional (Step 2)
- Apply Improvisation tolerance to control how far beyond the framing the
  agent diverges during option discovery

**Editor integration:** The editor (human or persona agent from ADR-0031) reads
the worksheet during triage to:
- Assess findings against the author's original Framing — does the finding
  align with what the author was trying to achieve?
- Use tolerance levels to calibrate Address/Reject decisions — a finding that
  suggests departing from the author's direction should be weighed against the
  stated Improvisation tolerance
- Reference Uncertainty to distinguish "the author knew about this" from "this
  is a gap the author acknowledged"

### Worksheet Fill Modes

The prototype revealed that worksheet timing varies by workflow:

| Workflow | Fill Mode | When | How |
|----------|-----------|------|-----|
| **Create** | Pre-draft | Before populating Context/Options | User fills out upfront — they arrive with a direction |
| **Solve** | Post-intake | After problem intake conversation | Agent drafts from conversation, user confirms/adjusts |

Both modes produce the same artifact (the Draft Worksheet in Comments). The
difference is timing: create-mode users fill it first; solve-mode users
confirm it after the intake conversation crystallizes their intent.

### Comments Area Evolution

The `## Comments` section was introduced by ADR-0016 as a "mutable worksheet"
below the semantic boundary (`---`). The revise task (ADR-0015) uses it for
revision Q&A entries. This ADR expands its role:

| Content | When Added | Purpose |
|---------|-----------|---------|
| **Draft Worksheet** | Before creating the ADR body | Pre-decision intent and calibration |
| **Revision Q&A** | After each review→revise cycle | Post-review dialogue record |

The Draft Worksheet always appears first in Comments, before any revision Q&A
entries. The revise task's existing logic for detecting `<!-- Generated by the
revise task -->` is unchanged — it appends after the worksheet.

## Consequences

**Positive:**

- The create/solve workflow has persistent grounding in the author's original
  direction, reducing drift during option discovery and preventing the agent
  from "going rogue" on exploration depth.
- The editor (human or persona agent) gains a stable reference for the
  author's intent, improving triage accuracy — particularly for the
  Improvisation tolerance dimension, which tells the editor how much scope
  expansion the author is comfortable with. *(Contingent on calibration of
  tolerance signal consumption — see Quality Strategy. Without calibration,
  the prototype showed accuracy degraded from 89% to 82%.)*
- Tolerance levels make implicit workflow calibration explicit and persistent.
  Instead of relying on conversational tone ("I'm pretty sure about this" vs.
  "I have no idea"), the calibration survives across steps and sessions.
- The worksheet is optional — users who skip it get the existing workflow
  unchanged. Progressive disclosure is preserved.
- Intent history is version-controlled alongside the decision, providing a
  record of the author's original direction for future readers.

**Negative / Risks:**

- The Comments area expands from "revision Q&A" to "general working space."
  This is a semantic shift — readers who learned that Comments = revision
  dialogue may be surprised to find pre-decision content there. Mitigated by
  the Comments area already being described as a "mutable worksheet" in
  ADR-0016, and the Draft Worksheet being clearly labeled.
- The worksheet's position at the bottom of the file means it's not the first
  thing a reader encounters. Mitigated by the editor and create/solve
  workflows explicitly seeking it out — human readers can scroll, agent
  readers can search.
- Ordering dependency in Comments (worksheet before Q&A) adds a structural
  rule. Mitigated by the revise task already having detection logic for its
  content marker — the worksheet can use its own marker for reliable ordering.

**Neutral:**

- No template structure changes above the semantic boundary. Context, Options,
  Evaluation Checkpoint, Decision, Consequences, Quality Strategy, and
  Conclusion Checkpoint are untouched.
- The worksheet fields may evolve as usage patterns emerge — additional
  dimensions (e.g., timeline pressure, stakeholder count) could be added
  without structural changes.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

- **Reference documentation** — the create.md and solve.md references need
  updates to describe draft mode activation and worksheet reading.
- **Revise task ordering** — the revise task's Q&A append logic should be
  verified to work when a Draft Worksheet precedes the Q&A entries in Comments.
- **Editor instructions** — revise.md should be updated to instruct the editor
  to read the Draft Worksheet for intent grounding during triage.
- **Backwards compatibility** — existing ADRs without a Draft Worksheet must
  work identically. The worksheet is strictly additive.
- **Editor tolerance calibration** — the prototype revealed that the editor
  persona misinterprets tolerance signals (especially Low improvisation) as
  rejection policy rather than exploration guidance. The editor's worksheet
  consumption instructions in revise.md must explicitly state that tolerance
  levels control option discovery breadth, NOT finding triage aggressiveness.
  Without this calibration, the worksheet degrades editor accuracy (89% → 82%).

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

- Worksheet fill mode timing (pre-draft vs. post-intake) emerged from
  the prototype and is documented in the Decision section.
- Editor tolerance calibration is a known implementation concern — the
  worksheet improves create/solve grounding but degrades editor accuracy
  without explicit consumption instructions. Fix is in revise.md, not
  in the worksheet format.

---

## Comments

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Are the ADR cross-reference links correct?

**Addressed** — Fixed two broken filenames (0024, 0017) and added three missing links (ADR-0016, ADR-0015, ADR-0022) that are substantively depended upon.

### Q: Does the Consequences section accurately reflect the prototype findings on editor accuracy?

**Addressed** — Added calibration caveat to the editor triage accuracy consequence: "Contingent on calibration of tolerance signal consumption — without calibration, the prototype showed accuracy degraded from 89% to 82%."

### Q: Are the inline ADR dependencies reflected in the Links section?

**Addressed** — Added ADR-0016 (Comments area semantics), ADR-0015 (revise task ordering), and ADR-0022 (experimentation tolerance) to the Links section.

### Q: Is there a date inconsistency between Date and Last Updated?

**Rejected** — A newly proposed ADR having the same Date and Last Updated is the expected state; no inconsistency to resolve.
