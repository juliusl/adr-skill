---
source: https://adr.github.io/madr/
fetched: 2026-04-01
title: "Markdown Architectural Decision Records (MADR)"
---

## Markdown Architectural Decision Records [![part of ADR](https://img.shields.io/badge/part_of-ADR-blue.svg)](https://adr.github.io)

> "Markdown Architectural Decision Records" (MADR) `[ˈmæɾɚ]` – decisions that [matter `[ˈmæɾɚ]`](https://en.wiktionary.org/wiki/matter#Pronunciation).

An Architectural Decision (AD) is a justified software design choice that addresses a functional or non-functional requirement of architectural significance. This decision is documented in an Architectural Decision Record (ADR), which details a single AD and its underlying rationale. To capture these records in a lean way, the Markdown Architectural Decision Records (MADRs) have been invented: MADR is a streamlined template for recording architectural significant decisions in a structured manner.

Scientific publication: [Markdown Architectural Decision Records: Format and Tool Support](https://dblp.org/rec/conf/zeus/KoppAZ18.html).

## Contents

*   [News](#news)
*   [Overview](#overview)
*   [Example](#example)
*   [Applying MADR to your project](#applying-madr-to-your-project)
    *   [Initialization](#initialization)
    *   [Create a new ADR](#create-a-new-adr)
    *   [Lint ADRs](#lint-adrs)
*   [Using MADR in large projects and product developments](#using-madr-in-large-projects-and-product-developments)
    *   [Usage of categories](#usage-of-categories)
*   [Full template](#full-template)
*   [Older versions](#older-versions)
*   [License](#license)

## News

*   2024-09-17: Release of [MADR 4.0.0](https://github.com/adr/madr/releases/tag/4.0.0). Check out the "bare" and "minimal" templates at [https://github.com/adr/madr/tree/4.0.0/template](https://github.com/adr/madr/tree/4.0.0/template).
*   2024-09-02: Release of [MADR 4.0.0-beta](https://github.com/adr/madr/releases/tag/4.0.0-beta)
    *   To strengthen the importance for decisions in software architecture work, MADR spells out "Markdown Architectural Decision Records". They can still be used to sustain any decision, our focus is on architectural decisions.
*   2023-04-05: Two new Medium stories ["How to create Architectural Decision Records (ADRs) — and how not to"](https://medium.com/olzzio/how-to-create-architectural-decision-records-adrs-and-how-not-to-93b5b4b33080) and ["How to review Architectural Decision Records (ADRs) — and how not to"](https://medium.com/olzzio/how-to-review-architectural-decision-records-adrs-and-how-not-to-2707652db196). Metaphors, patterns, anti-patterns, checklists applicable (but not limited) to MADRs.
*   2022-11-22. MADR Version 1.0 was released five years ago. A new blog post ["The Markdown ADR (MADR) Template Explained and Distilled"](https://medium.com/olzzio/the-markdown-adr-madr-template-explained-and-distilled-b67603ec95bb) is available on Medium.
*   2022-10-09: Release of MADR 3.0.0.
*   2022-05-17: Release of MADR 3.0.0-beta.
*   2021-04-25: MADR examples featured in Medium stories.
*   2021-04-08: MADR recommended as an ADR format in "Design Practice Repository".
*   2020-09-29: MADR presented in the keynote at the workshop "Second Software Documentation Generation Challenge (DocGen2)".
*   2019-07-08: MADR referenced in Architectural Decisions — The Making Of.
*   2018-04-13: Mentioned in @vanto's presentation about ADRs.
*   2018-04-03: Scientific publication: Markdown Architectural Decision Records: Format and Tool Support.

## Overview

An [Architectural Decision (AD)](https://en.wikipedia.org/wiki/Architectural_decision) is a software design choice that addresses a functional or non-functional requirement that is architecturally significant. This might, for instance, be a technology choice (e.g., Java vs. JavaScript), a choice of the IDE (e.g., IntelliJ vs. Eclipse IDE), a choice between a library (e.g., [SLF4J](https://www.slf4j.org/) vs [java.util.logging](https://docs.oracle.com/javase/8/docs/api/java/util/logging/package-summary.html)), or a decision on features (e.g., infinite undo vs. limited undo). Do not take the term "architecture" too seriously or interpret it too strongly.

It should be as easy as possible to a) write down the decisions and b) to version the decisions.

This repository offers a solution to record any decisions. It provides files to document any decisions using **M**arkdown and **A**rchitectural **D**ecision **R**ecords.

## Example

```
# Use Plain JUnit5 for advanced test assertions

## Context and Problem Statement

How to write readable test assertions?
How to write readable test assertions for advanced tests?

## Considered Options

* Plain JUnit5
* Hamcrest
* AssertJ

## Decision Outcome

Chosen option: "Plain JUnit5", because it is a standard framework and the features of the other frameworks do not outweigh the drawbrack of adding a new dependency.
```

For more examples see [examples](/madr/examples.html). For the MADR project itself, all ADRs are rendered at [decisions/](decisions/).

## Applying MADR to your project

### Initialization

Create folder `docs/decisions` in your project. Copy all files in [folder `template` from the MADR project](https://github.com/adr/madr/tree/develop/template) to the folder `docs/decisions` in your project.

Using `npm`, this can be done using the following command:

```
npm install madr && mkdir -p docs/decisions && cp node_modules/madr/template/* docs/decisions/
```

### Create a new ADR

1.  Copy `docs/decisions/adr-template.md` to `docs/decisions/NNNN-title-with-dashes.md`, where `NNNN` indicates the next number in sequence.
2.  Edit `NNNN-title-with-dashes.md`.

The filenames follow the pattern `NNNN-title-with-dashes.md`, where `NNNN` is a consecutive number.

### Lint ADRs

ADRs are written using Markdown. [markdownlint](https://github.com/DavidAnson/markdownlint#markdownlint) can be used to check formatting consistency.

## Using MADR in large projects and product developments

### Usage of categories

MADR logs may be categorized by defining subdirectories:

```
.
`-- decisions
    |-- backend
    |   |-- 0001-use-quarkus.md
    `-- ui
        `-- 0001-use-vuejs.md
```

## Full template

The current development version of the MADR template includes sections for: Context and Problem Statement, Decision Drivers, Considered Options, Decision Outcome (with Consequences and Confirmation), Pros and Cons of the Options, and More Information.

## Older versions

| Version | Branch | Homepage |
|---------|--------|----------|
| 1.x | release/v1 | README.md |
| 2.x | release/v2 | README.md |
| 3.x | release/v3 | index.md |

## License

This work is dual-licensed under [MIT](https://opensource.org/licenses/MIT) and [CC0](https://creativecommons.org/share-your-work/public-domain/cc0/).

```
SPDX-License-Identifier: MIT OR CC0-1.0
```
