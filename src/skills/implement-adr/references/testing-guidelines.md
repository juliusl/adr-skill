# Testing Guidelines

Every implementation plan task requires test and acceptance criteria. Select criteria based on code context and the source ADR's Quality Strategy.

## Quality Strategy Checklist

The nygard-agent ADR template includes a **Quality Strategy** section with checkboxes that signal which quality concerns apply to the decision. When generating task test criteria, read this section first:

- **Checked items (`[x]`)** — required. Include corresponding test criteria in relevant tasks.
- **Struck-through items (`~~`)** — not applicable. Omit these test types.
- **Unchecked items (`[ ]`)** — ambiguous; use code context analysis to determine applicability.

### Mapping Quality Strategy to Test Criteria

| Quality Strategy Item | What to Add |
|---|---|
| `[x] Fuzz testing` | Fuzz test criteria on tasks that handle user input or parsing |
| `[x] Unit testing` | Unit test criteria on tasks that expand public surface area |
| `[x] Load testing` | Load test criteria on tasks that introduce significant system load |
| `[x] Performance testing` | Benchmark criteria on tasks involving hot paths or resource-heavy processes |
| `[x] Backwards Compatible` | Compatibility verification criteria (API contracts, data migration) |
| `[x] Integration tests` | Integration test criteria for external dependencies (see below) |
| `[x] User documentation` | Documentation update criteria (README, CLI docs, code headers, usage guides) |
| `[x] Introduces major semantic changes` | Version bump criteria, downstream compatibility checks |
| `[x] Introduces minor semantic changes` | Regression test criteria verifying no unintended side effects |
| `[x] Tooling` | Update criteria for Makefiles, install targets, CI configs, and validation pipelines |

### Integration Tests

A checked `[x] Integration tests` item indicates an external dependency (databases, third-party APIs, message queues, file systems).

**When generating tasks:**

1. **If integration tests already exist** — add criteria to extend or modify them to cover the new behavior.

2. **If integration tests do not yet exist** — add a task to create them. If infrastructure or tooling decisions are unresolved (e.g., test container strategy, mock service setup), recommend an ADR for the gap instead of making ad-hoc tooling choices.

### Additional Quality Concerns

The ADR may also include free-text notes under `### Additional Quality Concerns`. Translate these into specific test criteria for the relevant tasks. They often capture domain-specific requirements not covered by the standard checklist.

## Testing by Code Context

### User Input Processing

Code that processes user-supplied data (form fields, CLI arguments, file uploads, API request bodies) is a common source of security vulnerabilities and crashes.

**Required testing:**
- **Fuzz testing** — Generate randomized, malformed, and boundary-case inputs to verify the code does not crash, hang, or produce incorrect results.
- **Input validation tests** — Verify that invalid inputs are rejected with clear error messages.
- **Encoding / injection tests** — Test for SQL injection, XSS, command injection, and path traversal as appropriate.

**Example task criteria:**
```markdown
### Test & Acceptance Criteria
- [ ] Fuzz test with at least 1000 randomized inputs; no panics or crashes
- [ ] Unit tests for all validation rules (valid, boundary, invalid cases)
- [ ] Injection test suite for SQL/XSS/command injection vectors
```

### Hot Path / Performance-Critical Code

Code on a hot path (request handlers, tight loops, serialization, core algorithms) must demonstrate acceptable performance.

**Required testing:**
- **Benchmarking** — Measure throughput and latency under expected load; establish a baseline and fail on regression beyond threshold.
- **Profiling** — Identify allocations, lock contention, or algorithmic inefficiency during review.
- **Load testing** — For network-facing hot paths, simulate concurrent requests.

**Example task criteria:**
```markdown
### Test & Acceptance Criteria
- [ ] Benchmark suite measuring p50/p99 latency and throughput
- [ ] Baseline established; CI fails on >10% regression
- [ ] Memory allocation profiled; no unnecessary heap allocations in hot loop
```

### Public APIs

Public-facing APIs (REST endpoints, library exports, SDK methods, CLI commands) define the contract with consumers and must be thoroughly tested.

**Required testing:**
- **Unit tests** — Cover happy path, edge cases, error cases, and boundary values for every public method or endpoint.
- **Contract tests** — Verify request/response schemas match documentation.
- **Error response tests** — Validate error codes, messages, and status codes.

**Example task criteria:**
```markdown
### Test & Acceptance Criteria
- [ ] Unit tests for all public methods (happy path + 3 edge cases minimum)
- [ ] Error response tests for all documented error codes
- [ ] Contract test validating response schema against OpenAPI spec
```

### Internal Modules

Internal modules (helpers, utilities, data access layers) need testing at key boundaries to prevent regressions.

**Required testing:**
- **Unit tests at boundaries** — Test the public interface of the module, not every private function.
- **Error handling tests** — Verify the module handles failures from its dependencies (e.g., DB down, file not found).

**Example task criteria:**
```markdown
### Test & Acceptance Criteria
- [ ] Unit tests for all exported functions
- [ ] Error handling tests for dependency failures (mock/stub dependencies)
```

### Integration Points

Code that integrates with external systems (databases, third-party APIs, message queues, file systems) needs tests that verify the integration works end-to-end.

**Required testing:**
- **Integration tests** — Test against real or containerized dependencies.
- **Contract tests** — Verify that the integration adheres to the expected external contract.
- **Resilience tests** — Verify behavior under failure conditions (timeout, connection refused, malformed response).

**Example task criteria:**
```markdown
### Test & Acceptance Criteria
- [ ] Integration test against containerized database
- [ ] Contract test validating query results match expected schema
- [ ] Resilience test: verify graceful handling of connection timeout
```

## Code Coverage Target

Maintain an overall code coverage bar of approximately **80%**.

This is a guideline, not a hard gate:
- **New code** should target ≥80% coverage.
- **Critical paths** (auth, payments, data integrity) should target higher.
- **Generated code or boilerplate** may be excluded from coverage metrics.
- Do not write low-value tests solely to hit a number — prioritize meaningful assertions.

## Writing Test Criteria in Tasks

When writing the "Test & Acceptance Criteria" section of a task:

1. **Be specific** — Name the type of test (unit, fuzz, benchmark, integration).
2. **Be measurable** — Include quantities where possible (e.g., "≥3 edge cases", "1000 fuzz inputs", "p99 < 50ms").
3. **Be contextual** — Match the test type to the code context using the table above.
4. **Include a "done" signal** — Define what passing looks like (e.g., "all tests green", "no regressions in benchmark", "coverage ≥ 80%").
