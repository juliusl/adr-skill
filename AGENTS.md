# Contributing to the ADR Skills

Instructions for agents and developers making changes to skills in this repo.

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
├── Makefile                     # Dev targets (test)
├── eval_queries.json            # Trigger evaluation queries for description optimization
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

## Evaluating Skill Description

The `eval_queries.json` file contains 20 queries (11 should-trigger, 9
should-not-trigger) for testing whether the skill's `description` field
triggers reliably. See the
[agentskills.io optimization guide](https://agentskills.io/skill-creation/optimizing-descriptions)
for the evaluation workflow.
