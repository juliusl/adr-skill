# Roadmap

Self-contained reference for solving a roadmap. Read this file when the user has a roadmap document to process — whether starting fresh or resuming at a specific milestone.

**Key relationship:** A roadmap contains ordered milestones. Each milestone is treated as a problem and delegated to S-1 Problem (one-to-many).

**Resume protocol:** Every solvable thing is resumable. When solve-adr is invoked on a roadmap that has prior progress, it checks milestone status markers and picks up at the next incomplete milestone. Resume is not a separate scenario; it's how solve works across sessions.

---

## Status Markers

Milestone progress is tracked using HTML comment status markers appended to milestone headings:

```markdown
### Milestone 1 <!-- status: complete -->
### Milestone 2 <!-- status: in-progress -->
### Milestone 3
```

### Valid Values

| Marker | Meaning |
|--------|---------|
| `<!-- status: complete -->` | All objectives solved and implemented |
| `<!-- status: in-progress -->` | Work has started; ADRs exist for this milestone |
| (no marker) | Pending — no work done yet. This is the default state. |

These are the only valid status values. The agent must not invent new values.

### Format Rules

- Markers are case-insensitive: `<!-- status: Complete -->` and `<!-- status: complete -->` are equivalent.
- Whitespace around the value is trimmed: `<!-- status:  complete  -->` is valid.
- The marker must appear on the same line as the `###` heading.
- If a marker contains an unrecognized value, treat the milestone as pending and log a warning.

### Immutability

Do not modify milestone content (the objective list). Status markers are appended to headings only. Changes to scope go through the user, not the agent.

---

## Defensive Logging

During roadmap processing, architectural decisions may emerge that aren't covered by the current milestone's objectives. When this happens:

1. Pause the current work
2. The decision is captured via S-1's normal `/author-adr` invocation
3. Add the new ADR to the current milestone's tracking
4. Resume

Every decision gets an ADR — even mid-milestone discoveries. This is inherited from S-1's defensive logging behavior.

---

**All steps must be visited in order. If a step is skipped or its entry condition is not met, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

---

## Roadmap Document Format

A roadmap follows this structure:

```markdown
# [project name] roadmap

[description — what is being built and why]

## Features
- [high-level capability]
- [high-level capability]

## Constraints
- [technical or organizational constraint]
- [technical or organizational constraint]

## Roadmap

[preamble — explains what milestones are and how items work]

### Milestone N
- [objective or user story]
- [objective or user story]

### Milestone N+1
- [objective or user story]

---

[Supplementary design sections — detailed specs that support specific milestones]
```

**Required sections:**
- Project description (the opening paragraph)
- `## Roadmap` with at least one `### Milestone N`

**Optional sections:**
- `## Features` — high-level capabilities the project delivers
- `## Constraints` — technical or organizational limits
- Supplementary sections below the `---` separator — detailed specs, format definitions, or design notes that support specific milestones

**Milestone items** are high-level objectives, not fully specified tasks. Each item describes a goal but does not design or explore the solution. Gaps are expected — they signal where ADRs are needed.

---

## Procedure

```
1. Load — read and parse the roadmap document
   ↓
2. Survey — identify milestone progress (complete, in-progress, pending)
   ↓
3. Select — determine which milestone to work on next
   ↓
4. Solve — delegate milestone to S-1 Problem lifecycle
   ↓
5. Update — record milestone completion status → more milestones? → loop to 3
   ↓
C. Conclusion — QA triage, code review, report, retrospective (defined in SKILL.md)
```

**On resume:** The agent reads the roadmap file and checks milestone status markers:
- No status markers → start at step 1 (load)
- Some milestones marked complete → enter step 3 (select next)
- A milestone is in-progress with existing ADRs → enter step 4 (solve, resume)
- All milestones complete → Conclusion

