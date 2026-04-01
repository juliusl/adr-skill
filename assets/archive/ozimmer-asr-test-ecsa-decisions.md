---
source: https://www.ozimmer.ch/practices/2020/09/24/ASRTestECSADecisions.html
fetched: 2026-04-01
title: "Architectural Significance Criteria and Some Core Decisions Required"
---

What is important when analyzing and designing software architectures, and what is not? This post proposes 5+2 criteria for the architectural significance of requirements (and other artifacts) and applies them to several examples. It also collects a few architectural decisions to be made early due to their impact, cost and risk (an outcome of [ECSA 2020](/authoring/2020/05/06/CallForParticipation.html)).

### Motivation

Let's assume it is the first day for you on a project that has been running for a while (for instance, one that modernizes some legacy system); you might be an external consultant that has just been hired or have switched role and/or project internally. What to work on first?

> As a software engineer with architecture responsibilities, I want to prioritize technical issues quickly so that architecturally significant issues are addressed at their most responsible moment and we do not have to revise decisions and designs unnecessarily later. When doing so, I want to be as objective as possible to avoid that loud voices receive more attention than they deserve so that the truly urgent and important things are tackled first.

The time budget per issue is one to two minutes at most, as it is not unusual to be confronted with 10 to 100 issues per day — thanks to email and auto-notifications in tools such as Kanban boards, issue trackers and source code management systems. Modern world! 😉

### Seven Criteria for Architectural Significance

