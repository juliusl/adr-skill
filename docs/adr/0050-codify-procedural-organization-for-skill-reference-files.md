# 50. Codify Procedural Organization for Skill Reference Files

Date: 2026-04-06
Status: Planned
Last Updated: 2026-04-06
Links: ADR-0049 (roadmap scenario — session where violations were discovered)

## Context

During the implementation of ADR-0049 (S-2 Roadmap scenario), multiple procedure violations were observed and corrected iteratively across this session:

1. **Agent skipped `/implement-adr`** by rationalizing "these are just documentation files, simple enough to do directly." The mandatory instructions only guarded against the user's framing, not the agent's own rationalization.
2. **Agent bypassed `auto_commit = true`** from user preferences by citing AGENTS.md git policy — but the preference file IS explicit user delegation.
3. **Audit found Rule 6 false-passes** — the audit agent saw domain-specific tables (criteria checklists) and marked Rule 6 satisfied, because "items with an identifier" was vague enough to match any table.
4. **53 subsections lacked identifiers** (Rule 11) — agents could skip sub-tasks and no one could reference which specific sub-task was violated.
5. **8 policy sections lacked P-identifiers** (Rule 12) — sections like "Before Making Changes" were treated as informational reference rather than mandatory policy.
6. **Guardrail patches were insufficient** — adding "if skipped, log justification" text did not prevent skipping because the document structure itself didn't make the skip points visible or auditable.

The root cause: skill reference files evolved organically. Each file had its own structure. Guardrail text was added but the document organization didn't enforce it. An agent reading a file with no step index, no policy section, and unnumbered subsections has no structural cues telling it "you must visit this section."

**Decision drivers:**
- Structural templates prevent violations better than textual warnings
- Procedure-template.md and worksheet-template.md already exist and embody Rules 1–12
- All 8 procedural reference files were patched in this session but not restructured to the template
- SKILL.md files already mostly conform (procedure tables, routing diagrams)

## Options

### Option A: Require all procedural reference files to follow procedure-template.md

All reference files that contain steps (create.md, review.md, revise.md, manage.md, problem.md, roadmap.md, plan-review.md, qa-planning.md, planning-practices.md) must be restructured to match the `procedure-template.md` structural template. Files that are purely informational (templates.md, tooling.md, testing-guidelines.md, cost-estimation.md, isolation.md, observation.md, profiles.md) are exempt.

The template enforces:
- Guardrail statement at the top
- Policies section before any steps (if the file has local policies)
- Procedure section with step index table + flow diagram
- Step sections with identifiers
- Subsections with parent-relative identifiers
- Conditional steps explicitly marked

Worksheet elements within procedures (like the Draft Worksheet in A-1) must follow `worksheet-template.md` structure.

**Strengths:**
- Every procedural file has the same shape — agents learn one pattern
- Step index table makes skips immediately visible — "Step 3a is in the index but wasn't visited"
- Policy sections at the top ensure rules are loaded before execution
- Template provides structural enforcement, not just textual warnings
- Existing templates are already created and proven

**Weaknesses:**
- Significant refactoring effort across 9 files
- Some files have unique structures that don't map cleanly to the template (e.g., manage.md is task-based rather than sequential)

### Option B: Keep current patched state — guardrails, index tables, and identifiers are sufficient

The current state after this session's patches already has:
- Guardrail statements in all 8 files
- Step index tables in all 8 files
- Subsection identifiers in all 8 files
- P-identifiers on all AGENTS.md policies

This option says the patches are sufficient without full template conformance.

**Strengths:**
- No additional refactoring work
- Existing patches address the specific violations found

**Weaknesses:**
- Patches were applied mechanically — each file still has its own idiosyncratic structure
- No structural enforcement that future edits maintain compliance
- The patches are the minimum fix, not the systematic solution
- Future files will be written ad-hoc unless there's a requirement to use the template

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — the evidence is from this session's direct experience.

## Decision

**Option A: Require all procedural reference files to follow procedure-template.md.**

