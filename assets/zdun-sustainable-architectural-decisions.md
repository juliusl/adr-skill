---
source: https://www.infoq.com/articles/sustainable-architectural-design-decisions
fetched: 2026-04-01
title: "Sustainable Architectural Design Decisions"
---

This article first appeared in **IEEE Software** magazine and is brought to you by InfoQ & IEEE Computer Society.

Software architects must create designs that can endure throughout software evolution. Today, software architecture comprises not only a system's core structure but also essential design decisions. So, to achieve sustainable architectures, we need sustainable design decisions. In correspondence with Heiko Koziolek's definition of architecture sustainability, we argue that architectural design decision sustainability involves

*   the time period when the right and relevant decisions remain unchanged, and
*   the cost efficiency of required changes to those decisions.

Although research has dealt with software architecture sustainability and corresponding evaluation methods, it hasn't yet revealed how to make architectural design decisions sustainable. In more than 10 industrial projects and eight research projects, we've applied various techniques to yield sustainable architectural design decisions. Here, we describe the fruits of our experiences: the challenges to achieving sustainable decisions, the criteria for such decisions, the solutions we tried, and the lessons we learned.

## Challenges to Achieving Sustainable Decisions

Many researchers and practitioners consider architectural knowledge (AK) to be an inherent part of software design. They point to the importance of capturing significant architectural design decisions together with other software artifacts because those decisions can ensure that the design rationale doesn't get lost. Capturing significant decisions and their rationale is difficult and often neglected because doing so requires considerable effort. In this context, architects should seek to make and document sustainable decisions. However, various challenges can hinder this desire.

## Decision Documentation Effort

A key challenge in various industrial cases is that the relatively high effort needed for design decision documentation often isn't well accepted. Many decision templates require filling in 10 to 20 fields to document a single design decision. Although each documentation attribute is important, this documentation frequently takes too much effort, so architects on high-pressure projects often neglect it. Therefore, software architects and project managers tend to disregard such practices over time, leading to decision rationale erosion in the long term.

Another consequence of strained time for decision documentation is low-quality documentation. For example, if a decision's rationale is "the end user wants it," the documentation isn't likely to be useful over time.

### Understanding the Links between Decisions and Other Software Artifacts

When documenting design decisions, it's important to establish links to other software artifacts such as requirements and architecture designs. However, although many architectural-decision-capturing templates implicitly mention requirements, establishing and maintaining the right set of traceability links between the decisions and other software artifacts is time-consuming and difficult. But as we'll see, capturing the right set of links can help increase decisions' sustainability.

### Avoiding Repetitive Effort

Many decisions are based on existing AK in the project or field. For example, in many service-oriented architecture (SOA) projects, service proxies and adapters must integrate legacy systems, each with only a slightly different design. So, each proxy and adapter requires its own decisions, but many of them are similar. Instead of documenting each decision on its own, we would like to reuse the AK and work only with variations of individual decisions, reducing the documentation effort and allowing decisions to be based more on timeproven knowledge.

### Dealing with Invalid or Bad Rationales

Decision drivers and the pros and cons of alternatives, recommendations, and rationales are particularly relevant when you're capturing design decisions. However, architects sometimes choose bad or invalid rationales, leading to decisions that can be questioned and hence are unsustainable. Consider this valid justification for the external stakeholders' needs:

_Alternative A best meets user expectations and functional requirements as documented in user stories, use cases, and the business process model._

Compare it to this poorly chosen justification:

_End users want it, but no evidence exists of a pressing business need._

## Decision Sustainability Criteria

To define decision sustainability in detail, we derived five key criteria.

### Strategic

During decision making, someone looking at strategic consequences should consider things such as the decisions' long-term impact—for example, future operations and maintenance effort.

### Measurable and Manageable

You can measure and evaluate a decision's outcome over time according to objective criteria, ideally numeric ones (as, for instance, propagated by quality attribute scenarios and workshops). Capturing all fine-grained decisions isn't possible, so architects must limit the decisions' granularity to a certain level of detail (such as creating a design class). This will lead to a more sustainable set of decisions and fewer traceability links. Moreover, limiting the number of dependencies between decisions reduces changes' ripple effect.

### Achievable and Realistic

The rationale for fitting the solution to the problem should be chosen pragmatically and made explicit. For example, architects can indicate that they have taken care to avoid over- or underengineering (that is, they should apply the "good enough" approach).

### Rooted in Requirements

Decision making should be grounded in domain-specific architecting experience and context. It should take into account the company environment as well as project requirements and constraints, including the development team's current skills, training budget, and process.

### Timeless

Decisions should be based on experience and knowledge that won't likely be soon outdated. For example, architects can choose platform-neutral architectural patterns or tactics.

### Criteria Discussion

These five criteria strongly relate to the decision life cycle because software engineers need to track every change, regardless of whether the decisions are still valid. So, the decisions' evolution across the life cycle clearly affects the degree of sustainability achieved at any time. Moreover, although not all these criteria apply to every decision, our experience shows that sustainable decisions often meet most of these criteria.

## Solutions and Lessons Learned

Table 1 summarizes the lessons we learned from applying techniques for organizing, documenting, and evolving architectural design decisions. Although these results are based on the various industry and research projects in which we've been involved, we focus here on one case from the COMPAS project: customer relationship management (CRM) fulfillment.

In the case from the COMPAS project, a customer initiates CRM fulfillment by placing an order for an ISP's services. The fulfillment process involves initializing the services, shipping the necessary equipment, charging the customer's account, and sending the customer an invoice. It also incorporates functionality provided by three service-based platforms including CRM, billing, and ISP provisioning systems. The main company's bundle of Internet and telecom services includes a network subscriber line, email addresses, Web-based administration, directory numbers, fax numbers, and voice-over-IP communication.