So much for motivation. Starting from insights from peers and then reflecting on my own heuristics, I ended up with seven criteria to assess whether issues qualify as [Architecturally Significant Requirements (ASRs)](https://en.wikipedia.org/wiki/Architecturally_significant_requirements):

1.  The requirement is directly associated with high _business value_ (benefit vs. cost) or _business risk_.[1](#fn:1)
2.  The requirement is a _concern_ of a particularly important _stakeholder_ such as the project sponsor or an external compliance auditor.
3.  The requirement includes _runtime Quality-of-Service (QoS) characteristics_ (such as performance needs) that deviate from those already satisfied by the evolving architecture substantially.
4.  The requirement causes new or deals with one or more existing _external dependencies_ that might have unpredictable, unreliable and/or uncontrollable behavior.[2](#fn:2)
5.  The requirement has a _cross-cutting_ nature and therefore affects multiple parts of the system and their interactions; it may even have _system-wide impact_, short term and/or in the long run (examples: security, monitoring).[3](#fn:3)
6.  The requirement has a _First-of-a-Kind_ (FOAK) character: For instance, this team has never built a component or subsystem that satisfies this particular requirement before.
7.  The requirement has been _troublesome_ and caused critical situations, budget overruns or client dissatisfaction _on a previous project_ in a similar context.

The list is ordered logically, from project goals and key players to analysis and design elements to organizational matters; this outside-in order does not imply that criterion 2 has less weight than criterion 1 or 3 (and so on). Criteria 1 to 5 are somewhat more objective and easier to agree upon and generalize than criteria 6 and 7, which are highly context-specific.

I have been using and teaching these criteria for several years now; I do not claim that they are complete, but picked the ones that I deem most useful.[4](#fn:4)

### A Scoring/Assessment Template (ASR Test)

A table is an obvious format for the resulting seven-criteria _ASR Test_:

_Issue_

Value/Risk

Key Concern

New QoS

Ext. Dep.

X-Cutting

FOAK

Past Pb

Score

\_\_

☐

☐

☐

☐

☐

☐

☐

\_\_

\_\_

☐

☐

☐

☐

☐

☐

☐

\_\_

\_\_

☐

☐

☐

☐

☐

☐

☐

\_\_

One option to fill the cells in such a table are basic values such as Y = yes, N = no, ? = unknown; if you prefer, you can also enter a bit more elaborate ones such as H, M, L and n/a ([acronym](/authoring/2020/07/02/ReviewAndMeetingMarkup.html) for "not applicable"). If you cannot agree on the significance of a particular requirement immediately, do not hesitate to mark it as open and track it as a new backlog item (or maintain a separate parking lot of open issues to be reconsidered periodically).

Instances of one criterion may appear several times when assessing a single requirement. For example, criterion 4 (external dependencies) is met twice if the splitting of user story unveils that two additional public Web APIs must be consumed to realize the story.

I do not fill out the table explicitly every time I screen and filter technical issues, but run the test rather (sub-)consciously while reading any issue that grabs my attention (in a chat message, an email, a Trello board and so on). If you do want to create an explicit representation, a project wiki, team space or other collaboration tool may host it.

_Note_: The ASR Test is _not_ a quantitative tool, a weight calculator; relevance assessments are always qualitative, subjective and [context-depending](/assets/presos/ZIO-ICWEKeynoteWADEC3v10p.pdf); the test makes formerly tacit knowledge somewhat explicit ("worst first"). Do not spend more time on assessing relevance than on tackling and solving issues!

### Examples and Light Tool Support: miro Board

The following table scores some requirements and constraints according to their architectural significance:

Requirement

Score

Explanation (Rationale)

_Data retention policy of 10 years required to achieve regulatory compliance_

**high**

Violation of this requirement would lead to fines (C1, C2); redesign might require change of database technology and hosting model (C4, C5)

_Technical constraint to prefer a particular messaging middleware and backend API_

**medium-high**

If a different architectural element is chosen, additional licensing and training cost arise (C2); standardized APIs promise interoperability (which has to be proven) (C3)

_Deployment pipeline automation_

**low-medium**

Needed to be able to test, deploy, release often, but not something end users and project sponsors are willing to care about and pay for usually (so team-internal concern) (C5)

_Name of Java class wrapping access to backend_

**low**

Decision that is not visible to external stakeholders; simple to change in IDEs that support refactoring (no C met)

For your convenience, I created a board in the online whiteboard platform miro for this table format (first five ASR criteria only); feel free to use and/or extend it (under a Creative Commons license): [https://miro.com/app/board/o9J\_kl49CmA=/](https://miro.com/app/board/o9J_kl49CmA=/). Please contact me if you want to get access to the miro template at [https://miro.com/app/board/o9J\_kl44WKk=/](https://miro.com/app/board/o9J_kl44WKk=/) too.

Here is an example of a filled-out miro template (issues in German, sorry for that!):

![miro template/example](/assets/images/ZIO-AppArchW1ASRTestSolution.png)

Based on the test result, I'd probably look at the payment change request in the online shop first and the design of the security credential cache next. I'll not get involved with the class renaming (unless I have to review the change w.r.t. project-wide coding guidelines).

### ECSA Working Sessions 2020: ASRs and Core Decisions

The Industry Program of [European Conference on Software Architecture (ECSA) 2020](https://ecsa2020.disim.univaq.it/track/ecsa-2020-industry-program?date=Thu%2017%20Sep%202020#program) that took place online triggered me to post my already existing ASR Test here and now: In one of the Working Sessions, my program co-chair Anton Jansen asked "Where and how to draw the line of architecting (and) decision making?", and I replied "Can [ASRs](https://en.wikipedia.org/wiki/Architecturally_significant_requirements) help? One of the participants commented that in his organization, ASRs are usually referred to as _drivers_, with business, functional, quality and constraints (see [this presentation from Fraunhofer IESE](http://wwwagse.informatik.uni-kl.de/teaching/sads/material/SSA_2018_03_Drivers%20Decisions.pdf) for details).

Back to original question in the session, attendees nominated the following architectural decisions that have a rather early [Most Responsible Moment (MRM)](https://wirfs-brock.com/rebecca/blog/2011/01/18/agile-architecture-myths-2-architecture-decisions-should-be-made-at-the-last-responsible-moment/) and qualify as examples of "core decisions" (a topic for a future post):

*   "What minimal functionality is expected of a product? What regulations to adhere to?"
*   "Which architectural style is used?"
*   "What technology stacks are supported?"
*   "How is integration done? What are the minimum integration options a product should have?"
*   "What governance structure should be in place for a product? What is the process for a team to contribute their project to the main product?"
*   "Compiler that teams must use, development environment, operating system version."

Certainly not complete, but not a bad start towards a catalog of such core decisions either. Other topics we touched upon (2x30 mins run out so quickly!) were [service granularity](https://github.com/socadk/design-practice-repository/blob/master/activities/SDPR-StepwiseServiceDesign.md) and ownership of edge hardware and software in [cloud, edge and fog computing](https://dzone.com/articles/cloud-vs-fog-vs-edge-computing-3-differences-that).[5](#fn:5)

### Concluding Thoughts

Here are the take-away messages from this post and some additional advice/remarks:

*   When deciding which technical issues to focus on, consider value, cost and risk implications. Follow an outside-in approach, starting from project goals and external interfaces that you will have to consume (and therefore rely on).
*   The "five plus two" ASR criteria that work best for me are:

> Business value and risk; key stakeholder concern; unusual quality-of-service requirement (at least one order of magnitude more advanced than previous ones); external dependencies that are uncontrollable, unpredictable or unreliable; cross-cutting, system wide impact

> First-of-a-kind character (novelty for team); bad experience and trouble in the past

*   I use the criteria not only when joining a project (as the motivating user story at the start of this post assumes), but also when returning to a project after an external event and checking my mailbox, when planning my week/day and so on.
*   Balance tactic progress (for instance, indicated by iteration [velocity](https://www.agilealliance.org/glossary/velocity) and long-term thinking and development of the system; some non-functional properties only become relevant (and might start to hurt) in the long term.
*   The ASR test presented in this post works for requirements (mostly non-functional but also functional), design elements (components and connectors, viewpoint depending) and [architectural decisions](/practices/2020/04/27/ArchitectureDecisionMaking.html).

ASR scoring is not an exact science, but qualifies as a "mighty" method element imho ([Michael Keeling's ECSA 2020 keynote](https://ecsa2020.disim.univaq.it/details/ecsa-2020-keynotes/3/Mighty-Methods-Four-Essential-Tools-for-Every-Software-Architect-s-Silver-Toolbox) has a definition and requirements/success criteria).

Do the above ASR criteria work for you? Is a criterion (test question) missing? [Contact me](/about/)!

– Olaf (a.k.a. ZIO)

PS: The ASR criteria and test are also available as a [story on Medium](https://medium.com/olzzio/architectural-significance-test-9ff17a9b4490).

#### Acknowledgements

I would like to thank Peter Eeles for his initial thoughts on qualifying architectural significance, [Jasmin Jahic](https://www.cl.cam.ac.uk/~jj542/) and [Mirko Stocker](https://microservice-api-patterns.org/about#mirko-stocker) for reviews of drafts of this post, Anton Jansen, Somayeh Malakuti and all other ECSA Working Session presenters and attendees for the lively discussions. I also thank Gerald Reif and HSR/OST [Application Architecture](https://studien.rj.ost.ch/allModules/28236_M_AppArch.html) students 2017 to 2020 for their input, feedback and lively discussions on the topic of architectural significance.

#### Notes

1.  Do you wonder whether technical risk is missing? Please have a look at the next six criteria 😅. The entire ASR Test, and all software architecture work, can be seen as risk prevention and mitigation work (when taking a project management perspective). [↩](#fnref:1)
    
2.  Note that dependencies may be compile time ones (library, package manager such as maven) or runtime ones (remote API, message queue) or even have a logical or organizational nature only (examples: approvals, deliveries). Also note that we might not know all dependencies and their properties when assessing architectural significance; hence, it might be appropriate (or even imperative) to ask questions and/or prototype. [↩](#fnref:2)
    
3.  You might want to differentiate between (1) deliberately independent and (2) heavily intertwined parts of the system here: (1) The new requirement might be in conflict with another one considering the desire to avoid dependencies; a tradeoff analysis or prioritization decision might be required. (2) This might be inherent to the problem or an [architectural smell indicating a need to refactor](http://rdcu.be/lFW6). [↩](#fnref:3)
    
4.  The issues can be requirements (incl. change requests) as called out in the introduction of the seven criteria above, but also pending architectural design decisions or even design activities on structural elements such as components and connectors.  [↩](#fnref:4)
    
5.  I'll let somebody else summarize these discussions elsewhere. [↩](#fnref:5)
