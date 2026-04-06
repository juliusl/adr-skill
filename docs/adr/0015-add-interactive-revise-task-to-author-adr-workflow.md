# 15. Add interactive revise task to author-adr workflow

Date: 2026-04-02

## Status

Accepted

## Context

The author-adr skill's review workflow (see `references/review.md`) produces a
structured review with a verdict of **Accept**, **Revise**, or **Rethink**. When
the verdict is "Revise," the review output includes specific findings and
recommendations — but the workflow stops there. The user is left to manually
interpret the review comments and apply changes to the ADR on their own.

This creates a gap between review and the next meaningful state of the ADR:

1. **No structured follow-through** — the review produces actionable comments,
   but there is no guided process for addressing them. The user must mentally
   track which comments they've handled and which they've deferred.

2. **No mechanism to disagree** — not all review comments are valid. A reviewer
   (especially an AI reviewer) may flag issues that the author intentionally
   chose to accept or that reflect a misunderstanding of context. Currently
   there is no way to formally record that a comment was considered and
   rejected.

3. **Lost context** — if the user addresses review comments in a separate
   session or after a delay, the connection between the review findings and the
   resulting edits is lost. There is no audit trail of what was addressed and
   what was not.

4. **Review-revise cycle is common** — dogfooding has shown that "Revise" is
   a frequent verdict for non-trivial ADRs. The skill should support the full
   create → review → revise → accept lifecycle, not just create → review.

### Considered Alternatives

1. **Batch revision** — present all review comments at once and let the user
   edit the ADR file directly. Rejected because this is effectively what
   happens today (no tooling support), and it doesn't solve the tracking or
   rejection problems.

2. **Automatic revision** — have the agent apply all review recommendations
   automatically without user input. Rejected because review comments often
   require judgment — the author may disagree, want different wording, or need
   to add context that only they possess. Automatic application also removes
   the author's ownership of the document.

## Decision

Add an interactive **revise** task to the author-adr skill workflow. The revise
task is triggered after a review that produces a "Revise" verdict and walks the
user through each review comment one at a time.

### Workflow

The revise task follows this flow:

1. **Load review comments** — parse the review output from the preceding review
   step. Each finding (from the completeness check, fallacy scan, anti-pattern
   check, consequence validation, and checklist) becomes a discrete revision
   item.

2. **Present each comment interactively** — for each revision item, display:
   - The review finding (quoted from the review output)
   - The relevant section of the ADR it applies to
   - The reviewer's recommendation (if any)

3. **Collect user response** — for each comment, the user can:
   - **Address** — write a response or revised text that the agent applies to
     the ADR. The agent updates the relevant section of the ADR with the user's
     input.
   - **Reject** — mark the comment as "rejected," indicating the author
     considered the feedback and chose not to act on it. The user may
     optionally provide a reason for rejection.

4. **Apply revisions** — after all comments are processed, the agent applies
   the accumulated changes to the ADR file.

5. **Produce a revision summary** — output a summary showing:
   - How many comments were addressed vs. rejected
   - The specific changes made to the ADR
   - Any rejected comments with the author's stated reasons

6. **Recommend re-review** — if substantive changes were made, suggest running
   the review again to verify the revisions resolved the original findings.

### Integration Point

The revise task slots into the existing workflow after the review step:

```
Creating an ADR → Reviewing an ADR → Revising an ADR → [re-review or accept]
```

In SKILL.md, the "Reviewing an ADR" section's Step 6 (Verdict) should be
updated to offer the revise task when the verdict is "Revise":

> Verdict: **Revise** — this ADR has addressable gaps.
> Would you like to interactively address the review comments now?

### Reference File

The revise workflow should be documented in a new `references/revise.md` file
following the same pattern as `references/review.md` — a self-contained
reference that the agent loads on demand when the revise task is activated.

## Consequences

- **Formalizes the review-revise workflow** — revision currently happens
  informally as a habit for experienced users. This ADR captures that behavior
  as a protocol, improving developer experience and enabling consistent
  agent-assisted revision regardless of user familiarity.
- **Gives authors a voice** — the "reject" option acknowledges that not all
  review feedback is actionable or correct. Authors can formally disagree with
  a finding while demonstrating they considered it.
- **Creates an audit trail** — the revision summary documents what was changed
  and what was intentionally left unchanged, providing traceability between
  review findings and ADR edits. The persistence format and location of the
  summary is out of scope for this ADR and will be addressed separately.
- **Moderate interaction cost** — typical reviews produce 4–7 findings, which
  is manageable. The step-by-step flow also serves as an opportunity for the
  user to verify agent reasoning, balancing autonomy with oversight.
- **Coupled to review output format** — the revise task assumes review comments
  can be parsed into discrete items. In practice, the review format is
  author-controlled and trending toward more formalization, limiting this risk.
- **New reference file** — adds `references/revise.md` to the skill. Changes to
  the review format would naturally require updating both files, and any such
  change would be captured by a future ADR.
- **Validation relies on dogfooding** — success criteria for human-agent
  interaction workflows are difficult to quantify at this stage. Validation
  will rely on continued dogfooding and qualitative assessment of the revision
  experience.

---

## Comments

<!-- Review cycle 1 — Verdict: Revise. Findings addressed inline. Predates structured Q&A addendum (ADR-0016). -->
