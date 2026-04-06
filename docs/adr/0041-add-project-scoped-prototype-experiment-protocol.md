# 41. Add project-scoped prototype experiment protocol for persona testing

Date: 2026-04-06
Status: Accepted
Last Updated: 2026-04-06
Links:
- Extends [ADR-0020](0020-establish-adr-directory-as-project-scoped-convention.md) (`.adr/` directory for project-scoped configuration)
- Depends on [ADR-0042](0042-add-project-scoped-preferences-to-prototype-adr.md) (prerequisite — adds `.adr/preferences.toml` reading to prototype-adr)
- Related to [ADR-0031](0031-add-author-adr-dispatch-hooks-for-custom-agent-delegation.md) (editor persona validation — source of the file-access confound finding)

## Context

ADR-0031 prototyped editor personas to automate the review→revise workflow. During validation, the experiment discovered a confound: **agents with file access read the current ADR state (which includes already-applied fixes) and reject findings as "already documented."** This changes what's being tested — persona accuracy vs. persona + grounded knowledge.

**Evidence from this session (2026-04-06):**

The same v4 editor persona was tested two ways against the same ground truth:

| Test mode | ADR-0020 | ADR-0021 | ADR-0029 | Total |
|-----------|----------|----------|----------|-------|
| GPA (persona in prompt, no file access) | 10/10 (100%) | 5/6 (83%) | 5/5 (100%) | 20/21 (95%) |
| Native custom agent (file access) | 8/10 (80%) | 4/6 (67%) | 5/5 (100%) | 17/21 (81%) |

The native agent's two extra misses on ADR-0020 and ADR-0021 both stem from reading the already-revised ADR body: it saw fixes were already present and rejected findings as redundant or documented. The GPA run, with the persona embedded inline, triaged on judgment alone.

ADR-0031's prototype findings (§Additional observation) noted this explicitly: *"When custom agents have file access, they read the source ADR and make grounded decisions ('already covered'), which changes what's being tested. Tests should control for file access to isolate persona quality."*

**The problem:** The `prototype-adr` skill is general-purpose — it should not be specialized with persona-testing rules. But this project runs persona experiments repeatedly (v1 → v2 → v3 → v4), and the file-access confound produces inconsistent results unless the experiment protocol explicitly controls for it.

**What's needed:** A project-scoped configuration that tells `prototype-adr` how to run persona experiments in this repo — specifically, to embed source material in prompts rather than giving file paths when testing persona accuracy.

**Constraints:**
- `prototype-adr` must remain general-purpose — no persona-specific logic in the skill itself.
- The protocol must be stored in `.adr/preferences.toml` (project-scoped, per ADR-0020). Reading this file requires ADR-0042 as a prerequisite.
- The skill reads `.adr/preferences.toml` for project-scoped config (per ADR-0042) — this is the natural extension point.

### Decision Drivers

- **Experiment reproducibility** — the same experiment should produce comparable results across sessions.
- **Skill generality** — `prototype-adr` must not be specialized for persona testing.
- **Configurability** — the protocol should be adjustable per project without changing skill code.

## Options

### Option 1: Add `[prototype.persona]` table to `.adr/preferences.toml`

Define a project-scoped experiment protocol in `.adr/preferences.toml`:

```toml
[prototype.persona]
# When testing persona accuracy, embed source material in prompts.
# Do not give agents file paths to ADRs being evaluated.
embed_source = true

# Ground truth location pattern — Comments section of the ADR
ground_truth = "comments"
```

The `prototype-adr` skill reads `.adr/preferences.toml` for project-scoped config (once ADR-0042 is implemented). Adding a `[prototype.persona]` table follows the existing pattern. When `embed_source = true`, the experiment runner embeds ADR content (up to the Comments separator) in the prompt instead of providing a file path.

**Strengths:**
- Uses existing config infrastructure — no new files or directories.
- Skill stays general-purpose — it just reads a config key.
- Protocol is version-controlled with the project.

