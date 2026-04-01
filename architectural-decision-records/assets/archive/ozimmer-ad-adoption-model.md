---
source: https://ozimmer.ch/practices/2023/04/21/ADAdoptionModel.html
fetched: 2026-04-01
title: "An Adoption Model for Architectural Decision Making and Capturing"
---

### Content Outline

*   [Motivation](#motivation)
*   [Model Dimensions](#model-dimensions)
*   [Level Overview](#level-overview)
*   [Level 1: Undefined and Unconscious](#level-1-undefined-and-unconscious)
*   [Level 2: Ad-hoc and Unstructured](#level-2-ad-hoc-and-unstructured)
*   [Level 3: Encouraged and Supported](#level-3-encouraged-and-supported)
*   [Level 4: Systematic, Selective and Diligent](#level-4-systematic-selective-and-diligent)
*   [Level 5: Optimized and Rigorous](#level-5-optimized-and-rigorous)
*   [Application Hints and Assessment Tool](#application-hints-and-assessment-tool-)
*   [Future Directions](#future-directions)
*   [Related Posts](#-related-posts)
*   [Summary and Outlook](#summary-and-outlook)

Architectural Decision (AD) making and capturing are essential tasks for enterprise architects and solution architects; Architectural Decision Records (ADRs) have become increasingly popular. However, principles and practices for AD management and ADR documentation vary. This post proposes five levels of AD adoption, inspired by maturity models in other areas.

#### Motivation

The scope of architecture work extended its focus from structure to decision rationale over the years, as explained in ["The Software Architect's Role in the Digital Age"](http://ieeexplore.ieee.org/stamp/stamp.jsp?arnumber=7725214) (2016). For instance, searching for "decision" in ["Open Agile Architecture"](https://pubs.opengroup.org/architecture/o-aa-standard-single/), a 2020 standard from The Open Group, returns 164 hits. It refers to one of the popular [ADR formats](https://adr.github.io/), proposed by Michael Nygard in 2011. Note that there are many more templates, and the concept of AD capturing is about 25 years old now; see ["Architectural Decisions — The Making Of"](/practices/2020/04/27/ArchitectureDecisionMaking.html) for a partial history.

When working with clients from different business sectors, we observed various AD management habits and practices. This post derives five levels of adoption from these observations. Such grouping helps with managing complexity, which is one purpose of an adoption (or maturity) model.

The specific future use cases for our proposed _Architectural Decision Adoption Model_ include:

1.  _Learn_ about proven practices: how thoroughly are ADs made and documented elsewhere?
2.  _Reflect_ and _assess_ whether one's current approach(es) are adequate for a given context.
3.  _Identify action points_ en route to improving current practices.

Note that naming matters and is hard… here, we decided for the term "adoption model" and neglected "maturity model" because more is not always better here.[1](#fn:1)

Let's start with the model dimensions that we identified.

#### Model Dimensions

According to our analysis, the AD making and capturing approaches in practice differ in several ways:

1.  _Usage scenario._ Why are Architectural Decision Records (ADRs) created, what is their function? Are they created once (after-the-fact) or do they steer the design work and evolve iteratively?
2.  _Scope and scale_. Do ADRs remain local artifacts or is there a sharing culture? When and how are ADRs shared: within a single team, across teams, organizational unit, and/or enterprise?
3.  _Structure and location._ This aspect deals with documentation. Is a [template](https://adr.github.io/) used to create ADRs? If so, is it minimal or elaborate? How is it established and maintained? Where are the resulting ADR logs located? Are ADRs searchable/findable?
4.  _Process and engagement_, including timing. Are [architectural significance criteria](https://medium.com/olzzio/architectural-significance-test-9ff17a9b4490) established? Has a [Definition of Done (DoD)](https://medium.com/olzzio/a-definition-of-done-for-architectural-decisions-426cf5a952b9) been established? Are decision making steps defined? Can Last Responsible Moment (LRM) or Big Design Upfront (BDUF) mentalities be seen, or a [Most Responsible Moment (MRM)](https://wirfs-brock.com/blog/2011/01/18/agile-architecture-myths-2-architecture-decisions-should-be-made-at-the-last-responsible-moment/) mindset?[2](#fn:2)
5.  _Tool support and automation._ Are off-the-shelf, general-purpose tools or custom–made, AD-specific tools used to create ADRs? Are these tools integrated with other tools for architecture design and project planning? How manual or automated is the capturing, recording, and reuse of the decisions?
6.  _Review culture_. When and how are ADs quality-assured? For instance, are reviews optional or mandatory? Is a formal sign-off and/or external approval required before executing on an AD? Does a date to revisit an ADR have to be defined (or an ADR validity period)?[3](#fn:3)
7.  _Learning_ (education, training). How are architects taught how to apply the chosen AD management practices and tools? Is this self-organized or supported by the organization? What kind of training material is compiled and shared? Are official classes available or even mandatory to attend?

We call these AD management characteristics model _dimensions_ or adoption level criteria. The seven dimensions and their combinations define the five levels of our adoption model.

#### Level Overview

We adopted the five "classic" maturity levels for knowledge management and add(ed) AD domain-specific semantics to them:[4](#fn:4)

1.  _Undefined and unconscious_ aka nonexistent/chaotic
2.  _Ad-hoc and unstructured_ aka initial/reactive/ad-hoc
3.  _Encouraged and supported_ aka defined/functioning/aware
4.  _Systematic, selective and diligent_ aka managed/proactive/integrated
5.  _Optimized and rigorous_ aka effective/automated/ubiquitous/self-managed

As you can tell, we have been unable to decide for a single adjective per level so far. 🤔 Suggestions are welcome!

The following figure visualizes the levels and how they do w.r.t. the adoption criteria and dimensions:

![Five Adoption Levels and Seven Dimensions of Criteria](/assets/images/ADAM-LevelsAndDimensions.jpg)

#### Level 1: Undefined and Unconscious

All architectural decisions are made intuitively and unconsciously, embedded in development activities. The architecture is fully emergent. No ADR is documented in any form, or if any one is documented due to a personal, individual initiative, this happens in free form, for instance using a text editor. There is no review process. The usage of these few ADRs is by chance and by the individual architect (after-the-fact). Learning by doing is the only approach to education and training.

#### Level 2: Ad-hoc and Unstructured

The need for intentional architectural decision making is realized among some teams and a few isolated initiatives have been started; [ADRs are created](/practices/2023/04/03/ADRCreation.html) and used by single projects or teams. However, the decision making and capturing process is still ad-hoc and unstructured. Documentation is created after-the-fact; reviewing is optional. The process is not unified among all teams. As far as tools are concerned, there might be checklists for documentation, and plain markdown or wiki pages might be used. Learning is self-organized and basic.

#### Level 3: Encouraged and Supported

One or more projects/teams use ADRs, still in after-the-fact documentation mode and/or as a means of communication. In many teams, architectural decisions are made systematically, informed and based on _Architecturally Significant Concerns (ASCs)_, including requirements and constraints — desired non-functional qualities in particular. This applies to hard-to-reverse and critical decisions in particular. However, the principle of [Just in-time Architecture (JITA), Just Enough Architecture (JEA)](https://xebia.com/blog/lean-architecture-principle-5-just-in-time-just-enough/) is not consistently applied, and the decision making and documentation is still centralized. Reviews are encouraged, but still initiated in a self-organized way. Minimal templates might be used for documentation, possibly supported by light tools such as the [(M)ADR tools](/practices/2022/11/22/MADRTemplatePrimer.html#madr-tools) or homegrown wiki extensions. A collection of recommended readings about ADs and ADRs exists, awareness sessions take place.

#### Level 4: Systematic, Selective and Diligent

In most teams, ADs are made systematically, depending on their type: informed and driven by ASCs for hard-to-reverse and critical decisions, subconsciously (i.e., embedded in development activities) for easy-to-undo-and-revise decisions. Decision identification is an explicit activity, helping with project planning JITA and JEA are actively and consciously applied in the organization. However, there still is a rather bureaucratic central governance regarding decision making and documentation; teams do not operate autonomously. Elaborate templates are used to document ADRs in a central repository, which is integrated with issue trackers such as Jira or similar tools. [ADR reviews](/practices/2023/04/05/ADRReview.html) are mandatory, ADR-related topic regularly are covered at community events.

#### Level 5: Optimized and Rigorous

Across the whole organization, hard-to-reverse and critical decisions are identified proactively and made systematically in an informed fashion starting from the ASCs. All teams and organizational units start from a curated set of recurring, common ASCs. Not-hard-to-reverse decisions are made subconsciously (i.e., embedded in other design and development activities). JITA and JEA is actively and consciously alive in the culture. The decision making and documenting process is mostly autonomous, and there is not much central governance except for reviewing critical decisions that affect several organizational units. A standardized, often elaborate template is used to capture the decisions into a global repository which is indexed, categorized and searchable. This repository is connected with architectural modeling tools. ADRs are shared and reused across the whole organization, but also with other actors in the same ecosystem (with sensitive and confidential information eliminated). They may be actively managed and take a guiding role as a design evolves. The ADR review process is documented and enforced, taking the AD type into account; AD boards or communities of practice exist to review critical decisions. AD making and documenting are a part of official training programs and onboarding packages.

#### Application Hints and Assessment Tool

At this point, you might be wondering whether and how you can leverage our adoption model with its levels, dimensions and the pointers to existing practices that appear in the descriptions.

The first step is the hardest usually… why don't you just give it a try and answer a "why" question about an existing design informally? Next, you may want to apply one of the existing templates to the decision you just recorded. If you are unhappy with the template, tweak it. Once you are happy with the result, have your ADR reviewed and share it. Once your community has provided feedback and agrees on the initial ADR, make it available to others as a learning example. You just went through the five adoption levels without noticing!

Another way to get started, a little more systematic, is a self assessment. A [spider web chart](https://en.wikipedia.org/wiki/Radar_chart) is well suited to visualize the current and desired adoption levels in a team or an organization:

![Spider web template to show the adoption level of your organization](/assets/images/ADAM-SpiderWebGraphic.jpg)

You may want to fill out the spider web for your sphere of influence and then decide for a level. The questions about the "Model Dimensions" above may serve as a checklist while doing so.

Finally, it is worth noting that not all teams and organizations have to go up to Level 5 in our humble opinion; sometimes, less is more. For instance, a small startup merely consisting of a single and small agile development team might be happy to operate on Levels 2 to 3, maybe move to Level 4 when it becomes successful and starts to grow. The 1000+ person architect community of a globally operating firm in an industry that is subject to audit and regulation might decide to strive for Level 5.

#### Future Directions

One of the barriers of capturing (and thereby reusing) ADRs has been [reported](https://www.sciencedirect.com/science/article/pii/S0164121206001415) by Antony Tang and his co-authors back in "A survey of architecture design rationale" in 2006 — time and effort is needed of development teams, enterprise architects and CIOs:

![Main barriers to documenting ADRs](/assets/images/ADAM-BarrierTable.jpg)

How to eliminate this barrier? Automation, information retrieval, natural language processing and machine learning come to mind.

To best of our knowledge, no mature tool has emerged yet that would retrieve ADRs automatically and with high precision from the vast unstructured documents (such as emails, meeting minutes, strategic documents etc.) and then store it — in spite of some efforts and research prototypes in this direction. However, the software architecture research community reports that there is potential, see for instance the master thesis ["Using machine learning and natural language processing to automatically extract information from software documentation"](https://odr.chalmers.se/server/api/core/bitstreams/54a19669-1376-4a7e-97f6-d68a2e643547/content). Such efforts could become more attractive now that ChatGPT and similar AI/ML offerings have become available and curiously experimented with.

Therefore, an additional Level 6 "Assisted and automated and ambient" can be envisioned for our model: Easy to use, low-threshold tools are used across the organization to capture intentionally made decisions in a template, possibly processing natural language forms or voice recordings. Such tools would convert the plain text into structured information and then commit it to a central repository. Other tools would extract decisions automatically from informal documents (meeting minutes etc.) or unveil unconsciously made decisions in the source code. Leveraging the central repository, a central design administrator can periodically review ADRs and structure them differently if needed.[5](#fn:5) The central repository can then be searched and continuously improved jointly. The book chapter ["Decisions Required vs. Decisions Made. Connecting Enterprise Architects and Solution Architects via Guidance Models"](http://soadecisions.org/download/zimmermann_chap_mistrik_book.pdf) proposes additional capabilities.

When starting to practice AD capturing and sharing and going up the adoption level staircase, the following posts in this blog might be useful:

*   [How to create ADRs — and how not to](/practices/2023/04/03/ADRCreation.html), new in 2023.
*   [Architectural Significance Test and Some Core Decisions](/practices/2020/09/24/ASRTestECSADecisions.html) provides a checklist with five plus two significance indicators (2020).
*   [Architectural Decisions — The Making Of](/practices/2020/04/27/ArchitectureDecisionMaking.html), introducing the rather light Y-statement template (2020).
*   [The Markdown ADR (MADR) Template Explained and Distilled](/practices/2022/11/22/MADRTemplatePrimer.html)
*   [A Definition of Done for Architectural Decision Making](/practices/2020/05/22/ADDefinitionOfDone.html), proposing five "done" criteria (2020).
*   [How to review ADRs — and how not to](/practices/2023/04/05/ADRReview.html), new in 2023.

["A Simple Framework for Architectural Decisions"](https://www.infoq.com/articles/framework-architectural-decisions/) at InfoQ reports that ADR are becoming increasingly popular, for instance at [Amazon Web Services](https://docs.aws.amazon.com/prescriptive-guidance/latest/architectural-decision-records/welcome.html) and [Google Cloud Platform](https://cloud.google.com/architecture/architecture-decision-records).

#### Summary and Outlook

ADRs are relatively easy to get started with, compared to design structure-centric notations and formats. This is one of the reasons they became popular, which also explains why practices for AD making, capturing and sharing vary quite a bit. To be able to benefit from AD recording and sharing, architecting skills and experience are required, which include communication skills and experience.

Adoption levels and dimensions help to find your way and pick the practices that have the best cost-benefit ratio in your context. We proposed seven adoption model dimensions in this post: _usage scenario_, _scope and scale_, _documentation rigor and location_, _process and engagement_, _tool support and automation_, _review culture_, _learning_ and education. The five levels of our AD Adoption Model that we derived from observing current AD practices with the help of these dimensions are:

**1.** **Undefined and unconscious**  
**2.** **Ad-hoc and unstructured**  
**3.** **Encouraged and supported**  
**4.** **Systematic, selective and diligent**  
**5.** **Optimized and rigorous**

We also envisioned an extra Level 6 leveraging next-generation, semi-intelligent ADR automation tools and assistants (but did not go up to 11 😉).

_To get started, how about reflecting how you and your current project do with respect to the seven model dimensions? And then filling out the spider diagram, for the as-is and the to-be situation?_

We hope you find our model and this post useful. Is anything missing? Let us know!

– [Olaf](/about) (Zimmermann) and Mohsen (Anvaari)

_October 10, 2024:_ There is a [Medium version](https://docsoc.medium.com/an-adoption-model-for-architectural-decision-making-and-capturing-1399ab81d802) of this post now.

**Acknowledgements**

Mirko Stocker, Stefan Kapferer reviewed an earlier version of this post and/or shared their experience regarding ADRs with us. Justus Bogner and Christian Ringler reviewed the June 21 version. Several global architect communities at clients, partners and employers provided input and inspiration as well.

**Notes**

1.  Is this naming decision worth capturing with the help of a template or is plain text good enough here? [↩](#fnref:1)
    
2.  In other words, can a "(too) much architecture approach" (i.e., proactive/intentional decision making in all cases) be observed or is a "just enough" approach to architecture design practiced (i.e., proactive/intentional decision making for hard-to-reverse decisions, reactive/reactive decision making for decisions that are easy to reverse)? [↩](#fnref:2)
    
3.  Note the difference between AD and ADR (see [definitions in this post](/practices/2022/11/22/MADRTemplatePrimer.html)). [↩](#fnref:3)
    
4.  For a general model and level names, refer to the conference paper ["Development and Application of a General Knowledge Management Maturity Model"](https://aisel.aisnet.org/cgi/viewcontent.cgi?article=1032&context=pacis2006), Proc. of PACIS 2006 Proceedings, AIS Electronic Library (AISeL). [↩](#fnref:4)
    
5.  Is this a new enterprise architect responsibility? [↩](#fnref:5)
