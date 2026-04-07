# ADR Templates

> All templates are available in [assets/templates/](../assets/templates/).

## Primary: Nygard Agent (default)

The **nygard-agent template** is the default format for ADRs created by this skill (per ADR-0017). It extends the original Nygard format with inline metadata, a dedicated Options section, a Quality Strategy checklist, and a Comments area for revision dialogue.

**Template:** [nygard-agent-template.md](../assets/templates/nygard-agent-template.md)

### Structure

1. **Title** — short noun phrase (e.g., "ADR 1: Use React for frontend")
2. **Inline metadata** — `Date:`, `Status:`, `Last Updated:`, `Links:` as compact key-value lines
3. **Context** — forces at play: technological, political, social, project-local
4. **Options** — dedicated section for alternatives with strengths/weaknesses
5. **Decision** — the response to the forces; active voice imperative ("We will...")
6. **Consequences** — all outcomes, positive and negative
7. **Quality Strategy** — checklist: semantic changes, testing types, backward compatibility. Inapplicable items struck through (`~~`).
8. **Comments** — below `---` separator, mutable worksheet for revision Q&A

### When to Use

- Default choice for all new ADRs
- Agent-developer workflows where ADRs drive implementation
- Teams that need quality planning at decision time

---

## Legacy: Standard Nygard

The original four-section format from Michael Nygard's 2011 article. Still supported for reading — `list` and `status` commands handle both inline `Status:` and `## Status` heading formats.

**Template:** nygard-template.md — with inline guidance: gh-joelparkerhenderson-nygard-template.md

---

## Alternative: MADR (Markdown Architectural Decision Records)

MADR extends the Nygard format with structured tradeoff analysis. The name stands for decisions that *matter* (`[ˈmæɾɚ]`). MADR is the recommended template from the adr GitHub organization.

**Full template:** madr-full-template.md
— official MADR 4.0.0 source: gh-adr-madr-full-template.md

**Minimal template:** madr-minimal-template.md
— official MADR 4.0.0 source: gh-adr-madr-minimal-template.md

### Key Features (Beyond Nygard)

- **Considered options with pros and cons** — enables future readers to understand the tradeoff analysis
- **Decision drivers** — explicit list of forces and concerns
- **Confirmation** — how the team will validate implementation
- **Metadata** — decision makers, date, status, links to related ADRs

### Structure (Full)

1. Title
2. Metadata (status, date, deciders, consulted, informed)
3. Context and Problem Statement
4. Decision Drivers
5. Considered Options
6. Decision Outcome (with justification)
7. Consequences (Good/Bad)
8. Validation/Confirmation
9. Pros and Cons of Options (detailed)
10. More Information

### When to Use

| Variant | Use Case |
|---------|----------|
| MADR Full | Team wants structured tradeoff analysis with explicit option comparison |
| MADR Minimal | Quick capture with considered options but less ceremony than full |

For a section-by-section walkthrough, see the MADR Template Primer (PRACTICES_NOTES.md).

---

## Alternative: Y-Statement

A single-sentence format originated from the Sustainable Architectural Decisions paper by Zdun et al.

**Template:** y-statement-template.md

### Short Form

> In the context of `<use case/user story>`, facing `<concern>` we decided for `<option>` to achieve `<quality>`, accepting `<downside>`.

### Long Form

> In the context of `<use case/user story>`,
> facing `<concern>`,
> we decided for `<option>`
> and neglected `<other options>`,
> to achieve `<system qualities/desired consequences>`,
> accepting `<downside/undesired consequences>`,
> because `<additional rationale>`.

### When to Use

- Inline documentation in code comments or commit messages
- Quick capture during meetings before a full ADR is written
- Seed for a fuller ADR — expand a Y-statement into Nygard or MADR later

---

## Choosing a Template

| Situation | Template |
|-----------|----------|
| Default (agent-developer workflow) | **Nygard Agent** |
| Structured tradeoff analysis needed | **MADR Full** |
| Quick capture, minimal ceremony | **MADR Minimal** or **Y-Statement** |
| Inline documentation in a single sentence | **Y-Statement** |
| Legacy projects using adr-tools | **Standard Nygard** |
| Custom template needed | Place `template.md` in your ADR directory's `templates/` folder |

## Other Templates

Many additional templates exist and are catalogued in joelparkerhenderson's ADR guide, including Alexandrian, Business Case, Merson, Planguage, and Tyree-Akerman formats.
