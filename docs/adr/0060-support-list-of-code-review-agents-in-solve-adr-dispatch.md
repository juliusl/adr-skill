# 60. Support list of code review agents in solve-adr dispatch

Date: 2026-04-07
Status: Accepted
Last Updated: 2026-04-07
Links: ADR-0059 (code review hook), ADR-0031 (dispatch hooks)

## Context

ADR-0059 added an optional code review hook (C-1) to solve-adr's conclusion sequence. The `[solve.dispatch].code_review` config accepts a single agent reference string. During dogfooding, experimentation with code reviewer personas (v1 through v4) revealed that specialized agents produce complementary findings:

- A **sweep agent** (mechanical, exhaustive) catches doc headers, spelling, identifier conflicts — fast, cheap, deterministic
- An **analytics agent** (judgment-based) catches security gaps, logic errors, consistency problems — slower, requires deeper reasoning

Running both in parallel on the same diff produces better total coverage than either alone. The current single-string config cannot express this.

**Decision drivers:**
- The config change must be backward-compatible — existing single-string values must continue to work
- Multiple reviewers should run in parallel to minimize wall-clock time
- Each reviewer produces independent findings; triage consolidates them
- The re-review step (C-1e) should re-dispatch all reviewers, not just one

## Options

### Option A: Change `code_review` to a TOML array

Replace the single string with a TOML array. A single string remains valid via TOML's scalar-to-array coercion or explicit handling.

```toml
[solve.dispatch]
code_review = ["juliusl-code-reviewer-sweep-v5", "juliusl-code-reviewer-analytics-v5"]
```

**Backward compatibility:** A bare string `code_review = "juliusl-code-reviewer-v1"` is handled by normalizing to a single-element list at parse time.

**C-1 changes:**
- C-1a: Entry condition checks if the list is empty (skip) or non-empty (proceed)
- C-1c: Dispatch all agents in parallel via separate `task` tool calls
- C-1d: Triage consolidates findings from all reviewers into a single list, deduplicating equivalent findings
- C-1e: Re-dispatch all agents in parallel
- C-1f: Gate checks all verdicts — if any reviewer says "Wait for Reviewer," the gate blocks

### Option B: Add a separate `code_review_additional` key

Keep `code_review` as a single string. Add `code_review_additional` as an array for extra reviewers.

```toml
[solve.dispatch]
code_review = "juliusl-code-reviewer-analytics-v5"
code_review_additional = ["juliusl-code-reviewer-sweep-v5"]
```

**Backward compatibility:** Fully backward-compatible — existing configs don't change.

**Downside:** Two config keys for the same concept. The "primary" vs "additional" distinction is artificial — all reviewers are peers.

### Option C: Use a TOML table of named reviewers

Replace the string with a table where each key is a named reviewer slot.

```toml
[solve.dispatch.code_review]
sweep = "juliusl-code-reviewer-sweep-v5"
analytics = "juliusl-code-reviewer-analytics-v5"
```

**Backward compatibility:** Breaking — existing `code_review = "string"` configs would fail to parse as a table.

**Upside:** Named slots let the dispatch template reference reviewers by role.

**Downside:** Breaking change. Over-engineered for the current use case.

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed — all options are straightforward and well-understood.

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps

**Validation needs:** None — the experimental evidence from the v1-v4 persona comparison already demonstrates the value of parallel reviewers.

## Decision

Option A — change `code_review` to a TOML array.

It is the simplest change that solves the problem. Backward compatibility is preserved by normalizing a bare string to a single-element list. The TOML spec allows arrays as first-class values, so no schema gymnastics are needed.

Option B was rejected because splitting one concept across two keys adds unnecessary complexity. Option C was rejected because the breaking change is not justified — named slots can be added later if needed.

## Consequences

**Positive:**
- Multiple reviewers can run in parallel, producing complementary findings
- Sweep (cheap model) and analytics (full model) can be paired for cost-effective coverage
- Backward-compatible — existing single-string configs continue to work

**Negative:**
- C-1d triage must handle deduplication across multiple reviewer outputs — findings may overlap
- C-1e re-review dispatches all reviewers again, increasing compute cost proportionally (practical use is 2-3 reviewers — sweep plus analytics — so cost scales modestly)

**Neutral:**
- The `[solve.dispatch]` config table gains no new keys — only the type of `code_review` changes

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [ ] Tooling
- [x] User documentation

### Additional Quality Concerns

The config parsing must handle three cases: absent (skip), string (normalize to single-element list), array (use directly). The implementation should fail gracefully if any array element is empty or whitespace-only — filter those out before dispatch.

SKILL.md Configuration section updates needed: the config table (`[solve.dispatch]` key descriptions) must document the new array type for `code_review`, the example TOML snippet must show the array form, and the key description must note the string-to-list normalization behavior.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** This is a narrow config schema change. The main implementation work is in SKILL.md C-1 substeps.

---

## Comments

### Draft Worksheet

**Framing:** The user wants to dispatch multiple code review agents in parallel. Driven by experimental evidence that sweep and analytics agents produce complementary findings. The direction is clear — change the config type from string to list.

**Tolerance:**
- Risk: Low — narrow config change, backward-compatible
- Change: Low — extends existing pattern
- Improvisation: Low — the solution space is small

**Uncertainty:** None — the v1-v4 experiments established the value proposition.

**Options:**
- Target count: 3
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- TOML array (simple, backward-compatible)
- Separate key (backward-compatible but splits one concept)
- TOML table (named slots but breaking change)
