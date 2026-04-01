# Supplemental Reference

> Consolidated summaries of medium-value assets. These provide useful context,
> strategic guidance, or tool documentation that supplements — but is not
> required for — core ADR authoring and review. Full originals are in `archive/`.
>
> Each fragment is tagged by relevant user intent. When preparing context,
> filter by tag to load only the fragments needed.
>
> **Tags:** `adopt` · `create` · `template` · `tooling`

## Practice Context

### AD Adoption Model
<!-- tags: adopt -->

*Archive: [archive/ozimmer-ad-adoption-model.md](archive/ozimmer-ad-adoption-model.md)*

A five-level organizational maturity model across seven dimensions for assessing
and improving AD practices. Not all organizations need Level 5 — Level 4
(Systematic, Selective & Diligent) is an ideal practical target for most teams.

**5 Levels:** Undefined → Ad-hoc → Encouraged → Systematic → Optimized

**7 Dimensions:** Usage scenario, scope/scale, structure/location,
process/engagement, tool support, review culture, learning/education.

Start simple with markdown files and peer reviews before investing in tooling.

### Decision Criteria Abstraction
<!-- tags: create -->

*Archive: [archive/jacquiread-decision-making-weightings.md](archive/jacquiread-decision-making-weightings.md)*

When defining ADR evaluation criteria, normalize DOWN to the same abstraction
level by decomposing high-level criteria into 3-4 specific sub-criteria. Avoid
weighted scoring systems — they create false precision and maintenance burden.

**Example:** "Maintainability" (too abstract) → API stability, major release
frequency, developer familiarity (concrete, comparable).

### Standardizing Decision Documentation
<!-- tags: create, adopt -->

*Archive: [archive/fabian-keller-documenting-ad.md](archive/fabian-keller-documenting-ad.md)*

At the end of every meeting, ask: "Did we just take an architectural decision
worth documenting?" Assign documentation responsibility immediately. Compares
Nygard's lightweight template with Tyree/Akerman's comprehensive format and
proposes a balanced hybrid.

### Academic Research (AKM)
<!-- tags: adopt -->

*Archive: [archive/ost-akm-page.md](archive/ost-akm-page.md)*

Pointers to Architectural Knowledge Management research: the WICSA 2015 paper
comparing seven AD templates, the SAKM research community, and ADMentor/SE-Repo
tool experiments.

## Alternative Tooling
<!-- tags: tooling -->

The skill standardizes on adr-tools, but teams may use other tools. Key
alternatives and their distinguishing features:

| Tool | Ecosystem | Archive | Key Differentiator |
|------|-----------|---------|-------------------|
| **ADG** | Go CLI | [archive/gh-adr-ad-guidance-tool.md](archive/gh-adr-ad-guidance-tool.md) | Multi-template (Nygard, MADR, QOC); decision models with linking, tagging, validation |
| **dotnet-adr** | .NET Global Tool | [archive/gh-endjin-dotnet-adr.md](archive/gh-endjin-dotnet-adr.md) | 7 built-in template patterns; custom template support |
| **Log4brains** | Node.js | [archive/gh-thomvaill-log4brains.md](archive/gh-thomvaill-log4brains.md) | Docs-as-code with hot reload, static site generation, CI/CD recipes |
| **VS Code ADR Manager** | VS Code | [archive/gh-adr-vscode-adr-manager.md](archive/gh-adr-vscode-adr-manager.md) | GUI editing with linting, snippets, multi-root workspace support |
| **Backstage ADR Plugin** | Backstage | [archive/gh-backstage-adr-plugin.md](archive/gh-backstage-adr-plugin.md) | Cross-entity ADR discovery, search across orgs/repos |
| **Talo** | .NET CLI | [archive/gh-canpolat-talo.md](archive/gh-canpolat-talo.md) | ADRs + RFCs + custom doc types; HTML export with Mermaid rendering |
| **ReflectRally** | SaaS | [archive/reflectrally-homepage.md](archive/reflectrally-homepage.md) | Collaborative web platform with immutable records, async review workflows |
