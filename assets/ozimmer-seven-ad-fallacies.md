---
source: https://ozimmer.ch/practices/2025/09/01/ADMFallacies.html
fetched: 2026-04-01
title: "Seven Architectural Decision Making Fallacies (and Ways Around Them)"
author: Olaf Zimmermann
published: 2025-09-01
updated: 2025-10-27
type: full
note: >-
  Originally linked via Medium (403-blocked). Canonical source is the author's
  own Jekyll blog at ozimmer.ch. HTML provided by human operator and converted
  to markdown.
---

# Seven Architectural Decision Making Fallacies (and Ways Around Them)

When making Architectural Decisions (ADs), things can go wrong. Biases and
misconceptions are common. As a result, answers to "why this design?" questions,
captured in AD Records (ADRs), may have little to do with the actual product
requirements and project context. Project/product success is at risk in that case.

## Motivating Example

> "We will build our online shop as a set of microservices because our cloud
> provider does that successfully in its infrastructure. They also provide a
> reference architecture for this variant of SOA, a de-facto standard for modern
> enterprise applications."

At least three fallacies are in action here.

## The Seven Fallacies (Not-To-Do List)

1. **Blind flight.** Skip context modeling and requirements analysis. Do not
   bother to elicit Non-Functional Requirements (NFRs), do not waste time to
   make high-level quality goals specific and measurable.

2. **Following the crowd.** Rest assured that what works for others will work for
   you too, no matter what the requirements might be. Also known as the lemming
   effect or *argumentum ad populum*. Variations include conference euphoria,
   guru seduction, tunnel vision, and narcissism.

3. **Anecdotal evidence to justify a design choice.** Find a positive or negative
   example in tribal knowledge and base your entire argumentation on it. Success
   or failure on one single project certainly implies future usages will have the
   same outcome.

4. **Blending whole and part, or class and instance.** If one concept in an
   architectural style has a bad reputation, the entire style must be bad. If one
   pattern from a pattern language does not fit, the entire language must be
   banned. Works in reverse too.

5. **Abstraction aversion.** Do not ask which concrete instance of a more general
   problem you are facing. Decide for concepts, technologies and
   products/services at the same time in a single mashup ADR. Don't compare
   carrots with carrots — compare them with parrots.

6. **Golden hammer and hope for a silver bullet.** One size fits all. Searching
   for alternatives is wasteful; relevant candidate solutions find their
   problems. Eventually a breakthrough innovation will make AD making trivial.

7. **Time dimension deemed irrelevant.** Once some evidence about a design option
   has been found, keep using it to justify future AD choices. IT is a stable
   domain after all. Architectures do not drift, and designs do not erode.

### Bonus Fallacy: AI Über-Confidence

- AI assistants have promising use cases (e.g., "vibe architecting"), but usage
  of generated design advice without quality assurance is an element of risk.
- If you bake fallacies into your prompt, what do you expect a stochastic parrot
  to reply?
- Accountability is an issue.

## Countermeasures: From Poor to Proper ADs

1. **Agree on landing zones** for the operating range. Make context explicit,
   compare with previous projects, elicit specific and measurable NFRs.

2. **Beat the street (when needed).** Reusing the solution to a different problem
   without checking requirement compatibility is a decision making smell. Apply a
   recognized method or your own heuristics.

3. **Provide balanced, fair judgments.** Use SMART NFRs as criteria. Make
   tradeoffs and confidence levels explicit. Résumé-Driven Development is an
   anti-pattern.

4. **Divide and conquer.** Don't mix system-wide arguments with local ones in a
   single ADR. Once an evolvable overall structure is in place, follow-on
   decisions can address more local concerns.

5. **Navigate between abstract and concrete.** Distinguish conceptual arguments
   from technological ones. Pattern selections and technology choices should go
   in separate (but related) ADRs.

6. **Continue to grow your architecture toolbox.** Stay curious, try new
   practices, patterns and tools. Learn from other architects and developers.

7. **Look back and think ahead.** Make the expected lifecycle explicit
   (disposable or durable). Specify review due dates for early ADs. Repeat older
   technical evaluations before using them as arguments.

## General Advice

- Don't let circumstances or loudmouths push you around. Invest in analysis time
  and building consensus.
- Identify ADs proactively, make them consciously, document them durably.
- Be aware of cognitive load, assess the cost of building/owning/changing, and
  the risk of over-architecting.
- Apply technology for its intended use, generative AI in particular.
- AD making is a team sport. Group decision making mitigates fallacy risk.

## Related Cognitive Biases

- Stereotyping Fallacy
- Confirmation Bias
- Law of the Instrument
- Not-Invented-Here syndrome

## Further Reading

- Eltjo Poort: "Architecture is Context"
- Ruth Malan: "Architecture Clues: Heuristics, Part ii. Decisions and Change"
- Gregor Hohpe: IT Architect Elevator
- Philippe Kruchten: "Controlling Your Architecture" (Azure DevOps Podcast ep.195)
- "Decision-Making Techniques for Software Architecture Design: A Comparative Survey"
