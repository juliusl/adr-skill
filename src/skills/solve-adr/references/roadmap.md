# S-2: Roadmap Execution

Self-contained reference for the Roadmap Execution scenario. Read this file when the user has existing Proposed ADRs that need implementation — the decisions are already made, the work is execution.

## When to Use

Activate S-2 when:
- The user wants to implement **existing Proposed ADRs** — not explore a new problem
- The user says "implement these ADRs," "continue solving," "drive this roadmap," or "implement milestones X to Y"
- A set of Proposed ADRs are ready for implementation and need dependency-ordered execution

**S-2 vs. S-1:** Use S-1 when the user has a *problem* (exploration needed). Use S-2 when the user has *decisions* (execution needed). If the user's request involves both ("solve this problem and implement it"), that's S-1 — S-1.3 handles implementation after decisions are made.

**S-2 vs. /implement-adr:** Use `/implement-adr` directly for a single ADR. Use S-2 when implementing a chain where ordering, dependency analysis, and progress tracking across ADRs matters.

## S-2.1: Survey

Identify the ADRs in scope and their current state. Delegate to `/author-adr` for listing and status — it owns format detection and knows how to enumerate ADRs regardless of naming convention (sequential `NNNN-` or work-item-referenced `{remote}-{id}-`).

1. **Gather scope** — from the user's request, determine which ADRs are included. This may be:
   - Explicit: "implement ADRs [these specific ones]"
   - Implicit: "implement all ADRs related to [topic]" → invoke `/author-adr` to list and filter
   - Continuation: "continue from where we left off" → find the next unimplemented ADR in a known chain

2. **Invoke `/author-adr`** to list ADRs and read their status. Author-adr's `list` and `status` commands handle both naming formats transparently.

3. **For each in-scope ADR**, extract:
   - Status (Proposed, Planned, Accepted, etc.)
   - Links to other ADRs (dependency signals)
   - Key decision content (enough to understand what it implements)

4. **Filter** — only Proposed ADRs need implementation. Accepted ADRs are already done. Planned ADRs are mid-implementation.

## S-2.2: Dependency Analysis

Determine the implementation order.

1. **Build dependency graph** — from each ADR's Links section and Context references, identify which ADRs depend on which. Common signals:
   - Explicit dependency statements in Context (e.g., "depends on [ADR ref]")
   - Sequencing requirements in Consequences (e.g., "after [ADR ref] is accepted")
   - Shared concepts where one ADR defines the foundation another builds on

2. **Topological sort** — order the ADRs so dependencies are implemented before dependents. If two ADRs have no dependency relationship, they can be implemented in either order (prefer lower number first).

3. **Present the plan:**
   ```
   Implementation order:
   1. [ADR ref] (foundation) — no dependencies
   2. [ADR ref] (builds on #1) — depends on #1
   3. [ADR ref] (builds on #2) — depends on #2
   ...
   Proceed? [Yes / Adjust / Stop at N]
   ```
   In autonomous mode, proceed without confirmation.

4. **Identify blockers** — if any dependency is not in scope (e.g., depends on an ADR that isn't Proposed), flag it. The chain may need to stop at that point.

## S-2.3: Execute

Implement each ADR in order by delegating to `/implement-adr`.

For each ADR in the chain:

1. **Invoke `/implement-adr`** with the ADR number. Let it run its full procedure (plan → review → QA → execute → finalize).

2. **Check result** — after `/implement-adr` completes:
   - If the ADR status is now `Accepted` → success, continue to next
   - If implementation failed or paused → stop the chain, report progress

3. **Check for gaps** — if `/implement-adr` discovers a gap that requires a new ADR:
   - Pause the chain
   - Invoke `/author-adr` to create the gap ADR
   - Add the new ADR to the chain in the correct position
   - Resume

4. **Progress tracking** — after each ADR completes, log the progress:
   ```
   ✅ [ADR ref]: Accepted ([title])
   🔄 [ADR ref]: Implementing... ([title])
   ⏳ [ADR ref]: Pending ([title])
   ```

**Session boundaries:** A single session may not have enough context to implement all ADRs in a long chain. When the session is nearing its limits:
- Stop at the current ADR boundary (don't start a new implementation mid-session)
- Report progress with a summary of what was completed and what remains
- The user can resume with "continue solving" in a new session

## S-2.4: Progress Report

After the chain completes (or stops), summarize:

```markdown
## Roadmap Progress

| ADR | Title | Status | Result |
|-----|-------|--------|--------|
| [ref] | [first title] | Accepted | ✅ Completed |
| [ref] | [second title] | Accepted | ✅ Completed |
| [ref] | [third title] | Proposed | ⏳ Next up |

**Completed:** N of M
**Next:** [ADR ref] ([title])
**Blocked:** None
```

## Defensive Logging

During roadmap execution, architectural decisions may emerge that aren't covered by the existing ADRs (e.g., a gap discovered during implementation). When this happens:

1. Pause the current implementation
2. Invoke `/author-adr` to create an ADR for the new decision
3. Review and accept the new ADR
4. Add it to the roadmap chain
5. Resume implementation

Every decision gets an ADR — even mid-roadmap discoveries.
