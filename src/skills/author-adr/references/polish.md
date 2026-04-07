# Polishing an ADR

Self-contained reference for the ADR quality loop: review → verdict → revise → re-review. Read this file when the user asks to review, revise, or polish an ADR.

Use this as the prompt for the configured agents (per ADR-0031, `[author.dispatch]`). The review agent executes the Review Phase (steps R-1 through R-6). The editor agent executes the Revision Phase (steps V-1 through V-6). Custom agents get the same checks — their persona shapes judgment, not task structure.

**All steps must be executed in order within each phase. If a step is skipped, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

---

## Policies

**Ignoring any of the below policies is a runtime violation ESPECIALLY when the user is away and operating autonomously**

| ID | Policy |
|----|--------|
| P-1 | Status cap — author-adr caps at Ready after review Accept verdict |
| P-2 | Dispatch compliance — use configured agents per `[author.dispatch]` |
| P-3 | Semantic boundary — never modify content above the `---` separator during revision |
| P-4 | Preserve existing addendum entries — do not modify or remove prior round Q&A entries |

### P-1: Status Cap at Ready

After a review Accept verdict, transition the ADR from `Proposed` to `Ready`. The author-adr skill never sets `Planned` or `Accepted` — those statuses belong to implement-adr.

### P-2: Dispatch Compliance

When `[author.dispatch]` keys are configured, use the configured agent for each phase. Do not substitute `general-purpose` or skip dispatch. The user configured these agents for a reason.

### P-3: Semantic Boundary

The `---` separator above `## Comments` divides the immutable decision record (above) from the mutable revision worksheet (below). When appending Q&A entries during revision, never modify content above the separator.

### P-4: Preserve Existing Addendum Entries

In multi-round revisions, existing Q&A entries from prior rounds must not be modified or removed. New entries are appended below existing ones.

---

## Review Perspectives

Apply three perspectives progressively:

1. **Friendly peer/coach** — early feedback for improvement
2. **Official stakeholder** — adequacy confirmation and agreement
3. **Formal design authority** — approval and enforcement

> Prioritize comments by urgency (H/M/L), provide concrete
> finding-recommendation pairs, and lead with questions rather than demands.

---

## Procedure

| ID | Description |
|----|-------------|
| R-1 | Implementability Check — verify 6 criteria that predict planning success |
| R-1a | Checkpoint State Review — evaluate Evaluation/Conclusion checkpoint assessments |
| R-1b | Experimentation Tolerance Spectrum — assess data support level |
| R-2 | Fallacy Scan — check justification against 7 decision-making fallacies |
| R-3 | Anti-Pattern Check — scan for 11 ADR creation anti-patterns |
| R-4 | Consequence Validation — review stated consequences for plausibility |
| R-5 | Review Checklist — answer 7 questions about the ADR |
| R-6 | Verdict — Accept, Revise, or Rethink |
| R-6a | Accept-with-Suggestions Polish Pass — dispatch editor for minor feedback |
| V-1 | Load Review Comments — parse review output and extract revision items |
| V-2 | Present Each Comment Interactively — show each finding with context |
| V-3 | Collect User Response — get Address, Reject, or Defer for each finding |
| V-3a | Defer Mechanics — acknowledge, scope, and redirect deferred concerns |
| V-4 | Apply Revisions — edit the ADR file with accumulated changes |
| V-5 | Produce a Revision Summary — output summary table of all actions |
| V-5b | Append Q&A Addendum to ADR — write revision dialogue to Comments section |
| V-5c | Record Review Cycle — append audit trail marker to Comments |
| V-6 | Recommend Re-Review — suggest re-review if substantive changes were made |

