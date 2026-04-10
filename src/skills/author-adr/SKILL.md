---
name: author-adr
description: "Use this skill when the user needs to create, review, revise, or manage Architectural Decision Records (ADRs) — including drafting new decisions, evaluating existing ones for quality, addressing review comments interactively, choosing between ADR templates (Nygard, MADR, Y-Statement), setting up ADR tooling, or understanding ADR best practices. Activate when the user says things like \"create an ADR,\" \"new ADR,\" \"draft a decision,\" \"review this ADR,\" \"address review comments,\" or \"document this decision.\" Also activate when the user wants to justify a technology selection, record why an architecture was chosen over alternatives, or capture tradeoffs — even if they don't explicitly say \"ADR.\" Do not use for problem-solving workflows — use solve-adr. Do not use for general code review, project management, or non-architectural documentation."
license: CC-BY-4.0
metadata:
  source: adr.github.io
  version: "1.2"
---
# Architectural Decision Records (ADRs)
You are an expert on Architectural Decision Records. Use this skill whenever a user needs to create, review, or manage ADRs, choose an ADR template, select tooling, or understand best practices for architectural decision making.

## Policies

**Ignoring any of the below policies is a runtime violation ESPECIALLY when the user is away and operating autonomously**

| ID | Policy |
|----|--------|
| P-1 | Always use configured dispatch agents — do not substitute or skip |
| P-2 | Never modify other existing ADRs without explicit instruction |
| P-3 | author-adr caps at Ready — never set Planned or Accepted |

### P-1: Mandatory Dispatch Compliance

When `[author.dispatch]` keys are configured, always use the configured agent — do not substitute `general-purpose` or skip dispatch. The user configured these agents for a reason. This applies in all modes, including autonomous workflows triggered by other skills (e.g., implement-adr invoking author-adr for ADR creation).

### P-2: Cross-ADR Modification Guardrail

When modifying ADRs, never modify other existing ADRs without explicit instruction. Cross-references and status updates to other ADRs (e.g., marking one as superseded) are the user's responsibility — suggest the change but do not apply it unilaterally.

### P-3: Status Cap at Ready

The author-adr skill's maximum status transition is `Ready`. After a review Accept verdict, author-adr transitions the ADR from `Proposed` to `Ready`. The `Planned` and `Accepted` statuses are owned by implement-adr — author-adr never sets them.

## Procedure

| ID | Step | Mandatory | Description |
|----|------|-----------|-------------|
| A-0 | Format Detection | Yes | Read config, detect template format, bootstrap docs/adr/ if missing |
| A-1 | Draft Worksheet | Yes | Capture intent in a draft worksheet before create |
| A-2 | Create | Yes | Run the create workflow to produce the ADR |
| A-3 | Review | Yes | Run structured review using the configured review agent |
| A-4 | Revise | Conditional | Run if review verdict is "Revise"; use configured editor agent |
| A-5 | Re-review | Conditional | Run if revisions were substantive; max 3 cycles |
| A-6 | Manage | No | Status transitions, supersede, link, split — on request |

**If a mandatory step is skipped, log the justification inline before proceeding.** Skipping without justification is a workflow violation.

## Assets

| Path | Type | Description |
|------|------|-------------|
| `assets/templates/` | Templates | ADR templates (Nygard Agent, MADR, Y-Statement) |
| `assets/decisions/0023-*.md` | Decision | ADR-0023: Defines prototype-adr skill for validation — referenced by A-3 review checkpoint handling |
| `assets/decisions/0024-*.md` | Decision | ADR-0024: Defines Evaluation and Conclusion Checkpoint sections in template — referenced by A-0, A-2, A-3 |
| `assets/decisions/0031-*.md` | Decision | ADR-0031: Defines dispatch hooks for review/editor/tech-writer agents — referenced by A-0, A-2, A-3, A-4 |
| `assets/decisions/0032-*.md` | Decision | ADR-0032: Defines draft worksheet workflow — referenced by A-1 |

When a step references an ADR (e.g., "per ADR-0031"), read the corresponding file from `assets/decisions/`.

