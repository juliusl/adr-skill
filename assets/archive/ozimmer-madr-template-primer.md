---
source: https://www.ozimmer.ch/practices/2022/11/22/MADRTemplatePrimer.html
fetched: 2026-04-01
title: "The Markdown ADR (MADR) Template Explained and Distilled"
---

### Content Outline

*   [TL;DR](#tldr)
*   [Background: ADs, ASRs and ADRs Defined](#background-ads-asrs-and-adrs-defined)
*   [The MADR Template](#the-madr-template)
*   [Reduced Template ("MADR light")](#reduced-template-madr-light)
*   [Example of Filled Out Template](#example-of-filled-out-template)
*   [MADR Tools](#madr-tools)
*   [Why ADRs and Why Markdown?](#why-adrs-and-why-markdown)
*   [Wrap Up](#wrap-up)
*   [Notes](#notes)

The Markdown Architectural Decision Record (MADR) Template turned five on Nov 22, 2022. MADR stems from documenting architectural decisions and used to be named "Markdown Architectural Decision Records"; but time told that it can be used to document any decision. This post explains the MADR template and its rationale, identifies its essential parts and introduces two emerging tool prototypes.

#### TL;DR

The MADR template evolved from previous ADR templates such as Michael Nygard's [ADR proposal in the Cognitect blog](https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions) and Olaf Zimmermann's [Y-statements](https://medium.com/olzzio/y-statements-10eb07b5a177). Core parts of previous templates include _context_, _decision_ and _consequences_; supplemental parts include _status_, _decision drivers_, _options_ with their _pros and cons_ and _more information_.

Version 3.0 of MADR introduced additional, optional metadata and a "Validation" section. The two sections recording positive and negative "Consequences" were merged to ease copy-paste from the options part.

The community appreciates MADR with 1.000 stars on GitHub. Worth trying!

#### Background: ADs, ASRs and ADRs Defined

Let's introduce the three [TLAs](/index/2020/10/30/DrivenByTLAs.html) AD, ASR and ADR.

The [ADR website](https://adr.github.io/) defines Architectural Decisions (ADs) as this:

> "An Architectural Decision (AD) is a justified software design choice addressing a functional or non-functional requirement that is architecturally significant."

With architectural significance being characterized as:

> "An Architecturally Significant Requirement (ASR) is a requirement that has a measurable effect on a software system's architecture and quality."[1](#fn:1)

An Architectural Decision and a decision log then are:

> "An Architectural Decision Record (ADR) captures a single AD and its rationale; the collection of ADRs created and maintained in a project constitute its decision log."

All these concepts originate from Architectural Knowledge Management (AKM); ADRs capture design decision rationale. That said, the usage of ADRs can be extended to justify and keep track of other decisions, as motivated in the post ["ADR = Any Decision Record? Architecture, Design and Beyond"](/practices/2021/04/23/AnyDecisionRecords.html).

#### The MADR Template

The main contribution of the MADR project and repository is its annotated [MADR template](https://github.com/adr/madr/blob/main/template/adr-template.md) (or M-ADR template). The repository provides pointers to [tools](https://github.com/adr/madr/blob/main/docs/tooling.md) and [background information](https://github.com/adr/madr/blob/main/docs/index.md#overview) as well.

The structure of the full template follows the decision identification, evaluation, making, enforcement journey that a single AD typically goes through. It is inspired by previous templates. The template sections are named as follows:

![Markdown Architectural renamed to Any Decision Record Template](/assets/images/MADR-FullTemplateVisualized.png)

Let's investigate the template sections. Note that the template uses `{this font}` for content placeholders.

**Title.** The title assigns a `{name}` to the AD so that it can be identified and searched for easily. Ideally, it should convey the essence of the problem solved and the solution chosen.

**Metadata.** The metadata elements are:

*   _status:_ `{proposed | rejected | accepted | deprecated | … | superseded by }`
*   _date:_ `{YYYY-MM-DD}` when the decision was last updated
*   _deciders:_ lists everyone involved in the decision
*   _consulted:_ lists everyone whose opinions are sought and with whom there is a two-way communication (such as subject matter experts)
*   _informed:_ lists everyone who is kept up-to-date on progress in one-way communication

These five elements are optional; they can be filled out or removed.

**Context and Problem Statement.** Describes the context and problem statement in a few sentences. One may want to articulate the problem in form of a question or provide an illustrative story that invites to a conversation. Links to collaboration boards or issue management systems can go here too.

**Decision Drivers.** Desired qualities, forces, faced concerns are identified here:

*   `{decision driver 1}`
*   …

**Considered Options.** This section lists the alternatives (or choices) investigated:

*   `{title/name of option 1}`
*   …

The template recommends listing the chosen option first (as a project-wide convention). One needs to make sure to list options that can solve the given problem in the given context (as documented in Section "Context and Problem Statement"). They should do so on the same level of abstraction; a mistake we have seen in practice is that a technology is compared with a product, or an architectural style with a protocol specification and its implementations. Pseudo-alternatives sometimes can be found too, but do not help.

**Chosen Option.** Here, the chosen option is referred to by its title. A justification should be given as well: `{name of option 1}` because `{justification}`. Some examples of justifications are: it is the only option that meets a certain k.o. criterion/decision driver; it resolves a particular force; it comes out best when comparing options. See [this post](/practices/2020/04/27/ArchitectureDecisionMaking.html#good-and-bad-justifications) for more valid arguments.

**Consequences.** This section discusses how problem and solution space look like after the decision is made (and enforced).

Positive and negative consequences are listed as "Good, because …" and "Bad, because …", respectively. An example for a positive consequence is an improvement of a desired quality. A negative consequence might be extra effort or risk during implementation.

**Validation/Confirmation.** This optional section describes how the implementation of/compliance with the ADR is evaluated (aka enforced), for instance by way of a design/code review or a test.

The [Decision-Centric Architecture Reviews (DCAR)](https://www.cs.rug.nl/~paris/papers/IEEESW14b.pdf) technique can be used to confirm/validate an AD. See ["A Definition of Done for Architectural Decision Making"](/practices/2020/05/22/ADDefinitionOfDone.html) for additional hints. This template section corresponds to the 'R' in the Definition of Done proposed in the blog post.

**Pros and Cons of the Options.** Here, the alternatives that address the problem can be explained and analyzed more thoroughly.

The template advises to provide an example or a description of the option. Then, "Good" and "Bad" options properties are asked for. For noteworthy "Neutral" arguments, the template suggests the form `Neutral (w.r.t.)`, because `{argument}`.

**More Information.** Here, one might want to provide additional evidence for the decision outcome (possibly including assumptions made) _and/or_ document the team agreement on the decision (including the confidence level) _and/or_ define how this decision should be realized and when it should be re-visited (the optional "Validation" section may also cover this aspect). Links to other decisions and resources might appear in this section as well.

#### Reduced Template ("MADR light")

Too much for your taste? Many sections are optional. We see five elements as the essence of an ADR:

![Markdown Architectural->Any Decision Record Template](/assets/images/MADR-MinimalTemplateVisualized.png)

This core is quite close to Michael Nygard's ADR proposal from 2011.[2](#fn:2)

If you want to reduce even further (which MADR does not recommend), simply state the chosen option and explain _why_ you picked it in a short single sentence.

#### Example of Filled Out Template

Let's decide for one of three options for logical system decomposition (yes, there are more):

\---
status: Accepted
date: 2022-11-22
deciders: ZIO
---

# AD: System Decomposition into Logical Layers

## Context and Problem Statement

Which concept is used to decompose the system under construction into logical building blocks?

## Decision Drivers

\* Desire to divide the overall system into manageable parts to reduce complexity
\* Ability to exchange system parts without affecting others

## Considered Options

1. Layers pattern 
2. Pipes-and-filters
3. Workflow

## Decision Outcome

We decided to apply the Layers pattern and neglected other decomposition pattern such as pipes-and-filters or workflow because the system under construction and its capabilities do not suggest an organization by data flow or control flow. Technology is expected to be primary driver of change during system evolution. 

### Consequences

\* Good, because the Layers pattern provides high flexibility regarding technology selections within the layers (changeability) and enables teams to work on system parts in parallel.
\* Bad, because there might be a performance penalty for each level of indirection and some undesired replication of implementation artifacts.

## More Information

\* The three decomposition options come from the Cloud Computing Pattern \[Distributed Application\](https://www.cloudcomputingpatterns.org/distributed\_application/).
\* The Layers pattern is featured in POSA Volume 1, see <http://www.dre.vanderbilt.edu/~schmidt/POSA-tutorial.pdf>

A follow-on decision will be required to assign logical layers to physical tiers.

You might want to copy-paste this MADR text into your preferred Markdown tool, for instance to render it.

#### MADR Tools

One can create and edit MADRs without having to install any software. The template can be populated in any text editor, and many people use Markdown extensions/add-ins in their IDEs to capture them. However, the community demanded some more support. Some light tools evolved in response. We feature two such tools briefly here, _ADR Manager (VS Code)_ and _ADR Manager_.

[ADR Manager (VS Code)](https://github.com/adr/vscode-adr-manager-introduction) is a Visual Studio Code Plugin, resulting from Steve Chen's bachelor thesis project at the University of Stuttgart. It supports two modes, basic and professional, and is organized by template sections (such as Decision Outcome):

![ADR Manager (VS Code) in Action](/assets/images/MADR-ADRManager.png)

The second tool, [ADR Manager](https://adr.github.io/adr-manager/#/), is a Web application connecting to a GitHub repository to render all ADRs. It also supports create, read, update, delete operations on ADRs in its editor:

![ADR Manager (Web/GitHub) in Action](/assets/images/MADR-screenshot-adr-manager-GH.png)

This application was developed by Daniel Abajirov, Katrin Bauer and Manuel Merkel in a student research project, also at the University of Stuttgart.

At present, both tools work with Version 2.1.2 of the MADR template and expect any existing ADR files to be formatted accordingly. They create (M-)ADRs in this format as well.

Several more tools are listed at [https://adr.github.io/madr/tooling.html](https://adr.github.io/madr/tooling.html).

#### Why ADRs and Why Markdown?

ADRs help you keep **CALM** because they ease AD making and capturing:

*   Collaborative content creation is enabled.
*   Accountability is supported and unnecessary reconsideration of issues avoided.
*   Learning opportunities are provided, both for newcomers and for the experienced.
*   Management likes them too because it is used to making and executing decisions.

Compared to some Wiki markup languages or proprietary document formats, Markdown has many advantages. As plain text, Markdown is version control- and collaboration-friendly.[3](#fn:3) By design, it forces you to focus on your message (here: ADR content) and logical text flow rather than presentation (page breaks, text formatting and on on) while writing. Tools such as HTML renderers are available, but not mandatory to use.[4](#fn:4)

#### Wrap Up

Architectural decision capturing has become an essential practice on agile and any other projects.[5](#fn:5) The MADR template and tools make it very easy to get going. 1000 stargazers can't be (too) wrong, can they? 😇

In general, one is free to revise the MADR template and adjust optional/required parts according to project/product context and development culture/practices. We recommend doing so early if at all (tools cannot be expected to work with custom versions). Stick to what you have decided for.

Thanks for reading and trying MADR out… as always, feedback and contributions are very welcome! Issues and pull requests in the MADR [GitHub repository](https://github.com/adr/madr) are good ways to reach us.

– Olaf Zimmermann (with [Oliver Kopp](https://github.com/koppor/), MADR project lead and template co-creator)

PS: There is a [Medium version of this post](https://medium.com/olzzio/the-markdown-adr-madr-template-explained-and-distilled-b67603ec95bb).

PPS: Chapter 3 of "Patterns for API Design" in the Addison Wesley Signature Series at Pearson features six AD narratives guiding through the conceptual level of API design. The narratives contain 29 recurring decisions with options and criteria. [Learn more](/patterns/2022/12/16/APIPatternsBookWebsite.html).

**Acknowledgement.** [Justus Bogner](https://xjreb.github.io/) reviewed an intermediate draft of this post and supervised the student projects yielding the two ADR Manager tools.

#### Notes

1.  Usually "measurable effect" is insufficient to decide on significance of a design issue. In response, the blog post ["Architectural Significance Criteria and Some Core Decisions Required"](https://ozimmer.ch/practices/2020/09/24/ASRTestECSADecisions.html) proposes 5+2 criteria for "Architectural Significance": business value/risk, stakeholder concern, quality level, external dependencies, cross-cutting, first-of-a-kind, past troublemaker. [↩](#fnref:1)
    
2.  Michael Nygard's ADR format is not the initial/first one ever created (see [here](/practices/2020/04/27/ArchitectureDecisionMaking.html) for evidence), but a very visible and well adopted one. [↩](#fnref:2)
    
3.  Think git, Gitlab, GitHub with features such as auto-merge, history, release management, CI/CD pipes and so on. [↩](#fnref:3)
    
4.  You might want to check out the universal document converter [pandoc](https://pandoc.org/). [↩](#fnref:4)
    
5.  See how Michael Keeling presents the relation in this [IEEE Software column](https://ieeexplore.ieee.org/document/9801811). [↩](#fnref:5)
