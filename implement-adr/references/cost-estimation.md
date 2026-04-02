# Cost Estimation

Guide for assigning t-shirt size cost estimates to implementation tasks.

## Size Definitions

| Size | Agent Turns | Typical Scope | Ambiguity Level |
|------|-------------|---------------|-----------------|
| **small** | ~1 turn | Single-file change, well-defined pattern | Low — clear what to do |
| **medium** | ~2–3 turns | Multi-file change, some design choices | Moderate — a few decisions to make |
| **heavy** | ~4+ turns | Cross-cutting change, may need research | High — significant judgment required |

## Calibration Examples

### Small Tasks

- Add a configuration constant or environment variable
- Create a data model / struct / class from a defined schema
- Write a unit test for an existing function
- Add a new CLI flag to an existing command
- Update documentation to reflect a change
- Add input validation for a single field

### Medium Tasks

- Implement a CRUD endpoint with validation and error handling
- Build a data access layer for a new entity
- Create a reusable utility module (e.g., retry logic, rate limiter)
- Write integration tests against a containerized dependency
- Implement a configuration system with multiple sources (env, file, defaults)
- Add authentication middleware to an existing API

### Heavy Tasks

- Design and implement a plugin / extension system
- Build a processing pipeline with multiple stages and error recovery
- Implement a caching layer with invalidation strategy
- Create a migration system (schema or data)
- Build an event-driven architecture component (pub/sub, CQRS)
- Implement complex business logic with many edge cases and rules

## Edge Cases

### Splitting Heavy Tasks

If a task is estimated as `[heavy]`, consider whether it can be decomposed
into smaller tasks:

- Can setup / scaffolding be separated? → Extract a `[small]` task
- Are there independent sub-components? → Split into multiple `[medium]` tasks
- Is research or prototyping needed first? → Add a `[small]` spike task

A `[heavy]` estimate is acceptable when the complexity is inherent and
indivisible, but it should be the exception rather than the norm.

### Uncertainty Premium

If a task's scope is unclear because of a decision gap (see Gap Detection),
add one size level:

- Would be `[small]` but has ambiguity → estimate as `[medium]`
- Would be `[medium]` but has ambiguity → estimate as `[heavy]`

Note the uncertainty in the task description so the user knows the estimate
may improve once the gap is resolved.

## Presenting Estimates

### Per-Task

Include the estimate in the task title:

```markdown
### Task 1.1: Set up project structure [small]
```

### Plan Summary

End the plan with an aggregate count:

```markdown
**Total estimated cost:** 3 small, 4 medium, 1 heavy
```

This gives the user a quick sense of overall effort without false precision.
