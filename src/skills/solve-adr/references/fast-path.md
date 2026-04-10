# S-3: Fast-Path Workflow

Self-contained reference for the S-3 fast-path scenario. Read when the user provides a list of findings from a retrospective, bug bash, or amendment to an existing decision.

## Trigger Types

S-3 activates for three source types:

- **Retrospective findings** — lessons from a post-mortem or sprint retro
- **Bug bash findings** — issues identified during a structured defect-hunting session
- **Amendments** — updates to existing decisions where the new direction is already understood

All other solve requests route to S-1 (Problem) or S-2 (Roadmap).

## Classification Test

Before doing anything else, apply this test to each finding:

> "Would seeing this in the ADR list give useful ambient context to someone orienting in this codebase?"

- **Yes** → ADR-worthy → route to Step 1 (Author Y-statement ADR)
- **No** → plan-only → route to Step 2 (Build plan list)

Use the ADR list as the bar, not the finding's importance. A critical bug fix that doesn't change an architectural decision is plan-only. A minor routing change that records a new architectural commitment is ADR-worthy.

## Procedure

```
Entry: Receive finding list, identify source (retro / bug-bash / amendment)
   ↓
Step 1: Classify — apply ADR list test to each finding
   ↓
Step 2: Author Y-statement ADRs — for ADR-worthy findings only
   ↓
Step 3: Build plan list — collect plan-only findings with [Source: <origin>] notes
   ↓
Step 4: Delegate — invoke /implement-adr with all Ready ADRs + plan-only task list
   ↓
C. Conclusion — C-1 → C-2 → C-3 → C-4 (same as S-1)
```

### Entry: Intake

Receive the finding list and identify the source. Record the source type (retro / bug-bash / amendment) and the date or session it came from — this becomes the provenance for any ADRs created.

If findings arrive as unstructured prose, parse them into discrete items before classifying.

---

### Step 1: Classify

Apply the ADR list test to each finding. Produce two lists:

- **ADR list** — findings that would add useful ambient context when browsing the ADR list
- **Plan list** — findings that are tasks, fixes, or improvements that don't change an architectural decision

Every finding must land in exactly one list. If a finding is ambiguous, err toward plan-only — the ADR list is a navigation aid, not an archive.

**Borderline example:** "We should use structured logging everywhere." This improves practice but doesn't change a decision already on record → plan-only. "We discovered that our chosen logging library can't handle structured output — switch to X." This changes the existing decision → ADR-worthy.

---

### Step 2: Author Y-statement ADRs

For each finding on the ADR list:

1. Create the ADR file using the Makefile.
2. Populate standard metadata:
   - `Date:`
   - `Status: Prototype`
   - `Last Updated:`
   - `Links:` (related ADRs, if any)
3. Write provenance in the **Context section**: where the finding came from (source type, date/session, finding text), and why it warrants an ADR entry. Example:
   > From the Q1 2026 retrospective: "The default retry timeout caused cascading failures in the load test." This finding changes how we configure retry behavior across all services — it warrants a decision record.
4. Write the decision as a **Y-statement in the Decision section**:
   > In the context of [X], facing [Y], we chose [Z] over [A] to achieve [B], accepting [C].
5. Fill the Quality Strategy checklist.
6. **Skip Options and Evaluation Checkpoint — absent by design.** The decision is already understood; these sections are not applicable to S-3 ADRs.
7. **Invoke author-adr A-3 onward** (review, revise, re-review). When dispatching the reviewer, include this note:
   > Options and Evaluation Checkpoint are absent by design — this is an S-3 fast-path ADR. R-1 criterion 1 (≥2 alternatives) does not apply. R-1 criterion 1 checks that at least two alternative options were considered; it is not applicable when the decision is already understood and the goal is traceability, not exploration.

Repeat for all findings on the ADR list. All ADRs must reach Ready status before proceeding to Step 4.

---

### Step 3: Build plan list

For each finding on the plan list, format it as a task with a source note:

```
Fix the timeout default in the retry handler
[Source: Q1 2026 retrospective — "timeout default caused cascading failures in load test"]
```

The `[Source: <finding-origin>]` note preserves traceability to the original finding in the plan. Include: source type, date/session if available, and the original finding text or a concise summary.

Collect all plan-only tasks into a single list. This list is passed to Step 4.

---

### Step 4: Delegate

Invoke `/implement-adr` with:
- All Ready ADRs from Step 2
- The plan-only task list from Step 3

Pass both in a single batch. The implement-adr skill will merge the plan-only tasks into the implementation plan alongside the ADR-driven tasks. Each plan-only task retains its `[Source: ...]` note in the plan.

**Zero-ADR case:** If classification in Step 1 produces zero ADR-worthy findings (all findings are plan-only), skip Step 2 entirely. In Step 4, invoke `/implement-adr` with only the plan-only task list. A bare task list is a valid input — implement-adr will generate a plan with no source ADRs. Note in the plan: "No new ADRs created — all findings were plan-only per the S-3 classification test."

---

### C. Conclusion

After Step 4 completes (implement-adr finishes), run the conclusion sequence defined in SKILL.md:

```
C-1 QA Triage → C-2 Code Review (optional) → C-3 Report → C-4 Retrospective (optional)
```

No separate conclusion procedure for S-3 — the same steps apply.
