#!/bin/bash
# Test: lifecycle subcommand — state mapping, recommendation, auto-execute
export ADR_TEST_ADAPTER=gh
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"

# Set up ADR + cached work item
nygard-agent-format.sh init
new.sh wi-nygard-agent gh 42 "Use PostgreSQL"

# Cache a work item in "resolved" state (should map to Accepted)
source "$SCRIPT_DIR/work-item-adapters.sh"
echo '{"remote":"gh","id":"42","title":"Use PostgreSQL","type":"issue","state":"resolved","url":"","description":"","labels":[],"created":"2026-04-01","updated":"2026-04-05"}' | ADR_CACHE_TS=2026-04-06T07:00:00Z cache_work_item

echo "=== interactive (no --auto) ==="
wi-nygard-agent-format.sh lifecycle gh 42

echo "=== auto execute ==="
ADR_CACHE_TS=2026-04-06T08:00:00Z wi-nygard-agent-format.sh lifecycle gh 42 --auto

echo "=== after transition ==="
wi-nygard-agent-format.sh status gh 42

echo "=== in sync ==="
wi-nygard-agent-format.sh lifecycle gh 42

echo "=== lifecycle log ==="
cat .adr/var/lifecycle.jsonl | jq -r '.from_status + " → " + .to_status'
