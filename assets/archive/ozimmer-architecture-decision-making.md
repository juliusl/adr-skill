---
source: https://www.ozimmer.ch/practices/2020/04/27/ArchitectureDecisionMaking.html
fetched: 2026-04-01
title: "Architectural Decisions — The Making Of"
---

![ZIO](/assets/images/olzzio.jpg)

ZIO[Contact](https://ozimmer.ch/about/) Consulting IT architect and software architecture coach.

Apr 27, 2020 (Updated: Jul 16, 2025)  
Reading time: 12 minutes

Architectural Decisions (ADs) have been answering "why" questions about design options since the inception of software architecture in the 1990s. Ways to capture them should be part of each architect's toolbox. Less is more — only the key ADs justify the effort, and only crisp and sound justifications will convince the reader. Let's see how to get there.

### Motivation

Let's start with an example, made up from a true story:[1](#fn:1)

> An agile software development team welcomes a new member, let's assume an external consultant. The newbie starts challenging the current design instantly and provocatively:
> 
> *   "Why did you decide for server-side rendering, a Single Page Application (SPA) executing JavaScript in the browser is way more responsive and efficient?"
> *   "Shouldn't the client and the server use JSON rather than XML to exchange data?"
> *   "How could you bet the future of the product on a dated technology such as JSF?"
> 
> These issues were discussed multiple times, but nobody remembers which options were considered and which criteria drove the final section. Actually, some of the opinion leaders have left the project or the company since then. The ones that stayed do not remember the line of thinking and detailed arguments at the time — so much has happened since then.

Does this sound familiar? If the same questions are reconsidered time and time again, progressive iterations turn into "we have been here before" circles. A decision log can answer the questions and "silence" the newbie (who probably had good intentions).  

### A Bit of History

Architectural Decisions (ADs) have always been important. Some teams make them a bit more explicit(ly) than others — but we all want to decide at the [most responsible moment](http://wirfs-brock.com/blog/2011/01/18/agile-architecture-myths-2-architecture-decisions-should-be-made-at-the-last-responsible-moment/), and even the most agile teams capture the essential ones somehow.

Method engineers and practice leaders therefore have had a lot to say about ADs for quite some time, and still have. We can identify three phases of awareness and adoption:

1.  _Rise/differentiator_ phase. In the 1990s, ADs received less attention than notations such as [UML](https://www.uml.org/) or [ADLs](https://en.wikipedia.org/wiki/Architecture_description_language), but they were definitely there already:
    *   The term "rationale" was introduced as one of three components of any software architecture in a research paper in 1992.[2](#fn:2)
    *   Software vendors and professional services firms established templates for their staff, but usually did not publish them (this was before blogging and open sourcing became popular). For instance, the work product _ARC-100_ in the IBM Global Services Method was called — surprise, surprise — _Architectural Decisions_.[3](#fn:3)
    *   The commercial Rational Unified Process (RUP)[4](#fn:4) also included a decision guideline, which has become a bit hard to find online (for instance, in OpenUP).
2.  _Trend/boom_ phase. In the 2000s, Architectural Knowledge Management (AKM) and ADs became a hot topic in software architecture research, and senior architects in industry began to share their AKM practices with the public:
    *   A Groningen workshop in 2004 kicked an intensive period of research off. For instance, A. Jansen [set the scene](https://www.researchgate.net/publication/220864796_Software_Architecture_as_a_Set_of_Architectural_Design_Decisions) and P. Kruchten donated a [taxonomy](https://www.researchgate.net/publication/245352502_An_Ontology_of_Architectural_Design_Decisions_in_Software-Intensive_Systems). R. Capilla, U. van Heesch and others suggested to let decisions form a dedicated viewpoint.
    *   J. Tyree and A. Akerman from CapitalOne took inspiration from the IBM e-business Reference Architecture (that came with pre-filled AD records) and motivated why ADs matter in an [article in IEEE Software](https://ieeexplore.ieee.org/document/1407822) that also presented a rich template.
    *   In my [PhD thesis](https://elib.uni-stuttgart.de/handle/11682/2682), I investigated whether AD issues recur when similar designs are used on multiple projects, and whether decision trees can be mined from the gained experiences. I suggested a method to identify the need for ADs and the available options in requirements and style definitions systematically. Service-Oriented Architecture (SOA) served as an exemplary architectural style that I tried my methods and models on. One such decision model became a company-internal knowledge asset; all concepts and many examples of AD issues, options, and criteria appear in my [research papers](/papers/) from that time.
    *   The [ISO/IEC/IEEE 42010](http://www.iso-architecture.org/ieee-1471/index.html) standard was released. Arguably one of the most readable standards of all times, it recommends to capture decision rationale and gives scoping and filtering advice (and clarifies many other concepts).
    *   Academic conferences such as WICSA and ECSA and scientific journals presented numerous research results, some of which found their way into practice or inspired others. For instance, the connection between ADs and patterns was investigated by a number of researchers ([ArchPaD](https://www.researchgate.net/publication/4322332_Combining_Pattern_Languages_and_Reusable_Architectural_Decision_Models_into_a_Comprehensive_and_Comprehensible_Design_Method) was my take, as joint work with Uwe Zdun). The Springer book ["Software Architecture Knowledge Management"](https://www.springer.com/us/book/9783642023736) compiled chapters from many AKM/AD researchers around the world; I was lucky enough to be able to contribute a [case study on SOA infrastructure decision reuse](http://soadecisions.org/soad.htm#soaira). ["A comparative study of architecture knowledge management tools"](https://medium.com/jss-papers-of-the-year/architectural-knowledge-management-tools-7c8fdb3a2c88) by Antony Tang et al has the full tool story (as of 2010).
3.  _Commodity/must have_ phase (since 2010). ADs have now made it into the industry project mainstream:
    *   M. Nygard received a lot of attention with his [Architecture Decisions Records (ADR) blog post](http://thinkrelevance.com/blog/2011/11/15/documenting-architecture-decisions). The topic regularly appears at software architecture conferences such as [SATURN](https://resources.sei.cmu.edu/news-events/events/saturn/presentations.cfm).
    *   The architecture documentation template arc42 dedicates [Section 9](https://docs.arc42.org/section-9/) to ADs and gives nine related tips.
    *   Tools were released. Open source ones that I contributed to include [Markdown Architectural Decision Records (MADR)](https://github.com/adr/madr), initiated by [Oliver Kopp](https://github.com/koppor/) (who needed a template for his master student projects), and AD Mentor, an add-in for Enterprise Architect (joint work with [Heiko Koziolek](http://www.koziolek.de/heiko/) and Thomas Goldschmidt from ABB).

So the ARC-100 work product from the 1990s was right: "there are no reasons not to have this work product." ADs are here to stay. There even is a [Wikipedia page on ADs](https://en.wikipedia.org/wiki/Architectural_decision) now.

We know about importance and history of AD making now — but we still aren't sure yet which ones to focus on and how to capture them. The latter topic (how to capture) is next up in this post; the former (which ones to capture) has to wait for another one — please have a look at [Hint 9-1: "Document only architecturally relevant decisions"](https://docs.arc42.org/tips/9-1/) in arc42 or [this workshop paper](http://soadecisions.org/download/SOAD-SHARK2012v13Final.pdf) if you cannot wait (the "meta issues" in the paper will work for any style).

### Y-Statements and other Templates

As a consulting IT architect at IBM, I followed the ARC 100 template, usually trimmed down a bit (following my first law of method adoption: "if in doubt, leave it out"). On the [SOAD PhD project](http://soadecisions.org/soad.htm), I proposed a richer meta model, optimized for decision reuse (i.e., decisions required, not decisions made). However, I had to learn not to overdo it: the maintenance effort of filled out, full-fledged decision records (that are shared between projects) became rather high.

My next two projects had senior managers and tech leads as sponsors, who did not want to spend too much of their precious time on reviewing lengthy text documents. To quote one of the catchers of my work: "can you fit each decision on one presentation slide?". Giving people what they want is not necessarily a bad idea, especially if these people sponsor your project. So I rethought my approach to AD capturing and sharing and tried to focus on the bare essentials of a decision.

The result: _(WH)Y-statements_, featured in a presentation called "Making Architectural Knowledge Sustainable: Industrial Practice Report and Outlook" at [SATURN 2012](https://insights.sei.cmu.edu/library/saturn-2012-presentations/), used on a large C-level enterprise transformation and IoT cloud project and published in an [IEEE Software/InfoQ article](https://www.infoq.com/articles/sustainable-architectural-design-decisions/):

```
1
2
3
4
5
6
In the context of the Web shop service, 
facing the need to keep user session data consistent and current across shop instances, 
we decided for the Database Session State Pattern 
and neglected Client Session State or Server Session State
to achieve cloud elasticity, 
accepting that a session database needs to be designed, implemented, and replicated.
```

Each template element appears on one line in the above example:[5](#fn:5)

1.  _context_: functional requirement (user story or use case) or an architecture component,
2.  _facing_: non-functional requirement, for instance a desired quality,
3.  _we decided for_: decision outcome, arguably the most important part,
4.  _and neglected_: alternatives not chosen (not to be forgotten!),
5.  _to achieve_: benefits, the full or partial satisfaction of the requirement,
6.  _accepting that_: drawbacks, impact on other properties/context and effort/cost.

This template is is much leaner than my previous attempts. Its six sections form one (rather long) sentence and can be visualized with the letter "Y", pronounced just like the word "why", which explains the name of the template:

![Y-Statement Template](/assets/images/Y-StatementTemplate.png)

Here is a comparison between my Y-statements and other popular templates:

![ADM Template Comparison](/assets/images/WICSA2015-ADTemplateComparisonTable.png)

I'd say that Y-statements do pretty well in this comparison (in terms of being lean); the five sections each in Nygard's ADRs and the arc42 tip can get rather long. The table and a more detailed comparison appears in this [WICSA 2015 paper](/assets/admentor-wicsa2015ubmissionv11nc.pdf) (that also presents ADMentor). You can find a Java annotation of this six-part structure in [e-ADR](https://github.com/adr/e-adr).

I use Y-statements in teaching. Others have picked them up too. The [Cards for Analyzing and Reflecting on Doomed Software (card42)](https://cards42.org/#adr) initiative by innoQ architects features them as the third[6](#fn:6) of many very nice cards. The [decision story](https://studenttheses.uu.nl/handle/20.500.12932/34920) approach and thesis start from them. And Herberto Graca seems to use them in the summary section of his ADR template, see his blog post ["Documenting Software Architecture"](https://herbertograca.com/2019/08/12/documenting-software-architecture/). There also was a lively discussion on [LinkedIn](https://www.linkedin.com/posts/ozimmer_softwarearchitecture-architecturehaikus-activity-6669898998922457088-nwVM).

### Good and Bad Justifications

Let's see which arguments have a chance to pass the reviewers of your AD records (it does not matter whether you use my Y-format or another one, but a why-question should always be answered):

*   "We have applied this design (pattern, technology, product) several times on successful projects that tackled similar requirements in a comparable context."
*   "We performed a Proof-of-Concept (PoC) or Proof-of-Technology (PoT) and the results were convincing; this approach will help us to achieve the required qualities."
*   "The skills to apply this technology successfully are available on the market, and using it increases the chances of being able to hire decent software engineers."

I have also seen variations of the following "arguments" that will be received less well:

*   "Everybody does it. I was told that this is a good choice."
*   "We have always done it like that." or "I do not know any alternative, and I do not have time to look for one."
*   "Experience with this pattern, technology, and/or product will look fantastic on my resume."

More examples and counter examples also can be found in [my SATURN 2012 presentation](https://resources.sei.cmu.edu/library/asset-view.cfm?assetID=31345).

_Exercise_: Look for some of the recent decisions made on your team. How to the decisions records do — do they answer why questions? Do the justifications convince you?

### Take Aways and Next Steps

You will enjoy AD making and capturing, and be a better architect if you keep in mind:

1.  Answers to "why?" questions are as important as anything else in your designs. There are no reasons not to document your key decisions and provide short but solid justifications for the options (patterns, technologies, and products) chosen.
2.  Avoid pseudo rationale and [killer phrases](https://wiki.c2.com/?KillerPhrases); refer to actual requirements and empirical evidence on your decision records. Do compare alternatives!
3.  Do not document everything; an AD log with more than 100 entries will probably put you readers (and you) to sleep, and be really hard to maintain. Focus on the [architecturally significant requirements](https://en.wikipedia.org/wiki/Architecturally_significant_requirements) and decisions — the ones that matter, the ones that are hard and costly to change. Some more concrete hints and definitions can be found [here](/practices/2020/09/24/ASRTestECSADecisions.html).
4.  Many templates exist, just pick one and stick to it. Or create one — and stick to it.
5.  Pick any "tool" that fits your culture and setup. In this context, a "tool" can be a plain text file, wiki, or application used for other purposes (for instance agile issue tracker or task board, UML modeling tool, version control system). Try Markdown![7](#fn:7)

**Where to go from here**

The Y-statement part of this post is also [available as a story on Medium](https://medium.com/@docsoc/y-statements-10eb07b5a177). This and other method elements are featured in the [Software/Service/API Design Practice Repository (DPR)](https://github.com/socadk/design-practice-repository) on GitHub.

Have a look at the [offerings](/services/) and [contact](/about/) page if you would like some of your ADs reviewed — in an empathic, constructive manner: I am not the guy who asked the nasty questions in the intro story! You can also contact me if some decisions still have to be made, of course :-)

Follow-up posts cover other items in my architect toolbox. For instance, I propose a [definition of done for ADs](/practices/2020/05/22/ADDefinitionOfDone.html) and an [architectural significance test](/practices/2020/09/24/ASRTestECSADecisions.html). There is a [Markdown ADR (MADR) template walkthrough](/practices/2022/11/22/MADRTemplatePrimer.html), and the new [Architectural Decision Guidance (ADG) tool](/practices/2025/07/17/ArchitectureDecisionGuidanceTool.html) is featured as well.

– Olaf (a.k.a. ZIO)

1.  stakeholder names removed and older technology included on purpose [↩](#fnref:1)
    
2.  Perry, Dewayne & Wolf, Alexander: [_Foundations for the Study of Software Architecture_](https://doi.org/10.1145/141874.141884). ACM SIGSOFT Software Engineering Notes, October 1992. The other two parts are "elements" (components, connectors) and "form" (properties and their qualities). [↩](#fnref:2)
    
3.  I will use this form and not architecture decisions simply because I am used to it from that time. The ARC-100 work product was part of the IBM Systems Integration/Global Services Method since 1998 (at least). [↩](#fnref:3)
    
4.  For more information on RUP, see [Wikipedia](https://en.wikipedia.org/wiki/Rational_Unified_Process) and [this overview article](https://www.researchgate.net/publication/220018149_The_Rational_Unified_Process--An_Introduction). [↩](#fnref:4)
    
5.  George Fairbanks provided inspiration with his [Architecture Haikus](https://www.georgefairbanks.com/blog/comparch-wicsa-2011-panel-discussion-and-haiku-tutorial/); decision outcome and its consequences are part of his one-napkin documentation approach.  [↩](#fnref:5)
    
6.  ok, the cards probably are ordered alphabetically [↩](#fnref:6)
    
7.  IMHO, usage of spreadsheets to capture structured text and design results is a warning sign, and smells like an instance of the [law of the instrument](https://en.wikipedia.org/wiki/Law_of_the_instrument) bias: if the hammer is your favorite (or only) instrument, all problems will appear as nails to you. [↩](#fnref:7)
