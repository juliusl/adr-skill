# 4. Author-adr skill recommends reviewer agent for consequence validation

Date: 2026-04-01

## Status

Accepted

## Context

The `author-adr` skill bundles a custom reviewer agent
(`assets/adr-reviewer.agent.md`) that performs structured quality checks on
ADRs — ecADR completeness, fallacy scans, anti-pattern detection, and a 7-point
review checklist. However, the current skill workflow does not actively recommend
using this agent after drafting an ADR. The review step is documented but
passive: a user must already know the agent exists and how to install it.

This creates two problems:

- **Discovery gap** — users who draft ADRs may never learn that a specialized
  reviewer agent is available, missing an opportunity to improve decision
  quality before the ADR is shared or implemented.
- **Consequence accuracy** — the Consequences section of an ADR (or the
  Pros/Cons in MADR format) is the most subjective part of the record. An
  AI-drafted ADR can assert consequences that sound plausible but are factually
  wrong or overly optimistic. The existing reviewer agent checks for structural
  anti-patterns (e.g., "Free Lunch Coupon" — ignoring negatives) but does not
  engage the user to validate whether stated consequences reflect reality.

The reviewer agent requires installation as a custom agent file
(`.agent.md`), which is a one-time setup step. Installing files into a user's
project without consent is an anti-pattern — the skill should ask before
modifying the project structure.

## Decision

We will update the `author-adr` skill to actively recommend the
`adr-reviewer.agent.md` after drafting an ADR, and extend the reviewer's
behavior to validate consequences interactively with the user.

### 1. Recommend review after drafting

After the skill creates or substantially edits an ADR (Step 5 in the current
"Creating an ADR" workflow), it will recommend reviewing the ADR with the
bundled reviewer agent. The recommendation will be a prompt, not an automatic
action:

> Would you like to review this ADR with the adr-reviewer agent? It will check
> for completeness, reasoning fallacies, and anti-patterns.

If the user agrees, the skill proceeds to check whether the agent is installed.

### 2. Ask before installing the custom agent

Before installing `adr-reviewer.agent.md`, the skill must ask the user for
consent:

> The adr-reviewer agent needs to be installed as a custom agent in your
> project. This will copy `adr-reviewer.agent.md` to your agents directory.
> Is that okay?

If the user consents, the skill installs the agent using the existing Makefile
target:

```bash
make -C <skill-root> install-agents
```

If the user declines, the skill falls back to the manual review process already
documented in the SKILL.md (the ecADR checklist, fallacy scan, and anti-pattern
check performed inline by the skill itself).

### 3. Reviewer validates consequences with the user

The `adr-reviewer.agent.md` will be extended so that when it reaches the
Consequences section (Nygard) or Pros/Cons sections (MADR), it does not simply
check for structural anti-patterns — it actively reviews each stated consequence
with the user to confirm accuracy:

For each consequence or pro/con assertion, the reviewer will:

1. **Present the assertion** — quote the specific consequence from the ADR.
2. **Assess plausibility** — flag assertions that appear speculative,
   unsubstantiated, or overly optimistic/pessimistic.
3. **Ask the user directly** — confirm whether the stated consequence matches
   their understanding of reality. For example:

   > The ADR states: "Reduces deployment time by 50%."
   > Is this based on measured data, an estimate, or an assumption? Should
   > we qualify this assertion?

4. **Suggest revisions** — if the user indicates a consequence is inaccurate or
   unverified, suggest revised wording that accurately reflects the level of
   certainty (e.g., "Expected to reduce deployment time" vs. "Reduces deployment
   time by 50%").

This ensures that the Consequences section — the part of the ADR most likely to
contain unvalidated assertions — is grounded in the user's actual knowledge
rather than plausible-sounding AI-generated claims.

### 4. Workflow integration

The updated workflow after creating an ADR:

```
ADR drafted (Step 5)
├─ Recommend review ───────► User declines → done
├─ User accepts
│   ├─ Agent installed? ───► Run adr-reviewer agent
│   ├─ Agent not installed?
│   │   ├─ Ask to install ─► User consents → install → run agent
│   │   └─ User declines ─► Manual review (inline checklist)
│   └─ Reviewer runs
│       ├─ Steps 1-3: ecADR, fallacy scan, anti-patterns (unchanged)
│       ├─ Step 4: Consequence validation (NEW — interactive)
│       ├─ Step 5: Review checklist
│       └─ Step 6: Verdict
└─ Done
```

## Consequences

**Positive:**

- Users are actively guided toward reviewing their ADRs, increasing the
  likelihood that quality checks happen before decisions are shared or
  implemented.
- The consent-based installation flow respects user autonomy and avoids
  unexpected project modifications.
- Interactive consequence validation catches the most common failure mode of
  AI-assisted ADR authoring: plausible but inaccurate assertions about outcomes.
- Fallback to manual review ensures the skill remains functional even if the
  user declines agent installation.

**Negative / Risks:**

- Adding an interactive review step after every ADR creation increases the
  time to complete the workflow. Users drafting multiple ADRs in quick
  succession may find the prompts disruptive. Mitigated by making the review
  a recommendation, not a requirement.
- The consequence validation step requires the reviewer agent to ask the user
  multiple questions, which may feel tedious for ADRs with many consequences.
  Mitigated by allowing the user to confirm all at once if they prefer.
- The agent installation prompt is a one-time friction point, but users who
  work across many repos will encounter it repeatedly. Mitigated by checking
  whether the agent is already installed before prompting.

**Neutral:**

- The existing manual review process documented in SKILL.md is unchanged and
  remains available as a fallback. This ADR adds an active recommendation layer
  on top of it.

---

## Comments

<!-- No review cycle on record. This ADR predates the formal review process. -->
