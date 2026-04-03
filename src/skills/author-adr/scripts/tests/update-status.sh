#!/bin/bash
# Test: update status of ADRs (both inline and heading formats)
mkdir -p docs/adr

# Legacy ADR with ## Status heading
cat > docs/adr/0001-legacy-format.md <<'EOF'
# 1. Legacy format

Date: 2026-01-15

## Status

Proposed

## Context

Legacy context.

## Decision

Legacy decision.

## Consequences

Legacy consequences.
EOF

# Modern ADR
new.sh nygard-agent "Modern format"

echo "=== before ==="
nygard-agent-format.sh status 1
nygard-agent-format.sh status 2
echo "=== update ==="
nygard-agent-format.sh status 1 Accepted
nygard-agent-format.sh status 2 Proposed
echo "=== after ==="
nygard-agent-format.sh status 1
nygard-agent-format.sh status 2
