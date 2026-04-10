# 64. Add option-evaluation dispatch hooks for UX/DX reviewer and TPM integration

Date: 2026-04-09
Status: Accepted
Last Updated: 2026-04-09
Links:
- Extends [ADR-0031](0031-add-author-adr-dispatch-hooks-for-custom-agent-delegation.md) (author-adr dispatch hooks)
- Related to [ADR-0024](0024-add-checkpoint-sections-to-nygard-agent-template.md) (Evaluation Checkpoint)
- Related to [ADR-0059](0059-add-optional-code-review-hook-to-solve-adr-branch-completion.md) (code review dispatch hook)
- Related to [ADR-0060](0060-support-list-of-code-review-agents-in-solve-adr-dispatch.md) (code review agent list)

## Context

The `author-adr` dispatch hooks (ADR-0031) cover three hook points: `tech_writer` (Step 3b), `review` (A-3), and `editor` (A-4). The `solve-adr` code review hooks (ADR-0059/0060) cover `code_review` (C-2). No hook exists for option-time evaluation.

Three agents now exist that fill that role:

| Agent | Evaluates |
|-------|-----------|
| `juliusl-ux-reviewer-v1` | CLI, UI, and Guide interfaces — visual clarity, escape hatches, discoverability, usability |
| `juliusl-dx-reviewer-v1` | Type abstractions, APIs, build procedures, and dependencies — pragmatism, consistency, error surface, maintainability |
| `juliusl-tpm-v1` | Decision quality — ASR, START, ADMM tests, anti-pattern detection, justification validation |

None of these agents have integration points in the authoring workflow. ADR-0031 anticipated this gap and named "Solve — option evaluation" as a future hook point — these agents are the first concrete candidates for that hook. The Evaluation Checkpoint (Step 4 of `create.md`) is the existing gate between Options and Decision; it evaluates option readiness before a decision is made. UX/DX evaluation and TPM decision quality assessment operate on the same input at the same moment.

### Decision Drivers

- UX and DX agents exist but have no dispatch mechanism in the authoring workflow.
- TPM decision quality tests (ASR, START, ADMM) are applied manually or not at all during option analysis.
- The Evaluation Checkpoint is the Options→Decision gate — the natural integration point for pre-decision review.
- ADR-0031 established the dispatch pattern; new hooks must follow the same mechanism.
- All hooks must be optional — default behavior must be preserved for unconfigured setups.

## Options

### Option A: Enrich the Evaluation Checkpoint with UX/DX + TPM dispatch

Extend Step 4 of the create workflow with two new sub-steps:

- **Step 4a** — dispatch `ux_review` and `dx_review` agents in parallel on the Options section. Each agent runs its standard review procedure and returns findings using its established output format.
- **Step 4b** — dispatch `tpm` with the ADR content and UX/DX findings. The TPM applies ASR, START, and ADMM tests, detects anti-patterns, and produces a readiness assessment.

The TPM verdict feeds the checkpoint's Assessment field (`Proceed` / `Pause for validation` / `Skipped`).

New dispatch keys added to `[author.dispatch]`:

| Key | Agent |
|-----|-------|
| `ux_review` | UX reviewer |
| `dx_review` | DX reviewer |
| `tpm` | Technical PM |

**Good:** Reuses the existing checkpoint gate — no new workflow steps. Natural integration point. Follows the ADR-0031 dispatch pattern.

**Bad:** Extends the Evaluation Checkpoint's scope beyond its original completeness-check role. Makes the checkpoint heavier — 2–3 agent dispatches where there were none.

---

### Option B: Add a new "Option Quality Review" step before the Evaluation Checkpoint

Insert **Step 3c** between Options drafting and the Evaluation Checkpoint:

- **Step 3c** — dispatch `ux_review` and `dx_review` in parallel on the Options section. Incorporate findings into the option analysis before the checkpoint.
- **Step 4a** — dispatch `tpm` at the Evaluation Checkpoint.

Same dispatch keys as Option A.

**Good:** Clean separation — option review (Step 3c) is distinct from decision quality assessment (Step 4). The checkpoint retains its original completeness-check scope. UX/DX findings enrich options before the TPM evaluates them.

**Bad:** Adds a new step to the create workflow. Increases workflow complexity — authors must understand two new steps instead of one extended step.

---

### Option C: Dispatch at solve-adr level instead of author-adr

After `author-adr` returns a draft, `solve-adr` dispatches UX/DX and TPM at its Step 3 (Triage). If findings require changes, `solve-adr` re-invokes `author-adr` for revisions.

**Good:** Keeps `author-adr` simple — no new steps or sub-steps.

**Bad:** Only works when `author-adr` is invoked from `solve-adr`. Options are already evaluated when dispatch occurs — too late for option-level feedback. Loses the "evaluate before the decision converges" benefit that UX/DX review provides.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**
- Combined UX/DX→TPM pipeline at checkpoint not yet prototyped — integration validated by pattern extension from ADR-0031.

