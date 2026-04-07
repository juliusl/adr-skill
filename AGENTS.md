# Contributing to the ADR Skills

Instructions for agents and developers making changes to skills in this repo.

## Directory Structure

```
adr-skills/
├── AGENTS.md                    # This file — development guide
├── README.md                    # Project overview
├── Makefile                     # Dev targets (test, build-tools)
├── eval_queries.json            # Trigger evaluation queries for description optimization
├── crates/                      # Cargo workspace — Rust tooling (ADR-0028)
│   ├── Cargo.toml               # Workspace root
│   └── adr-db/                  # Plumbing CLI: JSONL → SQLite (ADR-0026, ADR-0027)
│       ├── Cargo.toml
│       ├── diesel.toml          # Diesel schema output config
│       ├── migrations/          # Diesel SQL migrations
│       └── src/                 # Rust source
├── docs/adr/                    # Project-level ADRs (decisions about these skills)
├── docs/plans/                  # Implementation plans generated from ADRs
├── src/skills/
│   ├── author-adr/              # Skill: create, review, manage ADRs
│   │   ├── SKILL.md             # Skill entry point (spec-compliant frontmatter + instructions)
│   │   ├── Makefile             # Downstream agent interface (init, new, list, etc.)
│   │   ├── references/          # On-demand documentation loaded by the agent
│   │   │   ├── create.md        # ADR creation workflow (significance, readiness, practices)
│   │   │   ├── review.md        # ADR review process (ecADR, fallacies, anti-patterns)
│   │   │   ├── manage.md        # ADR management (status, supersede, link, split)
│   │   │   ├── templates.md     # Template selection guide (Nygard primary, MADR, Y-Statement)
│   │   │   └── tooling.md       # Dual-format command reference + visualization guidance
│   │   ├── assets/              # Static resources and templates
│   │   │   ├── decisions/       # Bundled ADRs referenced by skill instructions
│   │   │   └── templates/       # Ready-to-use ADR templates (Nygard, MADR, Y-Statement)
│   │   └── scripts/
│   │       ├── adr-tools-3.0.0/ # Nygard format (bundled, 22 tests)
│   │       └── madr-tools/      # MADR format (custom, 9 tests)
│   ├── implement-adr/           # Skill: turn ADRs into implementation plans
│   │   ├── SKILL.md             # Skill entry point (spec-compliant frontmatter + instructions)
│   │   ├── Makefile             # Downstream agent interface (list-adrs, show-template)
│   │   ├── references/          # On-demand documentation loaded by the agent
│   │   │   ├── planning-practices.md  # Stage decomposition, task scoping, gap detection
│   │   │   ├── testing-guidelines.md  # Testing taxonomy by code context
│   │   │   └── cost-estimation.md     # T-shirt sizing guide and calibration
│   │   ├── assets/              # Static resources and templates
│   │   │   ├── decisions/       # Bundled ADRs referenced by skill instructions
│   │   │   └── templates/       # plan.md template
│   │   └── scripts/             # Reserved for future tooling
│   └── solve-adr/               # Skill: scenario-driven problem solving orchestrator
│       ├── SKILL.md             # Skill entry point (scenario-based procedures)
│       ├── Makefile             # Minimal — orchestrator has no scripts
│       ├── references/          # On-demand documentation loaded by the agent
│       │   ├── problem.md       # S-1 Problem Exploration workflow detail
│       │   └── roadmap.md       # S-2 Roadmap workflow detail
│       └── eval_queries.json    # Trigger evaluation queries for solve-adr
```

## Policies

All policies are listed here with identifiers. Detailed descriptions follow in the sections below.

