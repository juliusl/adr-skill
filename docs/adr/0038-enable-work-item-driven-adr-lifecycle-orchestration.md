# 38. Enable work-item-driven ADR lifecycle orchestration

Date: 2026-04-05
Status: Proposed
Last Updated: 2026-04-05
Links:
- [ADR-0034: Work-item-referenced naming](0034-adopt-work-item-referenced-naming-for-adr-files.md)
- [ADR-0035: Normalized work item data model](0035-define-normalized-work-item-data-model-for-vendor-agnostic-adr-tooling.md)
- [ADR-0036: Cache work item snapshots](0036-cache-work-item-snapshots-in-adr-var-directory.md)
- [ADR-0037: Delivered status and Deliverables section](0037-add-delivered-status-and-deliverables-section-to-wi-nygard-agent-template.md)
- [ADR-0018: Unified format-based scripts](0018-replace-adr-tools-and-madr-tools-with-unified-format-based-scripts.md)

## Context

The wi-nygard-agent format (ADRs 0034–0037) establishes a structural link between ADRs and work items: each ADR is named after its motivating work item, the work item data is normalized and cached locally, and the ADR lifecycle extends through `Delivered` with a Deliverables section tracking outcomes.

**Today's workflow is manual.** A developer must:
1. Create a work item in their tracker (GitHub/ADO)
2. Manually run the author-adr skill to draft the ADR
3. Manually run review→revise cycles
4. Manually invoke implement-adr to plan and execute
5. Manually update the ADR status at each transition
6. Manually close the work item when implementation completes

Each of these transitions requires the developer to remember the next step and invoke the right tool. There is no feedback loop between the work item system and the ADR tooling — they operate as disconnected systems that happen to share an ID.

**The opportunity:** Since the wi-nygard-agent format already ties ADRs to work items, the work item's state can drive the ADR lifecycle. When a work item transitions (e.g., from "New" to "Active"), the orchestrator knows which ADR is affected and what action to take. This transforms the work item from a passive reference into an active control plane for the decision lifecycle.

**What this enables:**
- A tech lead creates a GitHub issue describing an architectural problem. An agent or developer creates the ADR using the wi-nygard-agent format. When the ADR is reviewed and proposed, the issue is labeled `proposed`. When the decision is accepted and the work item reaches `resolved`, the orchestrator triggers `implement-adr` automatically. When deliverables are verified, the ADR transitions to `Delivered` and the issue is closed.
- In an ADO environment, a User Story linked to an ADR follows the same pattern: New → Active → Resolved → Closed maps to Prototype → Proposed → Accepted → Delivered.

### Decision Drivers

- **Reduce manual orchestration** — transitions between ADR lifecycle stages should not require the developer to remember and invoke the next tool
- **Bidirectional sync** — changes in the work item system should be reflected in the ADR, and vice versa
- **Vendor agnostic** — the orchestration model must work with GitHub, ADO, and local workflows using the normalized data model (ADR-0035)
- **Opt-in** — orchestration must be additive; developers who prefer manual workflows must not be forced into automation
- **Observable** — each automated transition should be logged so developers can understand what happened and why

## Options

### Option 1: State mapping table with manual trigger

Define a mapping between work item states and ADR statuses. When a developer invokes a `sync` command, the orchestrator reads the work item's current state, computes the target ADR status, and performs the transition (including invoking implement-adr if the transition is to Accepted). Sync is manual — the developer runs it when ready.

```
Work Item State → ADR Status → Action
─────────────────────────────────────────
open              Prototype     (no action)
active            Proposed      (no action — authoring in progress)
resolved          Accepted      trigger: implement-adr
closed            Delivered     trigger: verify deliverables
```

**Strengths:**
- Developer retains control — sync is explicit, not automatic
- Simple to implement — one command that reads state and performs actions
- Predictable — no background processes or webhooks
- Works offline with cached data (ADR-0036) for state lookup

