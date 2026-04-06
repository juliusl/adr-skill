# 43. Replace vendor prefix with remote in wi-nygard-agent naming

Date: 2026-04-06
Status: Planned
Last Updated: 2026-04-06
Links:
- [ADR-0034: Adopt work-item-referenced naming](0034-adopt-work-item-referenced-naming-for-adr-files.md)
- [ADR-0035: Normalized work item data model](0035-define-normalized-work-item-data-model-for-vendor-agnostic-adr-tooling.md)
- [ADR-0038: Work-item-driven lifecycle orchestration](0038-enable-work-item-driven-adr-lifecycle-orchestration.md)

## Context

ADR-0034 introduces the `wi-nygard-agent` format with a naming convention of `{vendor}-{id}-{slug}.md`, where `{vendor}` is a short prefix identifying the work item source system (`gh`, `ado`, `local`). ADR-0035 extends this with a normalized data model keyed on `vendor`, and ADR-0038's lifecycle command uses `<vendor> <id>` arguments.

The term "vendor" is semantically imprecise for what it actually identifies. The prefix doesn't identify a *vendor* (Microsoft, GitHub Inc.) — it identifies a *remote endpoint* that the tooling connects to. Two observations:

**1. The remote already identifies the vendor.** A GitHub remote (`gh`) points to github.com, which is a GitHub product. An ADO remote (`ado`) points to dev.azure.com, which is a Microsoft product. The vendor is implicit in the remote — you don't need to name the vendor separately because the endpoint already carries that information.

**2. "Remote" is a stronger abstraction than "vendor."** The `local` prefix is not a vendor — it's the absence of a remote. Calling it a "vendor" is a category error. "Remote" naturally accommodates the `local` case: it's the null remote, meaning "no remote endpoint, work item is local-only." The same logic extends to future systems: a Gitea instance, a self-hosted GitLab, or a Jira server are all remotes, not vendors.

The current naming convention (`{vendor}-{id}-{slug}.md`) is not yet implemented — ADR-0034, 0035, and 0038 are all at Proposed status. Renaming now has zero migration cost.

### Decision Drivers

- **Semantic accuracy** — the prefix should describe what it identifies (an endpoint), not an indirect association (a company)
- **Abstraction strength** — the term should naturally accommodate all cases including `local`
- **Zero migration cost** — the change should be made before implementation, not after

## Options

### Option 1: Keep `vendor`

Retain the existing `{vendor}-{id}-{slug}.md` convention from ADR-0034.

**Strengths:**
- No changes needed to any existing ADRs
- "Vendor" is a common term in software procurement context

**Weaknesses:**
- Semantically imprecise — the prefix identifies an endpoint, not a company
- `local` is not a vendor — the term doesn't fit all cases
- Becomes awkward with self-hosted instances (e.g., is a self-hosted Gitea a "vendor"?)

### Option 2: Replace with `remote`

Rename the prefix concept from `vendor` to `remote` across ADR-0034, 0035, and 0038. The naming convention becomes `{remote}-{id}-{slug}.md`.

| Remote | ID Source | Example Filename |
|--------|----------|-----------------|
| `gh` | GitHub Issue number | `gh-42-use-postgresql.md` |
| `ado` | ADO work item ID | `ado-1234-use-postgresql.md` |
| `local` | Short hash (8 chars) | `local-a1b2c3d4-use-postgresql.md` |

The short prefixes (`gh`, `ado`, `local`) stay the same — only the conceptual term changes from "vendor" to "remote."

**Strengths:**
- Semantically precise — the prefix identifies a remote endpoint, and the endpoint implicitly identifies the vendor
- `local` fits naturally — it means "no remote"
- Self-hosted instances map cleanly: a Gitea server is a remote, regardless of who wrote the software
- Aligns with git's `remote` concept, which developers already understand

**Weaknesses:**
- Requires updating three ADRs (0034, 0035, 0038) — but all are Proposed, not implemented
- "Remote" could be confused with `git remote` — though the context (work item systems, not git repos) disambiguates

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

In the context of **naming the work item source prefix in the wi-nygard-agent format**, facing **the semantic mismatch between "vendor" (a company) and what the prefix actually identifies (an endpoint)**, we decided for **replacing `vendor` with `remote` across ADR-0034, 0035, and 0038** (Option 2), to achieve **a semantically precise abstraction that naturally accommodates local, self-hosted, and third-party work item systems**, accepting that **three Proposed ADRs need a find-and-replace before implementation begins**.

