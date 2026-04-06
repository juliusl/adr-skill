# 49. Add Roadmap Scenario to Solve-ADR Skill

Date: 2026-04-06
Status: Accepted
Last Updated: 2026-04-06
Links: ADR-0044 (solve-adr skill extraction), ADR-0022 (experimentation tolerance)

## Context

solve-adr currently has one scenario: S-1 Problem. S-1 handles ad-hoc problems — the user describes a problem, the agent explores options, produces ADRs, and implements them. This works well for single problems but does not support structured, multi-milestone development workflows.

Projects often organize work as roadmaps — milestone-based planning documents that define a project vision, constraints, and ordered milestones with grouped objectives. An existing example is `docs/roadmaps/adr-atelier.md`, which structures work as:

1. A high-level project description
2. A features list
3. Constraints
4. Ordered milestones, each containing high-level objectives
5. Supplementary design sections that support specific milestones

Roadmaps are a natural fit for solve-adr because each milestone is essentially a batch of related problems to solve. The existing S-1 Problem lifecycle (Intake → Branch → Author → Triage → Implement → Report) already handles the per-milestone work — what's missing is the orchestration layer that reads a roadmap, tracks milestone progress, and feeds milestones into S-1.

**Decision drivers:**
- Roadmap-based workflows are already in use in this project (adr-atelier.md)
- solve-adr's description already mentions "implement milestones X to Y" as a trigger phrase
- The S-1 Problem lifecycle is composable — a milestone's objectives map naturally to S-1's intake step
- Resume protocol must extend to milestone-level progress, not just ADR-level

## Options

### Option A: Add S-2 Roadmap as a new scenario with `references/roadmap.md`

Add a new scenario (S-2: Roadmap) to the solve-adr procedure table. Create `references/roadmap.md` as the detailed workflow reference (mirroring how `references/problem.md` serves S-1).

The roadmap scenario lifecycle:
```
1. Load — read and parse the roadmap document
   ↓
2. Survey — identify current milestone progress
   ↓
3. Select — determine which milestone to work on next
   ↓
4. Solve — treat the milestone as a problem, delegate to S-1
   ↓
5. Update — record milestone completion in the roadmap
   ↓
6. Report — summarize roadmap progress
```

Each milestone delegates to S-1 Problem for the actual work. S-2 is an orchestration wrapper — it does not duplicate S-1's logic.

**Roadmap document format** (derived from `adr-atelier.md` model):
```markdown
# [project name] roadmap

[description paragraph — what is being built and why]

## Features
- [high-level capability list]

## Constraints
- [technical/organizational constraints]

## Roadmap

[preamble explaining milestone format]

### Milestone N
- [objective/user-story]
- [objective/user-story]

### Milestone N+1
- [objective/user-story]

---

[Supplementary design sections — detailed specs that support specific milestones]
```

**Strengths:**
- Clean separation of concerns — roadmap orchestration vs problem solving
- Reuses S-1 without modification
- Resume protocol extends naturally — milestone progress is visible in the document
- The roadmap document is human-readable and version-controlled

**Weaknesses:**
- Adds a new scenario to maintain
- Progress tracking requires writing back to the roadmap file (status markers)

### Option B: Extend S-1 Problem to accept roadmap documents as input

Instead of a new scenario, teach S-1 to detect when its input is a roadmap document and process milestones sequentially within the existing lifecycle.

**Strengths:**
- No new scenario to maintain
- Simpler routing

**Weaknesses:**
- Overloads S-1 with milestone orchestration logic
- S-1's lifecycle (single problem → ADRs → implement) does not cleanly map to multi-milestone workflows
- Makes S-1 harder to reason about and test
- Resume protocol becomes complex — need to distinguish "resume a problem" from "resume a roadmap at milestone N"

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [ ] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — this is a design-level decision with a clear model document to validate against.

## Decision

**Option A: Add S-2 Roadmap as a new scenario with `references/roadmap.md`.**

In the context of extending solve-adr to support milestone-based workflows, facing the need to process structured roadmap documents, we decided to add a new S-2 Roadmap scenario with a dedicated reference document, to achieve clean separation between roadmap orchestration and per-milestone problem solving, accepting that this adds one more scenario to maintain.

The Roadmap scenario composes with S-1 Problem — each milestone is delegated to S-1 as a structured intake. This means S-1 remains unchanged and focused. The roadmap document format follows the structure established by `docs/roadmaps/adr-atelier.md`.

## Consequences

**Positive:**
- solve-adr gains the ability to process structured roadmap documents milestone-by-milestone
- Each milestone benefits from the full S-1 lifecycle (ADR authoring, review, QA, implementation)
- Roadmap progress is visible in the document itself — human-readable status tracking
- Resume protocol extends naturally to milestone level

**Negative:**
- One more scenario to maintain in SKILL.md and test
- Progress tracking requires writing status markers back to the roadmap file

**Neutral:**
- SKILL.md procedure table grows from 2 entries (S-0, S-1) to 3 (S-0, S-1, S-2)
- Routing diagram needs an update to include roadmap triggers
- The roadmap document format becomes a convention that users must follow

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

The roadmap.md reference document must be self-contained — it should be usable without reading problem.md. However, it delegates to S-1, so it must clearly define the handoff interface.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR is intentionally lightweight — the real design artifact is `references/roadmap.md`, which this ADR authorizes creating. The document format is derived from the existing `docs/roadmaps/adr-atelier.md` model.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
solve-adr handles ad-hoc problems but not structured roadmaps. The adr-atelier.md document provides a clear model for what a roadmap looks like. The direction is to add a new scenario (S-2) that processes roadmap documents milestone-by-milestone, delegating each milestone to the existing S-1 Problem workflow.

**Tolerance:**
- Risk: Low — the design model already exists
- Change: Low — extends solve-adr without modifying S-1
- Improvisation: Low — follow the adr-atelier.md structure closely

**Uncertainty:**
- Known: the roadmap document structure (from adr-atelier.md model)
- Known: S-1 Problem lifecycle is composable
- Known: resume protocol pattern (from problem.md)
- Uncertain: exact progress tracking format within the roadmap file

**Options:**
- Target count: 2
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Option A: New S-2 scenario with references/roadmap.md — preferred, clean separation
- Option B: Extend S-1 to accept roadmaps — simpler routing but overloads S-1

<!-- Review cycle 1 — 2026-04-06 — Verdict: Accept. Minor: added ADR-0022 to Links header. -->
