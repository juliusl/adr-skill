# Contributing to the ADR Skill

Instructions for agents and developers making changes to this skill.

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
adr-skill/
├── AGENTS.md                    # This file — development guide
├── README.md                    # Project overview
├── Makefile                     # Dev targets (test, install-agents)
├── eval_queries.json            # Trigger evaluation queries for description optimization
└── author-adr/  # Skill root (copy this to install)
    ├── SKILL.md                 # Skill entry point (spec-compliant frontmatter + instructions)
    ├── Makefile                 # Downstream agent interface (init, new, list, etc.)
    ├── references/              # On-demand documentation loaded by the agent
    │   ├── practices.md         # AD practice guide with inline summaries
    │   ├── templates.md         # Template selection guide (Nygard primary, MADR, Y-Statement)
    │   └── tooling.md           # Dual-format command reference + visualization guidance
    ├── assets/                  # Static resources, templates, and distilled notes
    │   ├── index.md             # Curated asset index with summaries
    │   ├── PRACTICES_NOTES.md   # Tagged practice fragments (create, review, assess, etc.)
    │   ├── SUPPLEMENTAL.md      # Medium-value context (adoption model, alt tooling)
    │   ├── mermaid-chart-examples.md
    │   ├── adr-reviewer.agent.md  # Custom agent (installed via make install-agents)
    │   ├── templates/           # Ready-to-use ADR templates (Nygard, MADR, Y-Statement)
    │   └── archive/             # Full originals of distilled notes (deep context only)
    └── scripts/
        ├── adr-tools-3.0.0/     # Nygard format (bundled, 22 tests)
        └── madr-tools/          # MADR format (custom, 9 tests)
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
   make validate
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
   cd author-adr
   grep -oP '(?:assets|references)/[^\s)#]+\.md' SKILL.md | while read ref; do
     [ ! -f "$ref" ] && echo "BROKEN: $ref"
   done
   ```

4. **Verify SKILL.md stays under 500 lines** (spec recommendation for
   progressive disclosure):

   ```bash
   wc -l author-adr/SKILL.md
   ```

## Spec Constraints

From the [agentskills.io specification](https://agentskills.io/specification):

| Field | Constraint |
|-------|-----------|
| `name` | Lowercase + hyphens, 1-64 chars, must match parent directory name |
| `description` | 1-1024 chars, non-empty |
| `SKILL.md` body | Recommended < 500 lines |
| File references | Relative paths from skill root, keep one level deep |

## Adding or Modifying Practice Notes

Practice notes live in `assets/PRACTICES_NOTES.md` as tagged fragments.

- Each fragment has a `<!-- tags: ... -->` comment listing relevant intents
- Available tags: `create`, `review`, `assess`, `template`, `justify`, `adopt`
- Each fragment has an `*Archive:*` link pointing to the full original
- When adding a new fragment, choose tags based on which workflow steps
  (in SKILL.md's "Agent Workflow" section) the content supports

## Adding or Modifying Templates

Templates live in `assets/templates/`. When adding a new template:

1. Place the template file in `assets/templates/`
2. Add an entry in `references/templates.md`
3. Add an entry in `assets/index.md` under the Templates section

## Modifying Scripts

### adr-tools (Nygard format)

Bundled third-party scripts at `author-adr/scripts/adr-tools-3.0.0/`. Tests use
diff-based validation: `tests/*.sh` (commands) vs `tests/*.expected` (output).

```bash
make test-nygard
```

### madr-tools (MADR format)

Custom scripts at `author-adr/scripts/madr-tools/`. Same test pattern.

To add a new test:
1. Create `tests/<name>.sh` with the commands to run
2. Generate expected output by running the test manually
3. Save output as `tests/<name>.expected`
4. Verify: `make -C author-adr/scripts/madr-tools clean check`

```bash
make test-madr
```

## Evaluating Skill Description

The `eval_queries.json` file contains 20 queries (11 should-trigger, 9
should-not-trigger) for testing whether the skill's `description` field
triggers reliably. See the
[agentskills.io optimization guide](https://agentskills.io/skill-creation/optimizing-descriptions)
for the evaluation workflow.
