# 17. Adopt nygard-agent-template as default ADR format

Date: 2026-04-03

## Status

Accepted

## Context

The author-adr skill currently defaults to the standard Nygard template — a
four-section format (Status, Context, Decision, Consequences) originating from
Michael Nygard's original article. While this format is widely adopted, it has
gaps when used in an agent-developer workflow where ADRs drive immediate
prototyping and implementation:

1. **Alternatives are buried in Context.** The standard Nygard format has no
   dedicated section for options. Alternatives end up as subsections or
   paragraphs within Context, making them harder to find during review. The
   ecADR "Criteria" check (≥2 alternatives compared) relies on locating this
   information, and a dedicated section makes that structural.

2. **No quality planning.** When an ADR drives implementation (via the
   implement-adr skill), there is no place to capture quality-relevant
   considerations — semantic versioning impact, testing strategies, backwards
   compatibility — at decision time. These concerns are deferred to
   implementation planning, where the original context may be lost.

3. **No place for revision dialogue.** ADR-0016 introduced the concept of a
   revision addendum (`## Comments` below a `---` separator). The standard
   Nygard template has no such section, so the revise task must append it
   ad-hoc.

4. **Metadata is scattered.** The standard format uses a `## Status` heading
   with the value on the next line. Date is a separate line. There are no
   fields for links to related ADRs or a last-updated timestamp. A compact
   inline metadata block would improve scannability.

A new template — `nygard-agent-template.md` — has been designed during
dogfooding to address these gaps while preserving the Nygard philosophy of
simplicity and active-voice decision statements.

### Decision Drivers

- **Agent-developer workflow fit** — the template should support the full
  lifecycle: create → review → revise → implement, without ad-hoc extensions.
- **Quality as a first-class concern** — testing and compatibility impact
  should be captured at decision time, not deferred.
- **Simplicity** — retain Nygard's lightweight character. Avoid the ceremony
  of full MADR while adding the sections that have proven necessary.

## Options

### Option 1: Keep standard Nygard as default, offer agent template as alternative

Add the nygard-agent template alongside the existing templates. Users opt in via
config. This preserves backward compatibility but means the skill's review,
revise, and implement workflows must handle two different Nygard variants with
different section structures.

### Option 2: Adopt nygard-agent-template as the sole default

Replace the standard Nygard template as the default. The template becomes the
canonical format for ADRs created by the skill. Existing ADRs in standard
Nygard format remain valid — they just lack the new sections (Options, Quality
Strategy, Comments), which are optional from a parsing perspective.

### Option 3: Merge into MADR

Adopt MADR as the default and extend it with Quality Strategy. Rejected because
MADR's ceremony (Decision Drivers, Pros/Cons per option, Validation) is heavier
than what the agent-developer workflow needs, and it would break compatibility
with existing Nygard ADRs in the project.

## Decision

Adopt `nygard-agent-template.md` as the default ADR format for the author-adr
skill. The template extends Nygard with the following structure:

```markdown
# [ADR Number]: [Short Title]

Date: [creation date]
Status: [Prototype | Proposed | Accepted | Deprecated | Superseded by ADR-XXXX]
Last Updated: [last modified date]
Links:
- [text](url)

## Context
## Options
## Decision
## Consequences

## Quality Strategy
- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [ ] Backwards Compatible

### Additional Quality Concerns

---

## Comments
```

### Key design choices

1. **Inline metadata block** — `Date:`, `Status:`, `Last Updated:`, `Links:`
   as key-value lines at the top, replacing the `## Status` heading. More
   compact and scannable.

2. **`## Options`** — dedicated section for alternatives with strengths and
   weaknesses. Sits between Context and Decision so the reader encounters the
   problem, the options, then the choice.

3. **`## Quality Strategy`** — a checklist of quality concerns that the author
   marks as applicable. Inapplicable items are struck through (`~~`). The
   semantic change checkboxes track versioning impact — ensuring downstream
   users are not broken by unversioned changes. The testing checkboxes feed
   directly into the implement-adr skill's plan generation.

4. **`---` + `## Comments`** — the semantic boundary from ADR-0016, generalized.
   The `---` separates the immutable decision record (above) from the mutable
   worksheet (below). The revise task writes Q&A entries here. The author can
   also add notes manually.

5. **`Prototype` status** — the default initial status in the agent-developer
   workflow. Indicates the ADR is driving local prototyping, not yet ready for
   team review.

## Consequences

- **Unifies the skill's format assumptions** — review, revise, and implement
  workflows can rely on Options, Quality Strategy, and Comments sections
  existing in every new ADR, eliminating ad-hoc detection and fallback logic.

- **Existing ADRs remain valid** — ADRs written in standard Nygard format are
  not broken. They lack the new sections, but the skill can handle their
  absence gracefully (the sections are structurally optional for reading).

- **Inline metadata changes parsing** — tooling that expects `## Status` as
  a heading (e.g., `grep -m1 "^## Status"`) will need updating. This is
  addressed by ADR-0018 (unified tooling refactor).

- **Quality Strategy feeds implementation planning** — the implement-adr skill
  can read the Quality Strategy checklist to automatically generate testing
  tasks, reducing manual effort during plan generation.

- **Template is opinionated** — teams accustomed to standard Nygard or MADR
  must adopt new sections. The strikethrough convention for inapplicable
  quality items mitigates the overhead of unused checkboxes.

- **MADR is no longer a co-default** — the dual-format system (nygard vs madr)
  is simplified to a single default format. MADR templates remain available in
  `assets/templates/` for teams that need full ceremony, but the skill no
  longer prompts for format selection on first use.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [ ] Backwards Compatible

### Additional Quality Concerns

The template change affects every future ADR created by the skill. Existing
ADRs are not migrated — they continue to use their original format.