**Weaknesses:**
- Still manual — developer must remember to run sync
- One-directional — sync reads work item state but doesn't update it
- No real-time responsiveness — state changes aren't detected until sync runs

### Option 2: Bidirectional state mapping with event-driven triggers

Define the state mapping table (same as Option 1) but add event-driven triggers. The orchestrator watches for state changes in both directions:
- **Work item → ADR:** When the work item state changes (detected via MCP server polling or webhook), update the ADR status and trigger the appropriate action.
- **ADR → Work item:** When the ADR status changes (detected via file watcher or explicit command), update the work item in the source system.

**Strengths:**
- Fully automated — no manual sync needed
- Bidirectional — keeps both systems in sync
- Real-time — state changes propagate immediately (with polling) or near-instantly (with webhooks)

**Weaknesses:**
- Complex — requires polling loops, webhook handlers, or file watchers
- Failure modes — network errors, stale state, race conditions between manual and automated changes
- Over-engineered for the current use case — agent sessions are interactive, not long-running daemons
- Remote-specific webhook setup required — GitHub and ADO have different event systems
- Risk of automation loops — ADR change triggers work item update triggers ADR change

### Option 3: Orchestrator command with lifecycle-aware dispatch

Extend the format script with a `lifecycle` subcommand that serves as the entry point for state-driven actions. The developer invokes `lifecycle` with a remote and ID; the orchestrator:
1. Reads the cached work item state (ADR-0036)
2. Reads the current ADR status
3. Computes the next action based on the state mapping
4. Presents the recommended action to the developer (or executes it if `--auto` is passed)

The lifecycle command is the integration point — it doesn't watch for changes, but it knows the full state mapping and can recommend or perform the next step. It can also be called by downstream tools (e.g., a CI pipeline, a GitHub Action, or a developer alias).

```bash
# Interactive: recommend next action
wi-nygard-agent-format.sh lifecycle gh 42
# → ADR status: Proposed, Work item: accepted
# → Recommended: transition to Accepted, run implement-adr
# → Proceed? [y/N]

# Automated: execute without prompting
wi-nygard-agent-format.sh lifecycle gh 42 --auto
```

**Strengths:**
- Lifecycle-aware — understands the full state mapping and can recommend the right action
- Flexible — interactive by default, automatable with `--auto` flag
- Composable — callable from CLI, CI, or other tools
- Fits the format script architecture — one new subcommand, no daemons or watchers
- Observable — logs each transition with reason and trigger source
- Vendor agnostic — uses normalized state from cached work items (ADR-0035/0036)

**Weaknesses:**
- Still requires invocation — someone must call `lifecycle` (manually, from CI, or via an alias)
- Not real-time — state changes aren't detected until the command runs
- The `--auto` flag could perform destructive actions (e.g., triggering implement-adr on a premature acceptance)
- State mapping logic lives in the format script — if the lifecycle grows complex, it may outgrow a shell script

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

In the context of **reducing manual orchestration in work-item-driven ADR workflows**, facing **the gap between work item state changes and ADR lifecycle transitions**, we decided for **a lifecycle subcommand with state mapping and interactive/auto dispatch** (Option 3), and neglected **manual sync (Option 1, still requires the developer to remember the step) and event-driven bidirectional sync (Option 2, over-engineered for interactive agent sessions)**, to achieve **lifecycle-aware orchestration that recommends or performs the next action based on work item and ADR state**, accepting that **the command must still be invoked (not automatic) and the `--auto` flag requires trust in the state mapping**.

### State Mapping

| Work Item State | ADR Status | Trigger Action |
|----------------|-----------|----------------|
| `open` | `Prototype` | — (authoring in progress) |
| `active` | `Proposed` | — (review in progress) |
| `resolved` | `Accepted` | Invoke `implement-adr` |
| `closed` | `Delivered` | Verify Deliverables checklist (ADR-0037) |

