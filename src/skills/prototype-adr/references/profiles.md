# Profile Format Reference

Prototyping profiles are declarative TOML files stored in `.adr/profiles/` (per ADR-0020's project-scoped convention). They define environment setup — not experiment logic.

## Schema

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `name` | string | yes | — | Profile identifier |
| `isolation` | string | yes | — | Backend: `worktree`, `container`, or `acp-sandbox` |
| `image` | string | container only | — | Container image (e.g., `postgres:16-alpine`) |
| `setup` | array of strings | no | `[]` | Shell commands to run during environment setup |
| `validate` | array of strings | no | `[]` | Commands to verify the environment is ready |
| `teardown` | string | no | `automatic` | Teardown behavior: `automatic` or `manual` |
| `requires` | string | no | — | Set to `user-intervention` for open-system scenarios |

### Observe Section

| Field | Type | Required | Default | Description |
|-------|------|----------|---------|-------------|
| `observe.format` | string | no | `jsonl` | Observation output format |
| `observe.output` | string | no | `stdout` | Where to write observations |

### Checkpoints (Open-System)

For profiles that require user intervention, define checkpoints as an array:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `checkpoints[].name` | string | yes | Checkpoint identifier |
| `checkpoints[].prompt` | string | yes | Message displayed to the user |

## Examples

### Minimal Profile

```toml
name = "default"
isolation = "worktree"
teardown = "automatic"
```

### Container Profile

```toml
name = "database-spike"
isolation = "container"
image = "postgres:16-alpine"
setup = ["createdb spike_db"]
teardown = "automatic"

[observe]
format = "jsonl"
output = "stdout"
```

### Open-System Profile

```toml
name = "auth-flow"
isolation = "worktree"
requires = "user-intervention"
teardown = "manual"

[[checkpoints]]
name = "configure-oauth"
prompt = "Set up OAuth credentials and press continue"

[[checkpoints]]
name = "verify-callback"
prompt = "Verify the callback URL works"

[observe]
format = "jsonl"
output = "stdout"
```

## Progressive Complexity

Start with the minimal profile and add fields only when needed. Only `name` and `isolation` are required.