| ID | Policy | Description |
|----|--------|-------------|
| P-1 | Writing Style | Technical, simple, concise — no double negatives, clear logic |
| P-1a | ADHD Friendly Guidelines | Logical ordering, frontloaded actions, visual chunking |
| P-2 | Broken Test Policy | Stop and fix broken tests before proceeding |
| P-3 | Agent Procedure Writing Policy | 12 rules for writing agent skill procedures |
| P-4 | Git Policies | No commit/push without explicit user delegation |
| P-4a | Conventional Commits | Use `type(scope): summary` format for commit messages |
| P-5 | Formatting Convention | Use flat tables with classification columns for review artifacts |
| P-6 | ADR References in Skills | Bundle behavioral ADR refs as assets; inline provenance-only refs |
| P-7 | Before Making Changes | Run test suite and validate skills before starting work |
| P-8 | After Making Changes | Re-run tests, re-validate, check refs, verify line counts |
| P-9 | Spec Constraints | agentskills.io field constraints for SKILL.md |
| P-10 | Modifying References | Keep references self-contained; one file per workflow task |
| P-11 | Modifying Templates | Place in `assets/templates/`, add entry in `references/templates.md` |
| P-12 | Modifying Scripts | Diff-based test validation for adr-tools and madr-tools |
| P-12a | adr-tools (Nygard format) | Bundled third-party scripts, `make test-nygard` |
| P-12b | madr-tools (MADR format) | Custom scripts, `make test-madr` |
| P-13 | Rust Tooling | Cargo workspace in `crates/`, `make build-tools` |
| P-13a | Working on adr-db | Prerequisites, migrations, tests |
| P-14 | Evaluating Skill Description | Use `eval_queries.json` to validate trigger reliability |
| P-15 | Installed Skills | Never modify `~/.copilot/skills/` — test against repo source |
| P-16 | Broken Makefile Targets | Stop and fix broken Makefile targets before proceeding |
| P-17 | Autonomy Directives | Never take shortcuts when a procedure or plan has been established |

---

## P-1: Writing Style

All writing should follow this style

- **Technical and simple** — write for engineers/agents, not academics
- **No double negatives** — say what things *do*, not what they don't not do
- **Clear logic** — one idea per sentence, explicit cause-and-effect
- **Concise** — cut filler words; if a sentence works without a word, remove it
- **Do not arbitrarily wrap technical docs meant for agents** - If a document will be consumed by an agent, avoid manual wrapping in formatting.

### P-1a: ADHD Friendly Guidelines

In addition to the above writing style guidelines, writing must be presented in an ADHD friendly manner.

This DOES NOT mean:

- Using emojis
- Stating that information is ADHD friendly

This DOES mean:

- Order information logically — most important information first
- Frontload actions — put the command or instruction before the explanation, not after
- Use lists, flow-charts, and tables proactively
- Use whitespace and visual chunking — short paragraphs, consistent formatting patterns, clear separation between sections
- Do not use headers arbitrarily — organize around process flow, not arbitrary categories
- Keep justification brief
- State evidence and rationale explicitly — do not expect the reader to infer connections or fill in gaps
- Follow KISS — Keep It Simple, Stupid

NEVER:

- When giving a recommendation, do not preface the recommendation with this guideline, ADHD users do not need to be reminded that they have ADHD

## P-2: Broken Test Policy

ALWAYS, when encountering a broken test, STOP and fix the test before proceeding. A broken test is always in scope and ignoring it creates technical debt.

## P-3: Agent Procedure Writing Policy

This repo contains files for agent skills. When writing a new procedure or maintaining an existing procedure, follow these rules:

1) Instructions for agents **MUST** be procedural similar to any type of legal documentation
2) When gathering any type of user input, **PROACTIVELY** define a form/worksheet that can be followed procedurally
3) **ALL** rules for directions or policies **MUST** be placed at the **TOP** of the document. Any rules appended to the end of a document cannot be followed when executing a direction. This includes nested rules for each individual step or element of a procedure or worksheet.
4) **ALL** steps in the procedure must have a alpha-numeric identifier, and any reference to the step must use this identifier
5) **ANY** worksheets, forms, or input intake must also have a clear alpha-numeric identifier
6) **ALL** steps, subsections, and worksheet elements identified by Rules 4, 5, and 11 **MUST** be listed in a summary table with their identifier and a one-line description **BEFORE** the first detailed section. This table serves as the procedure's index — not to be confused with domain-specific tables (e.g., criteria checklists, status diagrams) that may also appear in the document.
7) Procedure **MUST** have a clear and logical flow presented before any of the individual step details
8) **ALWAYS** design guardrails that prevent the agent from skipping steps in a procedure. 
9) Steps or worksheet elements **MAY** be conditional, but they **MUST** be visited so that the reason the condition was not met can be recorded
10) Instructions **MUST** be generic. **NEVER** use examples from this repo.
11) **ALL** subsection of **ANY** step or element **MUST** also have a related alpha-numeric identifier. It should be clear that a subsection is related to a step. Subsections are considered sub-tasks of a step. Failure to label them makes it unclear to both user and agent that a procedure violation occurred.
12) **ALL** policies **MUST** also have a clear alpha-numeric identifier prefixed with 'P', any sub-policies **MUST** also have an identifier. Failure to label reduces visibility into what policy failed to activate making refinement less actionable.

