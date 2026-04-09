# Skilleton Case Study: Workflow Reliability

## Purpose

This case study compares three runs of the same implementation task — each with a different level of workflow structure — to measure how procedural agent skills affect engineering output quality and reliability.

## Setup

**Test subject:** [skilleton](https://github.com/juliusl/skilleton) — a Rust CLI for validating and rendering agentskills.io skill definitions.

**Task:** Milestone 3 — implement `init`, `check`, and `build` commands with TOML parsing, a Markdown rendering pipeline, and integration tests.

**Conditions:**

| | Branch A (main skills) | Branch B (atelier skills) | Branch C (zero workflow) |
|---|---|---|---|
| Skills source | `main` branch of adr-skills | `solve/adr-atelier/milestone-1` branch | None — general-purpose agent only |
| Working branch | `solve/skilleton/milestone-3` | `solve/skilleton/milestone-3-1` | `solve/skilleton/milestone-3-c` |
| Session | `b536efb8` | `aa0db0f3` | Sub-agent task (~5.5 min) |
| Merge base | `f2d7c87` | `f2d7c87` | `f2d7c87` |
| Prompt | `/solve-adr solve milestone 3` | `/solve-adr solve milestone 3` | "implement milestone 3 from the ROADMAP.md file" |
| Participation | Autonomous, auto-commit | Autonomous, auto-commit | Autonomous, auto-commit |

All three branches started from the same codebase state (`f2d7c87` — milestones 1 and 2 complete).

- **Branch A** and **Branch B** used the `/solve-adr` skill to orchestrate the full pipeline (author-adr → implement-adr → code review).
- **Branch C** received a bare prompt with no workflow skills — just "implement milestone 3."

**Known confound:** During Branch A's session the user discovered previously installed (older) skill files were present. Branch A may have run with a stale skill version rather than current `main`.

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

| Metric | Branch A (main skills) | Branch B (atelier skills) | Branch C (zero workflow) |
|---|---|---|---|
| ADRs authored | 3 (0009, 0010, 0011) | 2 (0009, 0010) | — |
| Plan / QA plan | Yes / Yes | Yes / Yes | — |
| Commits | 9 | 13 | 3 |
| main.rs LOC | 184 | 170 | 164 |
| Renderer LOC | 327 (render.rs) | 505 (render.rs) | 363 (build.rs) |
| Unit tests (renderer) | 11 | 12 | 6 |
| Integration tests | 11 | 11 | 4 |
| Total M3-specific tests | 22 | 23 | 10 |
| Test fixtures | 1 valid (2 procedures) | 1 valid + 1 invalid | 1 valid (in examples/) |
| Negative test fixtures | No | Yes | No |
| Shared validation helper | No (duplicated in check + build) | Yes (`run_validators()`) | No (validation only in check; build skips it) |
| Build validates before rendering | Yes | Yes | No |
| README | No (docs/cli.md) | Yes (top-level) | No |
| Commit granularity | Moderate (9 commits, mixed concerns) | Fine (13 commits, single concern each) | Coarse (3 commits) |
| Known issues at completion | Validation duplication, error count bug | None — all caught and fixed | Build command doesn't validate input |
| Time | Full session | Full session | ~5.5 minutes |

### What the numbers show

**Branch C** is the fastest and most compact. It produced working code in 3 commits and ~5.5 minutes. But it has half the test coverage of A or B, no negative fixtures, no documentation, and a structural gap: the `build` command renders Markdown from a skill without running any validation first — it will silently produce output from broken skills.

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

The three-way comparison reveals a clear gradient of workflow reliability.

### 1. Defect density by workflow level

| | Branch A (main skills) | Branch B (atelier skills) | Branch C (zero workflow) |
|---|---|---|---|
| Quality gates available | 11 | 11 | — |
| Gates activated | 5 (45%) | 10 (91%) | — |
| Code review findings | N/A (never ran) | Duplication caught + fixed | 1 medium, 2 nits (retroactive) |
| QA bugs found | Not assessed | 0 (N² bug caught during session) | 2 high, 4 medium, 2 low (retroactive) |
| Known issues at completion | Validation duplication, error count bug | None | 8 bugs (2 high) |

Branch C shipped with 2 high-severity bugs that would have been caught by either QA execution (Bug 1: build skips validation) or negative test fixtures (Bug 2: dangling criterion refs). Branch B caught and fixed equivalent issues during its session because the quality pipeline was active.

### 2. Gate activation reliability

Branch B activated 91% of quality gates versus Branch A's 45% and Branch C's 0%. The gates that Branch A and C missed — code review, QA, negative fixtures — are the gates that would have caught their respective bugs.

The atelier branch's skill refinements made dispatch rules explicit and quality gates harder to skip. When the instructions say "dispatch the configured editor agent," the treatment build dispatched it. When the instructions say "run re-review after revision," the treatment build ran it. Branch C had no such instructions and no such gates.

### 3. Self-correction capability

Branch B demonstrated a complete self-correction loop:
1. Implementation introduced an error-count bug (N² accumulation inside a loop)
2. QA execution caught the bug during validation
3. Bug was fixed in a dedicated commit (`0f0f5a9`)
4. Code review independently caught validation duplication
5. Duplication was refactored in a separate commit (`727dc5a`)
6. Re-review confirmed both fixes

Branch A never reached this loop. Branch C never had the opportunity — it went straight from implementation to "done." The 8 bugs found retroactively demonstrate what happens when the self-correction loop doesn't exist.

### 4. Decision calibration

| | Branch A | Branch B | Branch C |
|---|---|---|---|
| ADRs | 3 | 2 | — |
| Plans | 1 plan + 1 QA plan | 1 plan + 1 QA plan | — |
| Decision documentation | Over-scoped (test fixture got its own ADR) | Calibrated (ASR excluded non-architectural decisions) | — |

Branch C demonstrates the opposite extreme from Branch A: zero documentation is faster but leaves no trace of design intent. Branch B sits in the middle — sufficient documentation with structural exclusion of non-architectural decisions.

### 5. Speed vs. reliability tradeoff

Branch C completed in ~5.5 minutes. Branches A and B took full sessions. But Branch C shipped with 2 high-severity bugs that the retroactive review caught immediately. The time saved is real, but so are the bugs.

This is not an argument that every task needs the full solve-adr pipeline. It is evidence that when the pipeline runs, it catches real defects — and when it doesn't, those defects ship.

---

## Conclusion

The three-way comparison demonstrates a clear relationship between workflow structure and output reliability:

| Condition | Gate activation | Bugs at completion | Turns needed |
|---|---|---|---|
| Zero workflow (C) | — | 8 (2 high) | 1 |
| Main skills (A) | 45% | 2+ (never fully assessed) | 5 |
| Atelier skills (B) | 91% | 0 | 1 |

**Branch C** is the speed baseline — fast, functional, but brittle. The retroactive review found 8 bugs including 2 high-severity issues. No decisions were documented, no plan was created, no review occurred. The agent went straight from reading the roadmap to writing code.

**Branch A** added workflow structure but activated less than half of its quality gates. Code review never ran (stale skills), the editor was skipped, and re-review was omitted. The resulting code has duplicated validation logic and an error-count bug that was never flagged.

**Branch B** activated 91% of its quality gates in a single autonomous turn. Its QA caught a bug, its code review caught duplication, its re-review verified the fixes, and its scale check prevented documentation bloat. It produced the cleanest code with the most test coverage.

The agent is the same across all three runs. The difference is the instructions. Better workflow instructions produced better engineering judgment — not by making the agent smarter, but by making the quality pipeline reliable.

---

## Session References

| Item | Reference |
|---|---|
| Branch A session | `b536efb8-e511-4744-9e0f-5cf96c030a7a` |
| Branch B session | `aa0db0f3-28a6-4ffc-96e5-52a4690efd5a` |
| Branch C repo | `/Users/juliusl/gh/skilleton-c-round` |
| Comparison session | `68afa4f4-1d0d-4be0-8862-5115128e93fe` |
| Comparison report | `~/.copilot/session-state/68afa4f4-1d0d-4be0-8862-5115128e93fe/files/ab-comparison.md` |
| Branch A retroactive | Session `b536efb8`, turn 2 |
| Branch B retrospective | Session `aa0db0f3`, file `retrospective.md` |
| Branch C code reviews | Sweep: clean, Analytics: accept with feedback (1 medium, 2 nits) |
| Branch C QA bug hunt | 8 bugs found (2 high, 4 medium, 2 low) |