### Terminology Change

| Before | After |
|--------|-------|
| `{vendor}-{id}-{slug}.md` | `{remote}-{id}-{slug}.md` |
| `vendor` field in normalized model | `remote` field |
| `gh_adapter`, `ado_adapter` | unchanged (adapter names reference the remote, not the concept) |
| `ADR-{vendor}-{id}` cross-references | `ADR-{remote}-{id}` cross-references |
| `lifecycle <vendor> <id>` CLI args | `lifecycle <remote> <id>` CLI args |
| `vendor-agnostic` (design goal) | unchanged — describes a property, not the prefix |

### Scope of Changes

The rename applies to three Proposed ADRs:

1. **ADR-0034** — naming convention, format script, orchestrator interface
2. **ADR-0035** — normalized data model, adapter interface, type/state mappings
3. **ADR-0038** — lifecycle command arguments, JSONL audit trail schema

All instances of "vendor" that refer to the work item source prefix become "remote." Non-prefix uses of "vendor" (e.g., "vendor-agnostic" as a design goal) are unchanged — they describe a property (independence from any specific vendor), not the remote endpoint concept.

## Consequences

**Positive:**
- The naming convention reads accurately: `{remote}-{id}-{slug}.md` says "this ADR's work item lives at this remote." The remote implicitly identifies the vendor — no information is lost.
- The `local` prefix makes sense: "local remote" is a mild paradox, but "no remote" (which `local` represents) is coherent. "Local vendor" is a poor semantic fit — `local` is not a vendor in any conventional sense.
- Future self-hosted or novel work item systems map cleanly: a Gitea remote is just another remote, not a "Gitea vendor."

**Negative:**
- Three ADRs need a systematic find-and-replace of `vendor` → `remote`. Since none are implemented, the cost is editorial, not technical.
- "Remote" could be confused with `git remote` in conversation. Context disambiguates (work item remote vs git remote), but the potential for confusion exists.

**Neutral:**
- The short prefixes (`gh`, `ado`, `local`) are unchanged. Filenames look identical before and after this change.
- This ADR supersedes the "vendor" terminology in ADR-0034, 0035, and 0038 but does not supersede those ADRs themselves — their decisions remain valid, only the naming of the prefix concept changes.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [ ] Tooling
- [x] User documentation

### Additional Quality Concerns

The primary quality concern is completeness of the find-and-replace — every instance of "vendor" (as work item source prefix) in ADR-0034, 0035, and 0038 must be updated. A grep-based audit after the rename confirms completeness.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This is a terminology refinement, not a design change. The decision, naming convention, data model, and lifecycle command from ADR-0034/0035/0038 remain structurally identical — only the name of the prefix concept changes. The implementation cost is zero because none of these ADRs are implemented yet.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
The `{vendor}` prefix in the wi-nygard-agent naming convention is semantically imprecise — it identifies a remote endpoint, not a company. Renaming to `{remote}` is more accurate because the remote already identifies the vendor implicitly.

**Tolerance:**
- Risk: Low — pure terminology change on unimplemented ADRs
- Change: Low — find-and-replace across three Proposed ADRs
- Improvisation: Low — the rename is tightly scoped

**Uncertainty:**
- Certain: "vendor" identifies an endpoint, not a company; all affected ADRs are Proposed
- Uncertain: whether non-prefix uses of "vendor" (e.g., "vendor-agnostic") should also change

**Options:**
- Target count: 2
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Replace "vendor" with "remote" across ADR-0034, 0035, 0038
- Keep "vendor" as-is

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Should non-prefix uses of "vendor" (e.g., "vendor-agnostic") be renamed too?

**Addressed** — Clarified that non-prefix uses like "vendor-agnostic" are unchanged because they describe a property (independence from any specific vendor), not the remote endpoint concept. Added a row to the Terminology Change table making this explicit.

### Q: Is asserting "local vendor" as a "category error" overstating the case?

**Addressed** — Softened positive consequence from "category error" (a strong logical claim) to "poor semantic fit" — preserves the analytical point without asserting logical impossibility. The Context section retains the stronger analytical framing where the argument is built.

<!-- Review cycle 1 — 2026-04-06 — Verdict: Revise. 2 addressed, 0 deferred, 0 rejected. -->
