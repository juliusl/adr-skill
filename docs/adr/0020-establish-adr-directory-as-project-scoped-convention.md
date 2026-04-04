# 20. Establish .adr directory as project-scoped convention

Date: 2026-04-03
Status: Planned
Last Updated: 2026-04-03
Links:

## Context

The ADR skills (`author-adr` and `implement-adr`) currently produce artifacts — ADR documents, implementation plans, review verdicts, revision dialogues — but none of this operational data is captured in a structured, analyzable format. Once a session ends, the qualitative signal about skill performance is lost or scattered across ephemeral session context.

**The problem:** There is no project-scoped directory convention where skills can persist structured data — operational telemetry, intermediate artifacts, project-level preferences — across sessions.

**Why this matters:** Being able to analyze skill performance qualitatively would enable fine-tuning the skill guides. For example:
- **Review/revision cycles** — how often does a review finding get rejected during revision? If certain categories of findings are consistently rejected, the review guidance may be over-sensitive or poorly calibrated.
- **Revision wording** — when a revision's proposed wording is rejected, what does the user prefer instead? Patterns here could improve the revision workflow.
- **Implementation changelogs** — when `implement-adr` executes a plan, a structured changelog of commits and tasks would allow visualization and retrospective analysis of how plans translate to code.

**Current limitations:**
- The Copilot CLI does not expose its internal session store (SQLite) to agent skills. Skills cannot read or write to it programmatically.
- Relying on session context alone is inefficient (context window consumption) and lossy (data doesn't survive session boundaries).
- User-scoped config (`~/.config/adr-skills/preferences.toml`, per ADR-0011/0012/0014) is designed for preferences, not operational data. It is user-scoped, not project-scoped.
- Implementation plans in `docs/plans/` (per ADR-0005) track task completion but don't capture the meta-level signal about how the skill itself performed.

**Scope:** This ADR establishes a project-scoped directory convention for the ADR skills ecosystem. While the motivating use case is qualitative performance analysis (review cycles, revision outcomes, implementation changelogs), the convention is intentionally broader — it provides a home for any project-scoped data that doesn't belong in `docs/adr/` (decision records), `docs/plans/` (implementation plans), or `~/.config/adr-skills/` (user preferences).

### Decision Drivers

- **Project-scoped** — state must live with the project (not user-global), so different projects have independent data
- **Broadly useful** — the store should serve more than just analytics; project-scoped preferences and intermediate artifacts are also valid uses *(emerged during option evaluation)*
- **Structured and analyzable** — must support machine-readable formats (e.g., JSONL, SQLite) suitable for querying and visualization
- **Append-friendly** — skill sessions add data incrementally; concurrent writes should not corrupt the store
- **Low overhead** — writing state should not consume significant context window or session time
- **Portable** — no dependencies on Copilot CLI internals or platform-specific features
- **Hybrid git strategy** — raw data files are gitignored to avoid clutter; processed reports can be committed for shared visibility *(emerged during option evaluation)*

## Options

### Option 1: JSONL event log in a project directory

Create a `.adr-state/` directory at the project root. Each skill writes structured events as newline-delimited JSON (JSONL) files, organized by concern (e.g., `reviews.jsonl`, `revisions.jsonl`, `implementations.jsonl`). The directory is gitignored by default but can be tracked if the team wants shared analytics.

**Strengths:**
- Append-only writes — trivially safe for sequential skill sessions
- Human-readable and git-diffable if tracked
- Queryable with standard tools (`jq`, `grep`, Python/pandas)
- No dependencies — just file I/O from shell scripts or agent instructions
- Naturally project-scoped — lives in the repo root

**Weaknesses:**
- No query engine built in — analysis requires external tooling
- Multiple JSONL files can diverge in schema over time without enforcement
- Large files become unwieldy without rotation or compaction
- Not relational — cross-cutting queries (e.g., "reviews that led to revisions that led to implementation changes") require joining across files manually

### Option 2: SQLite database in a project directory

Place a SQLite database at `.adr-state/state.db`. Skills write structured records via SQL. The database is gitignored (binary files don't diff well). A schema migration mechanism ensures tables evolve safely.

**Strengths:**
- Rich querying with SQL — joins, aggregations, window functions
- Single file — easy to back up, copy, or share
- ACID transactions — safe even if a session crashes mid-write
- Schema enforcement — columns and types are explicit
- Mature ecosystem — every language has a SQLite driver

**Weaknesses:**
- Binary format — cannot be meaningfully git-tracked or diffed
- Requires SQLite tooling — the agent must invoke `sqlite3` or a wrapper script
- Schema migrations add complexity — need a versioning strategy
- Heavier than JSONL for simple append-only use cases
- Concurrent writes from multiple agent sessions (rare but possible) need WAL mode

### Option 3: XDG data home with project key

Use `~/.local/share/adr-skills/<project-id>/` (following XDG Base Directory Specification for data files, complementing the existing `~/.config/` for preferences per ADR-0011). The project key could be derived from the git remote URL or repo root path. Data files within can be JSONL or SQLite.

**Strengths:**
- Follows XDG conventions — `~/.config` for config, `~/.local/share` for data
- Keeps the project tree clean — no dotfiles or gitignore entries needed
- Naturally separates data from source — no risk of accidentally committing analytics
- Project isolation via unique key

**Weaknesses:**
- Not truly project-scoped — lives in the user's home directory, so it's user-scoped with a project partition
- Not portable across machines or collaborators — data doesn't travel with the repo
- Project key derivation is fragile — renaming the repo or changing remotes breaks the mapping
- Adds a second storage location beyond `~/.config/adr-skills/` — more surface area to manage
- Invisible to the project — harder to discover and reason about

## Decision

In the context of **needing project-scoped state for qualitative analysis of skill performance**, facing **the absence of a persistent store accessible to agent skills across sessions**, we decided for **a `.adr/` directory convention at the project root with a `var/` subdirectory for transient data files**, and neglected **SQLite (requires dedicated tooling and schema migrations, heavier than needed for append-only event logging) and XDG data home (user-scoped, not project-scoped)**, to achieve **a simple, portable, append-friendly store that lives with the project and supports hybrid git tracking**, accepting that **analysis requires external tooling and schema consistency is the responsibility of the writing skill**.

### Structure

```
.adr/                  # project-scoped root (git-tracked)
├── var/               # variable/transient data (gitignored via .adr/.gitignore)
│   ├── reviews.jsonl  # example: review cycle events
│   ├── revisions.jsonl
│   └── ...
└── (future uses: processed reports, project-scoped preferences, etc.)
```

### Conventions

1. **`.adr/` is git-tracked** — its existence and structure are visible and discoverable in the repo.
2. **`.adr/var/` is gitignored** — a `.gitignore` inside `.adr/` ignores the `var/` subdirectory. Raw data files are transient and local.
3. **JSONL format** — each event is a single JSON object per line, enabling simple appends and streaming reads.
4. **Skills write, tools read** — skills append events during sessions; analysis tooling reads the files offline. The specific analysis tools and reports are out of scope for this ADR.
5. **Future extensibility** — `.adr/` can host additional project-scoped concerns (reports, preferences, caches) as they emerge. Each new use should be captured in its own ADR.
6. **Supersedes `.adr-dir`** — the legacy `.adr-dir` file (used by adr-tools to locate `docs/adr/`) is no longer necessary. Any project-scoped configuration previously stored in ad-hoc root files should live in `.adr/` going forward. Scripts that currently read `.adr-dir` for path discovery will need to be updated to use a replacement mechanism within `.adr/` (e.g., a config file); the specific migration is deferred to the implementing ADR.
7. **Bootstrap via `init-data` target** — a new `init-data` Makefile target creates `.adr/`, `.adr/var/`, and `.adr/.gitignore`. This is separate from the existing `init` target (which bootstraps `docs/adr/` and the ADR format) because the data directory is not required for core skill operation — users opt in to the telemetry/state infrastructure explicitly.

## Consequences

**Positive:**
- Skills gain a durable, project-local place to record operational data — review cycles, revision outcomes, implementation traces — enabling qualitative performance analysis for the first time.
- The `.adr/` convention is extensible: future ADRs can define new uses (project preferences, caches, reports) without structural changes.
- Hybrid git strategy keeps the repo clean (transient data gitignored) while making the convention itself discoverable (directory tracked).
- JSONL is dependency-free — skills can append with `echo '{}' >> file.jsonl`, no drivers needed.

**Negative:**
- Introduces a new project-root dotdir (`.adr/`) that teams must understand and maintain.
- Schema consistency across JSONL files is unenforced — skills must self-discipline or a future ADR must define schemas.
- Analysis requires external tooling (`jq`, Python, etc.) — there is no built-in query layer.
- Cross-file queries (joining reviews with revisions) require manual orchestration until dedicated analysis tooling is built.
- Concurrent writes from multiple agent sessions to the same JSONL file are not guaranteed safe; a future ADR should address this when defining the specific tool that writes to `.adr/var/`.

**Neutral:**
- This ADR establishes the directory convention only. The specific event schemas, analysis workflows, and reporting formats are deferred to downstream ADRs.
- Data in `.adr/var/` is local to each developer. Team-wide aggregation, if needed, is a downstream concern — the convention does not prescribe how local data is shared or centralized.
- `.adr/` supersedes ad-hoc project-root files like `.adr-dir`. Existing `.adr-dir` files can be removed at the project's discretion.

## Quality Strategy

- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] User documentation

### Additional Quality Concerns

This ADR introduces a directory convention only — no code changes. Quality concerns are primarily around documentation (ensuring SKILL.md and references explain the `.adr/` convention) and backwards compatibility (existing projects without `.adr/` continue to work normally via graceful degradation).

---

## Comments

---

## Revision Addendum

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Does the ADR's scope match between the Options and Decision sections?

**Addressed** — Broadened the title, problem statement, and scope paragraph to explicitly frame `.adr/` as a general-purpose project-scoped directory convention, not just a state store for analytics. Title changed from "Use .adr directory convention for project-scoped state" to "Establish .adr directory as project-scoped convention."

### Q: Is the SQLite rejection rationale valid given that data files are gitignored?

**Addressed** — Removed "binary, not git-diffable" from the SQLite rejection. Replaced with "requires dedicated tooling and schema migrations, heavier than needed for append-only event logging."

### Q: Should the naming overlap between `.adr/` and `docs/adr/` be documented as a risk?

**Rejected** — `docs/adr/` stores documentation (ADRs are documentation). `.adr/` stores data relevant to the authorship and usage of files in `docs/adr/`. The dotfile convention is a clear signal that the user could choose to gitignore this directory, whereas `docs/adr/` makes it clear that ADRs should be stored in source control.

### Q: Should the ADR include a revisit trigger?

**Rejected** — Not needed at this stage. The convention is intentionally minimal; revisit conditions will emerge naturally when downstream ADRs define specific tools and schemas.

### Q: Should concurrent write behavior be documented?

**Addressed** — Added as a negative consequence: concurrent writes are not guaranteed safe, deferred to a future ADR when the specific tool is defined.

### Q: Should the ADR include scale estimates for expected data volume?

**Rejected** — Scale estimates belong in the ADR that adds the specific tool. The hybrid git strategy (gitignored `var/`) already mitigates the main risk of unbounded data in the repo.

### Q: Is the data aggregation model clear for team-wide analysis?

**Addressed** — Added as a neutral consequence: data in `.adr/var/` is local to each developer; team-wide aggregation is a downstream concern.

<!-- Round 2 -->

### Q: Is the filesystem write capability validated for agent skills?

**Rejected** — This is obvious from how skills work; skills have full shell access and can write to the project directory.

### Q: How does `.adr/` relate to the legacy `.adr-dir` file?

**Addressed** — Added convention 6: `.adr/` supersedes `.adr-dir`. Any project-scoped configuration previously in ad-hoc root files should live in `.adr/` going forward. Added as neutral consequence.

### Q: How is `.adr/` bootstrapped in a project?

**Addressed** — Added convention 7: a new `init-data` Makefile target creates `.adr/`, `.adr/var/`, and `.adr/.gitignore`. Separate from `init` (which bootstraps `docs/adr/`) because the data directory is opt-in.

<!-- Round 3 -->

### Q: How is the ADR directory path discovered once `.adr-dir` is removed?

**Addressed** — Added to convention 6: scripts that read `.adr-dir` will need updating to use a replacement mechanism within `.adr/`; specific migration deferred to implementing ADR.
