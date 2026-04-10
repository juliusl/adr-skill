# 68. Add post-solve retrospective and project-preference feedback loop to solve-adr

Date: 2026-04-10
Status: Planned
Last Updated: 2026-04-10
Links: ADR-0044, ADR-0048, ADR-0067

## Context

solve-adr completes its pipeline (C-1 QA triage, C-2 code review, C-3 report) without any structured reflection on the run itself. Teams accumulate knowledge about what works in their pipeline — which QA steps matter most, which reviewer agents add signal, how fast-path vs full-exploration should be calibrated — but this knowledge lives in people's heads.

`.adr/preferences.toml` already exists as a project-scoped preference file that solve-adr reads at startup (S-0). Project values override user-scoped values. This makes it the natural target for per-project pipeline refinements.

Without a feedback mechanism, teams must manually edit `.adr/preferences.toml` or base skill files to tune pipeline behavior. There is no prompt from the pipeline to capture lessons learned, and no structured format for doing so.

## Options

### Option A: C-4 retrospective step in solve-adr

After C-3 (Report), add a C-4 step that:
1. Asks structured retrospective questions about the completed solve run
2. Classifies each finding: pipeline preference (can be expressed as a `.adr/preferences.toml` key), or note-only
3. Proposes updates to `.adr/preferences.toml` for pipeline preference findings
4. In autonomous mode, applies updates and logs what changed; in guided mode, presents proposals for confirmation
5. Records the retrospective in `.adr/var/retro-<slug>.md` (gitignored transient data)

Project teams can then commit `.adr/preferences.toml` changes to version control to make them permanent.

### Option B: Standalone retro command

A separate invocation outside the solve pipeline. The user explicitly asks for a retrospective after a solve completes.

This is more flexible but requires the user to remember to run it. It also loses the pipeline context that makes the retrospective useful — the agent has access to the run artifacts (QA plans, ADRs, plan.md) during the solve.

### Option C: Free-form notes only

Save retrospective observations to a markdown file. No structured preference updates.

This captures notes but doesn't close the feedback loop — preferences still require manual editing.

## Evaluation Checkpoint (Optional)

**Assessment:** Skipped — the options are well-understood; Option A clearly addresses the problem without introducing external dependencies.

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None.

## Decision

Add C-4 retrospective step to solve-adr, immediately after C-3.

In the context of completed solve runs where pipeline lessons are fresh and all artifacts are available, facing the absence of any structured mechanism to capture and apply those lessons, we chose C-4 inline retrospective over a standalone command or notes-only approach to achieve a closed feedback loop where the pipeline itself prompts reflection and writes findings back to `.adr/preferences.toml`, accepting that C-4 adds a step to every solve completion and requires preference keys to be well-defined so findings can be expressed as config.

**C-4 procedure (in SKILL.md):**

1. **Prompt retrospective questions** — ask about the completed run:
   - What slowed things down?
   - What quality steps added the most value?
   - Were any steps unnecessary for this project's context?
   - What would you change for the next solve?
2. **Classify each finding:**
   - **Pipeline preference** — expressible as an existing key in the preference schema → propose a config update. If no matching key exists, the finding is note-only — the agent does not invent new keys; new keys require a schema change via a future ADR.
   - **Note-only** — useful context but not expressible as a current preference key → record in retro file
3. **Apply or propose updates:**
   - Autonomous mode: write proposed changes to `.adr/preferences.toml` and log what changed
   - Guided mode: present proposals, apply on confirmation
4. **Write retro record** — save findings to `.adr/var/retro-<slug>.md` (gitignored). Include: run summary, findings, preference changes applied.

**Preference keys the retrospective can write** (initial set):

```toml
[solve]
default_scenario = "problem"     # which scenario to default to
fast_path_sources = ["retro", "bug-bash", "amendment"]  # finding sources that trigger S-3 (requires ADR-0067)

[solve.retro]
enabled = true                   # whether C-4 runs at all
skip_when_no_findings = false    # skip C-4 if retrospective produces no actionable findings
```

**Note:** `fast_path_sources` depends on ADR-0067 (S-3 fast-path). If ADR-0067 is not accepted, this key should be removed from the initial set — it has no effect without S-3.

Teams can disable C-4 by setting `[solve.retro] enabled = false` in `.adr/preferences.toml`.

## Consequences

**Positive:**
- Teams can accumulate project-specific pipeline tuning through preference updates without editing base skill files.
- Retrospective is prompted at the natural moment: when the run is complete and artifacts are fresh.
- `.adr/preferences.toml` serves as the single project-scoped config, already read at S-0. No new file format is introduced.
- Retrospective records in `.adr/var/` provide a local log of pipeline evolution for the current working copy.

**Negative:**
- C-4 adds time to every solve completion. Teams that don't want it must opt out explicitly.
- The initial set of writable preference keys is small — not all pipeline tuning can be expressed as preferences without expanding the schema over time.

**Neutral:**
- C-4 runs after C-3. It does not block the report or affect ADR/plan artifacts.
- In autonomous mode, preference changes are written without confirmation. Teams should review `.adr/preferences.toml` changes before committing them to version control.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- [x] Tooling
- [x] User documentation

### Additional Quality Concerns

- SKILL.md for solve-adr must be updated: C-4 step added to the Conclusion sequence, preference keys documented in the Configuration section.
- `.adr/var/` must exist (gitignored). Bootstrap: `make init-data`. If `.adr/var/` is absent when C-4 runs, create it.
- The `[solve.retro]` preference block must be documented so teams know how to opt out.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** Option B was considered but rejected because the pipeline context is lost between sessions. The retro is most valuable immediately after C-3 while the agent still has access to the run's artifacts.

---

## Comments

### Draft Worksheet

**Framing:** Add a C-4 step to solve-adr that runs a structured retrospective after C-3, classifies findings as pipeline preferences or notes, writes preference updates back to `.adr/preferences.toml`, and records the retro in `.adr/var/`. Allows projects to self-refine the pipeline without editing base skill files.

**Tolerance:**
- Risk: Low — C-4 is additive; existing pipeline steps are unchanged; opt-out via preference
- Change: Low — new step after C-3, new preference keys, new retro file in `.adr/var/`
- Improvisation: Low — the mechanism is well-scoped

**Uncertainty:** The initial set of writable preference keys is small. Expansion will happen organically as teams identify what they want to tune — that's expected and acceptable.

**Options:**
- Target count: 2-3
- [ ] Explore additional options beyond candidates listed above

<!-- Review cycle 1 — 2026-04-10 — Verdict: Revise. 5 addressed (H: fast_path_sources dependency note; M: links added, self-refining reframed, audit trail reframed, classification rule added), 1 deferred (L: decision drivers list — omitted as implicit criteria are sufficient for this scope). -->

<!-- Review cycle 2 — 2026-04-10 — Verdict: Accept. All 5 cycle-1 findings verified resolved. No new issues introduced. Consequences proportionate, dependencies visible, classification rule clear. -->
