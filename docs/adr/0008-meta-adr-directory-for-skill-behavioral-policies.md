# 8. Meta-ADR directory for skill behavioral policies

Date: 2026-04-02

## Status

Superseded by [ADR-0011](0011-use-xdg-config-directory-for-user-configuration-state.md)

## Context

As the `implement-adr` skill evolves, behavioral decisions accumulate —
participation modes (ADR-0006, ADR-0007), execution preferences, stage ordering
heuristics, and similar runtime policies. Currently, each behavioral change
requires modifying SKILL.md directly. This is fragile:

- **SKILL.md grows in complexity** — behavioral rules intermix with workflow
  instructions and reference documentation.
- **Behavioral decisions lack their own rationale trail** — a change to a
  heuristic in SKILL.md has no Context/Decision/Consequences record explaining
  why the change was made.
- **Incremental adjustments require touching the skill's core specification** —
  every team-specific preference requires a fork or PR against the skill source.

Behavioral policies are project-specific: one team may prefer Weighted mode
(ADR-0007) while another prefers Guided (ADR-0006). These preferences should
live in the consumer repository, version-controlled alongside the project's own
ADRs, not baked into the skill distribution.

The `author-adr` skill already owns the ADR directory lifecycle (`init`, `new`,
`list`) via `.adr-dir`. Extending it to bootstrap a subdirectory for behavioral
policies is a natural fit.

## Decision

### 1. Store behavioral policies in `<adr-dir>/.meta/`

Behavioral decisions that direct the `implement-adr` skill will be stored as
ADRs in a `.meta` subdirectory inside the project's ADR directory. The location
is derived from `.adr-dir`:

- If `.adr-dir` contains `docs/adr`, the meta directory is `docs/adr/.meta/`.
- If `.adr-dir` contains `decisions`, the meta directory is `decisions/.meta/`.

The dotfile prefix keeps `.meta` hidden from casual directory listings and
prevents ADR tooling (e.g., `ls docs/adr/*.md`) from picking up meta-ADRs as
project-level decisions.

### 2. Convention

- ADRs in `.meta/` use the same Nygard format as project ADRs.
- The `implement-adr` skill reads this directory during Step 0 (Locate ADRs)
  and applies any active behavioral policies before proceeding.
- Each meta-ADR must have status `Accepted` to be applied. `Proposed` or
  `Deprecated` meta-ADRs are read but not enforced.
- Meta-ADRs are numbered independently from project ADRs.

### 3. Bootstrapping (author-adr responsibility)

The `author-adr` skill is responsible for creating the `.meta` directory. This
can be added as a step during `adr init`, or as a standalone Makefile target.
The `author-adr` skill already owns the ADR directory lifecycle, so extending
it to create `.meta` is a natural fit.

### 4. Fallback (implement-adr responsibility)

When `implement-adr` runs and the `.meta` directory does not exist, the skill
does not fail or prompt the user to create it. Instead, it silently stores
behavioral preferences (such as participation level) in the session context.
This means preferences are ephemeral — they apply to the current session only
and must be re-established in future sessions.

The skill should not surface any message about the missing directory. If
`author-adr` has initialized `.meta`, it gets used. If not, the skill operates
without it.

### 5. SKILL.md awareness

Both skill documents will reference this convention:

- `src/skills/implement-adr/SKILL.md` will include a "Behavioral Policies" section
  describing how to read `.meta/`, and the silent session-context fallback.
- `src/skills/author-adr/SKILL.md` will document the `.meta` directory as part of ADR
  directory initialization.

### 6. Alternatives considered

The meta-ADR directory lives in the **consumer repository** (the repo where
the skill is invoked), not inside the skill's own source tree. Different teams
using the same skill can have different policies.

The following directory locations were considered:

| # | Option | Path | Rationale |
|---|--------|------|-----------|
| A | **`docs/policies/`** | `docs/policies/` | Sits alongside `docs/adr/` in the documentation tree. Risk: `policies/` may conflict with a project's own policy documents (e.g., security policies, contribution policies). |
| B | **`docs/adr/policies/`** | `docs/adr/policies/` | Nested under the ADR directory. Risk: ADR tooling may pick up files from subdirectories as project ADRs. Also carries the naming conflict. |
| C | **`<adr-dir>/.meta/`** | e.g., `docs/adr/.meta/` | Derived from `.adr-dir`, co-locates with project ADRs. Dotfile prefix hides it from ADR tooling and casual listings. |
| D | **`docs/skill-policies/`** | `docs/skill-policies/` | Explicit about purpose. Verbose, decoupled from the ADR directory location. |
| E | **`docs/adr/meta/`** | `docs/adr/meta/` | Co-located but visible — ADR tooling interference risk. |

**Chosen:** Option C — `<adr-dir>/.meta/`

- **Derived from `.adr-dir`** — always co-located with the ADR directory
  regardless of where the project stores ADRs.
- **Dotfile prefix** — hidden from `ls` and ADR tooling glob patterns
  (`docs/adr/*.md`), preventing interference with the project ADR log.
- **Bootstrapped by `author-adr`** — ownership is clear; `implement-adr`
  is a consumer, not a creator.
- **Graceful absence** — `implement-adr` silently falls back to session
  context when the directory doesn't exist, so no hard dependency.

## Consequences

**Positive:**

- Meta-ADRs in `<adr-dir>/.meta/` allow the skill's behavior to evolve
  incrementally, with full rationale trails, without touching the core skill
  specification.
- The `.meta` directory is derived from `.adr-dir`, so it always co-locates
  with the project's ADR directory regardless of naming conventions.
- The silent session-context fallback means `implement-adr` works without any
  setup — the `.meta` directory is an opt-in enhancement, not a prerequisite.
- Other skills could adopt the same `.meta` pattern for their own behavioral
  policies.
- Behavioral preferences become version-controlled project artifacts, surviving
  across sessions and team members.

**Negative / Risks:**

- The `.meta` directory is hidden (dotfile), which aids tooling isolation but
  makes it less discoverable for newcomers. Mitigated by `author-adr`
  documenting it as part of initialization.
- Splitting bootstrap responsibility (`author-adr` creates, `implement-adr`
  consumes) introduces a cross-skill dependency. Mitigated by the silent
  session-context fallback — `implement-adr` never hard-fails on absence.
- Meta-ADRs add a new convention that must be documented and maintained.
  Mitigated by using the same Nygard format already in use, and by this ADR
  itself serving as the first example.

**Neutral:**

- Preferences stored in session context are ephemeral by design. Teams that
  want persistence should ask `author-adr` to initialize the `.meta` directory.
- The `.meta` directory convention is independent of the Weighted participation
  mode (ADR-0007). Either can be adopted without the other.
