---
source: https://www.ost.ch/en/research-and-consulting-services/computer-science/ifs-institute-for-software-new/cloud-application-lab/architectural-knowledge-management-akm
fetched: 2026-04-01
title: "Architectural Knowledge Management (AKM) - OST IFS Cloud Application Lab"
---

[Articles on architectural decisions](https://ozimmer.ch/tags/#architectural-decisions) in ["The Concerned Architect"](https://ozimmer.ch/tags/#architecture) blog. Example: ["ADR = Any Decision Record? Architecture, Design and Beyond"](https://ozimmer.ch/practices/2021/04/23/AnyDecisionRecords.html)

## Goals

*   Contribute to body of software architecture knowledge (both general and style-specific knowledge: SOA, cloud, microservices)
*   Transfer research results from [Software Architectural Knowledge Management (SAKM)](http://www.springer.com/us/book/9783642023736) research community to practice
*   Propose and empirically validate simple but powerful techniques for architectural decision identification, making, enactment - and integrate them into agile practices and tools (e.g., decision backlog, architecturally evident coding styles)

## Results (Overview)

*   [ADMentor](https://ifs.hsr.ch/index.php?id=13201&L=4#c48848), prototypical tool support for architectural decision modelling and reuse, and Cloud Design. See WICSA 2015 conference paper for more information.
*   Cloud Guidance Model collecting architectural decisions that recur when designing cloud-native applications and when migrating and refactoring existing applications for the cloud
*   Quality stories; POINT criteria for API design and management; service contract template
*   Contributions to [IEEE Software](http://ieeexplore.ieee.org/xpl/mostRecentIssue.jsp?reload=true&punumber=52)

## What is an Architectural Decision (AD)?

*   A decision that you wish you could get right early in a project (M. Fowler)
*   A design decision that is costly to change (G. Booch)
*   Architectural decisions directly or indirectly determine the non-functional characteristics of a system. Each decision describes a concrete, architecturally significant design issue for which several potential solutions exist, and provides rationale for the selection of the chosen solution(s).

## Decision Capturing Advice and Templates (in Alphabetical Order)

*   [ADR template by M. Nygard](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions)
*   IBM ARC 100 table template
*   [ISO/IEC/IEEE 42010 template](http://www.iso-architecture.org/ieee-1471/templates/)
*   Markdown ADR (MADR), see [The Markdown ADR (MADR) Template Explained and Distilled](https://medium.com/olzzio/the-markdown-adr-madr-template-explained-and-distilled-b67603ec95bb) and the [Architectural Decision Records](https://adr.github.io/) organization on GitHub
*   Table format in IEEE Software article by J. Tyree and A. Akerman
*   Y-Statements by O. Zimmermann

## Research Papers (Selection)

*   O. Zimmermann, L. Wegmann, H. Koziolek, T. Goldschmidt, Architectural Decision Guidance across Projects, Proc. of IEEE/IFIP WICSA 2015
*   O. Zimmermann, Architectural Decisions as Reusable Design Assets, IEEE Software, Volume 28, Issue 1, 2011
*   M. Anvaari, O. Zimmermann. Semi-automated design guidance enhancer (SADGE), Proc. of ECSA 2014

## Related Open Source Projects

*   [Markdown Architectural Decision Records (MADR)](https://github.com/adr/madr)
*   [ADMentor](https://github.com/IFS-HSR/ADMentor) and [Embedded Architectural Decisions](https://github.com/koppor/embedded-adl)
*   [Architectural Refactoring Tool (ART)](https://github.com/bisigc/art)
*   [Service Cutter](https://servicecutter.github.io/)
