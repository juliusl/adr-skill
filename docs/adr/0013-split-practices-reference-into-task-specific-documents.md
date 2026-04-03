# 13. Split practices reference into task-specific documents

Date: 2026-04-03

## Status

Accepted

## Context

The `author-adr` skill currently organizes practice guidance into a monolithic
`references/practices.md` that serves as an index pointing into
`assets/PRACTICES_NOTES.md` — a single file containing tagged fragments for all
intents (create, review, assess, template, justify, adopt). Additional context
lives in `assets/SUPPLEMENTAL.md` and full originals in `assets/archive/`.

This structure has several problems:

1. **Indirect loading** — SKILL.md points to `practices.md`, which points to
   `PRACTICES_NOTES.md` fragments via anchor links. The agent must navigate two
   hops to reach actionable content, and the tag-based filtering mechanism
   (`<!-- tags: create, review -->`) relies on the agent correctly parsing HTML
   comments — a latent association that is fragile.

2. **Tag system is not working** — dogfooding has shown that files in `assets/`
   are not being used by agents in practice. The tag-based fragment system
   depends on the agent voluntarily scanning HTML comments and selectively
   loading fragments — behavior that is not reliably observed. Assets that are
   not read provide no value regardless of their quality.

3. **Context bloat** — when the agent does load `PRACTICES_NOTES.md`, it loads
   all fragments (create, review, assess, adopt, justify) regardless of the
   current task. This wastes context window tokens and dilutes focus.

4. **Redundant review path** — the skill maintains both a "Reviewing an ADR"
   section in SKILL.md (with inline instructions) and a separate
   `adr-reviewer.agent.md` custom agent. The review logic is duplicated across
   these two locations, and the custom agent requires installation as a
   separate step. In practice, Copilot CLI defaults to the general-purpose
   agent unless the user explicitly names `adr-reviewer` — making the custom
   agent unreliable as the primary review mechanism.

5. **Archive overhead** — `assets/archive/` stores 19 full original source
   documents that were distilled into the notes files. These rarely provide
   value beyond what the distilled notes already contain, but they add to the
   file tree and could confuse agents scanning the directory.

The skill's SKILL.md already routes users to specific tasks (Creating, Reviewing,
Tooling, Templates). Each task should have a dedicated reference document that
the agent can load directly — one hop, one file, precisely scoped content.

### Considered Alternatives

1. **Improve the tag system** — replace HTML comment tags with YAML frontmatter
   or structured headers that agents parse more reliably. Rejected because
   dogfooding showed agents are not reading the asset files at all — the
   problem is not tag format but the indirection pattern itself. Even with
   better tags, the agent must still load the full file and filter, which
   doesn't solve context bloat.

2. **Consolidate into one improved file** — merge all practice content into a
   single, well-structured `practices.md` with clear H2 sections instead of
   tagged fragments. Rejected because directing an agent to "read section X of
   file Y" is harder to write reliable instructions for than "read file X." A
   one-file-per-task approach scales better for fine-tuning the skill —
   individual reference files can be iterated on independently without risk of
   breaking guidance for other tasks.

## Decision

Restructure `references/` so that each file maps to a specific task the skill
handles. Replace the monolithic `practices.md` and the tagged-fragment system
with focused, self-contained reference documents:

| New file | Covers | Replaces |
|----------|--------|----------|
| `references/create.md` | Assessing significance (ASR test), readiness (START), creation best practices, anti-patterns, Definition of Done (ecADR), Y-statement justifications, MADR primer | `PRACTICES_NOTES.md` fragments tagged `create`, `assess`, `justify`, `template`; relevant parts of `SUPPLEMENTAL.md` |
| `references/review.md` | ecADR completeness checks, 7 fallacies with countermeasures, 11 anti-patterns, 7-point review checklist, review perspectives, consequence validation | `PRACTICES_NOTES.md` fragment tagged `review`; full content of `adr-reviewer.agent.md` |
| `references/manage.md` | Updating existing ADRs, superseding decisions, status transitions, linking related ADRs, splitting Mega-ADRs | Currently implicit in SKILL.md guardrails; not covered in a dedicated reference |

Retain unchanged:
- `references/templates.md` — already task-specific
- `references/tooling.md` — already task-specific

Remove after migration:
- `references/practices.md` — replaced by the task-specific files
- `assets/PRACTICES_NOTES.md` — content absorbed into the new reference files
- `assets/SUPPLEMENTAL.md` — content absorbed into the new reference files
- `assets/index.md` — no longer needed as a discovery layer; SKILL.md links
  directly to references
- `assets/archive/` — full originals no longer needed; recoverable from git
  history or original URLs
- `assets/adr-reviewer.agent.md` — review logic absorbed into
  `references/review.md`
- `assets/ozimmer-seven-ad-fallacies.md` — absorbed into reference files
- `assets/nygard-documenting-architecture-decisions.md` — absorbed into
  reference files
- `assets/zdun-sustainable-architectural-decisions.md` — absorbed into
  reference files
- `assets/gh-joelparkerhenderson-adr.md` — absorbed into reference files
- `assets/adr-github-io-madr.md` — absorbed into reference files
- `assets/mermaid-chart-examples.md` — absorbed into `references/tooling.md`

Note: `assets/templates/` is out of scope for this ADR and will be addressed
separately.

Update SKILL.md to link directly to the new reference files in each workflow
step. For example, "Creating an ADR" step 1 would read:
```
1. **Assess significance** — read [references/create.md](references/create.md)
   and score the decision against the 7 ASR criteria.
```

For review, SKILL.md will direct the agent to load `references/review.md` and
use it as a prompt for a general-purpose agent — replacing the custom
`adr-reviewer` agent. This leverages general-purpose agents rather than relying
on custom agent discovery, which is currently unreliable (the CLI defaults to
the general-purpose agent unless the custom agent is explicitly named).

## Consequences

- **Simpler context loading** — each workflow step loads exactly one reference
  file. No tag parsing, no anchor navigation, no two-hop indirection.
- **Expected reduction in token usage** — the agent loads only the content
  relevant to the current task, leaving more context window for the user's
  actual ADR content.
- **Single review path** — review becomes a native skill capability using
  `references/review.md` as a prompt for a general-purpose agent. The
  `install-agents` Makefile target, custom agent installation instructions,
  and `adr-reviewer.agent.md` can be removed. This trades away the ability to
  invoke review as a standalone `/agent` command, but that invocation was
  unreliable in practice — the CLI defaults to the general-purpose agent
  unless the custom agent is explicitly named.
- **Better instruction authoring** — directing the agent to "read file X" is
  simpler and more reliable to write instructions for than "read section Y of
  file Z" or "filter fragments by tag." Individual files can be iterated on
  independently, making it easier to fine-tune guidance per task.
- **Easier maintenance** — adding guidance for a task means editing one file,
  not updating fragments, tags, archive links, and the index.
- **Loss of archive originals and standalone assets** — the full source
  documents in `archive/` and standalone asset files will be removed. Original
  sources can be recovered from git history or re-fetched from their URLs
  (cited in the distilled notes). Source citations will be preserved as inline
  references within the new reference files.
- **Migration effort** — SKILL.md references, AGENTS.md documentation, and the
  Makefile `install-agents` target must all be updated. All standalone asset
  `.md` files will be absorbed or removed. The `adr-reviewer` agent type in
  the custom agents list will no longer exist post-migration.
- **Manage is new** — `manage.md` covers a workflow that was previously only
  implied by SKILL.md guardrails. This is an opportunity to make ADR
  management guidance explicit, but the content must be authored from scratch
  rather than migrated from existing fragments.