```
R-1 — Implementability Check
  ↓
R-2 — Fallacy Scan
  ↓
R-3 — Anti-Pattern Check
  ↓
R-4 — Consequence Validation
  ↓
R-5 — Review Checklist
  ↓
R-6 — Verdict
  ├─ Accept ──► R-6a (optional polish) → Ready status → Done
  ├─ Revise ──► V-1 → V-2 → V-3 → V-4 → V-5 → V-6 (re-review?)
  └─ Rethink ──► Stop
```

**Conditional steps:** R-1a is conditional on the ADR using the checkpoint template format. R-6a is conditional on an Accept verdict with minor suggestions. V-6 is conditional on substantive changes — if no H/M items were addressed, log the reason and skip the re-review recommendation.

---

## Review Phase

The review agent executes steps R-1 through R-6. This phase produces a structured review with a verdict.

### R-1: Implementability Check

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

#### R-1a: Checkpoint State Review (Conditional)

**Condition:** The ADR uses the checkpoint template format.

If the ADR uses the checkpoint template format, also review the state of each checkpoint:

- **Evaluation Checkpoint** — check whether the assessment was `Proceed`, `Pause for validation`, or `Skipped`. If `Skipped`, evaluate whether the rationale is sound. If the checkpoint is **blank** (no assessment written), flag this as a finding — it means the checkpoint was ignored, not consciously skipped.
- **Conclusion Checkpoint** — check whether the assessment was `Ready for review`, `Needs work`, or `Skipped`. A blank conclusion checkpoint suggests the ADR was not self-checked before requesting review.
- **Validation needs** — if populated, check whether the listed experiments were actually run and findings incorporated. If validation needs are listed but not addressed, flag as a gap.
Note: ADRs created before ADR-0024 will not have checkpoint sections. This is expected and not a finding.

#### R-1b: Experimentation Tolerance Spectrum

Rather than a binary pass/fail, assess on a three-point spectrum:

- **Well-supported** — data, PoC, or prior experience backs the decision. No concern.
- **Needs validation** — the decision makes claims that could be tested but aren't. Recommend a prototype or spike before implementation.
- **Deliberately experimental** — the decision explicitly acknowledges uncertainty and is designed to learn. Acceptable if the ADR frames it honestly (not a Fairy Tale anti-pattern).

The key insight: "flying blind" is fine when *intentional* and *acknowledged*. It's a problem when the ADR presents unvalidated claims as fact.

### R-2: Fallacy Scan

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

### R-3: Anti-Pattern Check

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

### R-4: Consequence Validation

Review the Consequences section (Nygard format) or Pros/Cons sections (MADR format) to verify factual accuracy. AI-drafted ADRs are especially prone to plausible-sounding but unverified assertions.

For each stated consequence or pro/con:

1. **Present the assertion** — quote the specific consequence from the ADR.
2. **Assess plausibility** — flag speculative, unsubstantiated, or overly optimistic/pessimistic assertions. Look for quantitative claims without evidence, unqualified absolutes, or predictions stated as facts.
3. **Ask the user directly** — confirm whether the stated consequence matches their understanding of reality.
4. **Suggest revisions** — if inaccurate or unverified, propose revised wording that accurately reflects the level of certainty.

If the ADR has many consequences, group related ones — but never skip this step entirely.

**Autonomous mode:** The reviewer assesses plausibility on its own. Flag speculative assertions with `<!-- Unverified: [reason] -->`. The reviewer cannot confirm real-world accuracy — it can only flag assertions that lack cited evidence or use unqualified absolutes.

### R-5: Review Checklist

Answer each question:

1. Is the problem significant enough to warrant an ADR?
2. Do the considered options actually solve the stated problem?
3. Are the evaluation criteria valid and relevant?
4. Are criteria appropriately prioritized?
5. Does the chosen solution address the problem and criteria?
6. Are consequences (positive and negative) reported objectively?
7. Is the ADR actionable and traceable?

### R-6: Verdict

Provide a summary verdict:

- **Accept** — ADR is ready. Minor suggestions only. Transition status from `Proposed` to `Ready`.
- **Revise** — ADR has addressable gaps. List specific changes needed.
- **Rethink** — Fundamental issues with the decision or analysis. Explain why.

