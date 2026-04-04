# Nygard Agent Template

The default ADR template for the author-adr skill (per ADR-0017). Extends
Michael Nygard's original format with inline metadata, dedicated Options and
Quality Strategy sections, and a revision Comments area.

## Template

```markdown
# [Number]. [Short Title]

Date: [YYYY-MM-DD]
Status: [Prototype | Proposed | Accepted | Deprecated | Superseded by ADR-XXXX]
Last Updated: [YYYY-MM-DD]
Links:

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
- [ ] Integration tests
- [ ] User documentation

### Additional Quality Concerns

---

## Comments
```

## Section Guide

| Section | Purpose |
|---------|---------|
| **Inline metadata** | Date, Status, Last Updated, Links — compact key-value pairs replacing `## Status` heading |
| **Context** | Forces at play: technological, political, social, project-local |
| **Options** | Dedicated section for alternatives (supports ecADR ≥2 alternatives criterion) |
| **Decision** | The concrete commitments and design choices; active voice |
| **Consequences** | All outcomes — positive, negative, and neutral |
| **Quality Strategy** | Checklist of quality concerns; inapplicable items struck through (`~~`) |
| **Comments** | Below `---` separator — mutable worksheet for revision Q&A (per ADR-0016) |

## Quality Strategy Items

- **Semantic change checkboxes** — track versioning impact so downstream users
  aren't broken by unversioned changes
- **Testing checkboxes** — feed directly into the implement-adr skill's plan
  generation for automatic testing task creation
- **Backwards Compatible** — explicit signal for breaking change assessment
- **Integration tests** — explicit signal that the decision involves a dependency external to the immediate system (e.g., databases, third-party APIs, message queues, external services). If integration tests do not yet exist for the relevant dependency, an ADR should be scheduled to address the gap. If integration tests already exist, they must be kept up to date with the changes introduced by this decision.
- **User Documentation** — explicit signal to ensure user facing documentation has been updated. This includes README.md, manuals, cli docs, document headers in code, existing docs, or if new docs need to be created to explain usage patterns.
