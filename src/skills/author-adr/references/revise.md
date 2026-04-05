# Revising an ADR

Self-contained reference for the ADR revision workflow. Read this file when the user wants to address review comments after a review with a "Revise" verdict.

## When to Use

Activate this workflow when:
- A review has just completed with a **Revise** verdict
- The user asks to "address review comments," "revise this ADR," or "go through the revision"
- Review findings exist that need author response

## Revision Process

### Step 1: Load Review Comments

Parse the review output from the preceding review step. The review follows a structured format (see `review.md` Output Format):

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

### Step 2: Present Each Comment Interactively

For each revision item, starting with highest priority, present:

1. **The finding** — quote or summarize the review comment
2. **The relevant ADR section** — identify which part of the ADR is affected (Context, Decision, Consequences, etc.)
3. **The recommendation** — the reviewer's suggested change, if any

### Step 3: Collect User Response

For each comment, offer the user three choices (when `[author.dispatch].editor` is set to an agent reference, "the user" is the configured editor agent — see [Editor Dispatch](#editor-dispatch)):

- **Address** — the user provides revised text, additional context, or agrees with the suggested wording. The agent notes the revision to apply.
  - If the user agrees with a suggested rewording, confirm and note it.
  - If the user provides their own wording, use it verbatim.
  - If the user provides context, draft revised wording and confirm with them.

- **Reject** — the user marks the comment as considered but declined. The user may optionally provide a reason. Record the rejection with its reason.

- **Defer** — the concern is valid but out of scope for this ADR. The user provides a redirect destination (e.g., "ADR-NNNN," "the implementing ADR," "a future decision about X"). Record the deferral with its redirect.

#### Defer Mechanics

When the user (or editor agent) selects Defer:

1. **Acknowledge the concern** — confirm the finding surfaces something real.
2. **State the scope boundary** — explain why it's out of scope for this ADR.
3. **Provide redirect** — name where the concern belongs (specific ADR number, "the implementing ADR," "a future decision about X," etc.).

A simple test distinguishes Reject from Defer: "Does the concern belong somewhere else?" If yes, Defer. If the concern is simply wrong or irrelevant, Reject.

### Step 4: Apply Revisions

After all comments are processed, apply the accumulated changes to the ADR file:

- Edit only the sections that the user addressed. Do not modify sections where no revision was requested.
- Apply changes using precise text replacement — do not rewrite sections that weren't part of a finding.
- If multiple findings affect the same section, combine the revisions coherently.

### Step 5: Produce a Revision Summary

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

### Step 5b: Append Q&A Addendum to ADR

After producing the revision summary, generate a Q&A addendum and append it to
the ADR file. This preserves the revision dialogue alongside the decision record
for posterity (see ADR-0016).

**Building the addendum entries:**

Transform each revision item from Steps 2–3 into a Q&A entry:
- The **finding** becomes a `### Q:` heading, rephrased as a question about the
  ADR (e.g., "Does the ADR provide sufficient evidence?" rather than quoting the
  raw finding).
- The **user's response** (address or reject) becomes the answer, prefixed with
  `**Addressed**` or `**Rejected**`.
- For addressed items, briefly describe what was changed.
- For rejected items, include the author's stated reason.

**Detecting existing revision entries:**

Before appending, scan the ADR file for an existing `<!-- Generated by the revise task -->` comment under `## Comments`:

- **If not found (first round):** Append the HTML comment and Q&A entries
  directly under the existing `## Comments` heading:

  ```markdown
  ## Comments

  <!-- Generated by the revise task. Do not edit above the horizontal rule. -->

  ### Q: [finding phrased as question]

  **Addressed** — [what was changed]

  ### Q: [next finding]

  **Deferred** — [scope boundary explanation] → Follow-up: [redirect destination]

  ### Q: [next finding]

  **Rejected** — [author's reason]
  ```

- **If found (multi-round):** Append new Q&A entries below the existing ones,
  preceded by a round marker:

  ```markdown
  <!-- Round 2 -->

  ### Q: [new finding from this round]

  **Addressed** — [what was changed]
  ```

The `---` horizontal rule above `## Comments` is the **semantic boundary**
(already present in the template): content above is the immutable decision
record; content below is a mutable worksheet. Do **not** add an additional
`---` or a separate `## Revision Addendum` heading — the `## Comments` section
serves this role.

**Writing the addendum:**

Use the edit tool to append the addendum content under `## Comments` at the end
of the ADR file. Do not modify any content above the `---` separator (or above
existing Q&A entries in multi-round scenarios) as part of this step — ADR body
edits were already applied in Step 4.

### Step 6: Recommend Re-Review

If substantive changes were made (any H or M priority items addressed), suggest:

> Substantive changes were made. Would you like to re-review this ADR to
> verify the revisions resolved the original findings?

If only L priority items were addressed or all items were rejected, the re-review suggestion is optional.

When the editor is delegated (see [Editor Dispatch](#editor-dispatch)), the editor agent decides whether to re-review instead of the user. The loop continues per the dispatch config until the review verdict is Accept or the cycle limit is reached.

## Guard Rails

1. **Don't modify unaddressed sections** — only change ADR content that corresponds to a finding the user chose to address.
2. **Preserve author voice** — when the user provides custom wording, use it verbatim. Do not editorialize or "improve" the user's text.
3. **Record rejections honestly** — a rejected finding is a valid outcome. Do not pressure the user to address findings they chose to reject.
4. **Respect priority ordering** — present high-priority items first so the user can focus on what matters most. If the user wants to skip remaining low-priority items, allow it.
5. **One finding at a time** — present findings individually to give each one proper attention. Do not batch multiple findings into a single prompt unless the user requests it.
6. **Respect the semantic boundary** — the `---` separator above `## Comments` divides the immutable decision record (above) from the mutable revision worksheet (below). When appending Q&A entries, never modify content above the separator.
7. **Preserve existing addendum entries** — in multi-round revisions, existing Q&A entries from prior rounds must not be modified or removed. New entries are appended below existing ones.

## Editor Dispatch

Per ADR-0031, the `editor` hook in `[author.dispatch]` controls who handles the interactive steps in the revision workflow. When `editor` is set to an agent reference (anything other than `"interactive"`), that agent stands in for the user during:

- **Consequence validation** (review Step 4) — the editor agent confirms or flags consequences instead of the user.
- **Finding triage** (revise Step 3) — the editor agent chooses Address, Reject, or Defer for each finding.
- **Re-review decision** (revise Step 6) — the editor agent decides whether to re-review after revisions.

The editor agent receives this reference (`revise.md`) as its prompt along with the review output and ADR content. The agent's persona shapes its Address/Reject/Defer decisions — which findings it prioritizes, how it weighs scope boundaries, what editorial judgment it brings — but the task structure remains identical.

**Defer and the editor persona:** The Defer verb (ADR-0033) implements the v3 editor principle 1 IMPORTANT clause: "rejection ≠ ignoring — always note where the concern DOES belong." When the editor recognizes a scope-redirect pattern, it uses Defer to express "valid concern, wrong ADR" rather than approximating with Address or Reject.

**Multi-round convergence:** When the editor is delegated, the review→revise→re-review loop runs per the dispatch config. The editor agent decides whether to trigger re-review (Step 6) on each cycle. The loop continues until the review verdict is Accept or the cycle limit is reached.

**Default behavior:** When `editor = "interactive"` or the `[author.dispatch]` table is absent, all interactive steps prompt the user directly — identical to the current workflow. The guard rails above apply equally to human users and editor agents.

## Output Format

Structure the final output as:

```markdown
## ADR Revision: [title]

### Revision Summary

| # | Finding | Priority | Action | Change |
|---|---------|----------|--------|--------|
| 1 | ... | H | Addressed | ... |
| 2 | ... | M | Deferred | → [redirect] |
| 3 | ... | L | Rejected | ... |

**X addressed, Y deferred, Z rejected.**

[Re-review recommendation if applicable]
```

The revise task also appends Q&A entries to the ADR's `## Comments` section (see
Step 5b). The entries are written directly to the ADR — they are not part of the
conversational output above. The format appended under `## Comments` is:

```markdown
## Comments

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: [finding phrased as question]

**Addressed** — [what was changed]

### Q: [next finding]

**Deferred** — [scope boundary explanation] → Follow-up: [redirect destination]

### Q: [next finding]

**Rejected** — [author's reason]
```
