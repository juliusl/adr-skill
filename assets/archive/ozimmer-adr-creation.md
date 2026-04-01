---
source: https://www.ozimmer.ch/practices/2023/04/03/ADRCreation.html
fetched: 2026-04-01
title: "How to Create Architectural Decision Records (ADRs) — and How Not To"
---

![ZIO](/assets/images/olzzio.jpg)

ZIO[Contact](https://ozimmer.ch/about/) Consulting IT architect and software architecture coach.

Apr 3, 2023 (Updated: Apr 28, 2025)  
Reading time: 9 minutes

ADR stands for architectural decision record, but could just as well stand for _archive_ of _design rationale_. This post is about making ADRs valuable; a [sibling post](/practices/2023/04/05/ADRReview.html) covers ADR review.

#### The Nature of ADRs

[ADRs](https://adr.github.io/), first and foremost, log the outcome of design and decision making activities. Hence, we can view them as _journals_, akin to meeting outcome summaries or discussion notes.

Continuing with metaphors, I believe that a useful ADR also should have the following characteristics:

*   _Executive summary_: ADRs should be brief and distill their content to the bare minimum of essential information required to understand what has been chosen — and why, including benefits and other consequences.
*   _Verdict_ or _scale_: Selecting a concept, technology or product should bring benefits, but also comes at a price. Hence, the pros and cons of the chosen option should be balanced, tradeoffs explained and a judgment made. All neglected options should be evaluated as well.
*   _Letter of intent_: To be able to initiate decision follow up, an assertive writing style is in order. The decision makers should be identified, which helps to make them accountable for decision enforcement.
*   Decisions must be executed to have any effect; they might have to be revisited. Hence, ADRs also have the nature of _action plans_.

Yet another view/metaphor is to see an ADR as a _contract_ indicating that a particular AD meets the [definition of done](/practices/2020/05/22/ADDefinitionOfDone.html) for ADs, which includes agreement.

#### Good Practices

Compared to structure-centric notations and models, ADRs are light and easy to get started with. But getting them right so that they are faithful to the above metaphors still is not easy. I saw decision makers and recorders succeed with the following tactics:

1.  _Select by priority and significance._ An ADR should log the resolution of at least one [architecturally significant requirement](/practices/2020/09/24/ASRTestECSADecisions.html). The level of this significance and the severity of the consequences of the chosen solution (in terms of cost and risk) determine the importance and urgency of the decision making and capturing. The problem statement in an ADR should make readers curious and communicate a sense of urgency; questions containing terms from the domain vocabulary and previously chosen concepts and technologies help to achieve that.
2.  _Do not defer making and capturing high-impact decisions for too long_ (in the name of flexibility). ADs that are costly to undo simply cannot wait until sprint n, with n larger than 3. How often would you really want to exchange your database management system and your programming language, every two weeks? Hopefully not.[1](#fn:1)
3.  It often makes sense to _prioritize meta-qualities such as observability and ability to react_ over presumed long-term goals and visions such as advanced scalability to handle future workloads (that might never occur) in the decision criteria. Such weighting helps to avoid lock-in and design dead ends; it should become evident in the ADR. This is the verdict/scale nature of an ADR in action.
4.  _Root and justify decision in actual requirements and personal experience._ Be as objective as possible; no vendor bashing, no blaming and no sarcasm please. Some [biases](https://en.wikipedia.org/wiki/Cognitive_bias) will always be there, but it is good to note them. To notice it and to record them, actually, in the spirit of the journal nature of ADRs. See ["Decision Bias & Software Design: Why It Matters"](https://medium.com/capital-one-tech/decision-bias-software-design-why-it-matters-2457bdcd8881) for more related advice.
5.  _Invest in editorial quality, having an eye on the appropriate ADR length._ First impressions last, and you do not want readers to get the impression that an AD and its recording have been rushed (for instance, any typos left? punctuation issues?). Watch the word count of an ADR as it evolves. Sometimes, one presentation slide with few sentences is enough to provide an AD executive summary. However, more wicked problems may require more elaborate decision rationale, up to a few pages in a text document (with standard page margins and font size).
6.  _Split decisions into stages if no simple answer exist._ For instance, decide for a short-term compromise _and_ a mid-term solution _and_ a long-term vision. In other words, decide the same issue three times, possibly picking different options and giving time-sensitive justifications for each choice in the resulting three-part ADR. Revisit the AD at the end of each stage, effectively changing the ADR nature from letter of intent to action plan.
7.  _Disclose your confidence level._ It is ok to be in doubt, at least for some time. Fair reviewers appreciate honesty. Be ready to revise an AD as the design work evolves and you learn more about requirements and technical possibilities in the given context; once revised and agreed upon, the ADR turns into a contract between the involved stakeholders.

Following the above advice yields decent ADR content, but ADR logs should not turn into epics. Other anti-patterns exist as well; let's look at common ones now.

#### Anti-Patterns

The ADR capturing approaches in this section do not lead to ADRs resembling metaphors such as executive summary, verdict and action plan. They either dilute the ADR concept (with good intentions) or result from unprofessional, unethical behavior. Or both. 😉

Let's start with subjectivity creeping in. As discussed under "Good Practices", ADRs are supposed to be as objective as possible. This is not always the case:

*   _Fairy Tale_ (aka Wishful Thinking): A shallow justification is given, for instance only pros but no cons. Special cases of this anti-pattern are truisms and tautologies.
    
    > Here is a simple example: "We decided for a load balancer because it balances load, which is a good thing." There is room for improvement in (at least) two places in this ADR, can you spot them?
    
*   _Sales Pitch_: The language of marketing has its place, but ADRs are not that place. Avoid exaggerations and bragging. Check each and every adjective and adverb whether a) it is needed at all (try [Ockham's Razor](/misc) to do so) and b) its claim can be backed with evidence. Always be ready to answer questions about claims and their evidence. Including all related background information would bloat the ADR; hyperlinks or bibliographic references can be provided instead.
    
    > Example (exaggerated intentionally): "We chose this outstanding technology because is very much unrivaled in the marketplace; its splendid performance shines everywhere, and all its adopters are extraordinarily happy all the time." How factual is this statement? Which words can be deleted without loosing technical meaning? How many counter examples are needed to falsify it?
    
*   _Free Lunch Coupon_ (aka Candy Bar): No consequences are documented, or only consequences that seem to be harmless. The difficult ones are ignored accidentally or hidden deliberately, those that only materialize in the long run in particular.
    
    > Example: "We decided for an event-based architecture because it decouples the communication participants." This might be true, at least for the time dimension of [loose coupling](https://www.cloudcomputingpatterns.org/loose_coupling/). But what about the extra design, implementation, test effort? Doesn't the event schema definition couple the receiver to the sender (in the format dimension)?
    
*   _Dummy Alternative_: A solution is made up and presented as an option, but does not work at all in the given context. This move tries to make the preferred option shine and give the impression that multiple alternatives have been evaluated, which is not really the case.
    
    > Example: We decided to use PostgresSQL as our relational database. We could implement our own relational database management system, but this takes time and effort." Is this a new and valuable insight? Why are other existing relational database products and open source assets not mentioned?
    

The next slice of anti-patterns deals with the time dimension of architecting:

*   _Sprint_ (aka Rush): Only one option is considered. Only short-term effects are discussed, pertaining to the next two to three project iterations. However, hardly any software design does not have any alternatives, unless you have locked in to a certain vendor or cloud service provider deeply. And time tells whether a decision is sound.
    
    > Remedy: Search for valid alternatives, for instance online or via your professional networks. Report on the search results, an ADR is an activity journal after all. Report mid- and long-term consequences too.
    
*   _Tunnel Vision_: Only a local, isolated context is considered, for instance the benefit of a pattern choice for an API provider without looking at the developer experience on the API client side. Quite often, developmental qualities are covered, but the consequences for operations and maintenance are not taken into account sufficiently.[2](#fn:2)
    
    > Remedy: Mention system administrators and maintainers explicitly and acknowledge their wants and needs, just like those of developers and end users. Add criteria such as manageability and evolvability and comment how the chosen solution scores with respect to them.
    
*   _Maze_: The ADR topic does not match its content; the discussion derails and centers on details that are not relevant in the given context. For instance, there is an interesting discussion of the positive attributes of an option that has nothing to do with the problem and the relevant stakeholder concerns.[3](#fn:3)
    
    > Remedy: Refactor the ADR, move less relevant parts to an appendix or parking lot (if you are not ready to scrap them).
    

Other anti-patterns deal with record size and content nature:

*   _Blueprint or Policy in Disguise_: The writing style is not that of a journal reporting activities and their outcome, but reminds readers of a cookbook or law because of the amount of details provided and/or a rather commanding, authoritative voice.
    
    > Remedy: Reword and possibly shorten the ADR, applying the good practices from above.
    
*   _Mega-ADR_: A lot of detailed information about the architecture is stuffed into several multi-page ADRs serving as documentation master (or monster?).[4](#fn:4) For instance, component responsibilities and collaborations are specified, or multiple diagrams or code snippets appear inside the ADR (one per option is ok usually). While such information can be valuable, it should be presented in models or documents specifically created for that purpose.[5](#fn:5)
    
    > Remedy: Move the detail design to a separate document.
    
*   _Novel_ and _epic_ are similar to Mega-ADR and Blueprint or Policy in Disguise, but even more extreme: an entire [Software Architecture Document (SAD)](https://almbok.com/method/sad) is squeezed into a single ADR. The writing tone of such epic ADRs often is casual and jovial, while a true ADR is expected to be assertive and factual.
    
    > Remedy: See Blueprint or Policy in Disguise and Mega-ADR.
    

Finally, _Magic Tricks_ ("AD wizardry") receive special attention too. Examples include:

*   Non-existing or misleading context information to create a false sense of urgency. For instance, a pseudo-problem might be called out that does not really exist.[6](#fn:6)
*   Problem-solution mismatch. A solution seeks a problem to be solved. The ADR is used to sell something that has been decided for already, but does not really solve the design issue at hand.
*   Pseudo-accuracy. Quantitative weighted criteria scoring ("Nutzwertanalyse" in German) has its place, for instance in economics when preparing business cases, but is it able to structure and support AD making? What is the result of `4x vendor independence score + 3x licensing policy score / 2` and what does it mean? How do you measure vendor independence in discrete numbers, how do you count flexibility?[7](#fn:7)

> Remedy: Remove any instances of these magic tricks and rewrite the ADR following the good practices from above. Choose ADR size and style of writing based on the type of AD.

Dave Linthicum has something to say about such tricks in the context of cloud computing. He argues that they lead to buzzword-oriented architectures, see ["Don't let buzzwords drive your cloud architecture"](https://www.infoworld.com/article/3688930/dont-let-buzzwords-drive-your-cloud-architecture.html).

#### ADR Author Pledge

Act as responsible journalist! More specifically, promise to:

1.  Prioritize decision topics/issues based on their _architectural significance_.
2.  Decide on single _template_ and stick to this format.
3.  Size the ADR adequately, choosing an appropriate length. Present decision _question_ (issue, problem in context), _criteria_ (drivers) and _options_ as well as decision _outcome_ and _consequences_ (good and bad) explicitly.
4.  Invest in documentation _quality_: a) document thoroughly: refer to project goals and organizational context/constraints in the decision rationale and consider at least two options per issue, b) stay focused: apply the AD/ADR tactics and anti-patterns from above, c) be factual in your argumentation and trace the decision back to requirements.
5.  Be _honest and candid_. Disclose the maturity of the decision made (confidence level). Unveil your project experience that influenced or led to the decision.

When sticking to these guidelines, you have good chances that your ADRs do qualify as executive summaries, verdicts/scales and action plans. 😃

> Examples of well-done ADRs are given in other posts, for instance [this one](/practices/2020/04/27/ArchitectureDecisionMaking.html) and [this one](/practices/2022/11/22/MADRTemplatePrimer.html).

#### Concluding Thoughts and Outlook

This post featured six ADR metaphors, recommended seven ADR capturing practices, and identified eleven anti-patterns (with even more variants). It concluded with a seven-part ADR author pledge. Let's summarize:

![ADR Creation: Dos and Don'ts](/assets/images/ADRCreationInfographic.png)

When you are done making and capturing ADs, establish review needs and goals, and think about how to achieve them. The post ["How to Review Architectural Decision Records (ADRs) — and How Not To"](/practices/2023/04/05/ADRReview.html) covers these topis.

Once completed and reviewed, ADRs have to be "socialized" to be effective (communicated, discussed, agreed upon).

Note that I did not go into AD dependencies or ADR maintenance in this post. For instance, you may want to mark certain ADRs as outdated and superseded by other ones (or even delete them from your log).

**Related Posts and Resources.** Many people have blogged about ADRs since the topic became big (again). Here are some online resources I found useful when/for preparing this post:

*   Spotify engineers describe a workflow and provide checklists in ["When Should I Write an Architecture Decision Record"](https://engineering.atspotify.com/2020/04/when-should-i-write-an-architecture-decision-record/).
*   The Continuous Architecture website has a page ["Architectural Decision Record"](https://continuous-architecture.org/docs/practices/architecture-decision-records.html) that looks into decision timing.
*   The Agile Alliance has ADR-related content too, including ["Share the Load: Distribute Design Authority with Architecture Decision Records"](https://www.agilealliance.org/resources/experience-reports/distribute-design-authority-with-architecture-decision-records/) by Michael Keeling and Joe Runde and Eric Kaun.
*   ["Scaling the Practice of Architecture, Conversationally"](https://martinfowler.com/articles/scaling-architecture-conversationally.html) by Andrew Harmel-Law has a section on "Decision Records".
*   ["8 Learnings from using Architecture Decision Records (ADRs) at willhaben"](https://tech.willhaben.at/8-learnings-from-using-architecture-decision-records-adrs-at-willhaben-5b1594ebaffe) include "ADRs bring structure into discussions", "ADRs don't need to be perfect from the start", "ADRs have both short term and long term benefits", "ADRs need to be visible and accessible", "Know when to write an ADR (and when not)", "ADRs help you to learn from your mistakes".
*   The Quality Engineering community cares about ADRs too: ["ADRs—Explicit Decisions For Better And Faster Software"](https://qeunit.com/blog/adrs-explicit-decisions-for-better-and-faster-software/).
*   Michael Keeling also contributed videos, presentations and two columns in IEEE Software: ["The Psychology of Architecture Decision Records"](https://ieeexplore.ieee.org/document/9928205) (with an example) and ["Love Unrequited: The Story of Architecture, Agile, and How Architecture Decision Records Brought Them Together"](https://ieeexplore.ieee.org/document/9801811) (with a partial anthology).

Looking for a template and a simple, human- and machine-readable notation for your ADRs? ["The Markdown ADR (MADR) Template Explained and Distilled"](/practices/2022/11/22/MADRTemplatePrimer.html) might be worth visiting. The explanations in the blog post complement the ADR capturing advice in this one.

Chapter 3 of ["Patterns for API Design: Simplifying Integration with Loosely Coupled Message Exchanges"](https://api-patterns.org/book/) in the Addison Wesley Signature Series at Pearson features six narratives guiding through the conceptual level of API design: 29 recurring decisions with options and criteria.

Finally, Mohsen Anvari and I propose ["An Adoption Model for Architectural Decision Making and Capturing"](/practices/2023/04/21/ADAdoptionModel.html).

Happy ADRing! Looking for (even more) advice how to write ADRs? [Contact me!](/about)

– ZIO

There is a [Medium version](https://medium.com/olzzio/how-to-create-architectural-decision-records-adrs-and-how-not-to-93b5b4b33080) of this post.

**Acknowledgements.** Stefan Kapferer, Justus Bogner, Christian Ringler and Oliver Kopp reviewed earlier versions of this post and/or shared their ADR creation experiences with me. Global architect communities at clients, partners and employers provided input and inspiration as well.

**Notes**

1.  See the [this post](/practices/2020/09/24/ASRTestECSADecisions.html) for some early decisions, which should be documented and agreed upon as soon as possible. [↩](#fnref:1)
    
2.  "You build it, you run it" often helps to avoid this anti pattern (at a price). [↩](#fnref:2)
    
3.  This is the ADR version of the from frog-to-cucumber strategy for oral exams (which is not recommended: while both are green, not all biology examiners will let you get away with the topic switch from animal to plant if you decided to only study plants and now try to re-route the examination). 😉  [↩](#fnref:3)
    
4.  This is the AD equivalent to a [god/too large class](https://martinfowler.com/articles/class-too-large.html) in code refactoring. Michael Keeling calls it "Everything is an ADR" in [this podcast](https://techleadjournal.dev/episodes/113/). [↩](#fnref:4)
    
5.  For inspiration, see the artifact collection in the [Design Practice Reference](https://leanpub.com/dpr) and [Repository](https://socadk.github.io/design-practice-repository/artifact-templates/). [↩](#fnref:5)
    
6.  Some project managers apparently succeed with this strategy to create management attention, but they do walk on thin ice. [↩](#fnref:6)
    
7.  Note: content was truncated at fetch time; see original URL for full article. [↩](#fnref:7)