| ID | Description |
|----|-------------|
| Step 1 | Read and parse the roadmap document — extract context, milestones, supplementary sections |
| Step 2 | Determine current progress by reading status markers and scanning for existing ADRs |
| Step 3 | Select the next milestone to work on based on status and ordering |
| Step 4 | Delegate the selected milestone to S-1 Problem as a structured intake |
| Step 4a | Handoff to S-1 — map roadmap elements to S-1 intake fields |
| Step 4b | Branch naming — use `solve/<project-slug>/milestone-<N>` pattern |
| Step 4c | Two-level resume — milestone-level (S-2) + ADR-level (S-1) composition |
| Step 4d | Session boundaries — handle session limits at milestone boundaries |
| Step 5 | Update the roadmap file with milestone completion status markers |
| Step 5a | Continuation — check for remaining milestones and loop or conclude |

## Step 1: Load

Read and parse the roadmap document. Extract:

1. **Project context** — the description paragraph and any Features/Constraints sections. This becomes the problem context for downstream S-1 invocations.
2. **Milestones** — each `### Milestone N` section with its list of objectives.
3. **Supplementary sections** — any content below the `---` separator. Tag each section with the milestone(s) it supports (by reference or topic match).

**Validation:**
- The document must have at least one milestone. If not, warn and suggest the user structure their document.
- Milestones must be numbered and ordered. Non-sequential numbering (e.g., skipping from 5 to 7) is allowed — it signals intentionally deferred work.

**Present the parsed roadmap:**

```
Roadmap: [project name]
Milestones: N total

| # | Milestone | Items | Status |
|---|-----------|-------|--------|
| 1 | Milestone 1 | N items | Pending |
| 2 | Milestone 2 | N items | Pending |
| 3 | Milestone 3 | N items | Pending |
```

## Step 2: Survey

Determine current progress by reading status markers in the roadmap file.

