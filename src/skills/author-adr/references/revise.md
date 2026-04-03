# Revising an ADR

Self-contained reference for the ADR revision workflow. Read this file when
the user wants to address review comments after a review with a "Revise"
verdict.

## When to Use

Activate this workflow when:
- A review has just completed with a **Revise** verdict
- The user asks to "address review comments," "revise this ADR," or "go
  through the revision"
- Review findings exist that need author response

## Revision Process

### Step 1: Load Review Comments

Parse the review output from the preceding review step. The review follows a
structured format (see `review.md` Output Format):

```markdown
### Completeness (ecADR)
### Fallacies Detected
### Anti-Patterns Detected
### Consequence Validation
### Checklist
### Verdict: Revise
```

Extract discrete revision items from sections that report issues:

- **Completeness** — each criterion marked ⚠️ or ❌ becomes an item
- **Fallacies** — each detected fallacy becomes an item
- **Anti-Patterns** — each detected anti-pattern becomes an item
- **Consequence Validation** — each consequence flagged for revision becomes
  an item
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
2. **The relevant ADR section** — identify which part of the ADR is affected
   (Context, Decision, Consequences, etc.)
3. **The recommendation** — the reviewer's suggested change, if any

### Step 3: Collect User Response

For each comment, offer the user two choices:

- **Address** — the user provides revised text, additional context, or agrees
  with the suggested wording. The agent notes the revision to apply.
  - If the user agrees with a suggested rewording, confirm and note it.
  - If the user provides their own wording, use it verbatim.
  - If the user provides context, draft revised wording and confirm with them.

- **Reject** — the user marks the comment as considered but declined. The user
  may optionally provide a reason. Record the rejection with its reason.

### Step 4: Apply Revisions

After all comments are processed, apply the accumulated changes to the ADR
file:

- Edit only the sections that the user addressed. Do not modify sections where
  no revision was requested.
- Apply changes using precise text replacement — do not rewrite sections that
  weren't part of a finding.
- If multiple findings affect the same section, combine the revisions
  coherently.

### Step 5: Produce a Revision Summary

Output a summary table:

```markdown
### Revision Summary

| # | Finding | Priority | Action | Change |
|---|---------|----------|--------|--------|
| 1 | [finding title] | H | Addressed | [brief description of change] |
| 2 | [finding title] | M | Rejected | [author's reason] |
| ... | ... | ... | ... | ... |

**X addressed, Y rejected.**
```

### Step 6: Recommend Re-Review

If substantive changes were made (any H or M priority items addressed),
suggest:

> Substantive changes were made. Would you like to re-review this ADR to
> verify the revisions resolved the original findings?

If only L priority items were addressed or all items were rejected, the
re-review suggestion is optional.

## Guard Rails

1. **Don't modify unaddressed sections** — only change ADR content that
   corresponds to a finding the user chose to address.
2. **Preserve author voice** — when the user provides custom wording, use it
   verbatim. Do not editorialize or "improve" the user's text.
3. **Record rejections honestly** — a rejected finding is a valid outcome.
   Do not pressure the user to address findings they chose to reject.
4. **Respect priority ordering** — present high-priority items first so the
   user can focus on what matters most. If the user wants to skip remaining
   low-priority items, allow it.
5. **One finding at a time** — present findings individually to give each one
   proper attention. Do not batch multiple findings into a single prompt
   unless the user requests it.

## Output Format

Structure the final output as:

```markdown
## ADR Revision: [title]

### Revision Summary

| # | Finding | Priority | Action | Change |
|---|---------|----------|--------|--------|
| 1 | ... | H | Addressed | ... |
| 2 | ... | M | Rejected | ... |

**X addressed, Y rejected.**

[Re-review recommendation if applicable]
```
