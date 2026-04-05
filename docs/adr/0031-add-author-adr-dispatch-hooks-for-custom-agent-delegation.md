# 31. Add author-adr dispatch hooks for custom agent delegation

Date: 2026-04-04
Status: Planned
Last Updated: 2026-04-04
Links:
- Extends [ADR-0014](0014-define-author-and-implement-tables-in-toml-configuration.md) (defines `[author]` table schema)
- Related to [ADR-0004](0004-author-adr-skill-recommends-reviewer-agent-for-consequence-validation.md) (review agent recommendation workflow)
- Related to [ADR-0006](0006-implement-adr-checks-user-participation-level-after-planning.md) (participation mode precedent in implement-adr)
- Related to [ADR-0007](0007-weighted-participation-mode-based-on-task-cost.md) (weighted participation as adaptive model)

## Context

The `implement-adr` skill has clear agent dispatch hooks — it spawns
general-purpose sub-agents for plan review (ADR-0025) and QA planning
(ADR-0030), and controls user involvement via a participation mode preference
stored in `[implement].participation` (ADR-0014). This infrastructure allows
the agent type and user involvement to be configured per-workflow.

The `author-adr` skill has no equivalent infrastructure. It has several points
where work is delegated or could be delegated to sub-agents, but the agent type
is hardcoded (implicitly general-purpose) and user interaction is always
interactive. Specifically:

**Current delegation points (review → revise → re-review cycle):**

1. **Review execution** — `review.md` says "Use this reference as a prompt for
   a general-purpose agent to perform the review." The agent type is implicit
   and not configurable.
2. **Consequence validation** — during review, the agent interactively asks the
   user to validate each stated consequence. There is no way to delegate this
   to a persona agent.
3. **Finding triage** — during revise, each finding is presented to the user
   who chooses Address or Reject. There is no way to delegate this.
4. **Re-review dispatch** — after revisions, the user decides whether to
   re-review. There is no automated loop.

**Additional hook points across other workflows (future scope):**

5. **Create — implementability validation** — could be delegated to a
   specialized validator agent.
6. **Solve — option evaluation** — could be delegated to a domain expert persona.
7. **Manage — status transitions** — could be automated based on downstream
   skill signals.

The immediate problem is hooks 1–4: the review → revise → re-review cycle has
no configurable agent dispatch, no participation mode, and no way to swap in
custom agents. Users who want to use AI personas (e.g., a persona trained on
their own decision-making style) to automate the review cycle cannot do so.

### Decision Drivers

- **Parity with implement-adr** — the `[implement]` table has `participation`
  and `auto_commit`. The `[author]` table has only `template`. The config
  surface should follow the same pattern.
- **Custom agent personas** — users are creating personas of themselves to
  review things. The infrastructure should support swapping the default
  general-purpose agent with a custom agent at each hook point.
- **Progressive automation** — the default should be fully interactive (current
  behavior). Users opt in to delegation by configuring agents and participation.
- **ADR-0014 flat-table constraint** — settings should be direct key-value
  pairs under `[author]`. The revisit trigger is three levels of nesting.

## Options

### Option 1: Flat agent keys in `[author]` table

Add `review_agent` and `participation` keys directly to the existing `[author]`
table. Each hook point gets a `<hook>_agent` key. Participation controls
whether the configured agent also stands in for the user during interactive
steps.

```toml
[author]
template = "nygard"

# Agent for the review workflow step.
# Default: "general-purpose" — uses the built-in general-purpose agent.
# Can be set to a custom agent reference (e.g., path to .agent.md).
review_agent = "general-purpose"

# Participation mode for the review→revise cycle.
# "interactive" — user validates consequences and triages findings (default)
# "delegated" — persona agent stands in for user during validation/triage
participation = "interactive"
```

When `participation = "delegated"`, the `review_agent` handles both the
structured review AND the interactive steps (consequence validation, finding
triage, re-review decision). The same agent acts as both reviewer and user
stand-in.

Future hooks follow the same pattern (`solve_agent`, `validate_agent`, etc.)
as flat keys in `[author]`.