The mapping uses normalized work item states from ADR-0035. Remote-specific states (e.g., ADO's "New" vs GitHub's "open") are already normalized by the adapter layer.

### ADR → Work Item Sync

When the ADR status changes (via the `status` subcommand), the lifecycle command can optionally update the work item in the source system:

| ADR Status Change | Work Item Update |
|------------------|-----------------|
| → `Proposed` | Transition to `active`; add label/tag: `proposed` |
| → `Accepted` | Transition to `resolved` |
| → `Delivered` | Transition to `closed` |

This is opt-in — the `--sync` flag enables work item updates. Without it, the ADR status changes locally only.

### Lifecycle Subcommand

Added to `wi-nygard-agent-format.sh`:

```bash
case "$1" in
  lifecycle) shift; cmd_lifecycle "$@" ;;
  ...
esac
```

**Interface:**
```
wi-nygard-agent-format.sh lifecycle <remote> <id> [--auto] [--sync]
```

| Flag | Behavior |
|------|----------|
| (none) | Interactive — show recommended action, ask for confirmation |
| `--auto` | Execute the recommended action without prompting |
| `--sync` | After ADR status change, update the work item in the source system |

### Observability

Each lifecycle transition is logged to `.adr/var/lifecycle.jsonl` (following the ADR-0020 convention):

```json
{"remote":"gh","id":"42","from_status":"Proposed","to_status":"Accepted","action":"implement-adr","trigger":"lifecycle --auto","timestamp":"2026-04-05T08:00:00Z"}
```

This provides an audit trail of automated transitions, answering "who changed this status and why?"

### CI/CD Integration

The lifecycle command is composable — it can be called from CI pipelines:

```yaml
# GitHub Actions example
- name: Check ADR lifecycle
  run: |
    wi-nygard-agent-format.sh lifecycle gh ${{ github.event.issue.number }} --auto --sync
```

This enables: issue state change → CI triggers lifecycle → implement-adr runs → deliverables verified → issue closed. The full automation chain is possible but each step is independently controllable.

## Consequences

**Positive:**
- Developers no longer need to remember the next step in the ADR lifecycle — the lifecycle command reads both the work item state and ADR status and recommends (or performs) the appropriate action.
- The state mapping creates a clear, documented correspondence between work item states and ADR statuses, making the lifecycle predictable and auditable.
- Interactive by default, automatable with `--auto` — teams choose their comfort level with automation. Conservative teams use interactive mode; mature teams enable `--auto` in CI.
- The lifecycle log (`.adr/var/lifecycle.jsonl`) provides observability into automated transitions, supporting debugging and retrospectives.
- Composable with CI/CD — the command can be triggered by issue state changes in GitHub Actions or ADO Pipelines, enabling end-to-end automation from work item to delivered code.

**Negative:**
- The lifecycle command must still be invoked — it doesn't watch for changes. Teams wanting real-time automation must wire it into CI/CD triggers, which requires per-remote setup (GitHub Actions vs ADO Pipelines).
- The `--auto` flag performs actions (like invoking implement-adr) without confirmation. A premature state change in the work item could trigger unintended implementation. The mitigation is observability — the lifecycle log captures every automated action.
- State mapping logic adds complexity to the format script. If the lifecycle grows beyond the four-state mapping (e.g., adding approval gates, parallel implementation tracks), the shell script may need to be refactored into a more capable runtime.
- The `--sync` flag modifies external systems (closing issues, updating work items). Network errors during sync could leave the ADR and work item in inconsistent states. The intended mitigation is idempotent re-execution — re-running `lifecycle` recalculates the correct action regardless of prior failure. Idempotency must be a design requirement of the lifecycle command, not an assumption about the underlying system.