Scan each `### Milestone N` heading for an HTML comment status marker. Parse the value per the rules in [Status Markers](#status-markers).

**Survey also checks for existing ADRs** that reference the roadmap or its milestones. Use `make -f <skill-root>/Makefile list` and scan ADR context/links for roadmap references.

**Present findings:**

```
Roadmap progress: N of M milestones complete

| # | Milestone | Status | ADRs |
|---|-----------|--------|------|
| 1 | Milestone 1 | ✅ Complete | ADR-NNNN, ADR-NNNN |
| 2 | Milestone 2 | 🔄 In-progress | ADR-NNNN |
| 3 | Milestone 3 | ⏳ Pending | — |
```

## Step 3: Select

Determine which milestone to work on next.

**Selection rules:**
1. If a milestone is in-progress → resume it (step 4, resume path)
2. If no milestone is in-progress → select the first pending milestone
3. If the user requests a specific milestone → select that one (validate it's not already complete)
4. If all milestones are complete → skip to Conclusion (C-1 → C-2 → C-3 → C-4 in SKILL.md)

**Milestone dependencies** are implicit in ordering — Milestone N should complete before Milestone N+1. The user can override this by requesting a specific milestone.

**Present the selection:**

```
Next milestone: Milestone N — [first objective summary]

Items to solve:
1. [objective]
2. [objective]
3. [objective]

Supplementary context: [list any supplementary sections that support this milestone]

Proceed?
```

In autonomous mode, proceed without confirmation.

## Step 4: Solve

Delegate the selected milestone to S-1 Problem as a structured intake.

### Step 4a: Handoff to S-1

The milestone becomes S-1's problem input:

| Roadmap element | Maps to S-1 intake field |
|-----------------|--------------------------|
| Project description | Problem context |
| Constraints | Known constraints |
| Milestone objectives | Decisions to enumerate |
| Supplementary sections (for this milestone) | Additional context |

**Construct the S-1 intake:**

1. **Problem statement** — "Implement Milestone N of [project name]: [milestone summary]"
2. **Constraints** — carry forward from the roadmap's Constraints section
3. **Decisions to enumerate** — each milestone objective becomes a candidate decision. Some objectives may merge into a single ADR; others may expand into multiple.
4. **Additional context** — include relevant supplementary sections

**Invoke S-1** — pass the constructed intake to the Problem lifecycle. Include in the delegation prompt: "Run S-1 in sub-routine mode — skip conclusion, return control after implementation." S-1 runs its full lifecycle (Branch → Author → Triage → Implement) but skips conclusion. Conclusion runs once after all milestones complete (per Step 5a).

### Step 4b: Branch naming

When S-1 creates a feature branch for a roadmap milestone, use the pattern:

```
solve/<project-slug>/milestone-<N>
```

Example: `solve/my-project/milestone-3`

This nested path structure distinguishes roadmap-driven branches from ad-hoc problem branches (`solve/<problem-slug>`). The path nesting prevents namespace collisions — a problem slug would need to contain a `/` to conflict, which is not permitted in slug generation.

### Step 4c: Two-level resume

S-2 delegates to S-1, creating two levels of resume state:

| Level | Tracked by | State location |
|-------|------------|----------------|
| **Milestone level** | S-2 (Roadmap) | Status markers in roadmap file |
| **ADR level** | S-1 (Problem) | ADR status in `docs/adr/`, branch state |

When a session ends mid-milestone:
1. S-1's progress is preserved in ADR states and the feature branch
2. S-2's progress is preserved in the roadmap's status markers (milestone stays `in-progress`)

On resume:
1. S-2 reads the roadmap → finds the `in-progress` milestone
2. S-2 re-enters step 4 (solve) for that milestone
3. S-1's resume protocol takes over — it checks the branch and ADR states to pick up where it left off

The two levels compose naturally because each uses independent state storage. S-2 does not need to track S-1's internal state — it only needs to know whether the milestone is complete.

### Step 4d: Session boundaries

A single milestone may exceed session limits. When this happens:
1. S-1 reports its progress at the current task boundary
2. S-2 records partial progress (milestone stays in-progress)
3. The user resumes in a new session — S-2's survey step picks up where it left off

## Step 5: Update

After S-1 completes (or partially completes) a milestone, update the roadmap file:

1. **If all milestone objectives are implemented** — mark the milestone complete:
   ```markdown
   ### Milestone N <!-- status: complete -->
   ```

2. **If some objectives remain** — mark the milestone in-progress:
   ```markdown
   ### Milestone N <!-- status: in-progress -->
   ```

3. **Stage the updated roadmap file** — `git add <roadmap-path>`

### Step 5a: Continuation

After updating, check if more milestones remain:
- If yes → return to step 3 (select next milestone)
- If no → proceed to Conclusion (C-1 → C-2 → C-3 → C-4) defined in SKILL.md

In practice, each milestone is a substantial block of work. The agent typically completes one milestone per session and resumes in the next.

---

## Relationship to S-1 Problem

S-2 Roadmap is an orchestration layer over S-1 Problem. It does not duplicate S-1's logic:

| Concern | S-2 Roadmap | S-1 Problem |
|---------|-------------|-------------|
| Problem capture | Extracts from roadmap document | Conversational intake |
| Decision enumeration | Derived from milestone objectives | Derived from problem analysis |
| Branching | `solve/<project>/milestone-<N>` | `solve/<problem-slug>` |
| ADR authoring | Delegates to S-1 → author-adr | Invokes author-adr directly |
| Implementation | Delegates to S-1 → implement-adr | Invokes implement-adr directly |
| Conclusion | Shared — C-1, C-2, C-3, C-4 in SKILL.md | Shared — C-1, C-2, C-3, C-4 in SKILL.md |
| Progress tracking | Milestone status markers in file | ADR status in decision log |
| Resume granularity | Milestone level + S-1 ADR level | ADR level |

S-2 wraps S-1. All mandatory safeguards (plan review, QA, ADR for every decision) flow through S-1 unchanged.
