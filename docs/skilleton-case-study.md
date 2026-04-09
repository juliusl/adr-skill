# Skilleton Case Study: Workflow Reliability

## Purpose

This case study compares four runs of the same implementation task — each with a different level of workflow structure — to measure how procedural agent skills affect engineering output quality and reliability.

## Setup

**Test subject:** [skilleton](https://github.com/juliusl/skilleton) — a Rust CLI for validating and rendering agentskills.io skill definitions.

**Task:** Milestone 3 — implement `init`, `check`, and `build` commands with TOML parsing, a Markdown rendering pipeline, and integration tests.

**Conditions:**

| | Branch A (main skills) | Branch B (atelier skills) | Branch C (zero workflow) | Branch D (review-only) |
|---|---|---|---|---|
| Skills source | `main` branch of adr-skills | `solve/adr-atelier/milestone-1` branch | None | None |
| Working branch | `solve/skilleton/milestone-3` | `solve/skilleton/milestone-3-1` | `solve/skilleton/milestone-3-c` | `solve/skilleton/milestone-3-d` |
| Session | `b536efb8` | `aa0db0f3` | Sub-agent (~5.5 min) | Sub-agent (~16 min total) |
| Merge base | `f2d7c87` | `f2d7c87` | `f2d7c87` | `f2d7c87` |
| Prompt | `/solve-adr solve milestone 3` | `/solve-adr solve milestone 3` | "implement milestone 3 from the ROADMAP.md file" | Same as C, then review + fix pass |
| Participation | Autonomous, auto-commit | Autonomous, auto-commit | Autonomous, auto-commit | Autonomous, auto-commit |

All four branches started from the same codebase state (`f2d7c87` — milestones 1 and 2 complete).

- **Branch A** and **Branch B** used the `/solve-adr` skill to orchestrate the full pipeline (author-adr → implement-adr → code review).
- **Branch C** received a bare prompt with no workflow skills — just "implement milestone 3."
- **Branch D** tested the steelman hypothesis: same bare prompt as C, followed by a review + fix pass using the same code review agents Branch B used (`juliusl-code-reviewer-analytics-v5`, `juliusl-code-reviewer-sweep-v5`) plus adversarial QA. No ADRs, no plans, no QA plans.

**Known confound:** During Branch A's session the user discovered previously installed (older) skill files were present. Branch A may have run with a stale skill version rather than current `main`.

**D-round prompt fairness note:** The Phase 2 review prompt included pointed questions ("does build validate before rendering?", "are there negative fixtures?") that hint at known C-round gaps. This makes D's results a slight upper bound on what pure review-only buys.

---

## Session Turn Data

### Branch A — 6 turns, multiple interventions

| Turn | User prompt | What happened |
|---|---|---|
| 0 | `/solve-adr solve milestone 3` | No assistant response recorded — invocation failed or produced nothing visible |
| 1 | `/solve-adr solve milestone 3` (retry) | Full execution: 3 ADRs authored (0009, 0010, 0011), plan generated, QA ran, all tasks implemented. 18 files touched. |
| 2 | "perform a retroactive of how this session went" | Self-scored **78/100**. Identified 4 procedural gaps. |
| 3 | "prepare this branch for merging to base" | Code review ran (generic reviewer only). No significant issues found. |
| 4 | "Oh I see what happened I have the old skills installed" | Root cause identified: configured custom reviewers (`juliusl-code-reviewer-analytics-v5`, `juliusl-code-reviewer-sweep-v5`) were not available as agent types. |
| 5 | `/solve-adr solve milestone 3, this is a B round` | Kicks off Branch B |

**Observations:** The workflow required a retry on the first turn, a manual retroactive request to surface gaps, a separate merge-prep pass to run code review, and user debugging to discover the skills version issue. The full quality cycle was never completed autonomously — the user had to drive each gate manually.

### Branch B — 2 turns, single autonomous run

| Turn | User prompt | What happened |
|---|---|---|
| 0 | `/solve-adr solve milestone 3` | Complete solve cycle in one turn: 2 ADRs authored (0009, 0010), plan generated, plan reviewed, QA plan created, all tasks executed, QA validated, code review dispatched (both custom reviewers), re-review accepted. 13 files touched. |
| 1 | "do a retrospective to procedure adherence" | Retrospective saved to session files. Deviations limited to QA batching, implicit START check, QA plan not updated after triage. |

**Observations:** The entire pipeline — scale check, authoring, review, editor dispatch, planning, QA, implementation, code review, re-review — ran to completion in a single uninterrupted turn. The user's only interaction was requesting the retrospective after the fact.

### Branch C — 1 turn, ~5.5 minutes

| Turn | Prompt | What happened |
|---|---|---|
| 0 | "implement milestone 3 from the ROADMAP.md file" | Read ROADMAP.md and existing codebase, implemented CLI with 3 commands, added rendering module, created toy fixture, wrote integration tests. 3 commits, 5 files created/modified. |

**Observations:** The agent read the roadmap, surveyed the codebase, and went straight to implementation. No decisions were documented. No plan was created. No review or QA occurred. The agent self-verified by running `cargo test` (158 pass) and marked the task complete. Total elapsed time was ~5.5 minutes — significantly faster than either A or B.

### Branch D — 2 phases, ~16 minutes total

**Phase 1** (same bare prompt as C):

| Phase | Prompt | What happened |
|---|---|---|
| 1 | "implement milestone 3 from the ROADMAP.md file" | Same as C: read roadmap, implemented CLI with 3 commands, added rendering module, created toy fixture, wrote integration tests. 4 commits, ~6 minutes. |
| 2 | Review + fix pass with code reviewer stack + adversarial QA | Both custom code reviewers dispatched. Adversarial QA checks run. 10 issues found and fixed in 1 commit. ~10 minutes. |

**Phase 2 findings and fixes:**

| # | Severity | Issue found | Fix applied |
|---|---|---|---|
| 1 | Critical | `build` rendered Markdown from unvalidated skills | Added validation gate — exits non-zero on failures |
| 2 | High | `init` silently overwrote existing skills | Added existence check with clear error |
| 3 | High | Error messages lacked skill name/path context | All errors now include name and path |
| 4 | Medium | Validation logic duplicated between `check` and `build` | Extracted shared `run_validations()` helper |
| 5 | Medium | `&PathBuf` params instead of `&Path` | Changed to `&Path` (Clippy lint) |
| 6 | Medium | Duplicated slug extraction helpers | Added `ItemId::slug()` method |
| 7 | Medium | Build write-error omitted output path | Error now includes path |
| 8 | Low | No README | Added README.md |
| 9 | Low | No negative test fixture | Added `examples/invalid-skill/` |
| 10 | Low | Insufficient integration test coverage | Added 5 new integration tests |

**Observations:** The review + fix pass caught and fixed the critical bugs (build skips validation, init overwrites). It also added a README, a negative fixture, and 5 additional tests. However, Phase 1 output was nearly identical to Branch C — the bare prompt produced the same gaps. The review pass recovered some ground but the final test count (17) is still below Branch B (23).

---

## Quality Gate Comparison

Each row represents a gate in the solve-adr → author-adr → implement-adr pipeline. Branch C has no workflow, so these gates do not exist for it — that absence is the point.

| Gate | Branch A | Branch B | Branch C |
|---|---|---|---|
| **Scale check** | 3 ADRs authored — including ADR-0011 for test fixture selection | 2 ADRs authored — scale check's ASR test excluded toy skill fixture as non-architectural | N/A — no ADRs |
| **Author-adr review** | ✅ Dispatched `general-purpose` reviewer | ✅ Dispatched `general-purpose` reviewer | N/A |
| **Editor dispatch** | ❌ Skipped — agent applied 2 wording fixes directly instead of dispatching `juliusl-editor-v4` | ✅ Dispatched `juliusl-editor-v4` per config | N/A |
| **Plan generation** | ✅ `0009-0011.0.plan.md` created | ✅ `0009-0010.0.plan.md` created | ❌ No plan |
| **Plan review** | ✅ Reviewer found 2 gaps, both fixed | ✅ Reviewer found 4 findings, all addressed | N/A |
| **Plan re-review** | ❌ Skipped after revision | ✅ Completed | N/A |
| **QA plan** | ✅ Created by separate agent | ✅ Created by separate agent | ❌ No QA plan |
| **QA execution** | ⚠️ Ran, but 76 checkboxes left unchecked — audit trail incomplete | ⚠️ Ran and found a real bug; checkboxes not updated after triage | ❌ No QA |
| **Code review dispatch** | ❌ **Never ran** — custom reviewers not available (old skills installed). Self-scored 0/10. | ✅ Both `juliusl-code-reviewer-analytics-v5` and `juliusl-code-reviewer-sweep-v5` dispatched | ❌ No code review |
| **Code review re-review** | ❌ N/A (no initial review) | ✅ Both reviewers accepted on second pass | N/A |
| **Bug detection** | No bugs found or reported | QA executor caught N² error-count bug — fixed in commit `0f0f5a9` | No bugs found (simpler approach avoided the bug) |
| **Refactoring** | Validation logic duplicated across `check` and `build` commands | Analytics reviewer caught duplication → `run_validators()` extracted in commit `727dc5a` | Validation only in `check` — `build` skips validation entirely |

### Gate activation rate

- **Branch A:** 5 of 11 gates fully activated (45%)
- **Branch B:** 10 of 11 gates fully activated (91%)
- **Branch C:** 0 of 11 gates — no workflow pipeline exists

The one gap shared by A and B: QA plan checkboxes not marked after execution.

---

## Retrospective Findings

### Branch A self-retroactive (turn 2)

The agent self-scored **78/100** with these breakdowns:

| Category | Score | Key gap |
|---|---|---|
| ADR authoring | 9/10 | Editor dispatch skipped |
| ADR review | 8/10 | Editor not dispatched for polish |
| Plan generation | 10/10 | — |
| Plan review | 8/10 | Missing re-review cycle |
| QA plan | 7/10 | 76 checkboxes unchecked, Stage 5 never QA'd |
| Code review dispatch | **0/10** | Configured reviewers never ran |
| Implementation | 10/10 | All tasks complete, tests pass |
| Roadmap protocol | 10/10 | — |

The most significant finding: **code review scored 0/10**. The configured custom reviewers were never invoked. The agent didn't detect this gap — the user discovered it when investigating why the reviewers weren't found as agent types (Turn 4: stale skills installation).

### Branch B retrospective (session file)

The retrospective documented three deviations, all low severity:

1. **START check not explicitly logged** — readiness criteria checked implicitly via autonomous low-uncertainty merge, but not formally logged.
2. **QA batched across all stages** — procedure recommends per-stage QA boundaries; agent ran a single pass. The QA executor still caught a real bug, so the safeguard functioned despite batching.
3. **QA plan not updated after triage fixes** — cosmetic gap in the audit trail.

**What worked as designed:**
- All configured dispatch agents were used correctly (reviewer, editor, both code reviewers)
- Scale check prevented ADR bloat (toy skill excluded via ASR)
- QA caught the N² error-count bug
- Code review caught logic duplication → refactored
- Re-review confirmed all fixes — both reviewers accepted

---

## Engineering Output Comparison

| Metric | Branch A (main skills) | Branch B (atelier skills) | Branch C (zero workflow) | Branch D (review-only) |
|---|---|---|---|---|
| ADRs authored | 3 (0009, 0010, 0011) | 2 (0009, 0010) | — | — |
| Plan / QA plan | Yes / Yes | Yes / Yes | — | — |
| Commits | 9 | 13 | 3 | 5 (4 impl + 1 fix) |
| main.rs LOC | 184 | 170 | 164 | 212 |
| Renderer LOC | 327 (render.rs) | 505 (render.rs) | 363 (build.rs) | 398 (build.rs) |
| Unit tests (renderer) | 11 | 12 | 6 | 8 |
| Integration tests | 11 | 11 | 4 | 9 |
| Total M3-specific tests | 22 | 23 | 10 | 17 |
| Test fixtures | 1 valid (2 procedures) | 1 valid + 1 invalid | 1 valid (in examples/) | 1 valid + 1 invalid (in examples/) |
| Negative test fixtures | No | Yes | No | Yes (added in Phase 2) |
| Shared validation helper | No (duplicated in check + build) | Yes (`run_validators()`) | No (validation only in check; build skips it) | Yes (`run_validations()`, added in Phase 2) |
| Build validates before rendering | Yes | Yes | No | Yes (added in Phase 2) |
| README | No (docs/cli.md) | Yes (top-level) | No | Yes (added in Phase 2) |
| Commit granularity | Moderate (9 commits, mixed concerns) | Fine (13 commits, single concern each) | Coarse (3 commits) | Coarse (4 impl + 1 fix) |
| Known issues at completion | Validation duplication, error count bug | None — all caught and fixed | Build command doesn't validate input | None — all caught and fixed in Phase 2 |
| Time | Full session | Full session | ~5.5 minutes | ~16 minutes (6 impl + 10 review) |

### What the numbers show

**Branch C** is the fastest and most compact. It produced working code in 3 commits and ~5.5 minutes. But it has half the test coverage of A or B, no negative fixtures, no documentation, and a structural gap: the `build` command renders Markdown from a skill without running any validation first — it will silently produce output from broken skills.

**Branch D** is Branch C with a review + fix pass. The review caught and fixed the critical bugs (build validation gate, init overwrite guard), added a README, added a negative fixture, and added 5 integration tests. Total time: ~16 minutes. However, the final test count (17) is still below B's (23), and the commit history is coarser. The review *subtracted defects* but didn't fully close the *coverage gap*.

**Branch A** has more infrastructure (3 ADRs, plan, QA plan) but left quality gaps in the code: duplicated validation logic and an error-count bug that was never flagged because code review never ran.

**Branch B** has the most test coverage, the cleanest code (shared validation helper, bug fixed, negative fixtures), and the finest commit granularity. It achieved this while authoring fewer ADRs than Branch A — the scale check prevented documentation bloat without reducing engineering rigor.

---

## Post-hoc Review: Branch C

Branch C had no built-in quality gates, so the same reviewer stack used by Branch B's workflow was applied retroactively to evaluate what zero-workflow output looks like under structured review.

### Code Review — Sweep (`juliusl-code-reviewer-sweep-v5`)

All five mechanical checks passed: doc headers present, no spelling errors, no inverted naming, no identifier collisions. **Zero findings.**

### Code Review — Analytics (`juliusl-code-reviewer-analytics-v5`)

**Verdict: Accept with feedback.** No high-priority issues. Findings:

| # | Priority | Finding |
|---|---|---|
| M1 | Medium | `check` failure output lacks skill name/path context — scripted multi-directory runs can't tell which skill failed |
| N1 | Nit | `&PathBuf` → `&Path` in all three `cmd_*` function signatures (Clippy lint) |
| N2 | Nit | Inconsistent path types within `cmd_build` — `path` is `&PathBuf` but `output` is `Option<&Path>` |

### QA Bug Hunt (adversarial general-purpose agent)

The QA agent attempted ~10 adversarial inputs against the CLI and source code. **8 bugs found:**

| # | Bug | Severity | Description |
|---|---|---|---|
| 1 | `build` renders broken skills without validation | **High** | `build` never calls any validator — it renders Markdown from skills with broken refs, wrong prefixes, and policy conflicts, exiting 0 |
| 2 | Criterion reference existence not checked | **High** | `check` validates the `criterion:` prefix but not that the referenced criterion is actually defined in the skill |
| 3 | Multiline TOML strings inject Markdown structure | Medium | A `name` containing `## Injected Heading` renders as a real heading, corrupting document structure |
| 4 | Empty `name` and `description` pass validation | Medium | `check` returns `ok —  (skill:...)` for skills with empty required fields |
| 5 | `init` silently deletes existing procedure files | Medium | `SkillWriter::write` cleans stale `.toml` files before writing — `init` into an existing directory deletes user files |
| 6 | Empty task `subject`/`action` accepted | Medium | `check` passes; `build` renders `- **task:t1**:  — ` (dangling separator) |
| 7 | `compatible_with` dangling references not validated | Medium | Policies can reference `compatible_with = ["policy:ghost"]` where `policy:ghost` doesn't exist |
| 8 | `init` uses slug as human-readable name | Low | `init "my-skill"` generates `name = "my-skill"` instead of a human-readable display name |

**Bug 1** is the same structural gap identified in the engineering output comparison — `build` skips validation entirely. Branch B's `run_validators()` helper runs all validators before both `check` and `build`, preventing this class of bug.

**Bugs 2, 7** are pre-existing gaps in the M1/M2 validator modules (`validate.rs`, `conflict.rs`), not introduced by M3. However, M3's `check` command is the first user-facing consumer of these validators — a QA pass during implementation would have surfaced these gaps.

**Bugs 3, 4, 6** are input validation gaps that negative test fixtures would catch. Branch B's `tests/fixtures/invalid/` with broken TOML references is exactly the kind of fixture that surfaces these issues.

---

## Reliability Analysis

The four-way comparison reveals a clear gradient of workflow reliability.

### 1. Defect density by workflow level

| | Branch A (main skills) | Branch B (atelier skills) | Branch C (zero workflow) | Branch D (review-only) |
|---|---|---|---|---|
| Quality gates available | 11 | 11 | — | — |
| Gates activated | 5 (45%) | 10 (91%) | — | — |
| Code review findings | N/A (never ran) | Duplication caught + fixed | 1 medium, 2 nits (retroactive) | 10 issues found + fixed (Phase 2) |
| QA bugs found | Not assessed | 0 (N² bug caught during session) | 2 high, 4 medium, 2 low (retroactive) | 0 (all fixed in Phase 2) |
| Known issues at completion | Validation duplication, error count bug | None | 8 bugs (2 high) | None |

Branch D demonstrates that review *can* catch the critical bugs — it found and fixed the same build-validation and init-overwrite issues that C shipped with. However, the fix pass required a second agent session with pointed questions, and the coverage gap (17 tests vs. B's 23) was only partially closed.

### 2. Gate activation reliability

Branch B activated 91% of quality gates versus Branch A's 45%. Branches C and D have no workflow gates — the comparison is between B's proactive pipeline and D's reactive review.

The atelier branch's skill refinements made dispatch rules explicit and quality gates harder to skip. When the instructions say "dispatch the configured editor agent," the treatment build dispatched it. When the instructions say "run re-review after revision," the treatment build ran it.

### 3. Self-correction capability

Branch B demonstrated a complete self-correction loop:
1. Implementation introduced an error-count bug (N² accumulation inside a loop)
2. QA execution caught the bug during validation
3. Bug was fixed in a dedicated commit (`0f0f5a9`)
4. Code review independently caught validation duplication
5. Duplication was refactored in a separate commit (`727dc5a`)
6. Re-review confirmed both fixes

Branch D demonstrated a partial self-correction loop: the review found 10 issues and fixed them, but in a single batch commit rather than isolated fixes. There was no re-review to verify the fixes didn't introduce regressions.

Branch A never reached this loop. Branch C never had the opportunity.

### 4. Decision calibration

| | Branch A | Branch B | Branch C | Branch D |
|---|---|---|---|---|
| ADRs | 3 | 2 | — | — |
| Plans | 1 plan + 1 QA plan | 1 plan + 1 QA plan | — | — |
| Decision documentation | Over-scoped (test fixture got its own ADR) | Calibrated (ASR excluded non-architectural decisions) | — | — |

### 5. What review-only buys (Branch D vs. C)

Branch D tested the steelman hypothesis: can a post-hoc review + fix pass close the gap between zero-workflow output and full-pipeline output?

**What the review recovered:**
- ✅ Critical bugs fixed (build validation gate, init overwrite guard)
- ✅ README added
- ✅ Negative fixture added
- ✅ Shared validation helper extracted
- ✅ Clippy lints resolved

**What the review did not recover:**
- ❌ Test coverage gap: 17 tests vs. B's 23 (6 fewer)
- ❌ Commit granularity: 4+1 coarse commits vs. B's 13 fine-grained commits
- ❌ No decision documentation — no trace of why the CLI was designed this way
- ❌ No re-review to verify the fix pass itself

The review pass moved D from C's defect profile (8 bugs) to zero known bugs — a real improvement. But it did not produce the *coverage depth* or *commit discipline* that B's proactive pipeline generated. Review subtracts defects. It does not add the artifacts (tests, docs, decision records) that prevent future defects.

Even this result is generous to D. The Phase 2 review prompt included specific adversarial checks derived from B and C's findings — "does build validate before rendering?", "are there negative fixtures?", "does init guard against overwriting?" A truly blind review would not have asked these pointed questions. D's Phase 1 prompt was fair (same as C), but its Phase 2 prompt carried forward knowledge from the other experiments. D's results are an upper bound on what review-only actually buys — and it still fell short of B's coverage.

### 6. The Pareto trap

The steelman's original argument was a Pareto argument: 80% of the quality at 20% of the cost. But applying the Pareto principle here is confirmation bias — it assumes the remaining 20% is the unimportant part. In practice, the missing 20% is where mission-critical gaps live: the build command that silently renders broken skills, the init that overwrites user files, the 6 missing tests that would catch the next regression.

More fundamentally, the steelman's "lightweight alternative" prompt was itself parasitic on the heavyweight process. Every specific requirement in the original (invalidated) D-round prompt — overwrite guards, validation-before-build, negative fixtures — was knowledge *earned* by the pipeline running on earlier branches. You cannot skip the process that generates the lessons and still have the lessons. A lightweight approach that encodes heavyweight findings is just the heavyweight approach with the receipts removed.

---

## Conclusion

The four-way comparison demonstrates a clear relationship between workflow structure and output reliability:

| Condition | Bugs at completion | Tests | Review pass | Time |
|---|---|---|---|---|
| Zero workflow (C) | 8 (2 high) | 10 | None | ~5.5 min |
| Review-only (D) | 0 (fixed in Phase 2) | 17 | Post-hoc fix pass | ~16 min |
| Main skills (A) | 2+ (never fully assessed) | 22 | — | Full session |
| Atelier skills (B) | 0 | 23 | Integrated in pipeline | Full session |

**Branch C** is the speed baseline — fast, functional, but brittle.

**Branch D** proves the steelman's hypothesis *partially*: review catches bugs. But it also proves the limits: review recovers correctness (0 bugs) without recovering completeness (6 fewer tests, no decision trace, coarser commits). The review pass is reactive — it fixes what it finds, but it cannot generate the proactive artifacts (QA plans, test targets, negative fixtures) that the pipeline produces before implementation begins.

**Branch A** added workflow structure but activated less than half of its quality gates. Code review never ran (stale skills), the editor was skipped, and re-review was omitted.

**Branch B** activated 91% of its quality gates in a single autonomous turn. It produced the cleanest code with the most test coverage, the finest commit granularity, and zero bugs — while authoring fewer ADRs than Branch A.

The agent is the same across all four runs. The difference is the instructions. Better workflow instructions produced better engineering judgment — not by making the agent smarter, but by making the quality pipeline reliable.

## Architect Verdict

A senior software architect persona was given the case study data and asked to choose an approach for production code developed by autonomous agents.

**Choice: Branch B.**

**Eliminations:**

- **C** — disqualified immediately. 8 bugs including a build command that silently renders broken input. "Speed doesn't matter if you're shipping landmines."
- **A** — disqualified. 45% gate activation means the quality pipeline is a coin flip. Code review scored 0/10 and the agent didn't notice — the user had to debug it.
- **D** — tempting but insufficient for three reasons: the review prompt was tainted with knowledge from other branches (upper bound, not representative), it recovered correctness without completeness (17 tests vs. 23), and it doesn't compose across a team ("you'd need someone writing bespoke adversarial prompts every time — that's just manual QA with extra steps").

**Why B:** The only option where the quality system ran autonomously, caught real bugs, fixed them, and verified the fixes — without human intervention. The cost comparison isn't B vs. C. It's B vs. C-plus-the-incident-response when silent failures reach production.

---

## Session References

| Item | Reference |
|---|---|
| Branch A session | `b536efb8-e511-4744-9e0f-5cf96c030a7a` |
| Branch B session | `aa0db0f3-28a6-4ffc-96e5-52a4690efd5a` |
| Branch C repo | `/Users/juliusl/gh/skilleton-c-round` |
| Branch D repo | `/Users/juliusl/gh/skilleton-d-round-fair` |
| Comparison session | `68afa4f4-1d0d-4be0-8862-5115128e93fe` |
| Comparison report | `~/.copilot/session-state/68afa4f4-1d0d-4be0-8862-5115128e93fe/files/ab-comparison.md` |
| Branch A retroactive | Session `b536efb8`, turn 2 |
| Branch B retrospective | Session `aa0db0f3`, file `retrospective.md` |
| Branch C code reviews | Sweep: clean, Analytics: accept with feedback (1 medium, 2 nits) |
| Branch C QA bug hunt | 8 bugs found (2 high, 4 medium, 2 low) |
| Branch D Phase 2 | 10 issues found and fixed (1 critical, 2 high, 4 medium, 3 low) |
| Steelman debate | Session `e1bfc62a`, file `steelman-review.md` |
