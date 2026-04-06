# Isolation Backends

The prototype-adr skill supports three isolation backends, selected via profile configuration or user preference. Each provides a different tradeoff between isolation strength and environmental requirements.

## Backend Comparison

| Backend | Isolation Level | Dependencies | Best For |
|---------|----------------|--------------|----------|
| `worktree` | Git-level (shared filesystem) | `git` | Quick spikes, skill changes, config experiments |
| `container` | OS-level (full isolation) | Container runtime | Database schemas, service interactions, reproducible benchmarks |
| `acp-sandbox` | Agent-level (sub-agent) | ACP-compatible runtime | Parallel experiments, structured observation, A/B comparisons |

## Worktree Backend

The default and lowest-dependency backend. Uses `git worktree` to create an isolated copy of the repository.

**Setup:**
```bash
git worktree add .prototype/<adr-number> HEAD
cd .prototype/<adr-number>
```

**Teardown:**
```bash
git worktree remove .prototype/<adr-number>
```

**Strengths:**
- No dependencies beyond `git`
- Fast setup (seconds)
- Full access to project toolchain

**Limitations:**
- Shared filesystem — changes to system-wide tools affect the prototype
- Shared git history — the worktree shares the same repository
- Not suitable for experiments requiring clean-room environments

## Container Backend

Uses a container runtime (Docker, Podman, etc.) for OS-level isolation.

**Setup:**
```bash
docker run -d --name prototype-<adr-number> \
  -v $(pwd):/workspace \
  <image> \
  tail -f /dev/null
```

**Teardown:**
```bash
docker rm -f prototype-<adr-number>
```

**Strengths:**
- Full OS-level isolation
- Reproducible environments via container images
- Ideal for database experiments, service interactions

**Limitations:**
- Requires a container runtime
- Slower setup (image pull + container start)
- Some environments restrict container usage

## ACP Sandbox Backend

Uses Agent Communication Protocol to spawn sub-agents in isolated sandboxes.
This backend is opt-in and requires `[prototype].runtime = "acp"` in `preferences.toml`.

**Strengths:**
- Agent-level isolation with structured communication
- Parallel experiment execution via sub-agents
- Structured observation via agent messages

**Limitations:**
- Requires ACP-compatible runtime (e.g., Copilot CLI)
- Platform-specific — diverges from agentskills.io's platform-agnostic stance
- Most complex to set up and debug

## Fallback Logic

When the preferred backend is unavailable:

1. Check if the selected backend's dependencies are met
2. If not, warn the user and suggest alternatives
3. Default fallback order: `container` → `worktree`
4. `acp-sandbox` never auto-falls-back — it requires explicit opt-in

## Configuration

Set the default isolation backend in `preferences.toml`:

```toml
[prototype]
isolation = "worktree"  # or "container" or "acp-sandbox"
runtime = ""            # set to "acp" for acp-sandbox support
```