> **Note:** The "Accept" verdict triggers a status transition from `Proposed` to `Ready` — meaning the decision is reviewed and approved, ready for implementation. The `Ready` status is the maximum that author-adr sets (P-1). The `Planned` and `Accepted` statuses are set by implement-adr during plan execution.

When the verdict is **Accept**, append a review cycle marker to the ADR's `## Comments` section (see V-5c):

```markdown
<!-- Review cycle 1 — [YYYY-MM-DD] — Verdict: Accept. No findings. -->
```

#### R-6a: Accept-with-Suggestions Polish Pass (Conditional)

**Condition:** Accept verdict with minor suggestions.

When the verdict is **Accept** but includes minor suggestions (e.g., editorial improvements, phrasing refinements, additional context), those suggestions are recorded in the review cycle marker but not acted on by default. To prevent silently dropping feedback:

1. **If an editor agent is configured** (`[author.dispatch].editor`): dispatch the editor agent with the Accept suggestions for a lightweight polish pass. The editor applies non-blocking improvements and logs what it changed. This is not a full revision cycle — no re-review is needed.
2. **If no editor agent is configured** (`editor = "interactive"` or absent): present the suggestions to the user as optional improvements. Let the user decide whether to apply them.

The polish pass is optional — Accept means the ADR is ready. But configured editor agents should apply minor feedback rather than drop it.

---

## Revision Phase

The editor agent (or user, when `editor = "interactive"`) executes steps V-1 through V-6. This phase addresses review findings and produces a revision summary.

### When to Activate

Activate when a review completes with a **Revise** verdict, the user asks to "address review comments" or "revise this ADR," or review findings exist that need author response.

### Guard Rails

1. **Don't modify unaddressed sections** — only change ADR content that corresponds to a finding the user chose to address.
2. **Preserve author voice** — when the user provides custom wording, use it verbatim. Do not editorialize or "improve" the user's text.
3. **Record rejections honestly** — a rejected finding is a valid outcome. Do not pressure the user to address findings they chose to reject.
4. **Respect priority ordering** — present high-priority items first so the user can focus on what matters most. If the user wants to skip remaining low-priority items, allow it.
5. **One finding at a time** — present findings individually to give each one proper attention. Do not batch multiple findings into a single prompt unless the user requests it.
6. **Respect the semantic boundary (P-3)** — see Policies above.
7. **Preserve existing addendum entries (P-4)** — see Policies above.

### V-1: Load Review Comments

Parse the review output from the preceding review phase. The review follows a structured format:

```markdown
### Implementability
### Fallacies Detected
### Anti-Patterns Detected
### Consequence Validation
### Checklist
### Verdict: Revise
```

Extract discrete revision items from sections that report issues:
- **Implementability** — each criterion marked ⚠️ or ❌ becomes an item
- **Fallacies** — each detected fallacy becomes an item
- **Anti-Patterns** — each detected anti-pattern becomes an item
- **Consequence Validation** — each consequence flagged for revision becomes an item
- **Checklist** — each question answered with ⚠️ or concerns becomes an item
- **Verdict recommendations** — each specific change listed becomes an item

Skip sections that report clean results (e.g., "None detected").
Assign each item a priority based on the review:
- **(H)** — high priority, should be addressed
- **(M)** — medium priority, recommended
- **(L)** — low priority, optional improvement

### V-2: Present Each Comment Interactively

For each revision item, starting with highest priority, present:
1. **The finding** — quote or summarize the review comment
2. **The relevant ADR section** — identify which part is affected
3. **The recommendation** — the reviewer's suggested change, if any

### V-3: Collect User Response

For each comment, offer three choices (when `[author.dispatch].editor` is set to an agent reference, "the user" is the configured editor agent):

