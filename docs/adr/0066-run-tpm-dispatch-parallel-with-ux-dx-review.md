# 66. Run TPM dispatch parallel with UX/DX review

Date: 2026-04-10
Status: Accepted
Links:
- Amends [ADR-0064](0064-add-option-evaluation-dispatch-hooks-for-review-and-decision-arbitration.md)

## Context

Step 4a (UX/DX review) and Step 4b (TPM assessment) currently run sequentially — 4b waits for 4a to complete and consumes its findings as optional enrichment. In practice, the calling agent applies all revisions after both steps return. Sequential execution adds latency without functional benefit: the TPM's assessment is independent of UX/DX findings (it applies ASR, START, and ADMM tests on the ADR content, not on reviewer output).

TODOS.md items #12 and #13 also flagged: Step 4a's Redesign verdict doesn't gate Step 4b, and execution order between the checkpoint assessment and dispatch hooks was unspecified.

## Decision

In the context of **Step 4 Evaluation Checkpoint dispatch**, facing **unnecessary sequential latency between independent reviewers**, we chose **parallel dispatch of Steps 4a and 4b** over **the current sequential model** to achieve **faster checkpoint evaluation and simpler control flow**, accepting **that the TPM no longer receives UX/DX findings as enrichment input**.

Changes:
1. Dispatch Step 4a (UX/DX) and Step 4b (TPM) in parallel — both run simultaneously via `task` tool.
2. After all agents return, the caller consolidates findings and applies the combined verdict logic.
3. Remove the "runs after Step 4a, consuming Step 4a's findings" language from Step 4b.
4. Remove the UX/DX findings from Step 4b's dispatch context — it receives only the ADR content.
5. Add combined verdict resolution: if any agent returns Redesign → Pause; if any returns Revise — Major → apply revisions; otherwise proceed.

## Consequences

**Positive:**
- Checkpoint evaluation completes faster — both dispatches run simultaneously.
- Simpler control flow — no dependency chain between 4a and 4b.
- Resolves TODOS.md items #12 (execution order) and #13 (missing gate) by eliminating the sequential dependency entirely.

**Negative:**
- TPM loses optional UX/DX enrichment context. Low impact — the TPM tests (ASR, START, ADMM) operate on ADR content, not reviewer findings.

## Quality Strategy

- [x] Unit testing
- ~~Fuzz testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [x] User documentation
