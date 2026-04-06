# Observation Format

Prototype results are captured as structured observations in JSONL format, consistent with ADR-0021's extraction pattern.

## JSONL Schema

Each observation is a single JSON line with these fields:

| Field | Type | Required | Description |
|-------|------|----------|-------------|
| `objective` | string | yes | The prototype objective being tested |
| `result` | string | yes | Outcome: `pass`, `fail`, or `data` |
| `notes` | string | no | Human-readable explanation |
| `value` | object | no | Structured measurement data (for `data` results) |
| `timestamp` | string | no | ISO 8601 timestamp |

## Examples

### Pass/Fail Result

```json
{"objective": "Validate awk parsing across plan files", "result": "pass", "notes": "Parsed 5/5 plan files correctly"}
```

### Data Result

```json
{"objective": "Measure plan file growth", "result": "data", "value": {"5_tasks": "2.1KB", "10_tasks": "4.3KB", "15_tasks": "6.8KB"}}
```

### Failure Result

```json
{"objective": "Confirm JSONL output parseable by jq", "result": "fail", "notes": "jq chokes on nested arrays in value field"}
```

## Validation

Observations must be valid JSONL — each line parseable by `jq`:

```bash
# Validate all observations
cat observations.jsonl | jq -e . > /dev/null

# Extract pass/fail summary
cat observations.jsonl | jq -r '[.objective, .result] | @tsv'
```

## Feedback Loop

After prototyping completes, observations feed back into the ADR lifecycle:

1. **Summarize findings** — aggregate pass/fail/data results per objective
2. **Update the ADR** — append findings to the Context section as evidence, or update option Strengths/Weaknesses with empirical data
3. **Update checkpoint state** — if all objectives pass, set the Evaluation Checkpoint Assessment to `Proceed`
4. **Status transition** — validated decisions can progress from `Prototype` → `Proposed`

## Integration with implement-adr

The JSONL observation format matches the implementation summary format (ADR-0021), so the same awk/jq tooling works for both.