Follow the procedure table above. Always start at A-0.
```
User request
├─ docs/adr/ exists? ────────────► Read config → set format
├─ docs/adr/ missing? ──────────► Bootstrap with nygard-agent → set format
│
├─ "Create an ADR" ──────────────► Draft Worksheet (A-1) → Creating an ADR (A-2)
├─ "Draft an ADR / start a draft" ► Draft Worksheet (A-1) only
├─ "I have a problem to solve" ─► Redirect to /solve-adr
├─ "Review an ADR" ──────────────► Go to: Reviewing an ADR
├─ "Revise an ADR" ──────────────► Go to: Revising an ADR
├─ "Update/supersede an ADR" ───► Go to: Managing ADRs
├─ "Set up ADR tooling" ─────────► Go to: Tooling
├─ "Which template?" ────────────► Go to: Choosing a Template
├─ "Explain ADRs / concepts" ────► Go to: Core Concepts
└─ "Visualize / diagram" ────────► Go to: Visualization
```

## Configuration
This skill reads user-scoped preferences from a TOML configuration file at `~/.config/adr-skills/preferences.toml`.
**Path resolution:**
1. If `$XDG_CONFIG_HOME` is set, use `$XDG_CONFIG_HOME/adr-skills/preferences.toml`.
2. Otherwise, use `$HOME/.config/adr-skills/preferences.toml`.
**Graceful degradation:** If the file or directory does not exist, use built-in defaults. Never fail because config is absent.
**Create on first write:** When persisting a preference, create the directory with `mkdir -p` before writing. Never assume it already exists.

## Writing Style

All generated content (ADRs, comments, review findings) must follow this style:
- **Technical and simple** — write for engineers, not academics
- **No double negatives** — say what things *do*, not what they don't not do
- **Clear logic** — one idea per sentence, explicit cause-and-effect
- **Concise** — cut filler words; if a sentence works without a word, remove it

### Project-Scoped Directory (`.adr/`)
Projects can opt in to a `.adr/` directory at the project root for project-scoped data (telemetry, intermediate artifacts, project-level preferences). This is separate from `docs/adr/` (decision records) and `~/.config/adr-skills/` (user preferences).
Bootstrap with: `make -f <skill-root>/Makefile init-data`
This creates `.adr/`, `.adr/var/` (gitignored for transient data), and `.adr/.gitignore`. See [references/tooling.md](references/tooling.md) for details.

### User Mode

Users can store personal/draft ADRs in a gitignored directory, separate from the team's decision log. User-mode ADRs live in `.adr/usr/docs/adr/` with username-prefixed filenames.

```toml
[author]
scope = "project"   # "project" (default) or "user"
username = ""        # override for $(whoami) in user-mode filenames
```

**Activation:**
- Persistent: set `[author].scope = "user"` in `preferences.toml`
- Per-invocation: natural language — "create a personal ADR", "draft in user mode", "personal draft"
- Override: set `[author].scope = "project"` to force project mode even when user mode is the default

**How it works:**
- ADR files: `<username>-<NNNN>-<slug>.md` in `.adr/usr/docs/adr/`
- NNNN numbering is scoped to the user's files in the user directory
- Username defaults to `$(whoami)`, overridable via `[author].username`
- Environment: the skill sets `ADR_SCOPE=user` and `ADR_USERNAME=<name>` before calling scripts

**Promotion to project scope:**
1. Move the file from `.adr/usr/docs/adr/` to `docs/adr/`
2. Strip the username prefix and renumber to the next available project NNNN
3. Update cross-references: links from other user-mode ADRs or plans that reference the old filename