## Decision

**Option A — Enrich the Evaluation Checkpoint with UX/DX + TPM dispatch.**

In the context of integrating UX/DX reviewers and TPM into the ADR authoring workflow, facing the need for option-time quality evaluation before decisions converge, we chose to enrich the Evaluation Checkpoint with dispatch hooks over adding a new step or dispatching at solve-adr level, to achieve natural integration at the existing Options→Decision gate and reuse of the checkpoint's assessment mechanism, accepting that the checkpoint's scope expands beyond its original completeness-check role.

The Evaluation Checkpoint evaluates option readiness — UX/DX and TPM assessment operate on the same input. Option B separates review from assessment cleanly, but a pre-checkpoint review step (3c) doesn't reduce what the checkpoint must assess — both A and B end up dispatching the same agents; Option A does it in one stage. Option C works only when `author-adr` is invoked from `solve-adr`; standalone `author-adr` usage gets no option review. When options have already been written and `author-adr` returns a draft, UX/DX findings are most valuable before the decision converges — a solve-adr-level re-invoke loop is more expensive than a checkpoint-stage dispatch.

## Consequences

**Positive:**

- UX/DX findings are available before the decision converges, when option changes are still low-cost.
- TPM applies decision quality discipline — ASR, START, and ADMM tests run at the checkpoint rather than being applied manually or skipped.
- Option evaluation moves from single-agent to multi-perspective review without adding workflow steps.
- Follows the established dispatch pattern from ADR-0031 — no new configuration mechanism required.

**Negative:**

- The Evaluation Checkpoint becomes heavier — 2–3 agent dispatches where there were none. Each dispatch adds wall-clock time to the checkpoint step.
- Dispatch keys in `[author.dispatch]` grow from 3 to 6. At 8+ keys, a sub-table becomes warranted per the ADR-0014 flat-table constraint. Monitor density and revisit if keys continue to grow.

**Neutral:**

- All three hooks are optional. Empty or absent keys preserve current behavior — no UX/DX review, no TPM, checkpoint runs as before.
- UX, DX, and TPM hooks configure independently — any subset works. An author can enable TPM without enabling UX/DX review. When TPM is configured without UX/DX hooks, it operates on ADR content alone — UX/DX findings are an optional enrichment, not a prerequisite.

## Quality Strategy

- [ ] ~~Introduces major semantic changes~~
- [x] Introduces minor semantic changes (three new `[author.dispatch]` keys)
- [ ] ~~Fuzz testing~~
- [ ] ~~Unit testing~~
- [ ] ~~Load testing~~
- [ ] ~~Performance testing~~
- [x] Backwards Compatible (all hooks optional — absent or empty keys preserve current behavior)
- [ ] ~~Integration tests~~
- [ ] ~~Tooling~~
- [x] User documentation (dispatch configuration reference, Step 4a/4b step descriptions in `create.md`)

### Additional Quality Concerns

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Three specialized agents exist in `src/agents/` with no integration points in the skill workflows:
- `juliusl-ux-reviewer-v1` — evaluates CLI, UI, and Guide interfaces for user experience quality
- `juliusl-dx-reviewer-v1` — evaluates type abstractions, APIs, build procedures, and dependencies for developer experience quality
- `juliusl-tpm-v1` — applies decision quality tests (ASR, START, ADMM), detects anti-patterns, validates justifications

The user's hypothesis: UX/DX reviewers should run early — during option enumeration in the authoring workflow — and their findings should flow to the TPM for overall decision quality assessment. ADR-0031 explicitly identified "Solve — option evaluation" as a future hook point.

The existing dispatch hooks (ADR-0031) cover: review (A-3), editor (A-4), tech_writer (A-2). The code review dispatch (ADR-0059/0060) covers post-implementation review in solve-adr (C-2). No dispatch exists for option-time evaluation.

**Tolerance:**
- Risk: Low — extending an established dispatch pattern (ADR-0031)
- Change: Medium — adding new workflow steps and dispatch keys
- Improvisation: Low — user has clear direction on the pipeline flow

**Uncertainty:**
- Known: agents exist and work, dispatch pattern is established, Evaluation Checkpoint (ADR-0024) is the gate between Options and Decision
- Uncertain: exact insertion point — extend the Evaluation Checkpoint or add a new step before it

**Options:**
- Target count: 2-3
- [x] Explore additional options beyond candidates listed below

**Candidates:**
- A: Enrich the Evaluation Checkpoint (Step 4) with UX/DX + TPM dispatch — extend the existing gate
- B: Add a new "Option Quality Review" step before the Evaluation Checkpoint — separate review from assessment
- C: Dispatch at solve-adr level (Step 3 Triage) instead of author-adr — higher-level orchestration

<!-- Review cycle 1 — 2026-04-10 — Verdict: Accept. 3 minor suggestions triaged by editor. -->