**Templates:** Use [procedure-template.md](procedure-template.md) for new procedures and [worksheet-template.md](worksheet-template.md) for new worksheets. These templates embody Rules 1–12 structurally.

## P-4: Git Policies

1) Agents **must not** commit or push changes. Stage your work and let the
developer review, commit, and push manually.

This policy can **only** be bypassed with **EXPLICIT** instructions given or delegated by the user.

### P-4a: Conventional Commits

When asked to draft a commit message, use
<a href="https://www.conventionalcommits.org/">Conventional Commits</a> format:

```
<type>(<scope>): <short summary>

<optional body>
```

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`.
Scope is optional but encouraged (e.g., `skill`, `makefile`, `tooling`).

## P-5: Formatting Convention for Review Artifacts

Use **tables** when presenting items for review or posterity (QA findings,
recommendations, checklists). Do not split related items into separate
sections — if items differ by a property (e.g., classification, status,
action), add a **column** for that property instead. A single flat table is
easier to scan than multiple subsections with separate tables.

**Example — QA recommendations:** Instead of separate `### Quality Concerns`
and `### Preferences` subsections with different table schemas, use one table
with a `Classification` column:

```markdown
| # | Stage | Classification | Finding | Recommendation |
|---|-------|----------------|---------|----------------|
| 1 | 1 | Quality concern | Description | Action |
| 2 | 2 | Preference | Description | Reason for deferral |
```

