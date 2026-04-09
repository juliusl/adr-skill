# 63. Add tech-writer dispatch hook to author-adr

Date: 2026-04-08
Status: Accepted
Last Updated: 2026-04-08
Links:
- Extends [ADR-0031](0031-add-author-adr-dispatch-hooks-for-custom-agent-delegation.md) (defines dispatch hooks for review/editor agents)
- Related to [ADR-0014](0014-define-author-and-implement-tables-in-toml-configuration.md) (flat-table constraint referenced in Consequences)

## Context

The author-adr skill's `[author.dispatch]` table (ADR-0031) currently supports two
hook points: `review` (A-3) and `editor` (A-4). These hooks let users swap in
custom agents for the review→revise cycle while the inline agent handles all
other work — including the actual ADR content writing in A-2.

The content writing step (A-2, Step 3: "Draft the ADR") is the most
labor-intensive part of the procedure. The inline agent drafts Context, Options,
Decision, Consequences, and Quality Strategy sections. Writing quality depends
on the agent's general-purpose capabilities and whatever style instructions
exist in the skill context.

Users who care about writing quality want to delegate this work to a dedicated
technical writer agent — an agent with a persona tuned for technical
documentation (e.g., `juliusl-tech-writer-v1`). The current dispatch
infrastructure has no hook point for A-2 writing, so users cannot swap in a
custom writer without modifying the skill instructions.

### Decision Drivers

- **Consistency with ADR-0031** — the dispatch pattern already exists for
  `review` and `editor`. Adding `tech_writer` follows the same pattern:
  same instructions, configurable executor.
- **Separation of concerns** — analytical work (ASR test, readiness check,
  checkpoint evaluation) stays with the inline agent. Writing work goes to the
  tech-writer. The inline agent retains responsibility for validation.
- **Backward compatibility** — when no `tech_writer` is configured, the inline
  agent writes the content exactly as it does today. No change in behavior.
- **Progressive adoption** — users can adopt the tech-writer hook independently
  of the review and editor hooks.

## Options

### Option 1: Add `tech_writer` key to `[author.dispatch]`

Add a third dispatch key to the existing `[author.dispatch]` table. The
tech-writer agent is dispatched during A-2 for ADR body content writing.

```toml
[author.dispatch]
review = "general-purpose"        # A-3: reviewer agent
editor = "juliusl-editor-v4"      # A-4: editor agent
tech_writer = "juliusl-tech-writer-v1"  # A-2: writer agent
```

**Dispatch contract:**
- The inline agent completes A-1 (worksheet) and A-2 Steps 1–2 (ASR test,
  readiness check)
- At A-2 step 4, the inline agent dispatches the tech-writer with:
  - The ADR file (with draft worksheet already populated)
  - The problem statement and constraints
  - The template structure
  - Instructions to write Context, Options, Decision, Consequences, and
    Quality Strategy sections (Quality Strategy is included as a documentation
    task — the inline agent validates the selections during checkpoint review)
- The tech-writer writes the content and returns control
- The inline agent continues with A-2 Steps 4–6 (checkpoints, rename)

**Fallback:** When `tech_writer` is absent or empty, the inline agent writes
the content itself (current behavior). When the configured agent cannot be
resolved at runtime, fall back to inline writing and warn the user.

### Option 2: Generalized writer pipeline with pre/post hooks

Instead of a single `tech_writer` key, add a pipeline with `pre_writer` and
`post_writer` hooks. The pre-writer does research and outlining; the post-writer
does prose polishing.

```toml
[author.dispatch]
pre_writer = "research-agent"
post_writer = "prose-polisher"
```

**Rejected:** A pipeline could outperform single-pass when the research phase
needs a fundamentally different model or tool access than the writing phase —
e.g., a research agent that queries external APIs for competitor analysis before
handing structured notes to a prose agent. That scenario doesn't apply here: the
tech-writer operates on the draft worksheet and problem context already assembled
by the inline agent. A single tech-writer agent is expected to handle both
research and writing in one pass; if this proves insufficient, a pipeline can be
added as a refinement to the single `tech_writer` hook without breaking the
dispatch contract.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — Option 1 follows the established dispatch pattern
from ADR-0031. The dispatch mechanism is proven; applying it to content
generation (vs. evaluation) is low-risk but untested.

## Decision

In the context of **author-adr content quality**, facing **no dispatch hook for
A-2 writing work**, we chose **Option 1 (add `tech_writer` key to
`[author.dispatch]`)** over **Option 2 (generalized writer pipeline)** to
achieve **configurable writing delegation with minimal complexity**, accepting
**one additional dispatch key to configure and one more agent invocation per ADR
creation**.

The `tech_writer` key follows the same contract as `review` and `editor`: same
instructions, configurable executor. The tech-writer receives the ADR file with
the draft worksheet and all analytical context, writes the body content, and
returns control to the inline agent for validation.

**Integration point:** A-2 step 4 (Draft the ADR). The inline agent handles
all steps before and after.

**Dispatch mechanics:**
1. If `tech_writer` is configured: dispatch via `task` tool with the ADR file,
   problem context, and writing instructions
2. If `tech_writer` is absent or empty: inline agent writes content (no behavior
   change)
3. If configured agent cannot be resolved: fall back to inline, warn user

**Default value:** `""` (empty string) — inline agent writes content. This
preserves backward compatibility.

## Consequences

**Positive:**
- Users can customize ADR writing quality by configuring a dedicated tech-writer
  agent with a specialized writing persona
- The dispatch pattern is consistent across all three author-adr phases: writing
  (A-2), review (A-3), revision (A-4)
- Backward compatible — no configuration change required for existing users

**Negative:**
- One additional agent dispatch per ADR creation when configured — adds latency
  to the writing step
- The tech-writer must understand the nygard-agent template structure to populate
  sections correctly — the dispatch prompt must be clear about section
  expectations

**Neutral:**
- The `[author.dispatch]` table grows from 2 keys to 3 — still under the
  ADR-0014 flat-table constraint

## Quality Strategy

- [x] Introduces minor semantic changes
- ~~Introduces major semantic changes~~
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

- The dispatch prompt to the tech-writer must include the full template structure
  and section expectations — incomplete prompts produce incomplete ADRs
- P-1 (Mandatory Dispatch Compliance) already covers the tech-writer: when
  configured, it must be used

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** None.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
The user wants to delegate ADR content writing from the inline agent to a
configurable tech-writer agent. This extends the existing dispatch pattern
(ADR-0031) with a third hook point at A-2 step 4. The user has
`juliusl-tech-writer-v1` installed for prototyping.

**Tolerance:**
- Risk: Low — extending a proven dispatch pattern
- Change: Low — additive feature, no existing behavior changes
- Improvisation: Low — follow the ADR-0031 precedent

**Uncertainty:**
- Certain: The dispatch pattern works (proven by review/editor hooks)
- Certain: The integration point is A-2 step 4
- Uncertain: Exact dispatch prompt format for the tech-writer

**Options:**
- Target count: 2
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Add `tech_writer` key to `[author.dispatch]` — follows ADR-0031 pattern
- Generalized writer pipeline — pre/post hooks (likely over-engineered)

<!-- Review cycle 1 — 2026-04-08 — Verdict: Accept. 5 suggestions (1M, 4L). Polish pass applied. -->
