# 23. Add prototype-adr skill for structured decision validation

Date: 2026-04-03
Status: Proposed
Last Updated: 2026-04-03
Links:

## Context

The ADR skills ecosystem currently has two skills: `author-adr` (create, review, revise, manage decisions) and `implement-adr` (turn decisions into code). There is a gap between deciding and implementing: **prototyping**. When an ADR's Experimentation Tolerance assessment (per ADR-0022) identifies a decision that "needs validation" or is "deliberately experimental," there is no structured way to run that experiment, observe the results, and feed findings back into the ADR.

**The problem:** There is no skill for running controlled prototypes of architectural decisions and capturing the results in a way that informs the ADR lifecycle.

**Why this matters:**
- ADR-0022 introduces Experimentation Tolerance as a review criterion, but the review can only flag "this needs validation" — it can't execute the validation itself. A `prototype-adr` skill would close that loop.
- The `author-adr` solve workflow (ADR-0019) includes an "optional prototyping" step (Step 4), but it's unstructured — it says "create targeted spikes" without providing tooling, isolation, or observation infrastructure.
- ADR-0022's validation of new review criteria (comparing ecADR vs. implementability criteria side-by-side) is itself a prototyping problem — the A/B comparison workflow needs a structured home.

**What prototyping needs that implementation doesn't:**
- **Isolation** — prototypes should not pollute the main project. They need their own environment (branch, container, temp directory) where experiments can be messy without consequence.
- **Observability** — the point of prototyping is to learn. Results (measurements, logs, behavioral observations) must be captured, not lost with the prototype environment.
- **Disposability** — prototypes are designed to be thrown away. The artifact is the learning, not the code.
- **Adaptability** — projects and environments vary wildly. A prototype for a database schema is very different from one for an auth flow that requires live services. The skill must adapt to what the project can support.

**Open design tensions:**

1. **Closed vs. open systems.** Container-based prototyping offers the best isolation and reproducibility — you control the full environment. But some experiments require interaction with external systems (auth providers, APIs, cloud services) that can't be containerized. These need user intervention and can't be fully autonomous.

2. **Spec independence vs. platform leverage.** The existing skills follow the agentskills.io spec, which is platform-agnostic. But agent CLIs that support ACP (Agent Communication Protocol) could provide richer prototyping capabilities — sub-agent orchestration, environment management, structured observation. Using ACP would create a dependency on platforms like Copilot CLI. This could be optional (configuration-driven) rather than required.

3. **Recipe-driven vs. agent-directed.** Should the skill define prototyping recipes (like cloud-init profiles) that the user selects and parameterizes? Or should the agent assess the ADR and propose a prototyping approach? Probably both — profiles for common patterns, agent discretion for novel situations.

4. **Where does direction come from?** The `author-adr` skill could plant prototyping hints in the ADR itself — in the Options section (per-option "how to validate"), the AQC section (what to test), or a new section. This keeps the ADR as the single source of truth while giving `prototype-adr` high-level direction without prescribing execution details.

### Decision Drivers

- **Isolation-first** — prototypes must not contaminate the project's working tree, git history, or runtime state
- **Observable outcomes** — prototype results must be captured as structured data that feeds back into the ADR lifecycle
- **Environment-adaptive** — the skill must handle both closed-system (containerized, fully autonomous) and open-system (external dependencies, user intervention) scenarios
- **Platform-optional** — ACP/platform-specific features should be opt-in, not required; the skill must work with agentskills.io alone as a baseline
- **Profile-driven** — common prototyping patterns should be codified as reusable profiles, similar to cloud-init, reducing setup friction
- **ADR-anchored** — prototyping direction comes from the ADR itself; the skill reads cues from the decision record rather than requiring separate orchestration

## Options

### Option 1: Profile-driven prototyping with container-first isolation

The skill reads a prototyping profile (a declarative file similar to cloud-init) that specifies the environment, setup steps, validation commands, and observation hooks. Profiles are stored in `.adr/profiles/` or alongside the ADR. The default execution backend is containers (e.g., `docker run` with a mounted workspace), with a fallback to git-worktree or temp-directory isolation for environments without container support.

For open-system scenarios (auth, external APIs), profiles declare `requires: user-intervention` which switches the skill from autonomous to interactive mode — the agent sets up the environment but pauses for user action at defined checkpoints.

**Example profile:**
```yaml
# .adr/profiles/schema-migration.yaml
name: schema-migration
isolation: container
image: postgres:16
setup:
  - createdb prototype_db
  - psql -f schema.sql prototype_db
validate:
  - psql -c "SELECT count(*) FROM migrations" prototype_db
observe:
  - query_latency_ms
  - row_counts
teardown: automatic
```

