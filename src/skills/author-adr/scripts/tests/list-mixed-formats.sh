#!/bin/bash
# Test: list ADRs with mixed status formats (inline + heading)
mkdir -p docs/adr

# Legacy ADR with ## Status heading
cat > docs/adr/0001-legacy-format.md <<'EOF'
# 1. Legacy format

Date: 2026-01-15

## Status

Accepted

## Context

Legacy context.

## Decision

Legacy decision.

## Consequences

Legacy consequences.
EOF

# Modern ADR via script
new.sh nygard-agent "Modern format"
echo "---"
nygard-agent-format.sh list