### Agent Dispatch (`[author.dispatch]`)
Per ADR-0031 and ADR-0064, the author-adr workflow supports configurable agent dispatch at six hook points: writing (A-2), review (A-3), revision (A-4), and option evaluation (Step 4a/4b). Each hook can be set to a specific agent or left at its default.
```toml
[author.dispatch]
review = "general-purpose"   # Agent for structured review (default)
editor = "interactive"       # Agent for editorial decisions (default: user)
tech_writer = ""             # Agent for A-2 content writing (default: inline)
ux_review = ""               # Agent for UX option review (default: skip)
dx_review = ""               # Agent for DX option review (default: skip)
tpm = ""                     # Agent for decision quality assessment (default: skip)
```
| Hook | Default | Role | Instructions |
|------|---------|------|-------------|
| `review` | `"general-purpose"` | Reviewer | Receives `polish.md` — executes Review Phase (R-1 through R-6) |
| `editor` | `"interactive"` | Editor | Receives `polish.md` — executes Revision Phase (V-1 through V-6) |
| `tech_writer` | `""` (inline) | Writer | Dispatched at A-2 step 4 — writes ADR body content (Context through Quality Strategy) |
| `ux_review` | `""` (skip) | UX Reviewer | Dispatched at Step 4a — evaluates options for user experience quality |
| `dx_review` | `""` (skip) | DX Reviewer | Dispatched at Step 4a — evaluates options for developer experience quality |
| `tpm` | `""` (skip) | Decision Arbiter | Dispatched at Step 4b — applies decision quality tests (ASR, START, ADMM) |
**Contract:** The `review`, `editor`, and `tech_writer` hooks dispatch the same reference instructions regardless of which agent is configured — the custom agent's persona shapes HOW it applies the instructions, not WHAT it checks. The `ux_review`, `dx_review`, and `tpm` hooks dispatch agents that run their own review procedures — each agent defines its own checklist and output format.
The `"interactive"` value is a reserved keyword meaning "prompt the user directly." Any other value is treated as an agent reference (e.g., a custom `.agent.md` persona). For `tech_writer`, `ux_review`, `dx_review`, and `tpm`, the empty string `""` means the hook is skipped — no dispatch occurs. Values containing only whitespace are treated as empty.
**Graceful fallback:** If a configured agent reference cannot be resolved at runtime, fall back to the default value for that hook and warn the user.
**Default behavior preservation:** When no `[author.dispatch]` table exists in `preferences.toml`, behavior is identical to the current workflow (general-purpose review, interactive user prompts, inline content writing, no option evaluation dispatch).