Extended prose (e.g., Won't Fix rationale) can follow the table under its own
heading when needed.

## P-6: ADR References in Skills

Skills are installed to `~/.copilot/skills/` and run in any repo. They do not
have access to this repo's `docs/adr/` directory at runtime.

**When a skill instruction references an ADR behaviorally** (the agent needs
the ADR content to execute the step), the ADR must be bundled as an asset:

1. Copy the ADR to `src/skills/<skill>/assets/decisions/`
2. Add a row to the `## Assets` table in SKILL.md with the file path, ADR
   number, what it defines, and which step IDs reference it
3. In the instruction text, keep the ADR reference (e.g., "per ADR-0031") —
   the agent resolves it via the asset table

**When a reference is provenance only** (explains *why* a convention exists,
but the agent does not need the ADR content to execute), inline the
explanation and remove the ADR number. The agent cannot look up ADRs that
are not bundled.

**Rule of thumb:** If removing the ADR reference would leave the instruction
ambiguous or incomplete, bundle it. If the instruction stands on its own,
inline it.

## P-7: Before Making Changes

1. **Run the test suite** to establish a clean baseline:

   ```bash
   make test
   ```

   This runs both formats: 22 adr-tools tests + 9 madr-tools tests.

2. **Validate the skill** against the agentskills.io spec:

   ```bash
   make validate-setup   # one-time: installs skills-ref
   make validate          # validate author-adr
   make validate-implement # validate implement-adr
   make validate-all      # validate both
   ```

## P-8: After Making Changes

1. **Re-run tests** to confirm nothing is broken:

   ```bash
   make test
   ```

2. **Re-validate the skill** if you changed SKILL.md frontmatter:

   ```bash
   make validate
   ```

3. **Check for broken references** — all `.md` file links in SKILL.md and
   references/ must resolve to existing files:

   ```bash
   make check-refs
   ```
   ```

4. **Verify SKILL.md stays under 500 lines** (spec recommendation for
   progressive disclosure):

   ```bash
   wc -l src/skills/author-adr/SKILL.md src/skills/implement-adr/SKILL.md
   ```

## P-9: Spec Constraints

From the [agentskills.io specification](https://agentskills.io/specification):

| Field | Constraint |
|-------|-----------|
| `name` | Lowercase + hyphens, 1-64 chars, must match parent directory name |
| `description` | 1-1024 chars, non-empty |
| `SKILL.md` body | Recommended < 500 lines |
| File references | Relative paths from skill root, keep one level deep |

## P-10: Adding or Modifying References

Task-specific references live in `references/` (one file per workflow task):

- `create.md` — ADR creation workflow
- `review.md` — ADR review process
- `manage.md` — ADR management operations

When modifying a reference file, keep it self-contained — each reference should
be usable on its own without reading the others.

**Procedural references** (files that contain steps or tasks) **must** use
<a>procedure-template.md</a> as a structural starting point. Worksheet
elements within procedures **must** use <a>worksheet-template.md</a>.
Informational references (e.g., `templates.md`, `tooling.md`) are exempt.

## P-11: Adding or Modifying Templates

Templates live in `assets/templates/`. When adding a new template:

1. Place the template file in `assets/templates/`
2. Add an entry in `references/templates.md`

## P-12: Modifying Scripts

### P-12a: adr-tools (Nygard format)

Bundled third-party scripts at `src/skills/author-adr/scripts/adr-tools-3.0.0/`. Tests use
diff-based validation: `tests/*.sh` (commands) vs `tests/*.expected` (output).

```bash
make test-nygard
```

### P-12b: madr-tools (MADR format)

Custom scripts at `src/skills/author-adr/scripts/madr-tools/`. Same test pattern.

To add a new test:
1. Create `tests/<name>.sh` with the commands to run
2. Generate expected output by running the test manually
3. Save output as `tests/<name>.expected`
4. Verify: `make -C src/skills/author-adr/scripts/madr-tools clean check`

```bash
make test-madr
```

## P-13: Rust Tooling (crates/)

The `crates/` directory contains a Cargo workspace with Rust tooling. Currently
this includes `adr-db`, a plumbing CLI for ingesting JSONL data into SQLite.

### Building

```bash
make build-tools    # cargo build --release in crates/
```

The `build-tools` target is independent of `make test` — contributors who only
work on shell scripts do not need Rust installed.

### P-13a: Working on adr-db

**Prerequisites:**
- Rust toolchain (`rustup`)
- `diesel_cli` for migration authoring:
  ```bash
  cargo install diesel_cli --no-default-features --features sqlite
  ```

**Creating a new migration:**
```bash
cd crates/adr-db
DATABASE_URL="sqlite:///tmp/dev.db" diesel migration generate <name>
# Edit migrations/<timestamp>_<name>/up.sql and down.sql
DATABASE_URL="sqlite:///tmp/dev.db" diesel migration run
# schema.rs is regenerated — commit it to version control
```

**Running tests:**
```bash
cd crates && cargo test
```

`diesel_cli` is required only for creating or modifying migrations, not for
building the binary. The generated `schema.rs` is committed to version control
so `cargo build` works without `diesel_cli`.

## P-14: Evaluating Skill Description

The `eval_queries.json` file contains 20 queries (11 should-trigger, 9
should-not-trigger) for testing whether the skill's `description` field
triggers reliably. See the
<a href="https://agentskills.io/skill-creation/optimizing-descriptions">agentskills.io optimization guide</a>
for the evaluation workflow.

## P-15: Installed Skills

Files under `~/.copilot/skills/` are platform-managed. Agents **must not**
create, modify, or delete files in that directory.

When testing changes to skill scripts, Makefiles, or templates, run tests
against the repo source copy at `src/skills/<skill>/`, not the installed
copy.

## P-16: Broken Makefile Targets Policy

ALWAYS, when encountering a broken makefile target, STOP and fix the target before proceeding. A broken target is always in scope and ignoring it creates technical debt.

## P-17: Autonomy Directives

When operating autonomously, **NEVER** take shortcuts when a procedure or plan has been established. Resource constraints or session length are not valid reasons to skip procedures. Procedures are in place to safe-guard autonomously generated code.