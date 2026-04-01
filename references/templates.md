# ADR Templates

> All templates are available in [assets/templates/](../assets/templates/).

## Primary: Nygard ADR (adr-tools default)

The **Nygard template** is the default format used by adr-tools. It is the
simplest and most widely adopted ADR format, introduced by Michael Nygard in
his [2011 blog post](../assets/nygard-documenting-architecture-decisions.md).

**Template:** [nygard-template.md](../assets/templates/nygard-template.md)
— with inline guidance: [gh-joelparkerhenderson-nygard-template.md](../assets/templates/gh-joelparkerhenderson-nygard-template.md)

### Structure

1. **Title** — short noun phrase (e.g., "ADR 1: Use React for frontend")
2. **Status** — proposed, accepted, deprecated, superseded
3. **Context** — forces at play: technological, political, social, project-local
4. **Decision** — the response to the forces; active voice imperative ("We will...")
5. **Consequences** — all outcomes, positive and negative

### When to Use

- Default choice for all ADRs managed by adr-tools
- Teams that value simplicity and low ceremony
- Projects starting with ADRs for the first time

---

## Alternative: MADR (Markdown Architectural Decision Records)

MADR extends the Nygard format with structured tradeoff analysis. The name
stands for decisions that *matter* (`[ˈmæɾɚ]`). MADR is the recommended
template from the [adr GitHub organization](../assets/adr-github-io-madr.md).

**Full template:** [madr-full-template.md](../assets/templates/madr-full-template.md)
— official MADR 4.0.0 source: [gh-adr-madr-full-template.md](../assets/templates/gh-adr-madr-full-template.md)

**Minimal template:** [madr-minimal-template.md](../assets/templates/madr-minimal-template.md)
— official MADR 4.0.0 source: [gh-adr-madr-minimal-template.md](../assets/templates/gh-adr-madr-minimal-template.md)

### Key Features (Beyond Nygard)

- **Considered options with pros and cons** — enables future readers to
  understand the tradeoff analysis
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

For a section-by-section walkthrough, see the
[MADR Template Primer](../assets/PRACTICES_NOTES.md#madr-template-primer).

---

## Alternative: Y-Statement

A single-sentence format originated from the
[Sustainable Architectural Decisions](../assets/zdun-sustainable-architectural-decisions.md)
paper by Zdun et al.

**Template:** [y-statement-template.md](../assets/templates/y-statement-template.md)

### Short Form

> In the context of `<use case/user story>`, facing `<concern>` we decided for
> `<option>` to achieve `<quality>`, accepting `<downside>`.

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
| Using adr-tools (default workflow) | **Nygard** |
| Team wants structured tradeoff analysis | **MADR Full** |
| Quick capture, minimal ceremony | **MADR Minimal** or **Y-Statement** |
| Inline documentation in a single sentence | **Y-Statement** |
| Custom template needed | Place `template.md` in your ADR directory's `templates/` folder |

## Other Templates

Many additional templates exist and are catalogued in
[joelparkerhenderson's ADR guide](../assets/gh-joelparkerhenderson-adr.md),
including Alexandrian, Business Case, Merson, Planguage, and Tyree-Akerman
formats.
