# 54. Codify policy organization for skill files

Date: 2026-04-06
Status: Accepted
Last Updated: 2026-04-06
Links: ADR-0050 (codified procedural organization — left policies unaddressed)

## Context

ADR-0050 codified that procedural reference files must follow `procedure-template.md`. The template includes a Policies section at the top (before any steps), P-identifiers for each policy, and a summary table. ADR-0050 addressed **procedures** — steps, subsections, identifiers, flow diagrams — but did not address **policies**.

A survey of the four SKILL.md files reveals inconsistent policy handling:

| Skill | Has Policies Section | P-Identifiers | Inline Policies |
|-------|---------------------|---------------|-----------------|
| solve-adr | ✅ Top, P-1–P-4 | ✅ | None |
| implement-adr | ✅ Top, P-1–P-3 | ✅ | 3 enforcement statements buried in I-7b |
| author-adr | ❌ None | ❌ | 3+ critical guardrails buried in procedure steps |
| prototype-adr | ❌ None | ❌ P-0–P-6 are procedure steps, not policies | All enforcement embedded in steps |

Specific problems found:

1. **author-adr** has no `## Policies` section. Critical guardrails — mandatory dispatch compliance, cross-ADR modification guardrail, worksheet presence requirement — are embedded inline in procedure text. An agent reading author-adr loads policies after encountering them mid-execution, not before.

2. **prototype-adr** uses `P-0` through `P-6` as identifiers for its procedure steps. This directly conflicts with AGENTS.md Rule 12: "ALL policies MUST have a clear alpha-numeric identifier prefixed with 'P'." An agent seeing `P-3: Run Experiments` has no structural cue that this is a procedure step, not a policy.

3. **implement-adr** has policies at the top but also has inline enforcement statements in I-7b ("QA execution is mandatory regardless of participation mode") that function as policies but aren't in the Policies section.

**Root cause:** ADR-0050 focused on the nine procedural reference files classified in its File Classification table. SKILL.md files were noted as "already mostly compliant" and exempted. But SKILL.md files have their own policy organization problem that's distinct from reference file organization.

**Decision drivers:**
- AGENTS.md Rule 3: policies must be at the TOP, before any steps
- AGENTS.md Rule 12: policies must have P-identifiers
- solve-adr and implement-adr already demonstrate the target structure
- Agents load rules in document order — policies buried in procedures aren't loaded when decisions are made

## Options

### Option A: Require all SKILL.md files to have a Policies section at the top

All SKILL.md files must have a `## Policies` section positioned after the skill header and before `## Procedure`. The section must contain:

1. **Guardrail statement** — the standard enforcement header
2. **Summary table** — all policies listed with ID and one-line description
3. **Policy details** — each policy with P-identifier and content

**What qualifies as a policy:** A statement is a policy (not an inline enforcement) when it:
- Applies across multiple procedure steps (cross-cutting)
- Constrains behavior regardless of which step is executing
- Would cause a workflow violation if ignored

Inline enforcement statements that are step-specific (e.g., "log the justification before skipping") remain in their step. Cross-cutting enforcement statements get elevated to the Policies section.

**prototype-adr P-identifier conflict:** Rename prototype-adr's P-0 through P-6 to non-P-prefixed step identifiers to free the P-prefix exclusively for policies.

**Strengths:**
- Consistent structure across all four SKILL.md files
- Policies loaded before execution begins — agents see constraints before making decisions
- P-prefix becomes unambiguous — always means policy, never a procedure step
- Extends ADR-0050's structural enforcement approach to the remaining gap

**Weaknesses:**
- Requires editing all four SKILL.md files (two already comply, two need restructuring)
- Prototype-adr's step renaming is a breaking change for any existing references to P-0 through P-6
- Judgment call required: which inline statements qualify as cross-cutting policies

### Option B: Keep current state — skills without policies don't need a section

Some skills genuinely have no cross-cutting policies. author-adr's inline guardrails are step-specific — they apply only in the context where they appear. Adding a Policies section to every SKILL.md adds ceremony without value for skills that have no cross-cutting constraints.

**Strengths:**
- No refactoring work
- Skills that don't need policies aren't forced to invent them

**Weaknesses:**
- Doesn't address the prototype-adr P-identifier conflict — agents can't distinguish policies from procedures
- Doesn't address author-adr's mandatory dispatch compliance being buried in a Configuration subsection
- The "no cross-cutting policies" claim for author-adr is debatable — mandatory dispatch compliance affects every review/revise cycle, not just the configuration step

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — the evidence is from direct survey of all four SKILL.md files.

