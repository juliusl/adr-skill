# Nygard Agent Template

The default ADR template for the author-adr skill (per ADR-0017). Extends Michael Nygard's original format with inline metadata, dedicated Options and Quality Strategy sections, and a revision Comments area.

## Template

```markdown
# [Number]. [Short Title]

Date: [YYYY-MM-DD]
Status: [Prototype | Proposed | Ready | Planned | Accepted | Deprecated | Superseded by ADR-XXXX]
Last Updated: [YYYY-MM-DD]
Links:

## Context

## Options

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** [Proceed | Pause for validation | Skipped — <rationale>]

- [ ] All options evaluated at comparable depth
- [ ] Decision drivers are defined and referenced in option analysis
- [ ] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

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
- [ ] Integration tests
- [ ] Tooling
- [ ] User documentation

### Additional Quality Concerns

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** [Ready for review | Needs work | Skipped — <rationale>]

- [ ] Decision justified (Y-statement or equivalent)
- [ ] Consequences include positive, negative, and neutral outcomes
- [ ] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [ ] Links to related ADRs populated

**Pre-review notes:**

---

## Comments
```

## Section Guide

| Section | Purpose |
|---------|---------|
| **Inline metadata** | Date, Status, Last Updated, Links — compact key-value pairs replacing `## Status` heading |
| **Context** | Forces at play: technological, political, social, project-local |
| **Options** | Dedicated section for alternatives (supports implementability ≥2 alternatives criterion) |
| **Evaluation Checkpoint (Optional)** | Advisory gate between Options and Decision — agent assesses readiness, identifies validation needs for prototype-adr, supports skip with rationale (per ADR-0024) |
| **Decision** | The concrete commitments and design choices; active voice |
| **Consequences** | All outcomes — positive, negative, and neutral |
| **Quality Strategy** | Checklist of quality concerns; inapplicable items struck through (`~~`) |
| **Conclusion Checkpoint (Optional)** | Advisory gate between authoring and review — verifies completeness, supports skip with rationale (per ADR-0024) |
| **Comments** | Below `---` separator — mutable worksheet for revision Q&A (per ADR-0016) |

## Quality Strategy Items

- **Semantic change checkboxes** — track versioning impact so downstream users aren't broken by unversioned changes
- **Testing checkboxes** — feed directly into the implement-adr skill's plan generation for automatic testing task creation
- **Backwards Compatible** — explicit signal for breaking change assessment
- **Integration tests** — marks decisions with external dependencies (databases, third-party APIs, message queues). If no integration tests exist for the dependency, schedule an ADR to address the gap. Existing tests must cover changes from this decision.
- **Tooling** — marks decisions affecting build, install, CI/CD, or dev tooling. When checked: update Makefiles, install targets, CI configs, and build/ship/test infrastructure to include new artifacts (e.g., new scripts, new config files). A new component not wired into tooling is invisible to developers and CI.
- **User Documentation** — marks decisions requiring documentation updates. Covers README.md, manuals, CLI docs, code headers, and new docs for new usage patterns.
