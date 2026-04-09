# Follow-up Items — ADR-0063: Tech-Writer Dispatch Hook

Items deferred from the `solve/custom-tech-writer-dispatch` branch that are not blocking merge but should be tracked.

## Follow-ups

| # | Source | Item | Priority | Notes |
|---|--------|------|----------|-------|
| 1 | QA Rec #3 (Won't Fix) | Add dispatch observability marker to ADR Comments section (e.g., `<!-- content-author: agent-name -->`) so downstream agents can detect which agent wrote the content | Low | Warning mechanism exists for runtime; this adds machine-readable provenance |
| 2 | Untracked file | `src/agents/juliusl-tech-writer-v1.agent.md` is untracked — decide whether to commit it with this branch or separately | User decision | The agent file exists but was not part of the ADR-0063 implementation scope |