- **Address** — the user provides revised text, additional context, or agrees with the suggested wording.
  - If the user agrees with a suggested rewording, confirm and note it.
  - If the user provides their own wording, use it verbatim.
  - If the user provides context, draft revised wording and confirm with them.

- **Reject** — the user marks the comment as considered but declined. Record the rejection with its reason.

> ⚠️ **Self-check:** If your rejection rationale uses words like "deferred," "at implementation time," "belongs in," or "out of scope," reconsider — you are likely describing a Defer, not a Reject.

- **Defer** — the concern is valid but out of scope for this ADR. The user provides a redirect destination. Record the deferral with its redirect.

#### V-3a: Defer Mechanics

When the user (or editor agent) selects Defer:
1. **Acknowledge the concern** — confirm the finding surfaces something real.
2. **State the scope boundary** — explain why it's out of scope for this ADR.
3. **Provide redirect** — name where the concern belongs.

A simple test: "Does the concern belong somewhere else?" If yes, Defer. If the concern is simply wrong or irrelevant, Reject.

### V-4: Apply Revisions

After all comments are processed, apply the accumulated changes:

- Edit only sections the user addressed. Do not modify sections where no revision was requested.
- Apply changes using precise text replacement — do not rewrite uninvolved sections.
- If multiple findings affect the same section, combine the revisions coherently.

### V-5: Produce a Revision Summary

Output a summary table:

```markdown
### Revision Summary

| # | Finding | Priority | Action | Change |
|---|---------|----------|--------|--------|
| 1 | [finding title] | H | Addressed | [brief description of change] |
| 2 | [finding title] | M | Deferred | → [redirect destination] |
| 3 | [finding title] | L | Rejected | [author's reason] |
| ... | ... | ... | ... | ... |

**X addressed, Y deferred, Z rejected.**
```

#### V-5b: Append Q&A Addendum to ADR

After producing the revision summary, generate a Q&A addendum and append it to the ADR file. This preserves the revision dialogue alongside the decision record.

**Building the addendum entries:**
Transform each revision item from V-2/V-3 into a Q&A entry:
- The **finding** becomes a `### Q:` heading, rephrased as a question.
- The **response** becomes the answer, prefixed with `**Addressed**` or `**Rejected**`.
- For addressed items, briefly describe what was changed. For rejected items, include the reason.

**Detecting existing revision entries:**

Before appending, scan the ADR file for an existing `<!-- Generated by the revise task -->` comment under `## Comments`:

- **If not found (first round):** Append the HTML comment and Q&A entries after any existing content in `## Comments` (e.g., after a Draft Worksheet):

  ```markdown
  <!-- Generated by the revise task. Do not edit above the horizontal rule. -->

  ### Q: [finding phrased as question]
  **Addressed** — [what was changed]

  ### Q: [next finding]
  **Deferred** — [scope boundary explanation] → Follow-up: [redirect destination]

  ### Q: [next finding]
  **Rejected** — [author's reason]
  ```

- **If found (multi-round):** Append new entries below existing ones, preceded by a round marker:

  ```markdown
  <!-- Round 2 -->

  ### Q: [new finding from this round]
  **Addressed** — [what was changed]
  ```

The `---` horizontal rule above `## Comments` is the **semantic boundary**: content above is the immutable decision record; content below is a mutable worksheet.

**Writing the addendum:** Use the edit tool to append the addendum content under `## Comments` at the end of the ADR file. Do not modify any content above the `---` separator as part of this step — ADR body edits were already applied in V-4.

#### V-5c: Record Review Cycle

After the Q&A addendum is written (or after an Accept verdict with no findings), append a review cycle marker:

```markdown
<!-- Review cycle [N] — [YYYY-MM-DD] — Verdict: [Accept/Revise]. X addressed, Y deferred, Z rejected. -->
```

For an **Accept verdict with no findings**, the review phase appends:

```markdown
<!-- Review cycle 1 — [YYYY-MM-DD] — Verdict: Accept. No findings. -->
```