In the context of agents repeatedly skipping procedural steps despite textual guardrails, facing evidence that document structure matters more than warning text, we decided to require all procedural reference files to conform to procedure-template.md (and worksheet-template.md for intake elements), to achieve structural enforcement of P-3's 12 rules, accepting the refactoring effort across 9 files.

The template is the policy's enforcement mechanism. P-3 defines what must be present; the template defines where it goes. Following the template structurally prevents violation of Rules 3, 4, 6, 7, 8, 9, 11, or 12 — the placeholders force compliance.

### File Classification

| Skill | File | Classification |
|-------|------|----------------|
| author-adr | create.md | Procedural |
| author-adr | review.md | Procedural |
| author-adr | revise.md | Procedural |
| author-adr | manage.md | Procedural |
| author-adr | templates.md | Exempt (informational) |
| author-adr | tooling.md | Exempt (informational) |
| implement-adr | planning-practices.md | Procedural |
| implement-adr | plan-review.md | Procedural |
| implement-adr | qa-planning.md | Procedural |
| implement-adr | testing-guidelines.md | Exempt (informational) |
| implement-adr | cost-estimation.md | Exempt (informational) |
| solve-adr | problem.md | Procedural |
| solve-adr | roadmap.md | Procedural |
| prototype-adr | isolation.md | Exempt (informational) |
| prototype-adr | observation.md | Exempt (informational) |
| prototype-adr | profiles.md | Exempt (informational) |

9 procedural files require template conformance. 7 informational files are exempt.

## Consequences

**Positive:**
- Every procedural file has identical structure — one pattern to learn, one pattern to audit
- Step index tables make skips visible by listing every step that must be visited
- Future files start from the template — compliance is the default, not an afterthought
- Policy sections at the top ensure rules are loaded before step execution

**Negative:**
- 9 files need restructuring to match the template
- Some files have structures that don't map 1:1 (manage.md is task-based, not sequential) — these need judgment about how to adapt

**Neutral:**
- SKILL.md files are already compliant and do not need changes
- Informational references (templates.md, tooling.md, cost-estimation.md, testing-guidelines.md, isolation.md, observation.md, profiles.md) are exempt — they don't contain procedures
- The template may evolve as more patterns are discovered
- Enforcement of "future files start from template" requires updating AGENTS.md P-10 to reference procedure-template.md as the mandatory starting point for new procedural reference files

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

Each refactored file must preserve its behavioral semantics exactly. The restructuring changes document organization, not procedure content. A diff of the pre/post content (ignoring section ordering and formatting) should show only additions (guardrails, index tables) and heading renames (identifiers), not content deletions or rewrites.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** The evidence for this decision comes directly from this session — 6 distinct violations observed and corrected. The procedure-template.md and worksheet-template.md already exist at the repo root. Revisit after restructuring the first 3 procedural files to validate the template mapping approach and confirm the effort estimate.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Agents skip procedural steps despite guardrail text. The user observed this across multiple corrections in one session and decided to codify procedures using structural templates rather than relying on textual warnings. The templates (procedure-template.md, worksheet-template.md) already exist.

**Tolerance:**
- Risk: Low — templates already exist, pattern is proven in SKILL.md files
- Change: Medium — 8 files need restructuring
- Improvisation: Low — follow the template closely

**Uncertainty:**
- Known: the template structure (procedure-template.md)
- Known: which files are procedural vs informational
- Known: what violations look like (6 examples from this session)
- Uncertain: how manage.md (task-based, non-sequential) maps to the sequential template

**Options:**
- Target count: 2
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Option A: Require template conformance — structural enforcement
- Option B: Keep current patches — textual enforcement only

<!-- Review cycle 1 — 2026-04-06 — Verdict: Revise. 3 findings: R1 incomplete file classification (Address), R2 manage.md adaptation (Reject — scope discipline), R3 enforcement gap (Address). Editor: juliusl-editor-v4. -->
<!-- Review cycle 2 — 2026-04-06 — Verdict: Accept. All addressed findings verified adequate; R2 rejection justified. -->
