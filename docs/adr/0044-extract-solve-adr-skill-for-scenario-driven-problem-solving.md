# 44. Extract solve-adr skill for scenario-driven problem solving

Date: 2026-04-06
Status: Prototype
Last Updated: 2026-04-05
Links:
- [ADR-0019: Problem-first solve task](0019-add-problem-first-solve-task-to-author-adr-workflow.md)
- [ADR-0013: Split practices into task-specific documents](0013-split-practices-reference-into-task-specific-documents.md)
- [ADR-0023: Prototype-adr skill](0023-add-prototype-adr-skill-for-structured-decision-validation.md)
- [ADR-0032: Draft worksheet](0032-add-draft-worksheet-to-author-adr-workflow.md)

## Context

The `author-adr` skill currently owns the solve workflow (ADR-0019's `references/solve.md`). Solve is a problem-first workflow: the user describes a problem, the agent explores options, and the output is a Proposed ADR ready for review. It sits inside author-adr because it produces ADRs.

Three things have changed since ADR-0019:

**1. Solve is an orchestrator, not an author.** The solve workflow's value is in problem decomposition, option discovery, and convergence — not in ADR authoring mechanics (template selection, format detection, tooling). It delegates to author-adr for the actual ADR creation. Keeping it inside author-adr conflates two roles: the agent that *explores problems* and the agent that *writes decision records*.

**2. The skill ecosystem has matured.** Three companion skills now exist with stable interfaces:
- `/author-adr` — create, review, revise, manage ADRs
- `/prototype-adr` — run controlled experiments to validate options
- `/implement-adr` — turn accepted decisions into staged implementation plans

A solve workflow that orchestrates across all three skills doesn't belong inside one of them. It should sit alongside them and call into each as needed.

**3. Solve needs scenarios, not steps.** The current solve.md is a single linear workflow: problem intake → option discovery → convergence. Real problem-solving has multiple entry points:
- "I have a problem to solve" — the existing flow
- "Implement roadmap milestones X to Y" — multi-decision orchestration
- "Look at these issues and drive a fix" — triage-driven problem solving

These are fundamentally different workflows that share a common principle: explore the problem, make decisions defensively (log via `/author-adr`), and deliver results. A scenario-based skill design accommodates this diversity without a monolithic step list.

### Decision Drivers

- **Separation of concerns** — the problem-solving orchestrator should not be embedded in the decision-authoring skill
- **Cross-skill orchestration** — solve needs to call author-adr, prototype-adr, and implement-adr as peers, not as a parent calling into itself
- **Scenario extensibility** — new problem-solving entry points must be addable without modifying existing scenarios
- **Defensive logging** — every decision made during solve must be recorded via `/author-adr` for auditability
- **Preference awareness** — the skill must check and update automation preferences, recommending configurations when they're missing

## Options

### Option 1: Extract solve-adr as a new skill with scenario-based procedures

Create `src/skills/solve-adr/` as a fourth skill. The SKILL.md uses a scenario-based procedure table instead of sequential steps. Each scenario defines a distinct problem-solving entry point with its own workflow, but all scenarios share common behaviors: preference loading, defensive ADR logging, and cross-skill delegation.

The initial implementation includes one scenario (S-1: Problem Exploration). The architecture supports adding scenarios (e.g., S-2: Roadmap Execution, S-3: Issue Triage) without modifying existing ones.

Cross-skill calls use explicit skill invocations:
- `/author-adr` — when a decision needs to be recorded
- `/prototype-adr` — when an assumption needs validation
- `/implement-adr` — when an accepted decision needs execution

**Strengths:**
- Clean separation: solve-adr orchestrates, author-adr authors, prototype-adr validates, implement-adr executes
- Scenario table is extensible — new entry points don't modify existing workflows
- Cross-skill delegation is explicit and auditable
- Preferences are loaded once at startup and applied consistently across scenarios
- solve.md's content moves to the new skill — author-adr gets simpler

**Weaknesses:**
- Fourth skill increases the ecosystem surface area
- Users must learn which skill to invoke: "solve" vs. "create" distinction may confuse new users
- Cross-skill invocation adds latency (skill loading per delegation)
- solve.md removal from author-adr is a breaking change for users who invoke `/author-adr` with "help me solve"

### Option 2: Keep solve in author-adr, add orchestration hooks

Keep solve.md in author-adr but add explicit delegation points where the workflow calls out to `/prototype-adr` and `/implement-adr`. The current A-2 step ("Create or Solve") becomes the orchestration point. No new skill is created.

**Strengths:**
- No new skill to maintain — fewer moving parts
- Users don't need to learn a new skill name
- solve.md is already integrated into author-adr's procedure table
- No breaking change for existing `/author-adr` users

**Weaknesses:**
- author-adr becomes increasingly complex as scenarios are added
- Cross-skill calls from inside author-adr are semantically awkward (a skill calling its peers)
- Scenario extensibility requires modifying author-adr's SKILL.md for every new scenario
- The solve workflow's orchestration concerns pollute author-adr's focused authoring mission
- Adding roadmap and issue triage scenarios would make author-adr's description and trigger list unwieldy

### Option 3: Create solve-adr as a thin dispatcher that delegates entirely

Create `src/skills/solve-adr/` but make it a pure dispatcher with no domain logic. Each scenario is a one-line delegation to another skill. Problem exploration delegates to author-adr's solve.md. Roadmap execution delegates to implement-adr. Issue triage delegates to author-adr's create workflow.

**Strengths:**
- Minimal new code — solve-adr is just a routing table
- Domain logic stays in the skills that own it
- Easy to maintain — changes happen in the downstream skills

**Weaknesses:**
- No unified problem-solving workflow — each scenario is just a redirect
- No shared behaviors (preference loading, defensive logging) across scenarios
- Doesn't solve the core problem: the orchestration logic still lives elsewhere
- Users could just invoke the downstream skills directly — the dispatcher adds no value

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Pause for validation

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**
- Verify that prompt-based cross-skill invocation works for the S-1 workflow (agent can invoke `/author-adr` to create ADRs mid-scenario and resume the solve workflow after)

## Decision

In the context of **scaling the ADR skill ecosystem to support diverse problem-solving workflows**, facing **the solve workflow outgrowing its host skill (author-adr) and needing to orchestrate across all three companion skills**, we decided for **extracting solve-adr as a new skill with scenario-based procedures** (Option 1), and neglected **keeping solve in author-adr with hooks (Option 2, doesn't scale) and a thin dispatcher (Option 3, no unified behavior)**, to achieve **clean separation of concerns where solve-adr orchestrates problem-solving while delegating decisions, experiments, and implementation to their respective skills**, accepting that **the ecosystem grows to four skills and users must learn the solve/create distinction**.

### Skill Architecture

```
solve-adr (orchestrator)
├── /author-adr   — record decisions, review, revise
├── /prototype-adr — validate assumptions with experiments
└── /implement-adr — execute accepted decisions
```

### Procedure Table (Scenario-Based)

| ID | Scenario | Description |
|----|----------|-------------|
| S-0 | Startup | Load preferences, check automation config, recommend missing settings |
| S-1 | Problem Exploration | User provides problem + constraints → explore options → converge on decision |
| S-2 | *(future)* Roadmap Execution | Implement milestones X to Y across multiple decisions |
| S-3 | *(future)* Issue Triage | Analyze issues and drive fixes through the decision pipeline |

All scenarios share:
- **Preference loading** (S-0) — read `[solve]` from user and project-scoped `preferences.toml`, recommend settings when absent
- **Defensive logging** — every decision, no matter how small, is recorded via `/author-adr`
- **Cross-skill delegation** — explicit `/skill-name` invocations, never inline reimplementation

### S-0: Startup

1. Read `~/.config/adr-skills/preferences.toml` for `[solve]` section
2. Read `.adr/preferences.toml` for project-scoped `[solve]` overrides
3. If preferences are missing, recommend defaults and offer to persist them
4. Load `[author.dispatch]` keys for downstream `/author-adr` calls

### S-1: Problem Exploration (Initial Scenario)

```
User provides: problem, background, thought process, constraints
    ↓
S-1.1: Problem intake — capture problem statement, constraints, stakeholders
    ↓
S-1.2: /author-adr — create TBD ADR with draft worksheet, populate Context
    ↓
S-1.3: Option discovery — explore solutions respecting user's constraints
    ↓
S-1.4: Requirements refinement — fold emergent requirements back into Context
    ↓
S-1.5: Evaluation checkpoint — assess readiness, delegate to /prototype-adr if validation needed
    ↓
S-1.6: Convergence — user selects option, /author-adr drafts Decision + Consequences
    ↓
S-1.7: /author-adr review → revise cycle
    ↓
S-1.8: Handoff — offer /implement-adr for execution
```

### Cross-Skill Invocation Mechanism

The solve-adr agent delegates to companion skills by instructing the user (or, in autonomous mode, directly invoking) the target skill with the relevant ADR path. The mechanism is **prompt-based delegation**: the agent outputs a skill invocation instruction (e.g., "invoke `/author-adr` to create an ADR for this decision") and the platform routes it to the target skill's SKILL.md.

This is not a sub-agent launch or file-based contract — it is the same mechanism a user employs when manually switching skills. The solve-adr agent manages the orchestration state (which scenario step it's on, which ADRs have been created) and delegates atomic operations to the companion skills.

### Defensive Logging Principle

In all scenarios, the agent must:
- Create an ADR via `/author-adr` for every architectural decision encountered during problem solving
- Use `/author-adr` review workflow for quality assurance on each decision
- Never make a decision silently — if a choice affects architecture, it gets an ADR

This is the audit trail. The solve-adr skill's primary output is a set of reviewed, accepted decisions — not code.

### Preference Management

```toml
# ~/.config/adr-skills/preferences.toml
[solve]
participation = "guided"     # guided | autonomous
auto_delegate = false        # automatically invoke /implement-adr after acceptance

# .adr/preferences.toml (project-scoped)
[solve]
default_scenario = "problem" # which scenario to use when not specified
```

When a preference is not set:
1. Recommend the default value with a brief explanation
2. Ask if the user wants to persist it
3. If yes, write to the appropriate scope (user or project)

### Impact on author-adr

- Remove `references/solve.md` from author-adr
- Remove the "Solving a Problem" section from author-adr's SKILL.md
- Remove "I have a problem to solve" from the routing flow chart
- Update author-adr's description to remove solve-related triggers
- author-adr retains full ownership of: create, review, revise, manage, templates, tooling

### Impact on eval_queries.json

Update the trigger evaluation queries:
- Remove solve-related should-trigger queries from author-adr
- Add should-trigger queries for solve-adr
- Add should-not-trigger queries for solve-adr (e.g., "create an ADR" should NOT trigger solve-adr)

## Consequences

**Positive:**
- author-adr's scope narrows to its core mission: ADR authoring and lifecycle management. The skill description and trigger list become more focused.
- Problem-solving workflows can evolve independently. Adding a roadmap scenario doesn't require touching author-adr.
- Cross-skill delegation is designed to be explicit and traceable. A user reading the solve-adr procedure can see exactly when and why each companion skill is invoked.
- The workflow instructs the agent to record every decision via `/author-adr`, regardless of which scenario triggered it.
- Preference management is centralized in S-0, applying consistently across all scenarios.

**Negative:**
- The ecosystem grows from three skills to four. Users, documentation, and the eval_queries.json all need updating.
- Users who currently say "/author-adr help me solve X" will need to use "/solve-adr" instead. The transition period will have both paths active until author-adr's solve references are removed.
- Cross-skill invocation adds latency. Each `/author-adr` or `/prototype-adr` call loads the skill context. For autonomous workflows with many small decisions, this could be noticeable.
- solve-adr depends on all three companion skills. A breaking change in any companion skill's interface affects solve-adr.
- The scenario architecture's extensibility value depends on future scenarios (S-2, S-3) materializing. If they are never needed, Option 1's additional complexity over Option 2 yields no return. This is an accepted architectural bet — the design optimizes for anticipated growth at the cost of upfront complexity.

**Neutral:**
- The solve.md content migrates largely intact — the problem intake, option discovery, requirements refinement, and convergence steps are the same. What changes is the container (standalone skill vs. reference file) and the delegation model (explicit `/skill` calls vs. internal references).
- This ADR defines the architecture and initial scenario (S-1). Future scenarios (S-2, S-3) will be separate ADRs that add to the procedure table.
- The `local` preference scope (`.adr/preferences.toml`) follows the convention from ADR-0020 and ADR-0042.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [ ] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

The primary quality concern is ensuring author-adr's description and trigger list are updated to exclude solve-related triggers — otherwise both skills will trigger on the same queries, causing confusion. The eval_queries.json for both skills must be tested to verify clean separation.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This ADR defines the architecture for solve-adr and the initial scenario (S-1). Future scenarios are separate ADRs. The solve.md content migration is mechanical — the design work is in the scenario model and cross-skill delegation pattern.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Extract the solve workflow from author-adr into a dedicated `solve-adr` skill. The new skill orchestrates problem-solving end-to-end by delegating to the other three skills: `/author-adr` for decisions, `/prototype-adr` for experiments, `/implement-adr` for implementation. Instead of numbered steps, use named "scenarios" that define different problem-solving entry points (e.g., "user provides a problem" vs. "implement roadmap milestones" vs. "triage issues"). In all scenarios, the agent defensively logs decisions with `/author-adr` for audit trail.

**Tolerance:**
- Risk: Medium — new skill with cross-skill orchestration is a significant architectural change
- Change: High — willing to extract solve.md and restructure the workflow
- Improvisation: Medium — open to creative scenario design, but must respect existing skill contracts

**Uncertainty:**
- Certain: solve is distinct from create (problem-first vs. decision-first); the three companion skills exist and have stable interfaces; preferences.toml is the config mechanism
- Uncertain: exact scenario interface design; how to handle preferences for solve-adr vs. reusing existing skill preferences; whether solve-adr needs its own Makefile/scripts

**Options:**
- Target count: 2-3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Extract solve.md into a new `solve-adr` skill with scenario-based procedure table
- Keep solve in author-adr but add orchestration hooks for the other skills

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: How does solve-adr invoke companion skills at runtime?

**Addressed** — Added a "Cross-Skill Invocation Mechanism" section to the Decision. The mechanism is prompt-based delegation: the agent outputs a skill invocation instruction and the platform routes it. This is the same mechanism users employ when manually switching skills — not a sub-agent launch or file-based contract.

### Q: Should the Evaluation Checkpoint list the cross-skill invocation assumption as a validation need?

**Addressed** — Changed Assessment from "Proceed" to "Pause for validation." Added validation need: "Verify that prompt-based cross-skill invocation works for the S-1 workflow."

### Q: Are the consequence claims about traceability and enforcement overstated?

**Addressed** — Qualified two consequences: "explicit and traceable" → "designed to be explicit and traceable"; "ensures every decision is recorded" → "instructs the agent to record every decision."

### Q: Is the scenario model's extensibility value speculative since S-2/S-3 don't exist?

**Addressed** — Added a negative consequence acknowledging this as an accepted architectural bet: the design optimizes for anticipated growth at the cost of upfront complexity, and if future scenarios never materialize, Option 1's complexity yields no return over Option 2.

### Q: Should extraction and scenario redesign be split into separate ADRs?

**Rejected** — These are causally linked. The scenario model is the architectural justification for extraction. Without scenarios, extraction is just moving a file (no decision needed). Without extraction, scenarios pollute author-adr's scope (Option 2's weakness). The two are decided together because they motivate each other.

### Q: Should the breaking change migration timeline be specified?

**Rejected** — The "Impact on author-adr" section documents exactly what gets removed. When the old path gets removed is an implementation sequencing question that belongs in the implementation plan, not in this decision.

<!-- Review cycle 1 — 2026-04-06 — Verdict: Revise. 4 addressed, 2 rejected. -->
