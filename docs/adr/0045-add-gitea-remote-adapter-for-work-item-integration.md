# 45. Add Gitea remote adapter for work item integration

Date: 2026-04-06
Status: Accepted
Last Updated: 2026-04-05
Links:
- [ADR-0034: Work-item-referenced naming](0034-adopt-work-item-referenced-naming-for-adr-files.md)
- [ADR-0035: Normalized work item data model](0035-define-normalized-work-item-data-model-for-vendor-agnostic-adr-tooling.md)
- [ADR-0040: Extract adr-db-lib crate](0040-extract-adr-db-lib-crate-for-shared-database-access.md)
- [ADR-0043: Replace vendor with remote](0043-replace-vendor-prefix-with-remote-in-wi-nygard-agent-naming.md)

## Context

The wi-nygard-agent format (ADRs 0034–0038, 0043) defines a `{remote}-{id}-{slug}.md` naming convention with adapters that normalize work item data into a common model (ADR-0035). The format supports `gh`, `ado`, and `local` remotes — but none have been implemented or tested against a real work item system.

A local Gitea instance provides the ideal first integration target:

**1. Self-contained testing.** A Gitea container runs locally via Colima/Docker with zero external dependencies. No GitHub tokens, no ADO organization — just `docker run` and go. This enables repeatable, offline development of the wi-nygard-agent format.

**2. Gitea's API is a subset of GitHub's.** Gitea implements a GitHub-compatible REST API (`/api/v1/repos/{owner}/{repo}/issues`). The issue model maps directly to ADR-0035's normalized schema: issues have IDs, titles, bodies, labels, states (open/closed), and timestamps. This makes Gitea the simplest adapter to write — and it validates the normalized model against a real system.

**3. adr-db-lib is the right integration point.** ADR-0040 extracted adr-db-lib as a shared library for database access. Adding the Gitea adapter here means `adr-db` gains work item functions directly (e.g., `adr-db fetch gitea 42`), and future consumers (format scripts, lifecycle commands) share the same adapter.

**Terminology note:** This ADR uses `remote` terminology per ADR-0043. If ADR-0043 is not accepted, the field maps to `vendor` per ADR-0035's current schema. Implementation should be sequenced after ADR-0043's status is resolved.

### Decision Drivers

- **Dogfooding priority** — the wi-nygard-agent format needs a real work item system to validate against
- **Local-first development** — the integration must work offline with a local container
- **Normalized model validation** — the adapter must produce data conforming to ADR-0035's schema
- **Library integration** — the adapter should live in adr-db-lib for reuse across consumers

## Options

### Option 1: Add `gitea` remote adapter in adr-db-lib with REST API client

Add a `gitea` module to `adr-db-lib/src/` that implements a Gitea REST API client. The adapter fetches issues from a Gitea instance, normalizes them to the ADR-0035 schema, and stores them via the existing Diesel models. The `adr-db` CLI gains a `fetch` subcommand: `adr-db fetch gitea --url http://localhost:3000 --owner user --repo project 42`.

Configuration for the Gitea endpoint lives in `.adr/preferences.toml`:
```toml
[remote.gitea]
url = "http://localhost:3000"
token = ""  # optional for public repos
```

**Strengths:**
- Direct integration in the shared library — all consumers get Gitea support
- `adr-db fetch` provides a CLI interface for scripting and testing
- Normalized output validates ADR-0035's model against a real system
- Configuration follows the existing `.adr/preferences.toml` pattern (ADR-0042)

**Weaknesses:**
- Adds an HTTP client dependency to adr-db-lib (e.g., `reqwest` or `ureq`)
- adr-db-lib was designed for database access — adding API clients expands its scope
- Token management in preferences.toml (even optional) needs security consideration

### Option 2: Add Gitea adapter as a standalone CLI tool

Create a separate `gitea-adapter` binary (or shell script) that fetches Gitea issues and outputs normalized JSONL to stdout. The existing `adr-db ingest` pipeline consumes the output. No changes to adr-db-lib.

```bash
gitea-adapter --url http://localhost:3000 --owner user --repo project 42 | adr-db ingest
```

**Strengths:**
- No scope expansion for adr-db-lib — stays focused on database access
- Unix philosophy — small tools composed via pipes
- Easy to test independently
- No new dependencies in the library crate

**Weaknesses:**
- Another binary to build, install, and maintain
- JSONL pipe interface is loosely typed — schema mismatches caught at ingest time, not compile time
- Format scripts and lifecycle commands would need to shell out to the adapter instead of calling a library function
- Harder to share configuration (each tool reads preferences separately)

### Option 3: Add Gitea adapter in adr-db-lib with separate API client crate