This provides an audit trail of review activity. Every ADR that goes through a review should have at least one cycle marker in its Comments section, even when the review produces no findings.

### V-6: Recommend Re-Review (Conditional)

**Condition:** Substantive changes were made (any H or M priority items addressed).

If substantive changes were made, suggest:

> Substantive changes were made. Would you like to re-review this ADR to verify the revisions resolved the original findings?

If only L priority items were addressed or all items were rejected, the re-review suggestion is optional.

When the editor is delegated, the editor agent decides whether to re-review instead of the user. The loop continues per the dispatch config until the review verdict is Accept or the cycle limit is reached.

---

## Editor Dispatch

Per ADR-0031, the `editor` hook in `[author.dispatch]` controls who handles the interactive steps. When `editor` is set to an agent reference (anything other than `"interactive"`), that agent stands in for the user during:

- **Consequence validation** (R-4) — the editor agent confirms or flags consequences instead of the user.
- **Finding triage** (V-3) — the editor agent chooses Address, Reject, or Defer for each finding.
- **Re-review decision** (V-6) — the editor agent decides whether to re-review after revisions.

The editor agent receives this reference (`polish.md`) as its prompt along with the review output and ADR content. The agent's persona shapes its Address/Reject/Defer decisions but the task structure remains identical.

**Defer and the editor persona:** The Defer verb expresses "valid concern, wrong ADR" — rejection ≠ ignoring; always note where the concern DOES belong.

**Multi-round convergence:** When the editor is delegated, the review→revise→re-review loop runs per the dispatch config until the review verdict is Accept or the cycle limit is reached.

**Default behavior:** When `editor = "interactive"` or absent, all interactive steps prompt the user directly. The guard rails apply equally to human users and editor agents.

### Intent Grounding (Draft Worksheet)

Per ADR-0032, the editor (human or persona agent) reads the **Draft Worksheet** from the ADR's `## Comments` section for intent grounding during triage.

When a Draft Worksheet is present, use it during Address/Reject/Defer decisions:
- **Framing** — assess whether a finding aligns with the author's stated direction. A finding that contradicts it requires stronger justification.
- **Uncertainty** — distinguish "the author knew about this" from "this is a gap the author acknowledged."
- **Tolerance levels** — understand the author's appetite for scope expansion.

> ⚠️ **Tolerance calibration warning:** Tolerance levels control how much the agent diverges during **option discovery**. They do **NOT** control how aggressively the editor accepts or rejects review findings. Low improvisation tolerance means "stay close to the framing when exploring options," not "reject findings that challenge the approach." A finding about a genuine quality gap is valid regardless of the author's improvisation tolerance.

If no Draft Worksheet exists, proceed with triage using only the review output and ADR content.

---

## Output Formats

### Review Output

```markdown
## ADR Review: [title]

### Implementability
- Criteria: ...
- Documentation: ...
- Experimentation Tolerance: ...
- Scope Clarity: ...
- Actionable Consequences: ...
- Dependency Visibility: ...
- Checkpoint State: ... (if applicable)

### Fallacies Detected
[list or "None detected"]

### Anti-Patterns Detected
[list or "None detected"]

### Consequence Validation
[list of assertions reviewed, any revisions made]

### Checklist
[7 answers]

### Verdict: [Accept/Revise/Rethink]
[summary and specific recommendations]
```

### Revision Output

```markdown
## ADR Revision: [title]

### Revision Summary

| # | Finding | Priority | Action | Change |
|---|---------|----------|--------|--------|
| 1 | ... | H | Addressed | ... |
| 2 | ... | M | Deferred | → [redirect] |
| 3 | ... | L | Rejected | ... |

**X addressed, Y deferred, Z rejected.**
```

The revision also appends Q&A entries to the ADR's `## Comments` section (see V-5b) — written directly to the ADR file, not part of the conversational output.
