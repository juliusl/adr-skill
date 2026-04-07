# 53. Define TOML-to-Markdown export for wi-full-agent-adr

Date: 2026-04-06
Status: Accepted
Last Updated: 2026-04-06
Links: ADR-0051 (TOML schema), ADR-0052 (format tooling), ADR-0017 (nygard-agent template), ADR-0034 (work-item naming)

## Context

The wi-full-agent-adr format stores ADRs as TOML files (ADR-0051). TOML is optimized for tooling but is less readable for humans in contexts where Markdown renders natively — GitHub file browsing, documentation sites, PR reviews.

The adr-atelier roadmap states: "Enables a straight-forward conversion to Markdown which enables optional storing of ADR's as documents without cluttering the document w/ mutable data."

The key phrase is "without cluttering the document w/ mutable data." The TOML file contains mutable fields (status, plan progress, deliverables checklist, revision entries) alongside the immutable decision record. The Markdown export should produce a clean document — the decision record as it would appear in a documentation site — stripping the operational state.

**Decision drivers:**
- Human readability in contexts where Markdown renders (GitHub, docs sites)
- Clean separation between the decision record (immutable) and operational state (mutable)
- The export must be a subcommand of the format tooling (ADR-0052), not a separate tool
- The output should be recognizable as a nygard-agent-style ADR to maintain consistency with the existing decision log

## Options

### Option 1: Selective export — decision record only

Export only the decision record sections (Context, Options, Decision, Consequences) and stable metadata (title, date, status, links). Omit mutable operational state (plan, deliverables progress, revision entries, checkpoint checklists).

**Output format:**
```markdown
# {remote}-{id}. {title}

Date: {date}
Status: {status}
Last Updated: 2026-04-06
Work-Item: {remote}#{id}
Links: {links}

## Context

{context.body}

## Options

### {option.name}

{option.body}

## Decision

{decision.body}

## Consequences

**Positive:**
- {consequence}

**Negative:**
- {consequence}

**Neutral:**
- {consequence}
```

**Strengths:**
- Clean, focused document suitable for documentation sites
- No mutable state to go stale
- Output matches the existing nygard-agent template structure

**Weaknesses:**
- Loses Quality Strategy, checkpoint details, and plan data — users who want the full picture must read the TOML file
- Two representations can drift if someone edits the Markdown export

### Option 2: Full export — complete ADR with all sections

Export the entire TOML content as Markdown, including checkpoints, quality strategy, plan, and comments. This is a faithful 1:1 conversion.

**Strengths:**
- No information loss — the Markdown file is a complete representation
- Can serve as a backup or migration path back to Markdown

**Weaknesses:**
- The mutable operational state (plan status, deliverables progress) clutters the document
- Contradicts the roadmap's stated goal of "without cluttering the document w/ mutable data"
- Checkpoint checklists and plan stages are awkward in Markdown

### Option 3: Configurable export with profiles

Support multiple export profiles: `--profile=record` (Option 1), `--profile=full` (Option 2), `--profile=review` (includes Quality Strategy and checkpoints but not plan/comments).

**Strengths:**
- Flexible — different use cases get different outputs
- Users choose the right level of detail

**Weaknesses:**
- More code to maintain — each profile needs testing
- Decision paralysis — which profile should be the default?
- Over-engineering for a feature that may only be used in one mode

## Evaluation Checkpoint
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None.

## Decision

In the context of providing human-readable Markdown from TOML ADR files, facing the need for clean documentation without mutable operational state, we chose **Option 1 (selective export — decision record only)** over Option 2 (full export) and Option 3 (configurable profiles) to achieve clean, focused documents suitable for documentation sites, accepting that users who need the full operational picture must read the TOML source.

**Export rules:**

1. **Subcommand:** `adr-format export <remote> <id>` writes Markdown to stdout. Pipe to file as needed.
2. **Included sections:** meta (title, date, status, links, work-item), context, options, decision, consequences.
3. **Excluded sections:** evaluation_checkpoint, conclusion_checkpoint, quality_strategy, plan, comments, deliverables.
4. **Consequences grouping:** Positive, negative, and neutral consequences are grouped under labeled headers, matching the nygard-agent convention.
5. **Output is read-only** — the Markdown export is a rendered view. Edits go back to the TOML source. The export does not include a warning header — the `.toml` source file is the authoritative artifact.
6. **Idempotent** — running export twice produces identical output if the TOML source hasn't changed.
7. **Work-Item metadata** — the `Work-Item: {remote}#{id}` header line is derived from the TOML `work_item` field (ADR-0034 naming convention). If the TOML source has no `work_item` value, the line is omitted from the export.
8. **Empty sections are omitted** — if an optional section has no content (e.g., no negative consequences, no links), the section header is omitted entirely rather than rendered empty. This keeps the output clean and produces stable snapshots.

## Consequences

**Positive:**
- Documentation sites get clean, focused ADR documents without operational clutter
- The export matches the nygard-agent Markdown format, maintaining visual consistency across the decision log
- Simple implementation — straightforward template rendering from deserialized TOML structs

**Negative:**
- Quality Strategy, checkpoint details, and plan data are only visible in the TOML source
- If the exported Markdown is committed alongside the TOML source, there's a risk of forgetting to re-export after changes (could be mitigated by CI checks or pre-commit hooks if the need arises)

**Neutral:**
- The export subcommand can be extended with profiles later (Option 3) if demand emerges — the selective export is a solid default

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [ ] User documentation

### Additional Quality Concerns

The Markdown output format should be tested with snapshot tests — serialize a known TOML ADR to Markdown and compare against a golden file. This catches unintentional formatting regressions.

## Conclusion Checkpoint
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This is the smallest of the three ADRs in the milestone. It depends on ADR-0051 (schema) and ADR-0052 (tooling) being implemented first.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
The roadmap says the TOML format should enable "a straight-forward conversion to Markdown which enables optional storing of ADR's as documents without cluttering the document w/ mutable data." This drives toward a selective export that strips operational state.

**Tolerance:**
- Risk: Low — this is a rendering/output feature
- Change: Low — additive, no existing behavior changes
- Improvisation: Low — the roadmap specifies the direction

**Uncertainty:**
- Certain: Markdown export is needed, mutable data should be excluded from the export
- Uncertain: Whether the export should be committed alongside TOML files or generated on demand

**Options:**
- Target count: 2-3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Selective export (decision record only)
- Full export (1:1 conversion)
- Configurable profiles

### Revision Q&A — Review round 1 (Accept with minor suggestions)

**Finding 1 (Link to ADR-0034) — Addressed.**
Added ADR-0034 (work-item naming) to the Links header. The template uses `{remote}-{id}` which originates from that convention, and ADR-0051 already links to it. This ADR should document the same dependency.

**Finding 2 (Consequence wording: "in later milestones") — Addressed.**
Reworded from "mitigated by CI checks or pre-commit hooks in later milestones" to "could be mitigated by CI checks or pre-commit hooks if the need arises." There's no milestone or ADR that specifically plans this mitigation — the original phrasing implied a commitment that doesn't exist.

**Finding 3 (Work-Item metadata derivation / absent case) — Addressed.**
Added export rule #7 clarifying that the `Work-Item` line is derived from the TOML `work_item` field and is omitted when absent. An implementer shouldn't have to guess this behavior.

**Finding 4 (Empty sections behavior) — Addressed.**
Added export rule #8 specifying that empty optional sections are omitted entirely. This matters for snapshot testing (mentioned in Quality Strategy) — the golden file needs deterministic behavior for empty sections.
