# 55. Add Ready status to ADR lifecycle

Date: 2026-04-07
Status: Ready
Last Updated: 2026-04-07
Links: ADR-0054 (policy organization — previous workflow set Accepted prematurely)

## Context

During the ADR-0054 solve workflow, the author-adr skill transitioned ADR-0054 to `Accepted` status during the review cycle — before implementation. This contradicts the documented intent:

- `manage.md` line 68: "The `Accepted` status is set by the `implement-adr` skill after successful plan execution — `author-adr` does not transition ADRs to `Accepted`."
- `review.md` line 183: "The 'Accept' verdict does NOT trigger a status transition to `Accepted`."

Both guardrails existed but weren't enforced. The root cause is a semantic gap in the status lifecycle:

```
Prototype → Proposed → ??? → Planned → Accepted
```

After review passes with an "Accept" verdict, the ADR is in a liminal state: the decision has been reviewed and approved, but there's no status that reflects this. The agent defaults to `Accepted` because it's the closest available status — but `Accepted` means "fully implemented," which is wrong.

The user's framing: "An ADR cannot be accepted before it is implemented because before implementation they are decisions without action."

**Current lifecycle:**
```
Prototype ──► Proposed ──► Accepted ──► Deprecated
                               └──► Superseded
```

**Decision drivers:**
- `Accepted` is ambiguous — `manage.md` defines it as "agreed and ready for implementation" but `implement-adr` uses it to mean "fully implemented"
- The review "Accept" verdict has no corresponding status
- Agents fill semantic gaps with the closest available value
- `Planned` (set by implement-adr I-5) captures "decomposed into a plan" but not "reviewed and approved"

## Options

### Option A: Add a "Ready" status between Proposed and Accepted

Insert a `Ready` status that means "reviewed, approved, ready for implementation." This creates an explicit landing state for the review "Accept" verdict.

**New lifecycle:**
```
Prototype ──► Proposed ──► Ready ──► Planned ──► Accepted ──► Deprecated
                                                      └──► Superseded
```

| Status | Meaning | Set by |
|--------|---------|--------|
| Prototype | Drafted for prototyping | author-adr (create) |
| Proposed | Ready for review | author-adr (manual) |
| **Ready** | **Reviewed and approved — ready for implementation** | **author-adr (after Accept verdict)** |
| Planned | Decomposed into implementation plan | implement-adr (I-5) |
| Accepted | Decision fully implemented | implement-adr (I-8) |

**Status transition rules:**
- `author-adr` caps at `Ready` — it never sets `Planned` or `Accepted`
- `implement-adr` transitions `Ready` (or `Proposed`) → `Planned` at I-5
- `implement-adr` transitions `Planned` → `Accepted` at I-8
- The review "Accept" verdict triggers `Proposed` → `Ready`

**Strengths:**
- Eliminates the semantic gap — every lifecycle state has a name
- Makes the review→implementation boundary explicit
- Prevents agents from prematurely setting Accepted
- Compatible with existing `Planned` and `Accepted` semantics

**Weaknesses:**
- Adds a status to learn and manage
- Existing ADRs in `Accepted` status won't be retroactively updated (fine — they're already implemented)
- Scripts need to handle the new status (trivial — they're format-agnostic)

### Option B: Keep current lifecycle — enforce via textual guardrails

Strengthen the existing guardrails in manage.md and review.md. Add a status validation check to author-adr that prevents setting Accepted directly.

**Strengths:**
- No new status to learn
- Simpler lifecycle

**Weaknesses:**
- The previous workflow already had textual guardrails and they didn't work
- Agents need a status to set after review passes — without Ready, they'll continue defaulting to Accepted
- Treating the symptom (enforcement) rather than the cause (missing status)

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — the evidence is from direct observation of the ADR-0054 workflow.

## Decision

**Option A: Add a "Ready" status between Proposed and Accepted.**

In the context of agents prematurely setting Accepted status after review, facing evidence that textual guardrails alone don't prevent the behavior, we decided to add a Ready status that means "reviewed and approved, ready for implementation," to achieve an explicit boundary between author-adr's responsibility (cap at Ready) and implement-adr's responsibility (Planned → Accepted), accepting one additional status value in the lifecycle.

### Implementation Requirements

1. **Update manage.md** — add Ready to the lifecycle diagram, status table, and transition rules. Update the Accepted status meaning from "agreed and ready for implementation" to "decision fully implemented" to resolve the existing ambiguity.
2. **Update review.md** — the Accept verdict triggers `Proposed` → `Ready` transition. Update the existing guardrail (line 183, "caps at Proposed") to say "caps at Ready."
3. **Update implement-adr SKILL.md** — I-5 accepts `Ready` as a valid input status (alongside Proposed and Prototype)
4. **Update author-adr SKILL.md** — add a policy: "author-adr caps at Ready — never set Planned or Accepted"
5. **Update solve-adr problem.md** — triage step references "Proposed (reviewed, accepted)" which should say "Ready"

## Consequences

**Positive:**
- Every lifecycle state has a name — eliminates the semantic gap that caused the ADR-0054 incident
- Clear ownership boundary: author-adr ≤ Ready, implement-adr ≥ Planned
- The review "Accept" verdict maps directly to a status transition

**Negative:**
- One more status to learn (mitigated by the lifecycle diagram being self-documenting)
- Existing ADRs already in Accepted won't be retroactively relabeled

**Neutral:**
- Scripts are format-agnostic — they accept any status string, so no script changes needed
- `Planned` and `Accepted` semantics are unchanged
- ADRs that skip review (Prototype → Planned) are unaffected — implement-adr handles that path directly

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- [x] User documentation

### Additional Quality Concerns

Verify that implement-adr I-5 handles `Ready` → `Planned` transitions. The current guard (line 227) only lists `Prototype` and `Proposed` — `Ready` must be added.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** The semantic gap was directly observed in the ADR-0054 workflow. The fix is additive — one new status value — and doesn't change existing semantics.

---

## Comments

### Draft Worksheet

**Framing:**
Author-adr set Accepted on ADR-0054 before implementation. Textual guardrails existed but didn't prevent it. The user proposes adding a "Ready" status to fill the semantic gap between Proposed and Accepted.

**Tolerance:**
- Risk: Low — additive change, no existing semantics altered
- Change: Low — one new status value
- Improvisation: Low — user has a clear direction

**Uncertainty:**
- Known: the lifecycle gap (no status for "reviewed and approved")
- Known: the user's preferred solution (Ready status)
- Known: scripts are format-agnostic (accept any string)

**Options:**
- Target count: 2
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Option A: Add Ready status
- Option B: Enforce via textual guardrails