**Strengths:**
- Declarative — profiles are reviewable, shareable, and versionable
- Container isolation is the strongest guarantee of non-contamination
- Cloud-init analogy is well-understood by infrastructure engineers
- Profiles can be templated and reused across projects

**Weaknesses:**
- Container dependency limits where the skill works (CI-only in some orgs, unavailable in some agent runtimes)
- Profile authoring requires domain knowledge — the agent can propose but the user must validate
- YAML-based profiles are yet another configuration format to maintain
- Doesn't leverage agent-specific capabilities (ACP, sub-agents)

### Option 2: Agent-directed prototyping with optional ACP orchestration

The skill reads the ADR's Options and AQC sections to infer what needs prototyping, then proposes an approach. Isolation defaults to git-worktree (lightweight, no container dependency). If the agent runtime supports ACP, the skill can spawn sub-agents for parallel prototype execution, structured observation, and automatic result aggregation.

Without ACP, the skill operates in a simpler mode: single-threaded, sequential experiments, manual observation. ACP is opt-in via `[prototype].runtime = "acp"` in `preferences.toml`.

**Strengths:**
- No container dependency — works anywhere git works
- ACP integration unlocks sophisticated orchestration (parallel experiments, sub-agent observation) when available
- Agent infers prototyping direction from the ADR — less upfront configuration
- Graceful degradation — works without ACP, better with it

**Weaknesses:**
- Git-worktree isolation is weaker than containers — shared filesystem, shared toolchain
- ACP dependency (even optional) ties the skill to specific platforms, diverging from agentskills.io's platform-agnostic stance
- Agent inference of "what to prototype" may be unreliable without explicit cues
- Two operational modes (with/without ACP) doubles the testing surface

### Option 3: Hybrid — profiles for environment, agent for execution

Combine profile-driven environment setup with agent-directed experiment execution. Profiles define the *where* (isolation backend, dependencies, teardown) while the agent decides the *what* and *how* (which experiments to run, what to observe, how to interpret results).

ACP is one of several isolation backends, alongside containers and git-worktree, selectable via the profile:

```yaml
isolation: container | worktree | acp-sandbox
```

The ADR provides high-level direction (Options section hints, AQC validation goals), profiles provide environment specifics, and the agent bridges the two.

**Strengths:**
- Separation of concerns — environment config (profiles) vs. experiment logic (agent)
- Multiple isolation backends without picking one as primary
- Profiles are optional — the agent can use sensible defaults if no profile exists
- ACP is just another backend, not a privileged dependency

**Weaknesses:**
- Most complex option — three layers (ADR cues, profiles, agent logic) to coordinate
- Profile format needs careful design to be simple enough for common cases but expressive enough for edge cases
- Risk of over-engineering for a skill that doesn't exist yet — YAGNI concern

## Decision

In the context of **needing structured prototyping for ADR validation**, facing **diverse environments ranging from fully containerizable to requiring external services and user intervention**, we decided for **a hybrid approach: profile-driven environment setup with agent-directed experiment execution (Option 3)**, and neglected **profile-only (no agent intelligence) and agent-only (no declarative environment control)**, to achieve **environment-adaptive prototyping that works across isolation backends while keeping the ADR as the source of direction**, accepting that **the design has three coordination layers and the profile format needs careful scoping to avoid over-engineering**.

### Skill Architecture

```
author-adr                          prototype-adr                    implement-adr
    │                                     │                               │
    ├─ Options section ──────────────────► reads validation hints          │
    ├─ AQC section ──────────────────────► reads validation goals          │
    │                                     │                               │
    │                              ┌──────┴──────┐                        │
    │                              │   Profile    │                        │
    │                              │  (env setup) │                        │
    │                              └──────┬──────┘                         │
    │                                     │                               │
    │                              ┌──────┴──────┐                        │
    │                              │    Agent     │                        │
    │                              │ (experiment) │                        │
    │                              └──────┬──────┘                         │
    │                                     │                               │
    ├─ Context updated with findings ◄────┘                               │
    │                                                                     │
    ├─ Review/Revise cycle ───────────────────────────────────────────────►│
    │                                                                     │
    └─ Prototype → Proposed → Planned ───────────────────────────────────►│
```

### Prototyping Profiles