Split the concern: create `crates/adr-remote/` as a new library crate that handles all remote API interactions (Gitea now, GitHub/ADO later). `adr-db-lib` depends on `adr-remote` for work item fetching. `adr-db` gains the `fetch` subcommand via `adr-remote`.

**Strengths:**
- Clean separation: adr-db-lib handles database, adr-remote handles APIs
- Extensible — GitHub and ADO adapters go in the same crate later
- adr-db-lib stays focused on its original mission

**Weaknesses:**
- Three crates in the workspace before any format script exists — premature abstraction
- The remote adapters are simple (fetch JSON, map fields) — a separate crate adds complexity without proportional value at this scale
- Dependency graph gets deeper (adr-db → adr-db-lib → adr-remote)

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

In the context of **dogfooding the wi-nygard-agent format against a real work item system**, facing **the need for a Gitea adapter that normalizes work items and integrates with the existing Rust tooling**, we decided for **adding the `gitea` remote adapter directly in adr-db-lib with a REST API client** (Option 1), and neglected **a standalone CLI tool (Option 2, loose coupling but harder to share) and a separate API client crate (Option 3, premature at this scale)**, to achieve **direct library-level integration that all consumers share, with a CLI `fetch` subcommand for scripting**, accepting that **adr-db-lib's scope expands beyond pure database access to include remote API interaction, and an HTTP client dependency is added**.

### Remote Configuration

```toml
# .adr/preferences.toml
[remote.gitea]
url = "http://localhost:3000"
token = ""  # optional — empty for public repos, personal access token for private
owner = "user"
repo = "project"
```

**Token security:** `.adr/preferences.toml` is a project-scoped file that may be tracked by git. To avoid committing tokens, either add `.adr/preferences.toml` to `.gitignore`, or use the `ADR_GITEA_TOKEN` environment variable (the adapter checks the env var first, then falls back to the config file).

The `[remote.gitea]` section follows the pattern established by ADR-0042 (project-scoped preferences in `.adr/preferences.toml`).

### Adapter Interface

The Gitea adapter implements the normalized work item model from ADR-0035:

| Gitea Issue Field | Normalized Field | Mapping |
|-------------------|-----------------|---------|
| `number` | `id` | Direct (as string) |
| `title` | `title` | Direct |
| `body` | `description` | Truncated to 500 chars |
| `labels[].name` | `labels` | Array of label name strings |
| `state` | `state` | `open` → `open`, `closed` → `closed` |
| N/A | `type` | Default `issue`; promote to `bug` if `bug` label present |
| `html_url` | `url` | Direct |
| `created_at` | `created` | Direct (ISO 8601) |
| `updated_at` | `updated` | Direct (ISO 8601) |
| N/A | `remote` | `gitea` |

### CLI Subcommand

```bash
# Fetch a single issue and output normalized JSONL
adr-db fetch gitea 42

# Fetch and ingest into the database
adr-db fetch gitea 42 | adr-db ingest
```

The `fetch` subcommand reads `[remote.gitea]` from `.adr/preferences.toml` for the endpoint configuration.

### Naming Convention

With the `gitea` remote, ADR filenames follow the pattern:
```
gitea-42-use-postgresql.md
```

Cross-references: `ADR-gitea-42`. This extends the remote prefix table from ADR-0034/0043:

| Remote | ID Source | Example |
|--------|----------|---------|
| `gh` | GitHub Issue number | `gh-42-use-postgresql.md` |
| `ado` | ADO work item ID | `ado-1234-use-postgresql.md` |
| `gitea` | Gitea issue number | `gitea-42-use-postgresql.md` |
| `local` | Short hash | `local-a1b2c3d4-use-postgresql.md` |

### Library Structure

```
crates/adr-db-lib/src/
├── lib.rs          # pub mod declarations
├── db.rs           # connection + migrations (existing)
├── schema.rs       # Diesel schema (existing)
├── models.rs       # data models (existing)
└── remote/
    ├── mod.rs      # NormalizedWorkItem struct, RemoteAdapter trait
    └── gitea.rs    # GiteaAdapter: fetch issues, normalize to model
```

The `remote` module defines:
- `NormalizedWorkItem` — Rust struct matching ADR-0035's schema
- `RemoteAdapter` trait — `fn fetch_issue(&self, id: &str) -> Result<NormalizedWorkItem>`
- `GiteaAdapter` — implements `RemoteAdapter` using `ureq` (minimal HTTP client, no async runtime needed)

### HTTP Client Choice

Use `ureq` (not `reqwest`). `ureq` is a blocking HTTP client with no async runtime dependency. The adr-db tooling is synchronous — adding `tokio` or `async-std` for a single API call is unnecessary complexity.

