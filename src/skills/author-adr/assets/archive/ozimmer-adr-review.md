---
source: https://www.ozimmer.ch/practices/2023/04/05/ADRReview.html
fetched: 2026-04-01
title: "How to Review Architectural Decision Records (ADRs) — and How Not To"
---

![ZIO](/assets/images/olzzio.jpg)

ZIO[Contact](https://ozimmer.ch/about/) Consulting IT architect and software architecture coach.

Apr 5, 2023 (Updated: Mar 12, 2025)  
Reading time: 9 minutes

### Content Outline

*   [Review Perspectives and Goals](#review-perspectives-and-goals)
*   [Good Practices and Related Advice](#good-practices-and-related-advice)
*   [Review Anti-Patterns](#review-anti-patterns)
*   [Exercise: Two Versions of an ADR](#exercise-two-versions-of-an-adr)
*   [ADR Checklist](#adr-checklist)
*   [Reviewer Pledge](#reviewer-pledge)
*   [Wrap Up](#wrap-up)

This post reflects about reviewing ADRs. It identifies three review perspectives, recommends related practices and discusses anti-patterns. A review checklist is provided and an ADR reviewer pledge proposed.[1](#fn:1)

**ADR Benefits (Recap).** When explaining the [MADR template](/practices/2022/11/22/MADRTemplatePrimer.html), I stated that ADRs keep you CALM:

*   **C**ollaborative content creation is enabled.
*   **A**ccountability is supported.
*   **L**earning opportunities are provided, both for newcomers and for the experienced.
*   **M**anagement likes them too because it is used to making and executing decisions.

Done well, ADRs also lead to:

1.  Productivity increase with ADR logs serving as traceable community assets, avoiding unnecessary reconsideration of issues.
2.  Longer-lasting designs coming from the community involvement in the decision making.
3.  Risk reduction due to the checklist effect of ADR templates.

These benefits only have a chance to materialize if ADRs are written up properly. A [sibling post](/practices/2023/04/03/ADRCreation.html) covers ADR creation. But:

> How to check whether the ADRs can have the envisioned positive effects? How to assure the quality of the review process and the reviewed ADRs?

Let's find out in this post.

#### Review Perspectives and Goals

There are at least three modes of operation for any technical review(er):

1.  Friendly _peer_ and/or _coach_.
2.  Official _stakeholder_ affected by positive/negative consequences, project sponsor.
3.  Formal _design authority_.

The reviewers should be chosen based on _review goals_ and expectations; review rigor increases depending on who is reading. Looking for early feedback to improve ADR content and its presentation while writing? Ask a trusted friend or advisor. Looking for confirmation that the AD is adequate (i.e., will work and has a chance to be agreed upon)? Share it with the official project stakeholders. Ready to defend an agreed-upon decision and seek formal approval (as/if needed)? Make the ADR available to a broader, presumably less friendly audience.

#### Good Practices and Related Advice

ADR reviewers in any of the three roles should always have an eye on _scope_, _content_ and _style_ of their review comments; _actionability_ comes in as a desired quality. I group the advice that follows, drawn from experience in professional services and teaching, by these four themes.

Let's start with scope.

1.  _Deliver what is asked for._ Be disciplined. For instance, read only those decisions marked as ready for review and ignore all draft ADRs that you come across. Ask for and agree upon specific review areas if the review scope is not part of the tribal knowledge of the involved parties. For instance, I often specify documentation qualities such as accuracy, usefulness, readability when asking for feedback on blog posts like this. In the context of ADRs, you might want to distinguish content reviews from template conformance checks. Both have their place.
2.  _Prioritize comments by urgency and importance_ if there are many. Assign priority categories such as H(igh), M(edium), L(ow) or severity levels such as 1, 2, 3. Such classifications are also used in requirements engineering, bug tracking and help desk ticketing and therefore should be familiar to everybody involved in the review. Be ready to discuss and adjust your prioritization.
3.  _Document the scope and the goals of the review_, both asked for and performed. The two scopes might not always be identical (although they should); if there is too little overlap, it might be time to adjust and partially repeat the review. List the text documents, Web pages, design models or other artifacts studied, including version numbers or timestamps. Just like the review goals, any assumptions made before and during the review should be made explicit and important ones agreed upon. Chances are that your review reports live long and travel in unexpected ways[2](#fn:2); scope and goal information reminds future readers of the the original assignment and its results.

Over to content:

1.  _Justify comments and calls to actions by referencing desired qualities_ — for the system under construction and for its documentation. General documentation qualities that also apply to reviews include readability[3](#fn:3), consistency and completeness. Note that it is much harder to comment on missing parts rather than on present ones: first, you have to find the gaps and then you have to fill them. This take time and requires dedication and experience.
2.  _Acknowledge context and requirements_ at the time of decision making. When commenting on tradeoffs made and other ADR hot spots, give the benefit of the doubt to the ADR author and the decision makers — unless the review is part of a formal sign-off and final acceptance of a system developed under a fixed-price contract. Put yourself in the shoes of the AD maker and recorder. How would you decide? And why?
3.  _Be concrete and factual in option judgments._ Prefer "I suggest to evaluate the performance of O1 in more depth" over "O1 needs work" and "assuming that QA1 has priority over QA2, I would have preferred option O2 over option O1 because …" over "everybody knows O1 is a bad choice".

Next up is the commenting style:

1.  _Comment in a problem- and solution-oriented way._ Use a writing style that could be part of the conversations in, or notes from, a joint design workshop. Avoid direct confrontations, often caused by speculations or allegations (this is preferable in most cultures). For instance, you may want to lead and convince feedback receivers with questions about design issues or suggestions about design alternatives. For example, prefer "argument A1 pro Option O1 does not works for me, I have gained different experience" over "A1 is nonsense" and "have you tried O2?" over "you must use O2!".
2.  _Report your perception/impressions, but do not interpret or guess._ In particular, do not analyze the presumable root cause of the deficits of an ADR. Such work can wait for a joint discussion in a meeting, requested by the ADR authors. Ask for clarification if anything is unclear.
3.  _Be at least as factual, thorough and focused as the ADR that is reviewed_ when phrasing the findings. Even official reviewers and design authorities do not want to come across as know-it-alls or aggressors (at least I would hope so).
4.  _Criticize as strongly as needed, but also be motivating_. Comment on parts that you found easy to follow and sound as well as on those showing room for improvement. "I do not have time for positive comments, everything that I do not complain about is ok" is not a review attitude that I am willing to accept.[4](#fn:4)
5.  _Be fair and polite._ Use modest statements and wording, assuming that you appreciate such style yourself. No offenses and no swearwords please. To give an example, prefer "argument B2 against Option O2 might benefit from more explanations, I am unable to follow it easily" or "following the logical flow of text was difficult for me in this particular area" over "the description of O2 is plain wrong" or "your writing is poor".

Finally, let's look at actionability as an important review quality:

1.  _Make the feedback resolvable._ Clearly say what you want and expect to happen, and help the authors of the reviewed ADRs get there by providing finding-recommendation pairs. You may want to tag your comments with type/class information such as "FYI" versus "action required" (see [this post](/authoring/2020/07/02/ReviewAndMeetingMarkup.html) for more review markup, as well as a set of recurring general review comments and questions). From a wording point of view, prefer "things have changed since decision was made, have you considered this new option O3?" over "obviously O1 and O2 will not work, better options exist".
2.  _Offer help with comment resolution._ For instance, say what you would do. Provide or point at examples in case you report missing content (which is important but also difficult).
3.  _Review the review comments_ at least once before sending them to fix typos and other editorial issues. Check whether you would be able to resolve the review findings yourself, just by reading your own comments.[5](#fn:5)

Review findings and recommendations have much better chances of getting noticed when these practices are followed, which obviously is a prerequisite for meeting the goals both of friendly and of formal stakeholder reviews (increase quality and find agreement, that is). Reviews by external design authorities might get away with a sloppier approach when all that the reviewed authors want is approval. However, an opportunity to build and grow a community of AD makers is missed if review breadth, depth and quality are not invested in. Review recipients should reserve enough time to analyze and incorporate the received feedback and ask for clarification along the way.

#### Review Anti-Patterns

To help improve ADR quality, avoid the following anti-patterns when performing ADR reviews:

1.  _Pass Through_: Few, if any, comments are made; the document under review is only skimmed or not opened at all.[6](#fn:6) A variant is Over-Friendliness: all of the (few) comments are positive, and some of them appear to be rather shallow. While flattery makes the ADR authors feel good, ADR readers prefer that the review takes the ADRs to the next level of quality.
2.  _Copy Edit_: Wording and grammar rather than content are solely focused on. While it is ok to point out language-related problems as they harm readability, content comes first.
3.  _Siding, Dead End_ (aka Excursion): The review comments switch topic unexpectedly and/or multiple times. They deviate from the ADR content and stop suddenly, without providing tangible advice.
4.  _Self Promotion, Conflict of Interest_: The comments mostly recommend the reviewer's work. Variant: the chosen option is challenged with arguments that are not objective to make another solution option look better than it actually is in the given context; the reviewer has a commercial or other interest in this solution.[7](#fn:7)
5.  _Power Game_: The review fails to bring forward technical arguments but threatens the ADR authors directly or indirectly. For instance, it emphasizes the hierarchical position or other job-related influence that the reviewer possesses. Bragging about personal experience, possibly in another domain or requirements context, falls in this category.
6.  _Offended Reaction_: The review comments defend a position that is criticized in the decision rationale in a subjective and huffy way. "Hate To Say I Told You So" is a variant.
7.  _Groundhog Day_: The same message is repeated over and over. The same message is repeated over and over. The same message is repeated over and over.[8](#fn:8)

By the way, most of these anti-patterns may occur in any decision/design artifact review (not just ADR reviews).

#### Exercise: Two Versions of an ADR

Version A is informal, no template is used. It comprises one sentence, which is deeply structured:

```
1
2
3
4
5
We decided for Apache Kafka as our event sourcing infrastructure 
for future big data analytics projects because:
- it offers publish-subscribe semantics, 
- it supports time-based reasoning capabilities 
- it scales well.
```

Version B uses a popular, pattern-oriented ADR template (five entries):

```
1
2
3
4
5
6
7
8
9
10
11
12
Title: "ADR-01: Kafka as global event messaging infrastructure."
Context: "Kafka was presented at recent conference, 
  apparently the speaker was very convincing. Our ActiveMQ administrators are bored."
Decision: "We will replace all existing messaging software with Kafka topics, 
  including request-reply channels (point-to-point connections)."
Status: "decided"
Consequences:
 Good: "I can add Kafka to the skills section of my CV."
 Neutral: "We are in line with what everybody does these days."
 Bad: "We will have to re-implement all existing messaging endpoints 
  and migrate all data in transit. 
  Test cases and audit procedures will have to be revisited too."
```

🤔 Which ADR do you like better? And why? How many anti-patterns can you find?

_Hint:_ When applying the review advice in this post, you find arguments pro Version A and pro Version B. I do have a clear favorite though! [Contact me](/about) if you are curious.

#### ADR Checklist

If you struggle to come up with concrete review findings and recommendations, the following seven ADR-specific questions might be helpful:[9](#fn:9)

1.    Is the problem relevant enough to be solved and recorded in an ADR?
2.    Do the options have a chance to solve the problem? Are valid options missing?
3.    Are the decision drivers (criteria) mutually exclusive and collectively exhaustive?[10](#fn:10)
4.    If the criteria conflict with each other, are they prioritized?
5.    Does the chosen solution solve the problem? Is the decision rationale sound and convincing?
6.    Are the positive and negative consequences of the solution options reported as objectively as possible?
7.    Is the chosen solution described in an actionable way? Can it be traced back to requirements? Does the ADR define a validity period or review date?

The vocabulary of the ADR is worth checking as well; precise phrasings without subjective language, ambiguity or loopholes should be used.[11](#fn:11) You may also want to ask whether the decision is really done (see [this post](/practices/2020/05/22/ADDefinitionOfDone.html) for related criteria).

#### Reviewer Pledge

I'd like to suggest that any reviewer of a technical document should embrace a _review-as-a-service_ attitude:

1.  Apply proven practices to manage review scope and content and ensure a professional, both constructive and polite feedback style. See the section "Good Practices and Related Advice" for my recommendations (that do not claim to be complete).
2.  Make every reasonable effort to avoid (or spot and overcome) the seven review anti-patterns including their variants. Poorly planned and executed reviews are a waste of time.
3.  Use checklists such as the one above to make the review repeatable and its results reproducible.
4.  Make the review comments actionable by providing concrete recommendations and examples.
5.  Review like you want to be reviewed.

#### Wrap Up

Time to conclude:

*   Review perspectives matter because mismatches between expected and delivered review type may cause surprises and tension. There are at least three roles, and you get different type of feedback from each of them: friend/peer, stakeholder, external authority.
*   Common sense gets you going, and there is ADR-specific review advice. I shared 14 tactics that work for me, aiming at making the feedback relevant, actionable and digestible. Remember that it is much easier to criticize than to be criticized (for most people).
*   Seven ADR review anti-patterns that I have observed were discussed, from Pass Through to Groundhog Day. When you spot any of these, call them out and ask for more and more concrete and actionable comments.
*   The post provided a review checklist consisting of seven questions, which can also be used while [creating ADRs](/practices/2023/04/03/ADRCreation.html) rather than reviewing them.
*   An ADR reviewer pledge summarized the key messages of the post, again sevenfold.

![ADR Review: Dos and Don'ts](/assets/images/ADRReviewInfographic.png)

While some parts of this post are specific to ADs and ADRs, others apply to any design review (and possibly also other technical reviews). Much of the advice was on the review process, which indirectly (hopefully) also has a positive effect on the next revision of the reviewed ADRs. You might want to check that in a subsequent review, hopefully not ending up in Bill Murray's position in Groundhog Day. 😉

I hope you found this post useful. [Let me know](/about).

– ZIO

There is a [Medium version](https://medium.com/olzzio/how-to-review-architectural-decision-records-adrs-and-how-not-to-2707652db196) of this post.

**Related Posts**

If you liked this post, you might want to check out the following ones too:

*   ["Architectural Significance Test and Some Core Decisions"](/practices/2020/09/24/ASRTestECSADecisions.html)
*   ["Architectural Decisions — The Making Of"](/practices/2020/04/27/ArchitectureDecisionMaking.html), featuring Y-statements
*   ["A Definition of Done for Architectural Decision Making"](/practices/2020/05/22/ADDefinitionOfDone.html)
*   ["The Markdown ADR (MADR) Template Explained and Distilled"](/practices/2022/11/22/MADRTemplatePrimer.html)
*   Antoine Craske's ["How To Make Architecture Reviews That Feel Like Peer Reviews"](https://medium.com/qe-unit/how-to-make-architecture-reviews-that-feel-like-peer-reviews-ca1316b4f17d)

Also see ["How to create Architectural Decision Records (ADRs) — and how not to"](/practices/2023/04/03/ADRCreation.html).

The ADR review culture of a team or organization is one of the maturity dimensions in ["An Adoption Model for Architectural Decision Making and Capturing"](/practices/2023/04/21/ADAdoptionModel.html).

["A Lightweight Approach for Software Reviews" (LASR)](https://www.lasr-reviews.org/) concerns architectural decisions too.

**Acknowledgements**

Stefan Kapferer, Justus Bogner, Christian Ringler and Olly Kopp reviewed earlier versions of this post and/or shared their global ADR reviewing experiences with me. Global architect communities at clients, partners and employers provided input and inspiration as well.

**Notes**

1.  ADR stands for architectural decision record, but could also stand for _archive_ of _design rationale_. [↩](#fnref:1)
    
2.  I have been there! [↩](#fnref:2)
    
3.  or _Reader Experience_, RX, to quote one of the readers of our ["Patterns for API Design"](https://api-patterns.org/book/) 😀 [↩](#fnref:3)
    
4.  Ask my [book co-authors](https://api-patterns.org/about) whether I applied and enforced this advice. 😉 [↩](#fnref:4)
    
5.  "I do not have time to be that rigorous" is a poor argument; when committing to the review, allocate time for this extra cycle. I do, and if I forget, I work extra hours or ask for more time. [↩](#fnref:5)
    
6.  One of my students just reported that he observed that the longer a review subject is, the less comments are made. So you might want to decide for the ADR log size that you share consciously (what are the review goals?); slice it if needed… applying the [Pagination](https://api-patterns.org/patterns/quality/dataTransferParsimony/Pagination) pattern to ADRs rather than Web search results or API responses. [↩](#fnref:6)
    
7.  A certain amount of self-referencing is ok if motivated, scoped and portioned properly. Dosage matters! [↩](#fnref:7)
    
8.  If you have not seen the [movie of that name](https://en.wikipedia.org/wiki/Groundhog_Day_\(film\)#Plot) from 1993, you might be struggling with the anti-pattern name. 😉 [↩](#fnref:8)
    
9.  Assuming that the ADR template in use provides the information elements that the questions fish for. If not, that's a review finding too! [↩](#fnref:9)
    
10.  This does not always have to be the case, but helps/brings clarity. [↩](#fnref:10)
     
11.  Note: content was truncated at fetch time; see original URL for full article. [↩](#fnref:11)
