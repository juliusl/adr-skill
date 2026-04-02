# 10. Implement-adr optionally commits on task completion

Date: 2026-04-02

## Status

Accepted

## Context

[ADR-0009](0009-implement-adr-uses-incremental-progress-tracking-during-plan-execution.md)
introduces incremental progress tracking: agents update `- [ ]` → `- [x]` in
the plan file as acceptance criteria are satisfied. This creates a live progress
record, but the record exists only in the working tree until the user manually
commits.

When a task's criteria are all satisfied, there is no mechanism to create a
**git commit** capturing the completed work. Without task-boundary commits:

- **No reviewable history** — pull request reviewers see a single large diff
  rather than incremental task-scoped changes traceable to plan decisions.
- **No interruption resilience** — if a session ends mid-plan, completed tasks
  are uncommitted and may be lost or mixed with incomplete work.

This is a behavioral preference that varies by user and project. Some users
manage their own git workflow carefully (e.g., using `git add -p`, maintaining
specific staging states) and would not want the skill creating commits. Others
want the convenience of automatic checkpoints.

The existing participation mode framework (ADRs 0006–0008) provides a convention
for configurable behavioral policies stored as meta-ADRs in `<adr-dir>/.meta/`.
A commit-on-task-completion preference fits naturally into this framework.

## Decision

### 1. Add configurable "commit on task completion"

The skill will support an optional behavior: **create a git commit each time a
task's acceptance criteria are all satisfied** (i.e., all checkboxes for that
task are marked `- [x]`). This is an opt-in behavioral preference, disabled by
default.

### 2. Alternatives considered

Three checkpoint strategies were evaluated against the stated goals: reviewable
history and interruption resilience.

| # | Strategy | Mechanism | Reviewable History | Interruption Resilience | Trade-offs |
|---|----------|-----------|--------------------|-----------------------|------------|
| A | **Commit per task** (chosen) | `git commit` after each task completes | ✅ Each commit maps to one plan task | ✅ Completed tasks are committed immediately | More commits; requires identifying task-related files |
| B | **Commit per stage** | `git commit` after all tasks in a stage complete | ⚠️ Coarser granularity — one commit per stage | ⚠️ Partial stage work is uncommitted if interrupted mid-stage | Fewer commits; simpler file identification |
| C | **Tag per task, commit per stage** | `git tag` at each task boundary; `git commit` at stage end | ✅ Tags provide task-level markers within stage commits | ⚠️ Tags exist only locally; uncommitted work still at risk | Combines markers with coarser commits; tag management overhead; tags on uncommitted work are fragile |

**Why Option A:** Per-task commits provide the finest granularity and the
clearest mapping from git history to plan tasks. The main cost — more commits —
is mitigated by the opt-in default (disabled) and the user's ability to squash
before merging. The file-identification challenge exists in all strategies (even
per-stage commits must determine which files belong to the stage) and is
addressed by guard rails below.

### 3. Commit behavior

When enabled and all checkboxes for a task are marked `- [x]`:

1. **Stage the plan file** — `git add <plan-file>`.
2. **Stage implementation files** — `git add` any files created or modified as
   part of the task. The agent identifies task-related files by tracking which
   files it created, edited, or deleted during the task's execution.
