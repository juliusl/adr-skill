---
source: https://www.fabian-keller.de/blog/documenting-architecture-decisions
fetched: 2026-04-01
title: "Documenting Architecture Decisions"
---

Take a moment and think of a big architectural decision you made in your last project. That one point were you started just with the problem description and had to choose either to go one way to solve it or the other. That was a tough decision to make, right?

Now, can you know show me based on what facts, criteria and opinions that decision was made? Who was actually involved in making that decision? From a retrospective point of view, wouldn't it now be really helpful to go through that decision again and see how it turned out in the end? Did the architects anticipate the pain points experienced later on in the project? Were they unforeseeable based on the knowledge present at the decision point? How could those decisions be improved in the future?

## The Problem with Decisions

Projects typically document a lot. There are docs for the people involved, for the requirements, for the architecture, a complete backlog with all tasks that were worked on, for the tests, and if you are lucky also some end-user documentation telling how to operate and use the software. And if you are particularly lucky, there is some form of documentation where key parts of the system are explained, enlightening the _why_ parts of the architecture and not only the _how_ part (e.g., the Cloud Foundry [Diego design notes](https://github.com/cloudfoundry/diego-design-notes)).

However, on most projects one key ingredient to the documentation is missing: decision documentations. Decisions are made on a regular basis in projects, some more important and others less important. Some decisions have huge consequences. People even take days to evaluate, analyze, discuss and agree on a decision.

Now the problem with decisions is that they only live in the minds of people that made them. People forget or leave, and so do the decisions that got the project to where it is today. Everyone is doing a sprint retrospective, but heck, no one evaluates how good the decisions made turned out in the end. The organization around the project has no chance to learn the most valuable lessons from those decisions, as those decisions just fade from the awareness of all project members.

[Recent research confirmed](https://pdfs.semanticscholar.org/89a5/11310ab99f1b1da69b420b6ea78137fcb39d.pdf) that a standard approach for documenting decisions is very helpful for project teams. Also, the documentation process needs to be easy and not consume too much time.

## Solving the Problem with Decisions

To solve the problem with lacking documentation of decisions adhere to the following two simple principles:

### Standardize Decision Documentation

As decisions (and what you can learn from them) are so valuable, it is important to frequently document them in a standardized manner. Decisions need to be documented with short and concise, but complete prose. Preferably, they live in text files right next to the source code as then decisions are omnipresent and receive sufficient attention.

Having a standardized decision documentation format…

*   lowers the barrier of entry to document a decision, as there is a template ready to be populated.
*   helps people to document all important parts of the decision.
*   eases onboarding of new team members, as they learn to appreciate the decision documentation to learn about the project.
*   allows to use/build tooling, if desired.

For example, one standardized format are [ADRs (Architecture Decision Records)](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions) which even have [basic tooling support](https://github.com/npryce/adr-tools).

### Get Everyone to Document Decisions

Documenting decisions is not hard. The difficulty is in detecting when a decision was made and pouring that into the decision documentation. People need to start being aware of when they make a decision with a lasting impact on the project. There are also a lot of decisions being made in every project that are not worth to document. Take these examples and your gut feeling will tell you which decisions are worthwhile to be documented:

1.  Decision whether to use a pipes-and-filters based or event-based architecture.
2.  Decision whether to place the button to the left or right of the component.
3.  Decision whether to develop 2 or 5 components to build the feature.
4.  Decision whether to use framework X or Y.
5.  Decision to write a fluent API for a builder.

Based on the little information provided decisions 1, 3, and 4 definitely require a documented decision. Decision 2 does not need extra documentation and for decision 5 it depends how important the fluent API is for the project (it could in fact just be an implementation detail or an outstanding feature).

To raise the awareness for architecture decision documentations at the end of every meeting simply ask the question: did we just take an architectural decision worth documenting? If there is then agree upon who is going to document the decision. After the decision has been documented, all other team members involved in the discussion should peer-review the decision document.

## Existing Templates for Documenting Architecture Decisions

Documenting architectural decisions is not a new thing, indeed many people have come up with templates to document them. A very pragmatic and simple documentation format for decisions are the [ADRs (Architecture Decision Records)](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions). An ADR always contains the date the decision was made, a status, the broader context of the decision, the decision itself and its consequences. That makes it quick to write and [easy to grasp](https://github.com/npryce/adr-tools/blob/master/doc/adr/0008-use-iso-8601-format-for-dates.md) when reading the decisions. While the format is very simple, it may not be everyone's taste, as there are quite a lot of more details you could potentially put in a decision documentation.

Jeff Tyree and Art Akerman introduced a very detailed template for architecture decisions in their article "[Architecture Decisions: Demystifying Architecture](https://personal.utdallas.edu/~chung/SA/zz-Impreso-architecture_decisions-tyree-05.pdf)" back in 2005. The template includes the following pieces of information:

*   **Issue:** the architectural issue being addressed by the decision
*   **Decision:** the decision made
*   **Status:** the decision's status
*   **Group:** an architectural ontology to group related decisions.
*   **Assumptions:** assumptions made about the environment
*   **Constraints:** additional constraints the decision may pose upon the environment
*   **Positions:** a list of viable alternatives
*   **Argument:** why the specific alternative was chosen
*   **Implications:** consequences the decision imposes
*   **Related decisions:** link to related decisions
*   **Related requirements:** link to the requirements that are affected by this decision
*   **Related artifacts:** link to related architecture, design or other documents
*   **Related principles:** list principles or policies that affected the decision
*   **Notes:** capture notes and issues the team discusses

While being truly overwhelming to when documenting smaller decisions, the template captures some interesting information not documented in the ADRs (such as a list of viable alternatives).

## A Template for Documenting Architecture Decisions

Based on the above templates I have made a custom template that I use throughout projects. The template is filled with advice on how to document the decision and makes it easy to onboard people. Grab the template, modify it to your needs and start documenting architecture decisions:

[View Gist on GitHub](https://gist.github.com/FaKleiser/2f9c63b6e1d436abb7358b68bf396f57)
