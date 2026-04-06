#!/bin/bash
# Test: Local adapter produces normalized work item JSON
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
source "$SCRIPT_DIR/work-item-adapters.sh"

echo "=== local work item ==="
ADR_LOCAL_ID=a1b2c3d4 ADR_DATE=2026-04-06T00:00:00Z local_adapter "My local work item" | jq -S .

echo "=== validation ==="
ADR_LOCAL_ID=deadbeef ADR_DATE=2026-04-06T00:00:00Z local_adapter "Validate me" | validate_work_item && echo "VALID"

echo "=== error on empty title ==="
local_adapter "" 2>&1 || true
