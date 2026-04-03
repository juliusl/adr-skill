# [ADR Number]: [Short Title]

Date: [Should be the date this was first created]
Status: [Prototype | Proposed | Accepted | Deprecated | Superseded by ADR-XXXX]
Last Updated: [Should be the date it was last changed]
Links: [This should be a list of relevant links to this ADR. It can be other ADR's or supporting documents]
- [text](url)

## Context

[Describe the forces at play, including technological, political, social, and
project local. These forces are likely in tension and should be called out as
such. The language in this section is value-neutral — it is simply describing
facts.]

## Options

[List options with strengths and weaknesses. Include how they address the issue.]

## Decision

[State the decision that was made. Use full sentences, with active voice.
"We will ..."]

## Consequences

[Describe the resulting context after applying the decision. All consequences
should be listed here, not just the positive ones. A particular decision may
have positive, negative, and neutral consequences.]

## Quality Strategy

[If the ADR is describing a change to an existing system,
then a strategy should be planned to maintain system quality and avoid degradation]

- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing [If the decision introduces parsing or user input, or anything that modifies existing parsing or user input, fuzz testing should be used to find crashes]
- [ ] Unit testing [If the decision expands the public surface area of the code base, or is addressing a new class of problems, unit tests should be added to validate the change]
- [ ] Load testing [If the decision introduces a significant amount of load on the system, load tests should be done to understand stress points]
- [ ] Performance testing [If the decision involves any type of hot path or resource heavy process, benchmarks should be written or updated to catch degradation]
- [ ] Backwards Compatible [If the decision involves breaking compatibility, backwards compatibility should be considered as a quality preservation stratagem]

[If any of the above cannot apply, use markdown's strikethrough notation to cross it off from the list]

### Additional Quality Concerns

[Any additional quality concerns should be documented here so that planning and implementation can account for them]

---

## Comments

[This section is to capture additional dialogue regarding the ADR. It should be formatted as a list of question/answers]
