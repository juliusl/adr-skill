# Contributing to the ADR Skills

Instructions for agents and developers making changes to skills in this repo.

## Writing Style

All writing should follow this style

- **Technical and simple** — write for engineers/agents, not academics
- **No double negatives** — say what things *do*, not what they don't not do
- **Clear logic** — one idea per sentence, explicit cause-and-effect
- **Concise** — cut filler words; if a sentence works without a word, remove it
- **Do not arbitrarily wrap technical docs meant for agents** - If a document will be consumed by an agent, avoid manual wrapping in formatting.

## ADHD Friendly Guidelines

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

## Git Policy

Agents **must not** commit or push changes. Stage your work and let the
developer review, commit, and push manually.

When asked to draft a commit message, use
[Conventional Commits](https://www.conventionalcommits.org/) format:

```
<type>(<scope>): <short summary>

<optional body>
```

Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`, `build`.
Scope is optional but encouraged (e.g., `skill`, `makefile`, `tooling`).

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
│   └── implement-adr/           # Skill: turn ADRs into implementation plans
│       ├── SKILL.md             # Skill entry point (spec-compliant frontmatter + instructions)
│       ├── Makefile             # Downstream agent interface (list-adrs, show-template)
│       ├── references/          # On-demand documentation loaded by the agent
│       │   ├── planning-practices.md  # Stage decomposition, task scoping, gap detection
│       │   ├── testing-guidelines.md  # Testing taxonomy by code context
│       │   └── cost-estimation.md     # T-shirt sizing guide and calibration
│       ├── assets/              # Static resources and templates
│       │   ├── decisions/       # Bundled ADRs referenced by skill instructions
│       │   └── templates/       # plan.md template
│       └── scripts/             # Reserved for future tooling
```

## Before Making Changes

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

## After Making Changes

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

## Spec Constraints

From the [agentskills.io specification](https://agentskills.io/specification):

| Field | Constraint |
|-------|-----------|
| `name` | Lowercase + hyphens, 1-64 chars, must match parent directory name |
| `description` | 1-1024 chars, non-empty |
| `SKILL.md` body | Recommended < 500 lines |
| File references | Relative paths from skill root, keep one level deep |

## Adding or Modifying References

Task-specific references live in `references/` (one file per workflow task):

- `create.md` — ADR creation workflow
- `review.md` — ADR review process
- `manage.md` — ADR management operations

When modifying a reference file, keep it self-contained — each reference should
be usable on its own without reading the others.

## Adding or Modifying Templates

Templates live in `assets/templates/`. When adding a new template:

1. Place the template file in `assets/templates/`
2. Add an entry in `references/templates.md`

## Modifying Scripts

### adr-tools (Nygard format)

Bundled third-party scripts at `src/skills/author-adr/scripts/adr-tools-3.0.0/`. Tests use
diff-based validation: `tests/*.sh` (commands) vs `tests/*.expected` (output).

```bash
make test-nygard
```

### madr-tools (MADR format)

Custom scripts at `src/skills/author-adr/scripts/madr-tools/`. Same test pattern.

To add a new test:
1. Create `tests/<name>.sh` with the commands to run
2. Generate expected output by running the test manually
3. Save output as `tests/<name>.expected`
4. Verify: `make -C src/skills/author-adr/scripts/madr-tools clean check`

```bash
make test-madr
```

## Rust Tooling (crates/)

The `crates/` directory contains a Cargo workspace with Rust tooling. Currently
this includes `adr-db`, a plumbing CLI for ingesting JSONL data into SQLite.

### Building

```bash
make build-tools    # cargo build --release in crates/
```

The `build-tools` target is independent of `make test` — contributors who only
work on shell scripts do not need Rust installed.

### Working on adr-db

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

## Evaluating Skill Description

The `eval_queries.json` file contains 20 queries (11 should-trigger, 9
should-not-trigger) for testing whether the skill's `description` field
triggers reliably. See the
[agentskills.io optimization guide](https://agentskills.io/skill-creation/optimizing-descriptions)
for the evaluation workflow.

## Formatting Convention for Review Artifacts

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

## ADR References in Skills

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