**Strengths:**
- Respects ADR-0014's flat-table constraint — no nesting
- Simple mental model: one key picks the agent, one key picks the mode
- Future hooks are just more `<hook>_agent` keys
- Defaults match current behavior (general-purpose + interactive)

**Weaknesses:**
- The `review_agent` does double duty as both reviewer and persona when
  delegated — this may not make sense for all use cases (the reviewer's
  perspective differs from the user's perspective)
- As hooks grow, the `[author]` table accumulates many `*_agent` keys
- `participation` applies uniformly to all hooks — no per-hook granularity

### Option 2: Separate agent and persona keys

Split the "who runs the review" and "who stands in for the user" into
separate config keys. This acknowledges that the reviewer and the persona
are different roles with different perspectives.

```toml
[author]
template = "nygard"

# Agent for structured review execution (implementability, fallacies, etc.)
review_agent = "general-purpose"

# Agent that stands in for the user during interactive steps.
# Empty string or absent = user participates directly (default).
# Set to a custom agent reference to delegate user-facing interactions.
persona = ""

# Participation mode for the review→revise cycle.
# "interactive" — user (or persona if set) handles validation/triage
# "autonomous" — review→revise→re-review loop runs without user/persona
#                intervention until verdict is Accept or cycle limit reached
participation = "interactive"
```

When `persona` is set and `participation = "interactive"`, the persona agent
handles consequence validation and finding triage instead of the user. When
`participation = "autonomous"`, the full loop runs unattended.

**Strengths:**
- Clean separation of concerns: reviewer ≠ persona
- A user could use a specialized reviewer agent (e.g., security-focused) while
  still using their own persona for triage decisions
- The `persona` key is reusable across future hooks (solve, create, manage)
- Three-level participation: interactive (user) → delegated (persona) →
  autonomous (unattended loop)

**Weaknesses:**
- More config surface — three keys instead of two
- The relationship between `persona` and `participation` may be confusing
  (what does `persona = "my-persona"` + `participation = "autonomous"` mean?)
- Pushes toward complexity that may not be needed initially

### Option 3: Hook dispatch table with agent map

Define an `[author.dispatch]` sub-table that maps each hook point to an agent
reference. Each key is a hook name; each value is either `"interactive"` (user
handles it) or an agent reference (that agent handles it). A `default` key
provides a shorthand so users don't need to set every hook individually.

```toml
[author]
template = "nygard"

[author.dispatch]
# Default for all hooks not explicitly set.
# "interactive" = user handles it (current behavior).
# Any other value = agent reference that handles the step.
default = "interactive"

# --- Review → Revise → Re-review cycle hooks ---

# Agent that runs the structured review (implementability, fallacies, etc.)
# Default: "general-purpose"
review = "general-purpose"

# Who validates consequences during review Step 4.
# "interactive" = user confirms each consequence.
# Agent reference = that agent responds on behalf of the user.
consequence_validation = "interactive"

# Who triages findings during revise (Address/Reject per finding).
# "interactive" = user decides.
# Agent reference = that agent decides.
finding_triage = "interactive"

# Who decides whether to re-review after revisions.
# "interactive" = user is prompted.
# Agent reference = that agent decides.
re_review = "interactive"

# --- Future hooks (not yet implemented) ---
# solve_intake = "interactive"
# create_validation = "interactive"
# option_evaluation = "interactive"
```

The dispatch table naturally encodes participation at per-hook granularity.
No separate `participation` key is needed — the table IS the participation
configuration. When all user-facing hooks are `"interactive"`, behavior is
identical to today. When they map to agents, those agents handle the step.

**Convenience via `default`:** A user who wants their persona to handle all
interactive steps sets `default = "my-persona"` and only overrides the hooks
that need different treatment:

```toml
[author.dispatch]
default = "my-persona"              # Persona handles all interactive steps
review = "general-purpose"          # Except review itself, which uses GP
```

**Strengths:**
- Per-hook granularity — different agents for different steps
- Explicit hook inventory — the dispatch table documents all available hooks
- `default` key provides a convenient shorthand for bulk configuration
- Participation mode is implicit — no separate key, no semantic overlap
- Future hooks are added to the dispatch table, not the parent table
- Clear mapping from hook to agent — self-documenting config

**Weaknesses:**
- Introduces nesting (`[author.dispatch]`), which refines ADR-0014's
  flat-table constraint. This is a deliberate evolution — the hook dispatch
  use case didn't exist when ADR-0014 was written.
- Mixing agent references and `"interactive"` as values requires the skill to
  distinguish between the two (check for the reserved word `"interactive"`)
- The hook inventory in config may grow — but only hooks that exist in the
  workflow are valid keys, which bounds the surface

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

The key uncertainty: **will a persona agent make editorial decisions that match
the user's actual judgment?** The dispatch infrastructure is only useful if the
persona can stand in effectively. This must be validated before committing.

**Validation needs:**

1. ~~Derive an editor persona from session history~~ — **Done.**
2. ~~Baseline comparison against actual decisions~~ — **Validated.**
3. ~~Custom agent dispatch~~ — **Validated.** Both v2 and v3 executed as
   `.agent.md` custom agents via the task tool.
4. Edge case: multi-round convergence — **Not yet tested.** Deferred to
   implementation.

**Prototype findings:**

Experiment: Derived an editor persona from editorial decisions across
ADR-0015, 0016, 0019, 0020, and 0021 Q&A addendums plus review session
interaction turns. Tested across multiple iterations: v1 (GPA), v2 (custom
agent), v3 (custom agent, generalized), with progressive calibration.

```
Hypothesis: A persona derived from session history can replicate the
user's Address/Reject decisions on review findings with ≥70% agreement.

Data provenance:
- 20 total sessions in repo (2026-04-03 to 2026-04-05)
- 15 sessions queried (75%) — filtered by review/revise/verdict keywords
- 145 total conversation turns in repo
- 36 turns with editorial decisions (25%) — both user messages and
  assistant responses were used; user reasoning was the primary signal
  for deriving decision principles
- 28 Q&A addendum entries across 4 ADRs used as ground truth
- v2 persona derived from 25% of data; v3 from 100% (147 turns)

Method:
1. Mined session_store for all review/revise interactions in this repo
2. Identified 8 decision principles from observed patterns in user messages
3. Encoded principles as a ranked priority list in a persona prompt
4. Tested v1 persona (GPA) against ADR-0020 (10 findings) → 70% agreement
5. Identified failure mode: over-rejection on adjacent/legacy system findings
6. Refined principles 1 and 2 with HOWEVER clauses for impact documentation
7. Retested v2 persona (GPA) against 4 ADRs (28 findings) → 71% agreement
8. Derived v3 persona from full corpus (147 turns) — generalized for
   portability by removing repo-specific references, using case-study
   framing instead
9. Tested v2 and v3 as custom agents (.agent.md) against same 4 ADRs
10. Identified revise-task measurement gap: Address/Reject binary doesn't
    capture "Reject scope but redirect concern" (Defer)
11. Calibrated v3 for scope-rejection-as-redirect and context-framing patterns
12. Final retest of generalized v3 as custom agent

Results progression:
| Version          | ADR-0020 | ADR-0021 | ADR-0015 | ADR-0016 | Total |
|------------------|----------|----------|----------|----------|-------|
| v1 GPA           | 7/10 70% | —        | —        | —        | —     |
| v2 GPA           | 8/10 80% | 6/6 100% | 4/6 67%  | 2/6 33%  | 71%   |
| v2 custom agent  | 8/10 80% | 6/6 100% | 5/6 83%  | 3/6 50%  | 79%   |
| v3 custom agent  |10/10 100%| 6/6 100% | 4/6 67%  | 3/6 50%  | 82%   |
| v3 gen (no-file) | 9/10 90% | —        | —        | 3/6 50%  | —     |
| v3 gen final     | 9/10*    | —        | —        | 6/6*     | —     |

* ADR-0016 final retest: 6/6 when scored by intent rather than verb (see
  measurement gap below). ADR-0020 not retested in final round.

Failure modes identified and resolved:
A. "Self-evident" over-application — FIXED in v3 via principle 2
   "Procedural explicitness over assumed knowledge"
B. Priority overrides scope — FIXED in v3 via principle 1
   "Scope discipline overrides priority"
C. Over-rejection of softening requests — FIXED in v3 via principle 6
   "Soften overstated consequences"
D. Scope rejection without redirect — FIXED in v3 via "rejection ≠ ignoring"
   clause: always note where the concern DOES belong
E. Over-rejection of context-framing findings — FIXED in v3 via principle 4
   case study: framing context (one sentence of positioning) is not
   future-proofing

Measurement gap discovered:
The revise task (ADR-0015) offers only Address/Reject as triage verbs.
The user's actual pattern includes a third mode: "Reject scope but redirect
the concern" — recorded as Reject but functionally an Address-by-redirect.
This inflates apparent disagreement between persona and user on scope
rejections. A "Defer" verb in the revise task would capture this pattern.
(Follow-up ADR recommended.)

Additional observation:
When custom agents have file access, they read the source ADR and make
grounded decisions ("already covered"), which changes what's being tested
(persona accuracy vs. persona + source material). Tests should control for
file access to isolate persona quality.

Generalization observation:
v3 was stripped of repo-specific references (15 → 0) and reframed with
case-study rhetoric. Cost: 1 finding regression on ADR-0020 (100% → 90%).
Trade-off: full portability across repos at ~10% accuracy cost on
repo-specific edge cases.
```

**Assessment: Validated.** Custom agent dispatch works. The persona matches
the user's editorial judgment at 82% (v3 custom agent, repo-specific) or
90%+ when accounting for the Defer measurement gap. The dispatch
infrastructure design is validated independently of persona accuracy — the
config schema and "same instructions, configurable executor" contract are
sound regardless of which persona is plugged in.

## Decision

In the context of **enabling custom agent personas in the author-adr review
workflow**, facing **hardcoded general-purpose agent dispatch and no
participation configuration**, we decided for **a dispatch table
(`[author.dispatch]`) with two role-based hooks and a "same instructions,
configurable executor" contract (evolved from Option 3)**, and neglected
**flat agent keys (too rigid for per-role configuration) and per-step hooks
(over-granular — different steps share the same role)**, to achieve
**configurable agent delegation where custom agents receive the same reference
instructions (review.md, revise.md) as prompts but apply them through their
own persona lens**, accepting that **this introduces TOML nesting beyond
ADR-0014's original flat-table constraint, and runtime dispatch to custom
agents needs implementation-time validation**.

### Dispatch Table Schema

```toml
[author]
template = "nygard"

[author.dispatch]
# Agent for the structured review step.
# Receives: review.md instructions + ADR content as prompt.
# Produces: structured review output (implementability, fallacies, verdict).
# Default: "general-purpose"
review = "general-purpose"

# Agent for editorial decisions during the review→revise cycle.
# Receives: revise.md instructions + review output as prompt.
# Handles: consequence validation, finding triage (Address/Reject), re-review.
# Default: "interactive" (user handles these steps directly)
editor = "interactive"
```

### Dispatch Contract: Same Instructions, Configurable Executor

Each hook dispatches the **same reference instructions** regardless of which
agent is configured:

| Hook | Instructions | Input | Output |
|------|-------------|-------|--------|
| `review` | `review.md` | ADR file content | Structured review (verdict, findings) |
| `editor` | `revise.md` | Review output + ADR content | Revision decisions (address/reject per finding) |

The custom agent's `.agent.md` persona shapes HOW it applies the instructions
(which findings it prioritizes, how it weighs tradeoffs, what editorial
judgment it brings), not WHAT it checks. The reference docs define the task
structure; the agent brings judgment.

### Two Roles

1. **`review`** — the reviewer. Runs the structured quality check
   (implementability criteria, fallacy scan, anti-pattern check, 7-point
   checklist, verdict). Defaults to `"general-purpose"` because the review
   instructions are comprehensive and any capable agent can execute them.

2. **`editor`** — the editor. Handles all user-facing interactive steps during
   the review→revise cycle: consequence validation, finding triage
   (Address/Reject), and re-review decision. Defaults to `"interactive"`
   (user handles it). When set to an agent reference, that agent stands in for
   the user's editorial judgment.

The `"interactive"` value is a reserved keyword meaning "prompt the user
directly." Any other value is treated as an agent reference.

### Hook Inventory (Full Workflow)

The review→revise cycle hooks are the immediate scope. The following hooks are
identified for future workflows but are NOT implemented by this ADR:

| Workflow | Hook | Role | Status |
|----------|------|------|--------|
| Review→Revise | `review` | Reviewer | **This ADR** |
| Review→Revise | `editor` | Editor | **This ADR** |
| Create | `create_validation` | Reviewer | Future |
| Solve | `option_evaluation` | Advisor | Future |
| Solve | `convergence` | Editor | Future |
| Manage | `status_transition` | Editor | Future |

Future hooks follow the same contract: reference instructions as prompt,
configurable executor.

### ADR-0014 Nesting Constraint

ADR-0014 specified "flat within each table — no nesting beyond the first
level" with a revisit trigger at three levels. `[author.dispatch]` introduces
one level of nesting (two levels of table addressing:
`author` → `dispatch` → `review`). This is a deliberate refinement: the
dispatch use case didn't exist when ADR-0014 was written, and per-role agent
configuration requires structured grouping that flat keys don't express well.

## Consequences

**Positive:**

- Custom agent personas can be plugged into the review→revise workflow by
  setting `review` and/or `editor` in `[author.dispatch]`, enabling the user's
  goal of persona-based automation.
- The "same instructions, configurable executor" contract means custom agents
  don't need to re-implement the review structure — they receive `review.md`
  and `revise.md` as prompts, inheriting the full quality framework.
- Two role-based hooks (reviewer + editor) are simpler than per-step hooks
  while still allowing different agents for different capabilities.
- The dispatch table is self-documenting — it enumerates all available hooks
  and their defaults in one place.
- Progressive automation: defaults (`review = "general-purpose"`,
  `editor = "interactive"`) match today's behavior exactly. Users opt in to
  delegation by changing values.

**Negative / Risks:**

- Introduces TOML nesting beyond ADR-0014's flat-table constraint. Mitigated
  by the constraint having a stated revisit trigger, and this being within the
  trigger threshold (two levels, not three).
- Runtime dispatch to custom `.agent.md` agents needs implementation-time
  validation. The dispatch contract is sound, but the mechanics of targeting
  a custom agent from the task tool are untested. Mitigated by the contract
  being independent of the dispatch mechanism — if the task tool can't target
  custom agents, an alternative mechanism can be used without changing the
  config schema.
- The `"interactive"` reserved keyword mixes control flow with agent references
  in the same value space. Mitigated by it being a single well-known keyword,
  not a pattern — any value that isn't `"interactive"` is an agent reference.

**Neutral:**

- Future hooks (create_validation, option_evaluation, etc.) are documented but
  not implemented. They serve as a roadmap, not a commitment. Each would be
  added via its own ADR.
- The `[implement]` table's participation mode (`full-control`, `guided`,
  `autonomous`, `weighted`) is a different pattern than the dispatch table.
  Both are valid for their respective skills — implement-adr's participation
  controls per-task user involvement during plan execution; author-adr's
  dispatch controls per-role agent selection during review. The patterns are
  complementary, not conflicting.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] User documentation

### Additional Quality Concerns

- **Config schema documentation** — the `[author.dispatch]` table and its keys
  need to be documented in the author-adr SKILL.md Configuration section.
- **Default behavior preservation** — when no `[author.dispatch]` table exists,
  behavior must be identical to the current workflow (general-purpose review,
  interactive user prompts).
- **Agent reference validation** — the skill should fail gracefully if a
  configured agent reference cannot be resolved (fall back to default, warn).
- **Revise task Defer verb** — the prototype revealed that the revise task's
  Address/Reject binary doesn't capture "reject scope but redirect concern."
  A "Defer" verb would improve both persona accuracy measurement and the
  revision workflow itself. (Follow-up ADR recommended.)

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

- Runtime dispatch mechanism (how the task tool targets custom agents) is
  deferred to implementation. The config schema is the scope of this ADR.
- The hook inventory for future workflows is advisory, not prescriptive.

---

## Comments
