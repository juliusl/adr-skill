# 7. Weighted participation mode based on task cost

Date: 2026-04-02

## Status

Accepted

Partially supersedes [ADR-0006](0006-implement-adr-checks-user-participation-level-after-planning.md)

## Context

ADR-0006 introduced three participation levels for the `implement-adr` skill:
Full control, Guided, and Autonomous. Users choose one mode, and it applies
uniformly across all stages of the plan.

In practice, plans often contain a mix of stage complexities. A plan might begin
with lightweight foundation stages (scaffold project structure, add config
constants) and then progress into stages with heavy, ambiguous tasks (design a
plugin system, implement caching with invalidation). Applying a single
participation mode uniformly creates friction:

- **Autonomous mode** is efficient for lightweight stages but risky for heavy
  ones where the agent may make poor design choices without user input.
- **Guided mode** is safe for heavy stages but unnecessarily slow when every
  task in a stage is a well-defined `[small]` change.

The existing three modes in ADR-0006 treat participation as a per-plan setting.
A more adaptive approach would vary participation at the **task level** within
each stage, based on the cost estimate already assigned to each task.

## Decision

### 1. Add a "Weighted" participation mode

We will add a fourth participation mode — **Weighted** — to the options
presented after plan generation (ADR-0006, Decision §1). The updated prompt
becomes:

> How much participation would you like during implementation?
> 1. **Full control** — I'll review each stage and select what to start
> 2. **Guided** — Summarize the plan, let me pick stages or request changes
> 3. **Autonomous** — Execute the plan, check in at major milestones
> 4. **Weighted** — Automatically adjust based on task complexity

### 2. Weighted mode behavior

In Weighted mode, the skill evaluates each **task** independently as it
proceeds through a stage. The evaluation uses the cost estimate already
assigned to the task (per the cost estimation guide):

**Per-task rule:**

- **`[small]` tasks** → execute **autonomously**. No approval needed. The
  agent proceeds immediately to the next task.
- **`[medium]` or `[heavy]` tasks** → act as a **sentinel**. The agent pauses,
  summarizes what it is about to do, and waits for user approval before
  proceeding.

This means within a single stage, the agent may autonomously complete several
small tasks in sequence, then stop at the first medium or heavy task for
guidance. After the user approves the sentinel task and it completes, the
agent continues evaluating subsequent tasks by the same rule.

**Example:**

```
Stage 2: Authentication
  Task 2.1: Add auth config constants        [small]  → autonomous ✓
  Task 2.2: Create user model from schema    [small]  → autonomous ✓
  Task 2.3: Implement JWT middleware          [medium] → sentinel ⏸ (pause for approval)
  Task 2.4: Add rate limiting to auth routes  [small]  → autonomous ✓
  Task 2.5: Write integration tests           [medium] → sentinel ⏸ (pause for approval)
```

**Stage boundaries:** At the end of each stage, the agent reports a summary
of what was completed, regardless of whether individual tasks were autonomous
or guided. This preserves the stage-boundary reporting from ADR-0006.

### 3. Interaction with other modes

Weighted mode does not replace the other three modes from ADR-0006. The four
modes serve different preferences:

| Mode | Granularity | Best for |
|------|-------------|----------|
| **Full control** | Per-stage approval | Users who want to direct every phase |
| **Guided** | Plan-level with stage selection | Users who want oversight with flexibility |
| **Autonomous** | Milestone-only | Users who fully trust the plan |
| **Weighted** | Per-task, cost-driven | Users who want adaptive oversight |

A user may switch modes mid-session. For example, starting with Weighted and
switching to Full control for a stage they want to closely supervise.

## Consequences

**Positive:**

- Weighted mode provides adaptive oversight — autonomous where safe, guided
  where complexity warrants it — without requiring the user to manually
  classify stages.
- The per-task evaluation reuses cost estimates already produced during
  planning, adding no extra work for the user or agent.
- Small tasks that are well-defined and unambiguous execute without
  interruption, keeping momentum on straightforward work.
- Medium and heavy tasks naturally become checkpoints, catching the cases
  where agent judgment alone is most likely to go wrong.

**Negative / Risks:**

- Adding a fourth participation mode increases the choice surface when the
  user is prompted. Mitigated by the option being self-explanatory and by the
  fact that it could become the default over time.
- The cost-estimate-driven rule assumes estimates are accurate. A misestimated
  `[small]` task that is actually complex would execute without review.
  Mitigated by the agent still pausing on errors or ambiguity regardless of
  mode, and by the cost estimation guide's uncertainty premium (gap → bump
  one size level).
- Frequent sentinel pauses in a stage with many medium tasks could feel
  disruptive. Mitigated by the user's ability to switch to Autonomous or
  Guided mid-session if the pauses become excessive.

**Neutral:**

- Weighted mode does not replace the other three modes. Users who prefer
  a uniform participation level can still choose Full control, Guided, or
  Autonomous.
- The per-task evaluation happens at plan execution time, not plan generation
  time. The plan itself is unchanged.