In the COMPAS project, we were confronted with short development iterations because working prototypes had to be delivered frequently. So, collecting all the requirements and delivering a full architecture design and documentation upfront wasn't feasible. On the other hand, if important design decisions weren't sustainable, they could impede continued development and integration. Central requirements for this project were to capture and document the relevant compliance requirements stemming from the relevant regulations and legislation, as well as the ISP's business policies, and ensure that the systems complied with them.

### Initially Apply Lean, Minimalistic Decision Documentation

To reduce design documentation effort, we experimented with lean, minimalistic documentation. One approach was our (WH)Y approach (named for its Y-shaped visualization), inspired by George Fairbanks's proposed documentation of design rationale. This approach reduces the documentation to a statement in this form:

_In the context of \<use case/user story u\>, facing \<concern c\> we decided for \<option o\> to achieve \<quality q\>, accepting \<downside d\>._

Consider the following decision in the context of our CRM fulfillment example:

_In the context of_ checking customer's accounts and signing orders, facing that duties are not adequately segregated (SOX 404), _we decided to_ ensure that customers' accounts are verified by the financial department while the orders are checked and signed by the sales department to achieve proper segregation of duties, accepting that the order processing time is longer.

Another lean approach we used, ADvISE (Architectural Design Decision Support Framework), employs questionnaires to help architects focus on important decisions. Similar recurring decisions, which are tedious to document, can be captured through an automatically generated questionnaire, which asks only essential questions about the recurring decision. So, we can document recurring decisions faster, reduce the burden of capturing AK, and produce a more sustainable set of decisions. In our experience, architects prefer using lean documentation rather than elaborate, large decision templates. On the other hand, people who are new to a project or set of decisions typically prefer to read the full-blown templates.

### Compile Guidance Models of Recurring Decisions

Recurring decisions are more timeproven than those that are used for the first time. To reduce effort in capturing recurring decisions on similar projects, we introduced guidance models to record decisions from previous projects and derive decision instances from the recurring ones. In our experience, the effort reduction gained from documenting recurring decisions and the improved quality through reuse lead to more accurate decision documentation and less maintenance effort. However, this approach requires an initial investment for creating the guidance model, and it's not applicable for completely new decisions.

### Reuse Existing AK

Much AK is codified as well-known design patterns, corporate knowledge, or proven practices. For example, in the guidance model, we used a significant number of patterns for our key design decisions. Our experience shows that such codified AK eases the creation of a guidance model that compiles proven design solutions. Regarding corporate knowledge, many companies, for example, require software architects to document a significant amount of information regarding compliance with national and international regulations such as Basel II or the Sarbanes-Oxley Act, internal business policies, or the rationale of such knowledge. In our observations, capturing and maintaining relationships between sources and architectural design decision rationales can prevent AK's vaporization, helping it endure throughout the decision's lifetime.

### Establish Explicit Traceability Links between Decisions and Requirements

Many software maintenance tasks need well-documented design decisions to model explicit traceability links between decisions and requirements. Although existing traceability approaches are valid attempts to support this goal, maintaining a large number of links is difficult. However, you can avoid this by explicitly specifying required links. For example, you can check all use cases for architecturally significant requirements and all architectural decisions to ensure no link is forgotten. The (WH)Y approach can help because it explicitly relates requirements as an influential factor of the architectural rationale, in the form of either a use case or user story.

### Establish Traceability Links among Decisions, Architecture, and Code

Developers and architects often neglect the traceability links among decisions, architectural design, and code. So, it's difficult to efficiently analyze and understand particular changes' effects before implementing and deploying them. Moreover, code changes could cause design or decision documentation to become invalid or inconsistent with the implemented system. Therefore, these links' sustainability greatly affects the architecture's sustainability. In our experience, sustainable sets of traceability links are best established semiautomatically. Manually establishing them is too much work, and fully automated approaches often lead to many inaccurate or imprecise links that won't last.

### Follow Rationale Guidelines

From analyzing many rationales in our projects, we learned that rationales should follow three guidelines. First, they should be precise and avoid commonsense statements, truisms, and "killer phrases" (defensive, negative, or pejorative phrases). Second, they should highlight decision drivers (a desired system quality such as subsecond response time) and explain why recommendations in guidance models or other trusted sources were or weren't followed. Finally, they should refer to actual project requirements, not just generic background information from the literature.

## Guidelines to Achieve Sustainable Decisions

We learned the following lessons in our work that can serve as guidelines and assessment for achieving sustainable decisions:

1.  Use a lean/minimalistic approach for the initial decision documentation.
2.  Prioritize and capture all important decisions that are relevant enough for documenting and understanding the target architecture.
3.  Detail the particularly important decisions with full-blown templates only after the initial work has been done (that is, when the decision makers are content with the architectural decisions made and confident that these decisions don't have to be revised any time soon).
4.  Use the lean/minimalistic versions from step 1 as a short version of documented decisions with the right granularity level to provide an overview of the detailed decisions, as well as for trivial or obvious decisions.
5.  Wherever possible, use existing architectural knowledge, either from guidance models or from other sources. Review and extend such knowledge and fit it to the context of the specific decision.
6.  Ensure that traceability links are established between decisions and both requirements and architectural designs/code. Provide automated consistency checking to make sure the traceability links are in sync after a change. Limit the number of dependencies between decisions.

_Note: Content was truncated during fetch. See the original source for the complete article._