**Mandatory dispatch compliance:** See P-1.
### A-0: Format Detection
Before any ADR operation, determine which ADR format to use:
1. **Read the config file** — resolve the config path (see [Configuration](#configuration)) and read `[author].template` from `preferences.toml`.
   - If set (e.g., `"nygard-agent"`, `"wi-nygard-agent"`, `"nygard"`, or `"madr"`), use it directly.
   - If absent, default to `"nygard-agent"`.
   - Also read `[author.dispatch]` keys (`review`, `editor`, `tech_writer`, `ux_review`, `dx_review`, `tpm`) if present. Store for use during create, review, and revise workflows. If absent, use defaults (`review = "general-purpose"`, `editor = "interactive"`, `tech_writer = ""`, `ux_review = ""`, `dx_review = ""`, `tpm = ""`).
2. **If `docs/adr/` does not exist** — bootstrap the decision log using the default nygard-agent format:
   ```bash make -f <skill-root>/Makefile init DIR=docs/adr ```
3. **Cache the format** — for the rest of the session, pass `ADR_AGENT_SKILL_FORMAT=nygard-agent` (or the configured format) to all Makefile targets.
### A-1: Draft Worksheet
Per ADR-0032, a draft worksheet captures the author's original intent and workflow calibration before the create workflow runs. The agent always runs this step. The user may decline to fill specific fields, but the worksheet structure must be present in the ADR before proceeding to A-2.
**Activation triggers:** Any create request triggers A-1 first. Direct triggers: "draft an ADR," "start a draft," "I have an idea for a decision."
**Workflow:**
1. **Create the ADR file** — `make -f <skill-root>/Makefile new TITLE="tbd"` to create a placeholder ADR.
2. **Fill the Draft Worksheet** — populate the following structure in the `## Comments` section (below the `---` semantic boundary):
   ```markdown
   ---
   ## Comments
   ### Draft Worksheet
   <!-- Captures original intent and workflow calibration. -->
   **Framing:**
   <!-- What's the core idea? What triggered this? What direction are you leaning? -->
   **Tolerance:**
   - Risk: [Low | Medium | High] — appetite for experimental or unproven options
   - Change: [Low | Medium | High] — acceptable departure from current state
   - Improvisation: [Low | Medium | High] — creative divergence from this framing
   **Uncertainty:**
   <!-- What do you know for certain? What are you unsure about? -->
   **Options:**
   - Target count: [2-3 | 3-5 | open]
   - [ ] Explore additional options beyond candidates listed below
   **Candidates:**
   <!-- Pre-identified option candidates with brief notes. Leave empty if starting from scratch. -->
   ```
3. **Fill mode** depends on the workflow:
   - **Create workflow** (user arrives with direction) — fill the worksheet **before** populating Context/Options. The user provides the framing upfront.
   - **Solve workflow** (user arrives with a problem) — fill the worksheet **after** the problem intake conversation. The agent drafts the worksheet from the conversation and the user confirms/adjusts.
4. **Hand off** — after the worksheet is filled, proceed to [Creating an ADR](#creating-an-adr). The create workflow reads the worksheet for grounding (see [references/create.md](references/create.md)).

**Autonomous low-uncertainty merge:** In autonomous mode with low uncertainty (Tolerance: Risk Low, Improvisation Low), A-1 and A-2 may be merged into a single pass. The worksheet still appears in Comments, but is written alongside the ADR body — not in a separate step. The worksheet's value scales with uncertainty; low-uncertainty problems don't benefit from a separate round-trip.

**Comments area evolution:** The `## Comments` section holds both the Draft Worksheet (pre-decision intent) and Revision Q&A entries (post-review dialogue). The Draft Worksheet always appears first, before any revision Q&A entries.
### A-2: Create
#### Creating an ADR
Read [references/create.md](references/create.md) for the full creation workflow including significance assessment, readiness checks, good practices, and anti-patterns.
1. **Assess significance** — score the decision against the 7 ASR criteria. If it's not architecturally significant, suggest informal documentation.
2. **Check readiness** — verify the START criteria: Stakeholders, Time/MRM, Alternatives, Requirements, Template.
3. **Pick a template** — default to Nygard Agent. Use MADR if the user needs structured tradeoff analysis. See [Choosing a Template](#choosing-a-template).
4. **Draft the ADR** — populate the ADR body from the template in [assets/templates/](assets/templates/).
   - **If `tech_writer` is configured** (non-empty value in `[author.dispatch]`): dispatch the tech-writer agent via the `task` tool with the ADR file path (with draft worksheet from A-1), the problem context, the template structure selected in step 3, and writing style instructions. The tech-writer writes Context, Options, Decision, Consequences, and Quality Strategy sections. Quality Strategy is a documentation task — the inline agent validates the selections during checkpoint review. After the tech-writer returns, the inline agent verifies all required sections are populated and content aligns with the draft worksheet. If the tech-writer returns partial or malformed output, the inline agent completes the remaining sections and warns the user. If the configured agent cannot be resolved at runtime, fall back to inline writing and warn the user.
   - **If `tech_writer` is absent or empty** (default): the inline agent writes content directly, preserving current behavior.
   See [references/create.md](references/create.md) Step 3b for the full dispatch procedure.
5. **Create via Makefile** — always use the Makefile target:
   ```bash make -f <skill-root>/Makefile new TITLE="Use PostgreSQL" ```
   Only fall back to calling scripts directly if the Makefile is unavailable. See [Escape Hatch](#escape-hatch-direct-script-usage) for direct usage.
6. **Validate completion** — check the implementability criteria: Criteria, Documentation, Experimentation Tolerance, Scope Clarity, Actionable Consequences, Dependency Visibility.
7. **Recommend review** — after creating the ADR, recommend reviewing it:
   > Would you like to review this ADR? It will be checked for completeness, > reasoning fallacies, and anti-patterns.
   If the user agrees, proceed to [Reviewing an ADR](#reviewing-an-adr).
**Problem-solving workflows:** For problem-first workflows (exploring options before committing to a solution), use `/solve-adr` instead. It orchestrates across `/author-adr`, `/prototype-adr`, and `/implement-adr`.
### A-3: Review
Read [references/polish.md](references/polish.md) for the full quality loop process. Direct the review agent to the Review Phase section (steps R-1 through R-6). By default this is a general-purpose agent; a custom agent can be configured via `[author.dispatch].review`.
The review process covers:
1. **Implementability check** — verify the 6 implementability criteria
2. **Fallacy scan** — check against 7 architectural decision-making fallacies
3. **Anti-pattern check** — scan for 11 ADR creation anti-patterns
4. **Consequence validation** — interactively verify stated consequences with the user
5. **7-point checklist** — structured quality assessment
6. **Verdict** — Accept (→ Ready status), Revise, or Rethink
7. **Accept-with-suggestions** — if the verdict is Accept but includes minor suggestions, dispatch the editor agent for a lightweight polish pass (see polish.md §R-6a). If no editor agent is configured, present suggestions to the user as optional improvements.
8. **Revision handoff** — if the verdict is "Revise":
   - If `editor` is `"interactive"` (or absent): offer to interactively address the review comments. If the user agrees, proceed to [Revising an ADR](#revising-an-adr).
   - If `editor` is an agent reference: automatically proceed to [Revising an ADR](#revising-an-adr) — the configured editor agent stands in for the user during triage. Do not ask for permission; the delegated editor handles the review→revise loop.
### A-4: Revise
Read [references/polish.md](references/polish.md) for the full quality loop process. Direct the editor agent to the Revision Phase section (steps V-1 through V-6). Use this after a review produces a "Revise" verdict. When `[author.dispatch].editor` is configured with an agent reference (not `"interactive"`), the configured editor agent stands in for the user during triage — see [Agent Dispatch](#agent-dispatch-authordispatch).
The revision process covers:
1. **Load review comments** — parse the structured review output into discrete revision items
2. **Present each comment** — show findings one at a time with context
3. **Collect user response** — for each comment, the user can address it, reject it, or defer it to another ADR
4. **Apply revisions** — update the ADR with the user's approved changes
5. **Produce revision summary** — document what was addressed, deferred, or rejected
6. **Recommend re-review** — suggest re-review if substantive changes were made
### A-6: Manage
Read [references/manage.md](references/manage.md) for the full management reference including status transitions, superseding, linking, and splitting.
**Guardrail (P-2):** See P-2 (Cross-ADR Modification Guardrail).
### Choosing a Template
| Situation | Template | File |
|-----------|----------|------|
| Default (agent-developer workflow) | Nygard Agent | [nygard-agent-template.md](assets/templates/nygard-agent-template.md) |
See [references/templates.md](references/templates.md) for full template details and guidance.
## Core Concepts
An **Architectural Decision (AD)** is a justified design choice that addresses a functional or non-functional requirement that is architecturally significant.
An **Architecturally Significant Requirement (ASR)** is a requirement that has a measurable effect on the architecture and quality of a software/hardware system.
An **Architectural Decision Record (ADR)** captures a single AD and its rationale. The collection of ADRs in a project is its **decision log**.
All of this falls under **Architectural Knowledge Management (AKM)**, but ADR usage can be extended to design and other decisions ("any decision record").
## Tooling
This skill uses a unified script architecture via `ADR_AGENT_SKILL_FORMAT`:
| Format | Template | When to Use |
|--------|----------|-------------|
| `nygard-agent` (default) | Nygard Agent | Agent-developer workflows, quality-aware decisions |
| `wi-nygard-agent` | Nygard Agent (work-item) | Team workflows with work item traceability (ADR-0034) |
| `wi-full-agent-adr` | TOML (full agent) | Automated workflows with mandatory checkpoints and typed schema (ADR-0051) |

### Cross-Reference Convention

| Format | Reference Pattern | Example |
|--------|------------------|---------|
| `nygard-agent` | `ADR-NNNN` | `ADR-0034` |
| `wi-nygard-agent` | `ADR-{remote}-{id}` | `ADR-gh-42`, `ADR-ado-1234`, `ADR-local-a1b2c3d4` |
| `wi-full-agent-adr` | `ADR-{remote}-{id}` | `ADR-gh-42`, `ADR-ado-1234`, `ADR-local-a1b2c3d4` |

Both conventions coexist in mixed decision logs. Existing `ADR-NNNN` references are not migrated.
### Makefile Targets (Required)
**Always use Makefile targets.** Only fall back to direct script usage if the Makefile is genuinely unavailable (e.g., not on `PATH`, broken environment).
```bash
# Set format (default: nygard-agent)
export ADR_AGENT_SKILL_FORMAT=nygard-agent

make -f <skill-root>/Makefile init DIR=docs/adr     # bootstrap ADR directory
make -f <skill-root>/Makefile init-data              # bootstrap .adr/ project-scoped directory
make -f <skill-root>/Makefile new TITLE="Use PostgreSQL"
make -f <skill-root>/Makefile rename NUM=2 TITLE="Use PostgreSQL"  # rename ADR file and heading
make -f <skill-root>/Makefile list                   # list all ADRs
make -f <skill-root>/Makefile status NUM=2 STATUS=Proposed  # update status

# Work-item-referenced naming (wi-nygard-agent format):
export ADR_AGENT_SKILL_FORMAT=wi-nygard-agent

make -f <skill-root>/Makefile new REMOTE=gh ID=42 TITLE="Use PostgreSQL"
make -f <skill-root>/Makefile rename REMOTE=gh ID=42 TITLE="Use PostgreSQL"
make -f <skill-root>/Makefile status REMOTE=gh ID=42 STATUS=Proposed
```

```bash
# TOML format (wi-full-agent-adr — requires adr-db binary, run make build-tools first):
export ADR_AGENT_SKILL_FORMAT=wi-full-agent-adr

make -f <skill-root>/Makefile new REMOTE=gh ID=42 TITLE="Use PostgreSQL"
make -f <skill-root>/Makefile rename REMOTE=gh ID=42 TITLE="Use PostgreSQL"
make -f <skill-root>/Makefile status REMOTE=gh ID=42 STATUS=Proposed

# Export TOML ADR to Markdown:
adr-db author export gh 42
```
### Escape Hatch: Direct Script Usage
Only use direct scripts when the Makefile is unavailable. See [references/tooling.md](references/tooling.md) for full command docs:
```bash
export PATH="$PWD/scripts:$PATH"
nygard-agent-format.sh init docs/adr
new.sh nygard-agent "Use PostgreSQL"
nygard-agent-format.sh rename 2 "Use PostgreSQL"
nygard-agent-format.sh list
nygard-agent-format.sh status 2 Proposed

# Work-item-referenced (wi-nygard-agent):
new.sh wi-nygard-agent gh 42 "Use PostgreSQL"
wi-nygard-agent-format.sh rename gh 42 "Use PostgreSQL"
wi-nygard-agent-format.sh list
wi-nygard-agent-format.sh status gh 42 Proposed

# TOML format (wi-full-agent-adr — requires adr-db binary):
new.sh wi-full-agent-adr gh 42 "Use PostgreSQL"
adr-db author list
adr-db author rename gh 42 "Use PostgreSQL"
adr-db author status gh 42 Proposed
adr-db author export gh 42
```
### Visualization
Use **Mermaid** for all diagrams. Diagrams are valuable when complex relationships between processes or entities benefit from visual compression, but overuse can overload context — use sparingly. When comparing options, prefer **markdown tables** over diagrams. See [references/tooling.md](references/tooling.md) for guidelines and syntax patterns.
## Deep References
For detailed guidance beyond what is covered above, consult these references on-demand:
- [references/create.md](references/create.md) — full ADR creation workflow with significance assessment, readiness checks, and anti-patterns
- [references/polish.md](references/polish.md) — complete quality loop: review, verdict, revision, re-review
- [references/manage.md](references/manage.md) — status transitions, superseding, linking, splitting, and guardrails
- [references/templates.md](references/templates.md) — template details and selection guide
- [references/tooling.md](references/tooling.md) — unified script architecture and command reference
