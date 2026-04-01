---
source: https://ozimmer.ch/practices/2023/12/01/ADDefinitionOfReady.html
fetched: 2026-04-01
title: "A Definition of Ready for Architectural Decisions (ADs)"
---

### Content Outline

*   [Definitions of Ready and Done for Agile User Stories](#definitions-of-ready-and-done-for-agile-user-stories)
*   [Logical AD Management/Modeling Steps](#logical-ad-managementmodeling-steps)
*   [Five Criteria for AD Readiness](#five-criteria-for-ad-readiness)
*   [Big/Early ADs](#bigearly-ads)
*   [Example: Selection of an Integration Style](#example-selection-of-an-integration-style)
*   [Related Posts](#related-posts)
*   [Summary](#summary)

It is important to hit the [most responsible moment](http://wirfs-brock.com/blog/2011/01/18/agile-architecture-myths-2-architecture-decisions-should-be-made-at-the-last-responsible-moment/) for an Architectural Decision (AD) about a pattern or other concept, technology or product. But when is an AD actually ready to be _made_? This posts suggests five criteria to decide whether it is time.

### Definitions of Ready and Done for Agile User Stories

Let's start with some context and background information. [Definition of Ready (DoR)](https://www.agilealliance.org/glossary/definition-of-ready) is an essential Agile concept that helps avoid misunderstandings about maturity and keep teams focused. Common DoRs such as [INVEST](https://en.wikipedia.org/wiki/INVEST_\(mnemonic\)) typically check whether features (often user stories) can be implemented.[1](#fn:1) Jumping from the start to the end of sprints/iterations, there also is the notion of [Definition of Done (DoD)](https://www.agilealliance.org/glossary/definition-of-done); only done-done features pass the sprint/iteration review. Both DoR and DoD are about features; they do not provide criteria for checking readiness or completion of technical design work.

Switching perspectives from requirements and features to design and decisions, I proposed a [Definition of Done for ADs](/practices/2020/05/22/ADDefinitionOfDone.html) in a previous post. It contains five existence criteria to help decide whether an AD is ready to be executed upon: _evidence_ that chosen option will work, _criteria_ for option selection, _agreement among stakeholders_, _documentation_ of AD outcome and rationale and _realization/review plan_ for the AD outcome. I abbreviated these five done-done criteria (or exit conditions) as **ecADR**.

Traveling back from "done" to "ready", let's talk about entry conditions for AD making (selection of an option based on requirements in context, that is) in this post — as well as criteria for those ADs that should become ready early on. To set the stage, let's look at the journey that each AD takes.

Five basic logical steps for Architectural Decision Management and Modeling (ADMM) are (note: these steps pick up those on Wikipedia):

1.  _Identification of a design issue and options that address it._ AD selection happens here. Both a problem (question) and possible solutions to it are researched. This step is partially a learning activity and also a creative one.
2.  _Criteria collection and option analysis._ Option evaluation is the scope of this step, based on a set of criteria (aka decision drivers) specific to the design problem from Step 1. Additional solution options may be identified wh le evaluating.
3.  _Decision making._ The AD is taken: an option is chosen, based on the criteria and analysis from Step 2. Rationale for the option selection is developed. Agreement among stakeholders that they are ok with the AD outcome should also be reached in this step.
4.  _Decision capturing._ [Architectural Decision Record (ADR)](https://adr.github.io/) documentation is the scope of this step, logging what happened why in Steps 1 to 3. The AD outcome, its justification and consequences are recorded. See [this post](/practices/2023/04/03/ADRCreation.html) for related documentation advice.
5.  _Decision enforcement._ AD execution, i.e., implementation and review of the AD outcome and the implementation, is required to make the AD effective.[2](#fn:2) It is worth monitoring whether the design problem from Step 1 is actually solved and how the implementation scores w.r.t. the criteria from Step 2.

![ADM Steps](/assets/images/ADM-FiveStepsPerAD.png)

You can move from Step 4 (documentation) to Step 5 (enforcement) when an AD satisfies the [ecADR DoD](/practices/2020/05/22/ADDefinitionOfDone.html). And you can advance an AD from Steps 1 and 2 to Step 3 when is meets the DoR from this story.

Note that the steps are logical, not assuming a stiff sequential execution; it is perfectly fine to go back and forth (a spiral or a funnel might more be accurate ADDM metaphors, but the circle keeps the illustration simple). For instance, additional options and/or criteria have to be taken into account when there is no clear favorite and/or no consensus can be found in Step 3; this means a return to Step 1 and Step 2. The outcome of Step 3 and the ADR from Step 4 might have to be revisited in Step 5; new design issues (aka decisions required) might be identified, causing another execution of the ADMM process. It is also ok to run through steps multiple times for the same AD (ideally, no endless loop is entered though😉).

The AD DoR is our main focus in this post. Using a journey metaphor, I suggest the following criteria:

1.  _The travelers are known._ The decision makers, consulting stakeholders and people affected by the AD outcome are known and able to participate. For instance, they might be grouped in a [Responsible, Accountable, Consulted and Informed (RACI)](https://en.wikipedia.org/wiki/Responsibility_assignment_matrix) matrix.
2.  _It is time to travel._ The problem is both important and urgent; its [Most Responsible Moment (MRM)](https://wirfs-brock.com/blog/2011/01/18/agile-architecture-myths-2-architecture-decisions-should-be-made-at-the-last-responsible-moment/)[3](#fn:3) has come. The option selection (decision outcome) is useful and needed now.
3.  _Journey start and destination are clear._ Problem context as well as requirements and other decision drivers (aka criteria) have been analyzed and documented.[4](#fn:4)
4.  _Possible routes for the journey are agreed upon._ At least two design alternatives (decision options) have been identified. Their pros and cons are already known, or it is possible and planned to find out about the positive and negative consequences of the alternatives in the given context now.
5.  _The logbook for the trip is ready to be written._ An ADR template (community asset or own) has been chosen and validated, for instance, in a trial. This condition should be easy to meet from the second or third AD onwards (or via long-time template use). The template also has been instantiated so that the ADR can be populated as soon as the AD has been taken (which is Step 4 in ADDM); ADR writing might even begin while the AD making is ongoing (Step 3, that is).

No big surprises, probably, but worth collecting and checking still.

The five criteria lead to the following checklist (note: requirements moved to position 4):

```
1
2
3
4
5
6
7
AD-nn Definition of Ready (DoR):

* [ ] Stakeholders are known (decision makers and catchers)
* [ ] Time (most responsible moment) has come/is now
* [ ] Alternatives/options for problem solving exist and are understood (at least two)
* [ ] Requirements/criteria and context/problem are known
* [ ] Template for AD recording has been chosen and log record been created
```

Can you guess the backronym this checklist leads to? Indeed, answers to the five criteria give you a great START into the AD making!

### Big/Early ADs

Now that we know when an AD is ready to be made, how do we know which ones hopefully meet their "ready" definition early on, which means that they should not be revisited and made again in every sprint/iteration?

Previous posts started the discussion, for instance, ["Architectural Significance Criteria and Some Core Decisions Required"](/practices/2020/09/24/ASRTestECSADecisions.html). Generalizing from these and other examples, ADs with an early MRM (criterion 2 in the DoR) include:

1.  Those with _high significance_ score, for instance from an [Architectural Significance Test](/practices/2020/09/24/ASRTestECSADecisions.html). A (H/H) ranking in a utility tree/quality attribute scenario representation can also be an indicator.[5](#fn:5)
2.  Those requiring financial investment and causing significant _cost_, as well as those with other tough _consequences_. Cost might come from software licenses, training, consultancy, cloud operations; other consequences might include impact on staff.
3.  Those that take a rather _long time to execute_ upon, for instance due to a need for architectural spikes and proof-of-concepts, trainings, recruiting, and so on. Software product purchases, for instance, might cause a rather intense procurement process that takes time to complete.
4.  Those with _many or unclear outgoing dependencies_ as "one thing leads to another", with the "thing" in itself being an AD in this context. Strategic decisions and choices of architectural principles such as loose coupling, for example, only frame the decision making. Hence, they immediately trigger ADs such as "How are we going to promote principle xyz?".
5.  Those that take a _long time to make_ before they can be considered done, for instance because there are many stakeholders and goal conflicts are expected (impacting the Agreement-A in the [ecADR DoD](/practices/2020/05/22/ADDefinitionOfDone.html)). Hard-to-revise ADs also should be made carefully/thought through thoroughly (related to the R in ecADR).
6.  Those with a _high level of abstraction_ requiring refinement, for instance selection of an architectural style, e.g., layers? pipes-and-filters? service orientation? Abstract decisions usually cause follow-on decisions about design details, technology and product choices, and so on. Composite patterns such as integration brokers require ADs about topology, technologies and message delivery. Note that abstract ADs might require more orientation and learning work to prepare an informed, well-justified decision.[6](#fn:6)
7.  Those with an _unusual problem/solution space_, outside of the team's comfort zone. Note that this might also be a reason to defer the decision; but we look at getting started early, not at finishing early.

These seven criteria are not [Mutually Exclusive and Collectively Exhaustive (MECE)](https://en.wikipedia.org/wiki/MECE_principle), and they not intend to be. In other words, they overlap, and probably some of your rules of thumb are missing. That's ok — please let me know if you miss important ones!

A [previous post](/practices/2020/09/24/ASRTestECSADecisions.html) and the [arc42 Solution Strategy](https://docs.arc42.org/section-4/) provide examples of "big" and early ADs, as well as related prioritization and documentation advice.

### Example: Selection of an Integration Style

Let's pick a big AD, on [integration style](https://www.enterpriseintegrationpatterns.com/ramblings/08_integrationstyles.html) in the [Lakeside Mutual](https://github.com/Microservice-API-Patterns/LakesideMutual) scenario and sample system. In this insurance scenario, the risk management and the policy/offer management services have to exchange information about customers and their enquiries.

Let's assume that the project stands at an early stage, [Solution Strategy](https://docs.arc42.org/section-4/) in arc42 terms. Desired qualities include guaranteed delivery, decoupling of message senders and receivers in the time dimension, and support for multiple programming languages and platforms.

The criteria from above — stakeholders RACIfied, time right (but not tight), question, criteria and options defined, ADR template picked — evaluate to true as follows:

```
1
2
3
4
5
6
7
8
1. [x] S: The following responsible and involved roles are identified: 
       Coding architect, integration specialist, sys admin (ops) personnel. 
2. [x] T: We agree that the style issues requires an AD, which is a big one. Its MRM has come: 
       Switching from one integration style to another is costly, the AD is hard to change once made.
3. [x] A: We understand the alternative integration styles with their pros and cons: 
       File Transfer, Shared Database, Remote Procedure Invocation, Messaging
4. [x] R: We know our integration requirements, including guaranteed delivery.
5. [x] T: The Markdown ADR (MADR) template has been decided upon to record ADs.
```

With this evaluation, this AD qualifies as being ready to be made (START). If you are curious how the architects of the sample system decided, have a look at the architecture diagrams and the source code on [GitHub](https://github.com/Microservice-API-Patterns/LakesideMutual).

For a discussion of the four integration styles, including Messaging, see the book "Enterprise Integration Patterns" (EIP) and its [supporting website](https://www.enterpriseintegrationpatterns.com/patterns/messaging/IntegrationStylesIntro.html). Streaming and the Web when used as [Data Transfer Resource](https://api-patterns.org/patterns/responsibility/informationHolderEndpointTypes/DataTransferResource#) are two more styles, as discussed and compared in the IEEE Software article ["The Web as a Software Connector"](https://www.pautasso.info/biblio-pdf/IEEESW-2018-WWW-Insights.pdf).[7](#fn:7)

Are you looking for a template and a simple, human- and machine-readable notation for your ADRs? ["The Markdown ADR (MADR) Template Explained and Distilled"](/practices/2022/11/22/MADRTemplatePrimer.html) might be worth visiting. Rich ADR [capturing](/practices/2023/04/03/ADRCreation.html) and [review](/practices/2023/04/05/ADRReview.html) advice is also available.

Mohsen Anvari and I propose ["An Adoption Model for Architectural Decision Making and Capturing"](/practices/2023/04/21/ADAdoptionModel.html). The DoR fits nicely on the fourth level, "Systematic, Selective and Diligent". Hans-Peter Hoidn shares his experiences with decision-centric architecture boards [here](/guest/2023/05/17/ArchBoardHPH.html).

Chapter 3 of our ["Patterns for API Design: Simplifying Integration with Loosely Coupled Message Exchanges"](https://api-patterns.org/book/) in the Addison Wesley Signature Series at Pearson features six narratives guiding through the conceptual level of API design. 29 recurring decisions are identified with options and criteria, ready to be made. Sample Y-Statement ADRs are presented as well, for instance prompting API designers to decide between [Embedded Entity](https://api-patterns.org/patterns/quality/referenceManagement/EmbeddedEntity.html) and [Linked Information Holder](https://api-patterns.org/patterns/quality/referenceManagement/LinkedInformationHolder.html).[8](#fn:8)

### Summary

The key messages of this post are:

*   Check whether the five ready-to-START criteria from this post are met before making an AD: _stakeholders, time, alternatives, requirements, template_. This might be easy or take a while, from a single meeting to a longer proof-of-concept project.
*   Applying the AD DoR lets you and your team be effective and efficient; no effort is spent on trying to make ADs that either are not important and urgent or not prepared well. In lean software development terms, you avoid waste.
*   Make each AD (selecting an option) when its START criteria are met.
    *   Note that no viewpoint or even role switch is implied here; ADs are prepared, made and documented by the architects[9](#fn:9) and then executed by the entire team and all affected stakeholders. This is a difference between the AD DoD and the feature/story DoD mentioned earlier.
*   Don't decide too early, this harms flexibility. Don't decide too late either.
    *   Procrastinating ADs beyond their most responsible moment might be a sign of uncertainty or even fear. This holds for "big" ADs in particular.
    *   Use the seven rules of thumb for "big" ADs to guide your timing: high significance, cost and consequences, long time to execute, many or unclear dependencies, long time to make, high level of abstraction, unusual problem/solution space.
*   Do not consider an AD that passed the START gate done before it is actually done; see sibling [ecADR DoD](/practices/2020/05/22/ADDefinitionOfDone.html) post for criteria. As always, be pragmatic and apply common sense when assessing the state of an AD, taking project context and development/design culture in/of the team into account.[10](#fn:10)

Your feedback is appreciated — do the above five criteria work for you? Did I miss a "ready" or "big" criterion? [Let me know](/about/)!

– Olaf (a.k.a. ZIO)

There is a [Medium version](https://medium.com/@docsoc/a-definition-of-ready-for-architectural-decisions-ads-2814e399b09b) of this post.

**Acknowledgements**

I would like to thank Mirko Stocker and Stefan Kapferer for participating in "brain dumps" and fruitful discussions during the preparation of the first version of this post. Mirko and Stefan, as well as Daniel Lübke and Christian Ringler, reviewed intermediate draft versions.

**Notes**

1.  INVEST stands for Independent, Negotiable, Valuable, Estimable, Small, Testable. [↩](#fnref:1)
    
2.  Although many decisions made never get here (for various reasons). [↩](#fnref:2)
    
3.  MRM is a notion suggested by Rebecca Wirfs-Brock in her reaction to the principle "Decide as late as possible" from [Lean Software Development](https://en.wikipedia.org/wiki/Lean_software_development#Decide_as_late_as_possible). [↩](#fnref:3)
    
4.  Ideally in [SMART](https://socadk.github.io/design-practice-repository/activities/DPR-SMART-NFR-Elicitation.html) form but at least identified and drafted. [↩](#fnref:4)
    
5.  See [this article](https://arnon.me/2010/05/utility-trees-hatching-quality-attributes/) for a utility tree example. [↩](#fnref:5)
    
6.  Popularity of an option alone, for instance on social media, does not qualify as an evaluation criterion or outcome argument (in my humble opinion). This observation actually shines through in [architecture antipatterns](https://architecture-antipatterns.tech/) such as Cargo Culting, Domain Allergy and Emotional Attachment. [↩](#fnref:6)
    
7.  The four styles from the EIP book are actual integration _styles_, while SOAP/HTTP and gRPC (also transmitted over HTTP) are technology realizations of [Remote Procedure Invocation](https://www.enterpriseintegrationpatterns.com/patterns/messaging/EncapsulatedSynchronousIntegration.html). [↩](#fnref:7)
    
8.  This [article at InformIT](https://www.informit.com/articles/article.aspx?p=3153211) discusses the pros and cons of these patterns. [↩](#fnref:8)
    
9.  As a responsibility and role, not necessarily a single person/individual. [↩](#fnref:9)
    
10.  This arguably holds for selection and usage of most if not all techniques, templates. [↩](#fnref:10)