3. **Create a commit** with a conventional message:

   ```
   <type>(<scope>): <brief summary>

   Plan: <plan-file-path>
   Task: <N.M> <task title> [<cost>]
   ADR: <adr-reference>
   ```

   Use the canonical [Conventional Commits](https://www.conventionalcommits.org/)
   type and scope that best describes the work (e.g., `feat`, `fix`, `refactor`,
   `docs`, `test`, `chore`). The summary should be a brief sentence describing
   what was done.

4. **Do not push** — commits are local only. The user decides when to push.

**Guard rails:**

- If the working tree has unstaged changes that the agent did not create or
  modify during this task, warn the user and ask whether to include them or
  commit only task-related files. **Autonomous mode fallback:** when running in
  Autonomous mode, do not prompt — commit only task-related files and log a
  warning noting the skipped unrelated changes.
- If a task modifies files that have merge conflicts or are in a dirty state
  from prior work, pause and ask the user to resolve before committing.
- If `git commit` fails due to pre-commit hooks (linters, formatters, security
  scanners), **pause and ask the user regardless of participation mode**. Hook
  failures are unexpected and may indicate code quality issues that require
  human judgment. The agent should report the hook's error output and let the
  user decide whether to fix the issue, skip the commit, or retry with
  `--no-verify`.
- The commit includes the updated plan file (with the newly checked boxes) so
  that progress is captured in version control.
- Auto-commit modifies the user's git state (staging area and commit history).
  Users who carefully manage their index (e.g., `git add -p`, curated staging)
  should leave this feature disabled to avoid conflicts with their workflow.

### 4. Capture the preference using the participation mode convention

The commit-on-task-completion preference follows the same behavioral policy
pattern established by ADRs 0006–0008:

1. **Prompt after the participation check** (Step 5) — after the user selects
   their participation mode, ask:

   > Would you like to create a git commit each time a task completes?
   > 1. **Yes** — Commit after each task's acceptance criteria are all satisfied
   > 2. **No** (default) — I'll manage commits myself

2. **Check for existing preference** — if a meta-ADR for commit-on-task-
   completion was loaded from `.meta/` in Step 0, apply it silently:

   > Auto-commit on task completion: **enabled** (from .meta policy).

3. **Persist via meta-ADR** — if `.meta/` exists, offer to save the preference:
   - Title: "Auto-commit on task completion: [enabled/disabled]"
   - Same Nygard format and fallback chain as participation mode (ADR-0008).

4. **If `.meta/` does not exist** — store in session context only. Do not
   prompt to create `.meta/`.

**Behavioral policies table (updated):**

| Policy | Meta-ADR Title Pattern | Effect |
|--------|----------------------|--------|
| Participation mode | "Participation mode: \*" | Sets the default participation level |
| Auto-commit | "Auto-commit on task completion: \*" | Enables/disables commits at task boundaries |

### 5. Interaction with participation modes

The auto-commit behavior is orthogonal to participation modes. It can be combined
with any of the four modes:

| Mode | Auto-commit enabled | Behavior |
|------|-------------------|----------|
| **Full control** | Yes | Commit after each approved-and-completed task |
| **Guided** | Yes | Commit after each completed task within approved stages |
| **Autonomous** | Yes | Commit after each completed task; commit only task-related files without prompting when unrelated changes exist; still pause on hook failures |
| **Weighted** | Yes | Commit after each completed task (autonomous or sentinel); same autonomous fallback for `[small]` tasks |

In all cases, the commit happens **after** the task's criteria are all checked
off, regardless of whether the task required user approval.

### 6. Revisit trigger

Revisit this decision after **5 plan executions** with auto-commit enabled to
validate:
- Whether per-task commits produce an acceptably clean git history or whether
  per-stage granularity would have been preferable.
- Whether the agent's file-identification heuristic (tracking files it touched)
  reliably separates task-related from unrelated changes.
- Whether the guard rails around dirty working trees and unrelated changes
  are sufficient or need refinement.
- Whether pre-commit hook failures are common enough to warrant a more nuanced
  response than always pausing.

## Consequences

**Positive:**

- Auto-commit on task completion creates reviewable, task-scoped commits in git
  history, giving pull request reviewers incremental diffs traceable to plan
  decisions rather than a single large diff.
- Completed tasks are committed immediately, providing interruption resilience —
  if a session ends mid-plan, finished work is already in version control.
- Commit messages include plan and ADR references, creating traceability from
  git history to architectural decisions.
- The preference is captured via the existing meta-ADR convention (ADR-0008),
  requiring no new infrastructure and allowing per-project customization.
- Disabled by default, so users who manage their own git workflow are unaffected.

**Negative / Risks:**

- Frequent commits (one per task) may produce a verbose git history for plans
  with many small tasks. Mitigated by the opt-in default (disabled) and by the
  user's ability to squash commits before merging.
- Auto-commit modifies the user's git staging area and commit history. Users who
  carefully manage their index (e.g., `git add -p`) may find this disruptive.
  Mitigated by the opt-in default and the explicit guard rail warning about this
  in the skill documentation.
- The agent must correctly identify which files belong to a task. The heuristic
  (track files the agent itself created or modified during the task) is
  reasonable but not perfect — files modified by build tools, formatters, or
  pre-commit hooks may be included or excluded incorrectly. Mitigated by the
  guard rail that warns about unrelated changes and asks the user.
- This feature has not been prototyped. The commit format, guard rails, and
  file-identification approach are designed but unvalidated. Mitigated by the
  revisit trigger above and the opt-in default.
- Pre-commit hooks may reject auto-commits, requiring user intervention
  regardless of participation mode. This creates an unavoidable pause point in
  otherwise autonomous workflows. Mitigated by the fact that hook failures
  indicate genuine issues requiring human judgment.

**Neutral:**

- Auto-commit depends on ADR-0009's progress tracking protocol (checkbox
  updates signal task completion), but ADR-0009 is independent of this ADR.
  The protocol applies regardless of whether auto-commit is enabled.
- The auto-commit preference is independent of participation mode. Either can
  be configured without the other.
