# 21. Append implementation summary to plan after execution

Date: 2026-04-03
Status: Planned
Last Updated: 2026-04-03
Links:

## Context

The `implement-adr` skill executes plans that decompose ADR decisions into staged tasks. When auto-commit is enabled (per ADR-0010), each task produces a git commit. However, once the session ends, there is no persistent record linking the ADR to the specific commits and task outcomes that implemented it. The plan file tracks acceptance criteria (per ADR-0009), but this is a separate document from the ADR itself.

**The problem:** There is no observable trace connecting an ADR's implementation plan to the specific commits and task outcomes produced during autonomous execution.

**Why this matters:**
- **Traceability** — given a plan, a developer should be able to see which commits implemented each task without searching the git log manually.
- **Quality analysis** — structured implementation data (task cost, commit hash, status) can be extracted and analyzed to improve planning accuracy over time. This aligns with the project-scoped state vision from ADR-0020.
- **Parity with author-adr** — the `author-adr` skill appends a revision dialogue to ADRs after review/revise cycles (per ADR-0016). The `implement-adr` skill should follow a similar pattern of appending structured post-execution data to the plan file.

**Current state:**
- The plan file (`docs/plans/*.plan.md`) contains a Summary table and acceptance criteria checkboxes, but no record of which commits were produced.
- The alternative of storing the entire agent session is not observable — session data is ephemeral and not accessible to downstream tooling.
- In manual-commit mode, the user has direct oversight and receives a summary in the session — the gap is specifically in autonomous (auto-commit) execution where the agent acts unsupervised.

### Decision Drivers

- **Observable** — implementation trace must be readable by humans and parseable by tools
- **Awk-friendly** — the summary format should be easily processable by `awk` and similar coreutils, benefiting any post-processing tooling
- **Auto-commit only** — the summary is only meaningful when commits are being created; in manual-commit mode, there are no commit hashes to record
- **Append pattern** — follow the established pattern of appending structured data to execution artifacts (similar to how author-adr appends revision Q&A to ADRs per ADR-0016)
- **Extractable as JSONL** — a script should be able to read a plan and/or ADR and produce structured JSONL output to stdout for downstream analysis

## Options

### Option 1: Append awk-friendly summary to plan file + awk extraction script

After plan execution completes (during the Finalize stage), append an implementation summary block to the plan file (`docs/plans/*.plan.md`). The block uses a pipe-delimited format that `awk` can trivially parse. A companion `awk` script reads this summary and emits each task as a line of JSONL to stdout.

**Strengths:**
- `awk` is a POSIX coreutil — zero dependencies, available everywhere
- Pipe-delimited records are both human-readable and machine-parseable
- Appending to the plan file keeps the execution artifact self-contained: plan + acceptance criteria + commit trace in one file
- JSONL output to stdout is composable — pipe to `jq`, append to `.adr/var/`, or redirect anywhere

**Weaknesses:**
- Plan files grow slightly (~10-20 lines for typical plans)
- Awk-friendly formatting constrains the summary layout
- Only captures commit hashes — not diffs, test results, or other artifacts

### Option 2: Store implementation log as separate JSONL in `.adr/var/`

Write implementation events directly to `.adr/var/implementations.jsonl` during execution, bypassing the plan file entirely.

**Strengths:**
- No plan file modification
- Can capture richer event data (timestamps, diffs, test output)
- Already aligned with ADR-0020's `.adr/var/` convention

**Weaknesses:**
- Not observable from the plan itself — requires knowing to look in `.adr/var/`
- Loses the self-contained traceability of having the trace in the plan
- Disconnected from the execution artifact

### Option 3: Embed commit hashes in the plan file's existing Summary table

Extend the existing plan Summary table with a `Commit` column, populated as tasks complete.

**Strengths:**
- No separate block needed — extends the existing table
- Plan is already the execution artifact — adding commits there is natural
- Extraction can still use awk on the plan file

**Weaknesses:**
- Markdown table columns are harder to parse with awk than pipe-delimited lines
- Mixing execution data into the planning table blurs the distinction between plan and trace
- Adding columns to an existing table format is a breaking change for any tooling that parses it

## Decision

In the context of **needing observable traceability between implementation plans and the commits produced during autonomous execution**, facing **the absence of any persistent link between plan tasks and code changes**, we decided for **appending an awk-friendly implementation summary to the plan file, with a companion `awk` script that extracts task state as JSONL to stdout**, and neglected **storing in `.adr/var/` only (not observable from the plan) and extending the existing Summary table (breaks tooling, blurs plan vs. trace)**, to achieve **self-contained plan traceability and toolchain-friendly extraction**, accepting that **plan files grow slightly and the format is constrained by awk-parseability**.

### Summary Format

The implementation summary is appended at the end of the plan file (`docs/plans/*.plan.md`). It uses a pipe-delimited, awk-friendly format:

```markdown
<!-- Implementation summary generated by implement-adr -->

<!-- BEGIN implementation-summary -->
# task_id | status | cost | commit | description
1.1 | done | small | abc1234 | Add init-data command to nygard-agent-format.sh
1.2 | done | small | def5678 | Add init-data Makefile target
1.3 | done | small | 9ab0123 | Update resolve_dir() to check .adr/adr-dir
2.1 | done | small | 456cdef | Update tooling.md reference
2.2 | done | small | 789abcd | Update author-adr SKILL.md
3.1 | done | small | bcd3456 | Update ADR status to Accepted
<!-- END implementation-summary -->
```

### Format Conventions

1. **Delimited by HTML comments** — `<!-- BEGIN implementation-summary -->` and `<!-- END implementation-summary -->` mark the extractable block.
2. **Header line** — starts with `#` (comment convention for awk), declares field names.
3. **Pipe-delimited fields** — `task_id | status | cost | commit | description`. Pipes with surrounding spaces for readability.
4. **Commit field** — short SHA (7 chars) from the auto-commit. Empty if no commit was made for that task.
5. **Only with auto-commit** — the summary is only appended when auto-commit mode is active. In manual-commit mode, the user has direct oversight and receives a summary in the session; the value of this persistent trace is in tracking unsupervised agent behavior.

### Revisit Trigger

Revisit after 3 implementations with auto-commit to validate that the awk format parses correctly against real summaries, plan file growth is acceptable, and the summary provides value beyond the plan's existing acceptance criteria checkboxes.

### Extraction Script

An `awk` script (`extract-summary.awk` or equivalent) reads the plan file and emits each task as a JSONL line to stdout:

```bash
awk -f extract-summary.awk docs/plans/0020.0.plan.md
```

Output (one JSON object per line):
```json
{"task_id":"1.1","status":"done","cost":"small","commit":"abc1234","description":"Add init-data command"}
```

The script does not decide where the output is written — it emits to stdout. Callers can pipe to `.adr/var/`, `jq`, or any other destination.

## Consequences

**Positive:**
- Plan files become self-contained execution artifacts: task decomposition + acceptance criteria + commit trace in one file.
- The awk-friendly format enables post-processing with POSIX coreutils — no external dependencies or runtime beyond `awk`.
- Following the append-to-artifact pattern keeps the approach consistent with how `author-adr` appends revision Q&A to ADRs (per ADR-0016).
- JSONL extraction to stdout is maximally composable — the script is a building block, not an end-to-end solution.

**Negative:**
- Plan files grow by approximately 10-20 lines for typical plans (5-15 tasks). For unusually large plans, growth could be proportionally larger.
- The pipe-delimited format is less flexible than JSONL natively — adding fields later requires updating the awk script.
- Only captures commit hashes, not richer implementation metadata (test results, timings).
- The summary is only produced during autonomous execution with auto-commit. Manual-commit workflows imply user oversight and do not generate a persistent trace — this is by design, as the value of the summary is in tracking unsupervised agent behavior (plan adherence, derailment detection).

**Neutral:**
- The extraction script outputs to stdout. Where this data is persisted (`.adr/var/`, a dashboard, etc.) is a downstream concern.
- This ADR does not prescribe changes to the plan file's existing Summary table — the new block is appended separately, not merged into existing structure.

## Quality Strategy

- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [x] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] User documentation

### Additional Quality Concerns

The awk script requires diff-based tests (input ADR → expected JSONL output). The summary format must be validated against real plan executions to ensure awk parseability. Existing ADRs without implementation summaries should be unaffected — the extraction script should produce no output for ADRs with no `<!-- BEGIN implementation-summary -->` block.

---

## Comments

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Should the ADR include a revisit trigger?

**Addressed** — Added a revisit trigger: revisit after 3 implementations with auto-commit to validate awk parseability, plan growth, and summary value.

### Q: Is the manual-commit blind spot documented as a consequence?

**Addressed** — Added as a negative consequence with refined framing: manual-commit implies user oversight; the summary tracks unsupervised agent behavior (plan adherence, derailment detection).

### Q: Is coexistence with revision Q&A entries addressed?

**Addressed** — Corrected a fundamental authorship error: the summary appends to the **plan file**, not the ADR. Renamed the ADR, rewrote options, decision, and consequences to reflect this. Coexistence with revision Q&A is no longer relevant.

### Q: Is the "no custom parsers" claim accurate?

**Addressed** — Reworded to "no external dependencies or runtime beyond `awk`."

### Q: Is the ~10-20 lines estimate grounded?

**Addressed** — Qualified as "approximately 10-20 lines for typical plans (5-15 tasks)."

### Q: Should a commit-less summary be considered for manual-commit mode?

**Rejected** — In manual-commit mode, the user has direct oversight and receives a summary in the session. The persistent trace is specifically for tracking unsupervised agent behavior — today this would require manually searching git log, so the summary puts it in one place.