**Neutral:**
- This ADR defines the orchestration model. The specific CI/CD pipeline configurations for GitHub Actions and ADO Pipelines are implementation details, not architectural decisions.
- The lifecycle log uses the same `.adr/var/` convention as work item caching (ADR-0036), maintaining consistency in the project-scoped data model.
- The `implement-adr` skill is the primary action triggered by the lifecycle command. No changes to implement-adr's interface are expected — the lifecycle command invokes it as an external tool via CLI, not as a library call. This assumption should be verified during implementation; if implement-adr requires additional context (e.g., work item metadata) that the lifecycle command doesn't currently pass, the interface may need extension.
- This ADR depends on the foundation ADRs (0034–0037) which are currently `Proposed`. Implementation of the lifecycle command is sequenced after those decisions are accepted and their tooling is implemented — the lifecycle command consumes the normalized data model (ADR-0035), cached snapshots (ADR-0036), and the Deliverables section (ADR-0037).

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [ ] Backwards Compatible
- [x] Integration tests
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

Integration tests are needed for the lifecycle command's interaction with MCP servers (GitHub and ADO). The `--sync` flag modifies external systems and must be tested against sandbox environments or mock servers. The `--auto` flag's behavior should be tested with edge cases (e.g., work item in unexpected state, ADR status already matches, missing cached data). User documentation must clearly explain the opt-in nature of `--auto` and `--sync` to prevent unintended automation.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This is the second of two companion ADRs extending the wi-nygard-agent format. ADR-0037 (Delivered status and Deliverables section) provides the template foundation. This ADR builds the orchestration layer that uses the status lifecycle and deliverables data.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
With work-item-referenced naming (ADR-0034) and status tracking (ADR-0037), the next step is using work item state changes to drive ADR lifecycle transitions automatically. A lifecycle command bridges work item status and ADR status.

**Tolerance:**
- Risk: Medium — introduces automation that could cause unintended status transitions
- Change: High — new command and orchestration layer
- Improvisation: Medium — open to different trigger mechanisms within the format dispatch architecture

**Uncertainty:**
- Certain: work item systems have state machines; ADRs have statuses; bridging them is valuable
- Uncertain: how to handle conflicts between work item state and ADR state; idempotency requirements; CI/CD integration patterns

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Lifecycle command in format script with manual/CI invocation
- Event-driven webhook integration per remote
- Status polling with reconciliation loop

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Does the state mapping table use ADR-0035's actual normalized states?

**Addressed** — The state mapping tables (Option 1 illustration, Decision section, and ADR → Work Item Sync table) used informal states (`new`, `accepted/approved`) not present in ADR-0035's normalized model (`open | active | resolved | closed`). Replaced all mapping entries with the four normalized states: `open` → Prototype, `active` → Proposed, `resolved` → Accepted, `closed` → Delivered. Fixed the reverse sync table to match. Updated the Context narrative to remove the reference to a GitHub issue "moving to `accepted`" (which isn't a valid state in either GitHub or the normalized model).

### Q: Is idempotency a verified property or an unverified design assumption?

**Addressed** — Reworded the N4 consequence from asserting idempotency as an existing property ("The mitigation is idempotency") to framing it as a design requirement: "The intended mitigation is idempotent re-execution... Idempotency must be a design requirement of the lifecycle command, not an assumption about the underlying system."

### Q: Does the ADR acknowledge its dependency on the still-Proposed foundation ADRs?

**Addressed** — Added a neutral consequence documenting the sequencing dependency: implementation of the lifecycle command is sequenced after ADRs 0034–0037 are accepted and their tooling is implemented. The consequence lists the specific dependencies (normalized data model, cached snapshots, Deliverables section).

### Q: Has the claim that implement-adr requires no interface changes been verified?

**Addressed** — Qualified the claim from "No changes to implement-adr's interface are required" to "No changes are expected" with explicit acknowledgment that this is an assumption based on CLI invocation (not library call) and should be verified during implementation. Added a note that the interface may need extension if implement-adr requires work item metadata the lifecycle command doesn't currently pass.
