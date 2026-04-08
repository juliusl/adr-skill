# 62. Accept git remote names and auto-detect adapter type

Date: 2026-04-08
Status: Accepted
Last Updated: 2026-04-08
Links:
- [ADR-0034: Work-item-referenced naming](0034-adopt-work-item-referenced-naming-for-adr-files.md)
- [ADR-0043: Replace vendor with remote](0043-replace-vendor-prefix-with-remote-in-wi-nygard-agent-naming.md)
- [ADR-0035: Normalized work item data model](0035-define-normalized-work-item-data-model-for-vendor-agnostic-adr-tooling.md)
- [ADR-0045: Gitea remote adapter](0045-add-gitea-remote-adapter-for-work-item-integration.md)

## Context

ADR-0034 established the `{remote}-{id}-{slug}` naming convention. ADR-0043 renamed the prefix concept from "vendor" to "remote" and defined the term as identifying a remote endpoint. Both ADRs list fixed adapter-type labels (`gh`, `ado`, `gitea`, `local`) as the remote value the user provides.

The original design intent was different: the user passes a **git remote name** (e.g., `origin`, `upstream`) and the system detects the adapter type by inspecting `git remote get-url <name>`. The adapter type is derived from the URL pattern, not supplied by the user. This intent was never captured in a decision record, and the implementation hardcoded the adapter-type labels as the only accepted values.

Two locations enforce the fixed-label assumption at the CLI boundary:

1. **`author.rs`** — `VALID_REMOTES = ["gh", "ado", "gitea", "local"]` with `validate_remote()` rejecting anything else. Called in all five subcommands that accept a remote argument: `new`, `rename`, `status`, `lifecycle`, and `export`.
2. **`wi-nygard-agent-format.sh`** — `validate_remote()` case statement accepting only `gh|ado|gitea|local`

A third location — **`work-item-adapters.sh`** — validates the `remote` field in the normalized data model (`check_enum("remote"; ["gh","ado","local","gitea"])`). This validates the *internal* adapter type after detection, not user input. It requires no change — the detection layer resolves the git remote name to an adapter type before the data model is populated.

**Terminology:** This ADR distinguishes between two meanings of "remote." A **git remote name** is what the user passes (e.g., `origin`, `upstream`) — a named endpoint configured via `git remote`. An **adapter type** is the internal label (`gh`, `ado`, `gitea`, `local`) stored in filenames and the normalized data model. ADR-0043 used "remote" to mean adapter type. This ADR shifts the CLI input to accept git remote names while preserving adapter types in storage. The operational distinction: git remote names are CLI input, adapter types are derived output.

This creates a usability gap: the user must know which adapter label maps to their git hosting provider. A user with `origin` pointing to `github.com` must type `gh`, not `origin`. The mapping is implicit knowledge that the tooling should derive automatically.

### Decision Drivers

- **Usability** — the user should pass what they already know (their git remote name), not an internal adapter label
- **Correctness** — the adapter type should be derived from the URL, which is the source of truth for where the remote points
- **Simplicity** — `local` remains a direct keyword since there is no git remote to inspect

## Options

### Option 1: Keep fixed adapter labels

Retain `gh`, `ado`, `gitea`, `local` as the only accepted remote values. Users must know which label maps to their hosting provider.

**Strengths:**
- No changes needed
- Labels are short and predictable

**Weaknesses:**
- Requires implicit knowledge — the user must map their hosting provider to an adapter label
- Diverges from the git conceptual model where "remote" means a named endpoint configured via `git remote`

### Option 2: Accept git remote names, detect adapter from URL

Change the remote argument to accept either a git remote name (e.g., `origin`) or the special keyword `local`. When a git remote name is given, run `git remote get-url <name>` and match the URL against known patterns to determine the adapter type. The detected adapter type — not the git remote name — is used in filenames and metadata for portability.

| URL pattern | Detected adapter |
|-------------|-----------------|
| `github.com` | `gh` |
| `dev.azure.com` or `visualstudio.com` | `ado` |
| `local` keyword (no URL lookup) | `local` |
| No pattern matches | Error — suggest checking remote configuration |

**Detection function signature:**

```rust
fn detect_adapter(remote: &str) -> Result<String, String>
```