## Decision

**Option A: Require all SKILL.md files to have a Policies section at the top.**

In the context of agents loading rules in document order and consistently skipping inline enforcement, facing evidence that 2 of 4 SKILL.md files lack policy sections and 1 misuses the P-prefix for procedure steps, we decided to require all SKILL.md files to have a structured Policies section at the top, to achieve consistent policy discovery and unambiguous P-identifiers, accepting the refactoring effort across the four SKILL.md files.

### Requirements

1. **Position:** `## Policies` section must appear after the skill header/description and before `## Procedure`
2. **Guardrail statement:** Include the standard enforcement header: "Ignoring any of the below policies is a runtime violation ESPECIALLY when the user is away and operating autonomously"
3. **Summary table:** All policies listed with ID and one-line description
4. **P-identifier exclusivity:** The P-prefix is reserved for policies. Procedure steps must use other identifier schemes
5. **Elevation criteria:** A statement qualifies for elevation to the Policies section when it is cross-cutting (applies across multiple steps) or constrains behavior regardless of which step is executing
6. **Minimum section:** If a skill genuinely has no cross-cutting policies, the Policies section contains only: "No cross-cutting policies. All enforcement is step-specific." This makes the absence explicit rather than ambiguous

### File-Specific Actions

| Skill | Action |
|-------|--------|
| solve-adr | Already compliant — no changes needed |
| implement-adr | Evaluate inline enforcement in I-7b for elevation; otherwise compliant |
| author-adr | Add Policies section; elevate mandatory dispatch compliance and cross-ADR guardrail |
| prototype-adr | Add Policies section; rename P-0–P-6 step identifiers to non-P-prefix scheme |

## Consequences

**Positive:**
- All four SKILL.md files have identical top-level structure — Policies before Procedure
- P-prefix is unambiguous — always a policy, never a step
- Policies are positioned for agents to encounter before procedure steps begin
- Completes the structural enforcement work started by ADR-0050

**Negative:**
- prototype-adr's step identifiers change — any references to P-0 through P-6 as steps must be updated
- Judgment required on which implement-adr inline statements to elevate

**Neutral:**
- Reference files are unaffected — they follow procedure-template.md which already has a Policies section
- The elevation criteria ("cross-cutting" vs "step-specific") is a guideline, not a rule — edge cases are resolved by the author's judgment
- AGENTS.md P-3 Rule 12 already requires P-identifiers for policies — this ADR makes SKILL.md files comply with the existing rule

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible — behavioral semantics preserved; step renaming changes identifiers but not skill execution logic. Cross-reference updates are tracked in File-Specific Actions.
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

Each refactored SKILL.md must preserve behavioral semantics. The restructuring changes document organization — elevating inline policies to a dedicated section and renaming identifiers — not procedure content. Existing inline enforcement statements that are step-specific stay in their steps.

prototype-adr's step renaming requires updating all cross-references. Search for `P-0` through `P-6` in all prototype-adr references and SKILL.md to ensure no dangling references.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This ADR completes the gap left by ADR-0050. The survey evidence is from the current state of all four SKILL.md files. solve-adr and implement-adr demonstrate the target structure.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
ADR-0050 codified procedural organization but left policies unaddressed. User identified this gap: "adr 50 addressed procedures but did not address policies." Survey confirmed 2/4 SKILL.md files have no Policies section and 1 misuses P-identifiers for procedure steps.

**Tolerance:**
- Risk: Low — extending an existing pattern (solve-adr already demonstrates it)
- Change: Low — 2 files need restructuring, 2 already comply
- Improvisation: Low — follow the existing pattern exactly

**Uncertainty:**
- Known: the target structure (solve-adr, implement-adr Policies sections)
- Known: which files need changes (author-adr, prototype-adr)
- Known: prototype-adr's P-identifier conflict
- Uncertain: which author-adr inline statements qualify as cross-cutting policies (judgment call)

**Options:**
- Target count: 2
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Option A: Require Policies section in all SKILL.md files
- Option B: Keep current state

<!-- Review cycle 1 — 2026-04-07 — Verdict: Accept. 2 findings: R1 backwards compatibility contradiction (Address), S1 consequence rewording (Address). Editor: juliusl-editor-v4. -->
