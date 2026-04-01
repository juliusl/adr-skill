---
source: https://jacquiread.com/posts/2024-09-11-decision-making-adrs-weightings-are-a-workaround/
fetched: 2026-04-01
title: "Decision Making: ADRs - Weightings Are a Workaround"
---

When defining criteria for your decision-making Architecture Decision Records (ADRs) you should avoid mixing levels of abstraction. Even when applying a modern and agile architecture technique such as ADRs, you will not get the desired effect if you do not take care in its execution.

Imagine a situation where 🧩 **compatibility** and 🛠️ **maintainability** are important to you. We can better understand our **maintainability** criteria by breaking it down. We want to know:

*   Whether the API is stable
*   How regularly major releases happen, and how long they are supported
*   How familiar your present and future developers are with the technology

These criteria have made it very clear what you are trying to achieve. But scoring these three **maintainability** criteria separately, and the scoring **compatibility** at a higher level can lead to biassed results.

There are three ways you could solve this:

*   **Use weightings on your criteria**
    
    These need to be reassessed when criteria are added and removed, and means you need to prepare and execute a mathematical equation on updates. Did somebody update the ratings but not the totals? That never happens 😉 - it's more likely to occur if the process is made more complex with weightings.
    
*   **Normalise your criteria UP to the same level of abstraction**
    
    You can attempt to keep your lower-level criteria in mind by listing them under the higher-level criterion, but it also makes them less accessible.
    
*   **Normalise your criteria DOWN to the same level of abstraction**
    
    This is a great way of improving your criteria as you will likely uncover important sub-criteria that you may not have thought about when assessing the higher-level criterion. You will have better data to make a decision as a result.
    

I prefer to apply normalising down to get the best of the benefits with the least down-sides. This is just like in good diagramming, good code, and good requirements, where we are looking to avoid mixing levels of abstraction.

But can you take this too far? Absolutely. It is usually adequate to split a high-level criterion into no more than 3-4 lower-level criteria, but as usual it depends on the scope and complexity of your ADR.

For more on ADRs check out Chapter 12 from the book [Communication Patterns](https://jacquiread.com/books/communication-patterns/) or to host your own private ADR training session for your business [get in touch](https://jacquiread.com/contact/).