If `remote == "local"`, return `"local"` immediately. Otherwise, run `git remote get-url <remote>`. If the command fails (remote doesn't exist), return an error. Match the URL against known host patterns to return the adapter type. If no pattern matches, return an error with a message like: "Could not detect adapter type for URL `<url>`. Supported hosts: github.com, dev.azure.com. For other forges, a future configuration mechanism will allow explicit adapter mapping."

**Filename behavior:** Filenames continue to use the adapter type (`gh-42-slug.toml`), not the git remote name. This keeps filenames portable across clones that may use different remote names for the same repository.

**Strengths:**
- Users pass what they know — `origin` instead of `gh`
- The adapter type is derived from the actual URL, which is the source of truth
- Filenames remain portable — `gh-42-slug.toml` works in any clone
- `local` is a clean special case — it means "no remote to inspect"

**Weaknesses:**
- Requires `git` to be available for non-local remotes
- URL pattern matching can be fragile for self-hosted instances — a Gitea instance at `code.example.com` or a GitLab instance has no distinguishing URL pattern. Unrecognized URLs produce an error rather than guessing wrong.
- Adds a subprocess call (`git remote get-url`) to every command that takes a remote argument

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

In the context of **how users specify the remote endpoint when creating and managing ADRs**, facing **the usability gap where users must supply internal adapter labels instead of git remote names**, we decided for **accepting git remote names and auto-detecting the adapter type from `git remote get-url` output** (Option 2), to achieve **a natural interface where the user passes what they already know (`origin`) and the system derives the adapter type from the URL**, accepting that **`git` must be available for non-local remotes and unrecognized URLs produce an error rather than guessing the adapter type**.

### Detection Logic

The `detect_adapter` function replaces `validate_remote` in the two CLI-boundary locations:

1. **`author.rs`** — replace `validate_remote()` with `detect_adapter()` that shells out to `git remote get-url` and pattern-matches the result. This affects all five subcommands that accept a remote argument: `new`, `rename`, `status`, `lifecycle`, and `export`. The detection logic is identical for all — resolve the git remote name to an adapter type, then proceed with the adapter type.
2. **`wi-nygard-agent-format.sh`** — replace `validate_remote()` with a `detect_adapter()` shell function using the same logic

**`work-item-adapters.sh` is unchanged.** Its `check_enum("remote"; ...)` validates the adapter type in the normalized data model, which is populated *after* detection. The detection layer sits above the data model.

### URL Pattern Matching

| Pattern | Adapter | Examples |
|---------|---------|----------|
| Host contains `github.com` | `gh` | `git@github.com:org/repo.git`, `https://github.com/org/repo` |
| Host contains `dev.azure.com` or `visualstudio.com` | `ado` | `https://dev.azure.com/org/project/_git/repo` |
| `local` keyword | `local` | No URL lookup — direct passthrough |
| No pattern matches | Error | Returns an error with the unrecognized URL and lists supported hosts |

Unrecognized URLs produce an error instead of guessing the adapter type. Self-hosted instances (Gitea, GitLab, Forgejo) have no distinguishing URL pattern — silently defaulting to any adapter would produce incorrect results for non-matching forges. A future ADR can introduce a configuration mechanism (e.g., a `[remotes]` table in `.adr/preferences.toml`) to map custom hosts to adapter types.

### Interface Change

All five subcommands that accept a `remote` argument change to accept git remote names:

Before:
```
adr-db author new gh 42 "Title" docs/adr
adr-db author status gh 42
adr-db author rename gh 42 "New Title"
adr-db author lifecycle gh 42
adr-db author export gh 42
```

After:
```
adr-db author new origin 42 "Title" docs/adr
adr-db author status origin 42
adr-db author rename origin 42 "New Title"
adr-db author lifecycle origin 42
adr-db author export origin 42
```

The filename output is the same in both cases: `gh-42-title.toml` (assuming `origin` points to `github.com`). The `list` and `init` subcommands are unaffected — they do not take a remote argument.

## Consequences

**Positive:**
- Users pass the git remote name they already know. The cognitive burden of mapping hosting providers to adapter labels is eliminated.
- `local` works identically — it remains a direct keyword that bypasses URL detection.

**Negative:**
- `git` must be installed and the command must be run inside a git repository for non-local remotes. Running `adr-db author` outside a git repo with a remote name will fail.
- If a git remote's URL changes (e.g., `origin` migrates from GitHub to Gitea), detection produces a different adapter type than what existing files use. Existing files named `gh-42-slug.toml` won't be found when detection returns `gitea`. The adapter type is resolved at creation time and baked into the filename — subsequent commands must use the same git remote that produced the original adapter type, or refer to existing files by their stored prefix.
- Unrecognized URLs produce an error. Self-hosted instances (Gitea, GitLab, Forgejo) require a future configuration mechanism to map custom hosts to adapter types.
- Every command invocation that takes a remote argument incurs a subprocess call to `git remote get-url`. This is a local operation with negligible latency but adds a hard dependency on the `git` binary.

**Neutral:**
- The normalized data model (ADR-0035) is unchanged — the `remote` field continues to hold adapter types (`gh`, `ado`, `gitea`, `local`). The detection layer sits above the data model.
- `work-item-adapters.sh` validation is unchanged — it validates adapter types in the data model, which are populated after detection.
- Existing TOML files with `gh-`, `ado-`, `gitea-` prefixes continue to work — `find_adr_file()` searches by the adapter-type prefix stored in the filename.

## Quality Strategy

- [x] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- ~~Backwards Compatible~~
- [x] Integration tests
- [x] Tooling
- [ ] User documentation

### Additional Quality Concerns

The URL pattern matching needs tests for SSH URLs (`git@github.com:...`), HTTPS URLs (`https://github.com/...`), and edge cases (ports, paths). Shell and Rust implementations must produce identical results for the same input. Unrecognized URLs must produce clear error messages. The `work-item-adapters.sh` validation tests remain unchanged — they validate the adapter type in the data model, not user input.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This ADR corrects a design drift where the implementation hardcoded adapter-type labels instead of accepting git remote names. The user clarified that backwards compatibility is not a concern — there are no existing users to break. The change touches two CLI-boundary files (Rust `author.rs`, shell `wi-nygard-agent-format.sh`) and adds a detection function to each. `work-item-adapters.sh` is unchanged — its validation operates on the internal data model.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
The `adr-db author` subcommand and shell scripts require users to pass fixed adapter labels (`gh`, `ado`, `gitea`, `local`) as the remote argument. The original design intent was that users pass a git remote name (e.g., `origin`) and the system auto-detects the adapter from `git remote get-url` output. This intent was never recorded, and the implementation drifted. The user has confirmed backwards compatibility is not a concern.

**Tolerance:**
- Risk: Low — the detection logic is straightforward URL pattern matching
- Change: Medium — three files need parallel changes (Rust + two shell scripts)
- Improvisation: Low — the design is clear from the original intent

**Uncertainty:**
- Certain: the adapter type should be derived from the URL, not user-supplied; `local` remains a keyword
- Uncertain: how to handle self-hosted instances with no distinguishing URL pattern

**Options:**
- Target count: 2
- [ ] Explore additional options beyond candidates listed below

**Candidates:**
- Accept git remote names, detect adapter from URL
- Keep fixed adapter labels

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Which subcommands change behavior — just `new` or all of them?

**Addressed** — Listed all five affected subcommands (`new`, `rename`, `status`, `lifecycle`, `export`) in both the Context and Decision sections. Added before/after examples for all five in the Interface Change section. Noted `list` and `init` are unaffected.

### Q: Does `work-item-adapters.sh` actually need to change?

**Addressed** — Reframed from "three locations enforcing fixed labels" to "two CLI-boundary locations" plus a third that validates the internal data model and requires no change. The distinction is now explicit in both Context and Decision.

### Q: Options→Decision drift on Gitea fallback behavior

**Addressed** — Reverted from "All other URLs → gitea" blanket fallback to the error-based approach described in the Options section. Unrecognized URLs now produce an error with a helpful message. Deferred configuration-based host mapping to a future ADR.

### Q: ADR-0043 predicted "remote" terminology confusion — does this ADR create it?

**Addressed** — Added a Terminology paragraph to Context establishing the operational distinction: "git remote name" = CLI input (e.g., `origin`), "adapter type" = derived output stored in filenames/data model (e.g., `gh`). Acknowledged that ADR-0043 predicted this confusion and this ADR resolves it through explicit terminology.

### Q: Is "remote changes → correct adapter automatically" really a positive consequence?

**Addressed** — Reclassified from positive to negative. If a git remote's URL changes, detection produces a different adapter type than existing files use, making them unfindable. Documented that the adapter type is resolved at creation time and baked into the filename.

### Q: Is the "< 10ms" subprocess cost substantiated?

**Addressed** — Replaced the unsubstantiated "< 10ms" claim with "local operation with negligible latency."

### Q: Is defaulting unknown URLs to `gitea` justified?

**Addressed** — Removed the Gitea fallback entirely in favor of returning an error for unrecognized URLs. GitLab and other forges have no distinguishing URL pattern, and silently defaulting would produce incorrect results. A future configuration mechanism is noted as the extension point.

<!-- Review cycle 1 — 2026-04-08 — Verdict: Revise. 7 addressed, 1 rejected (Finding 5 — SSH parsing already covered in Quality Strategy). -->