## Consequences

**Positive:**
- The wi-nygard-agent format gets its first real integration test. A Gitea issue can be fetched, normalized, and used to create a work-item-referenced ADR — validating the full pipeline from ADR-0034 through ADR-0035.
- `adr-db fetch gitea 42` provides a simple CLI entry point for both scripting and interactive use. Developers can test the adapter without writing Rust code.
- The normalized work item model (ADR-0035) is validated against real Gitea data. Any gaps in the model surface during implementation.
- Configuration follows established patterns (`.adr/preferences.toml`), making Gitea setup consistent with other project-scoped config.

**Negative:**
- adr-db-lib gains an HTTP client dependency (`ureq`). The library's compile time and binary size increase.
- adr-db-lib's scope expands from "database access" to "database access + remote API interaction." This is a deliberate trade-off — the alternative (Option 3) was rejected as premature. If a second remote adapter (GitHub, ADO) is added, re-evaluating the crate split is warranted.
- Token storage in `.adr/preferences.toml` is plaintext. For local development this is acceptable; for production Gitea instances, users should use environment variables or a secrets manager. This ADR does not address secret management.

**Neutral:**
- The `gitea` remote prefix extends the table from ADR-0034/0043 but doesn't require changes to those ADRs — they define the convention, this ADR adds a row.
- The `RemoteAdapter` trait provides a contract for future adapters (GitHub, ADO) without mandating their implementation now.
- This ADR is scoped to Gitea only. GitHub and ADO adapters are future work.

## Quality Strategy

- [ ] Introduces major semantic changes
- [x] Introduces minor semantic changes
- ~~Fuzz testing~~
- [x] Unit testing
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- [x] Integration tests
- [x] Tooling
- [ ] User documentation

### Additional Quality Concerns

Integration tests require a running Gitea instance. Tests should use Docker (via Colima) to start a Gitea container, create a test issue, fetch it, and verify the normalized output. Unit tests should mock the HTTP response to test the normalization logic independently.

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

This is the first dogfooding ADR for the wi-nygard-agent format. The solve-adr skill orchestrated this ADR's creation (first dogfood of solve-adr too). Integration tests are checked because Gitea is the first external dependency in the Rust tooling.

---

## Comments

### Draft Worksheet
<!-- Captures original intent and workflow calibration. -->

**Framing:**
Add Gitea as the first concrete work item provider for the wi-nygard-agent format (ADRs 0034–0038, 0043). Use a local Gitea instance via Colima/Docker for dogfooding. Integrate Gitea support into `adr-db-lib` (ADR-0040) so `adr-db` can interface with Gitea directly — fetching issues, normalizing work item data per the ADR-0035 data model, and supporting the `gitea` remote in the `{remote}-{id}-{slug}.md` naming convention.

**Tolerance:**
- Risk: Medium — first real integration with an external work item system
- Change: Medium — adding a new remote and adapter, extending adr-db-lib
- Improvisation: Medium — open to different integration approaches (REST API, git forge API)

**Uncertainty:**
- Certain: Gitea has a REST API; the normalized work item model (ADR-0035) defines the target schema; adr-db-lib (ADR-0040) is the right place for shared database logic; Colima is running
- Uncertain: Gitea API authentication for local instances; how to map Gitea issue fields to the normalized model; whether to add a `gitea` remote prefix or reuse `local`

**Options:**
- Target count: 3
- [x] Explore additional options beyond candidates listed below

**Candidates:**
- Add `gitea` remote to adr-db-lib with REST API adapter
- Reuse the `local` remote with Gitea as a metadata source
- Add Gitea support as a standalone CLI tool outside adr-db

<!-- Generated by the revise task. Do not edit above the horizontal rule. -->

### Q: Is the ADR-0043 (vendor→remote rename) sequencing dependency documented?

**Addressed** — Added a terminology note in Context stating this ADR uses `remote` per ADR-0043, and if ADR-0043 is not accepted, the field maps to `vendor` per ADR-0035. Implementation should be sequenced after ADR-0043.

### Q: Does the Remote Configuration section address token security for version-controlled preferences files?

**Addressed** — Added token security guidance: either `.gitignore` the preferences file or use `ADR_GITEA_TOKEN` environment variable (adapter checks env var first, falls back to config).

### Q: Should error handling categories be defined at the trait level?

**Rejected** — Error category design is implementation detail. The ADR's scope is where the adapter lives and what it produces, not failure mode taxonomy. If multiple adapters need consistent error handling, that's a future ADR.

<!-- Review cycle 1 — 2026-04-06 — Verdict: Revise. 2 addressed, 1 rejected. -->