**Weaknesses:**
- Couples experiment behavior to a config key name that needs documentation.
- Only addresses persona experiments — other experiment types may need different controls.

### Option 2: Add a `.adr/experiment-profiles/` directory with profile files

Create experiment profile files (per prototype-adr's existing profile system) that encode the protocol:

```yaml
# .adr/experiment-profiles/persona-accuracy.yaml
name: persona-accuracy
isolation: worktree
controls:
  - embed_source: true
  - strip_comments: true
```

**Strengths:**
- Fits the prototype-adr profile system naturally.
- Can define multiple experiment types with different controls.

**Weaknesses:**
- Adds a new directory and file format to manage.
- Overloads the profile system — profiles configure experiment *environments* (isolation backend, teardown behavior), not experiment *protocols* (what to embed, where ground truth lives). Mixing environment setup with epistemological controls conflates two concerns. Preferences are the right layer for cross-cutting behavioral flags; profiles are the right layer for per-experiment environment config.

### Option 3: Document protocol in a markdown file only

Add `.adr/experiment-protocol.md` as documentation that agents read when running experiments. No config key — just instructions.

**Strengths:**
- Simple — just a markdown file.
- Human-readable documentation.

**Weaknesses:**
- No machine-readable enforcement — agents must interpret prose.
- Easy to forget or ignore.
- Doesn't integrate with the skill's config reading workflow.

## Evaluation Checkpoint (Optional)

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:** None — the confound is already validated by the v4 experiment results in this session.

## Decision

In the context of **needing reproducible persona experiments that control for the file-access confound**, facing **the need to keep `prototype-adr` general-purpose while encoding project-specific experiment protocol**, we chose **a `[prototype.persona]` table in `.adr/preferences.toml` (Option 1)** over **experiment profile files (Option 2, overloads profile system) and markdown-only documentation (Option 3, no enforcement)** to achieve **a machine-readable experiment protocol that integrates with the existing config infrastructure**, accepting **the need to document the config key and that it only addresses persona experiments**.

### Configuration

```toml
# .adr/preferences.toml

[prototype.persona]
embed_source = true
procedure = """
When testing persona accuracy against ground truth:
1. Read the ADR file content up to the --- separator
2. Embed that content in the experiment prompt as inline text
3. Do not provide the file path to the agent being tested
4. Ground truth answers are in the Comments section (after ---)
5. Strip the Comments section before embedding to prevent data leakage
"""
```

### How it works

The `procedure` key is a multi-line string containing project-specific instructions. The skill does not interpret or parse it — it reads the string and follows the instructions as written. Different repos define different procedures for their own experiment types.

## Consequences

**Positive:**
- Persona experiments produce consistent, comparable results by controlling for the file-access confound.
- `prototype-adr` stays general-purpose — it reads a config key, not persona-specific logic.
- Protocol is version-controlled in `.adr/preferences.toml` alongside other project config.

**Negative:**
- Only addresses persona experiments. Other experiment types with their own confounds would need separate controls.
- Agents running experiments must check the config key — if they skip it, the confound reappears silently.

**Neutral:**
- The `ground_truth = "comments"` key documents where correct answers live. This is specific to the Nygard Agent template's Comments section structure — other templates may store ground truth differently.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [x] Backwards Compatible
- [ ] Integration tests
- [x] Tooling
- [ ] User documentation

### Additional Quality Concerns

- **Config reading** — `prototype-adr` must handle missing `[prototype.persona]` table gracefully (default: no embedding).
- **Content stripping** — the `---` separator parsing must correctly split ADR body from Comments. Edge case: ADRs without a Comments section should embed the full content.

## Conclusion Checkpoint (Optional)

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:** Evidence for the file-access confound comes from this session's v4 persona experiment (95% GPA vs 81% native agent). ADR-0031 §Additional observation documented the confound before it was quantified.

---

## Comments