Profiles are declarative YAML files stored in `.adr/profiles/` (per ADR-0020's project-scoped convention). They define environment setup, not experiment logic.

**Minimal profile:**
```yaml
name: default
isolation: worktree
teardown: automatic
```

**Container profile:**
```yaml
name: database-spike
isolation: container
image: postgres:16-alpine
setup:
  - createdb spike_db
observe:
  - format: jsonl
    output: stdout
teardown: automatic
```

**Open-system profile:**
```yaml
name: auth-flow
isolation: worktree
requires: user-intervention
checkpoints:
  - name: configure-oauth
    prompt: "Set up OAuth credentials and press continue"
  - name: verify-callback
    prompt: "Verify the callback URL works"
observe:
  - format: jsonl
    output: stdout
teardown: manual
```

### Isolation Backends

| Backend | Isolation Level | Dependencies | Best For |
|---------|----------------|--------------|----------|
| `worktree` | Git-level (shared filesystem) | `git` | Quick spikes, skill changes, config experiments |
| `container` | OS-level (full isolation) | Container runtime | Database schemas, service interactions, reproducible benchmarks |
| `acp-sandbox` | Agent-level (sub-agent) | ACP-compatible runtime | Parallel experiments, structured observation, A/B comparisons |

The `acp-sandbox` backend is opt-in and requires `[prototype].runtime = "acp"` in `preferences.toml`. Without it, the skill operates using `worktree` or `container` only, fully compatible with agentskills.io.

### ADR → Prototype Direction

The prototype skill is an **executor**, not a **decider**. Direction flows from the ADR, authored by `author-adr`:

**Author-adr's responsibility:** During authoring (solve or create workflow), capture what evidence or validations the decision needs. This is written into the ADR itself — in the AQC section, the Options section (per-option validation needs), or both. These are **prototype objectives**: concrete, testable statements about what the prototype should confirm or measure.

**Example prototype objectives (in an ADR's AQC section):**
```markdown
### Additional Quality Concerns

**Prototype Objectives:**
- Validate that awk can parse the pipe-delimited summary format across 5 real plan files
- Measure plan file growth for plans with 5, 10, and 15+ tasks
- Confirm extract-summary.awk produces valid JSONL (parseable by jq)
- Compare new review criteria findings vs. ecADR findings on ADRs 0020-0021
```

**Prototype-adr's responsibility:** Read the prototype objectives from the ADR, propose an execution plan (environment, isolation backend, steps), and run the experiments. The skill decides *how* to validate — which isolation backend, what tooling, what observation format — but not *what* to validate.

This keeps the ADR as the single source of truth for what the decision needs to prove, while the prototype skill handles the mechanics of proving it.

### Observation and Feedback

Prototype results are captured as structured observations and fed back into the ADR lifecycle:

1. **During prototyping** — the agent logs observations (measurements, behavioral notes, pass/fail results) as JSONL to stdout (consistent with ADR-0021's extraction pattern).
2. **After prototyping** — findings are summarized and appended to the ADR's Context section as new evidence, or used to update Options strengths/weaknesses.
3. **Status transition** — if the prototype validates the decision, the ADR can progress from `Prototype` → `Proposed`. If it invalidates, the ADR stays in `Prototype` for revision.

## Consequences

**Positive:**
- Closes the gap between deciding and implementing — experiments are structured, isolated, and observable rather than ad-hoc.
- The author-adr → prototype-adr contract is clean: author-adr declares *what* to validate (prototype objectives in the ADR), prototype-adr decides *how* to validate. The ADR remains the single source of truth.
- Profile-driven environment setup makes prototyping reproducible and shareable across team members.
- Multiple isolation backends (worktree, container, ACP) adapt to diverse project environments without forcing a single approach.
- Findings feed directly back into the ADR, enriching the decision record with empirical evidence.

**Negative:**
- Three-layer coordination (ADR cues, profiles, agent logic) adds design complexity — the profile format especially needs careful scoping.
- Container and ACP backends introduce optional dependencies that may not be available in all environments.
- The skill doesn't exist yet — this ADR is itself "deliberately experimental" per ADR-0022's Experimentation Tolerance spectrum.
- Profile format (YAML) adds another configuration surface beyond TOML preferences and Makefile targets.

**Neutral:**
- The skill is a new addition to the ecosystem, not a modification of existing skills. `author-adr` and `implement-adr` continue to work unchanged.
- Whether the skill follows agentskills.io spec or uses ACP is a deployment-time configuration choice, not a build-time dependency.
- The profile format and observation schema are intentionally under-specified in this ADR — they should be refined through prototyping the prototype skill itself (meta-prototyping).

## Quality Strategy

- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [ ] Backwards Compatible
- [ ] Integration tests
- [ ] User documentation

### Additional Quality Concerns

---

## Comments
