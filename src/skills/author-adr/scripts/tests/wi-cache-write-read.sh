#!/bin/bash
# Test: work item cache write, read, latest-wins, not-found, auto-create
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
source "$SCRIPT_DIR/work-item-adapters.sh"

echo "=== write and read ==="
echo '{"remote":"gh","id":"42","title":"First snapshot","type":"issue","state":"open","url":"","description":"","labels":[],"created":"2026-04-01","updated":"2026-04-01"}' | ADR_CACHE_TS=2026-04-06T07:00:00Z cache_work_item
lookup_work_item gh 42 | jq -r '.title, .cached_at'

echo "=== latest wins ==="
echo '{"remote":"gh","id":"42","title":"Updated snapshot","type":"issue","state":"closed","url":"","description":"","labels":[],"created":"2026-04-01","updated":"2026-04-06"}' | ADR_CACHE_TS=2026-04-06T08:00:00Z cache_work_item
lookup_work_item gh 42 | jq -r '.title, .state, .cached_at'

echo "=== not found ==="
lookup_work_item gh 99 && echo "FOUND" || echo "NOT_FOUND"

echo "=== dir auto-created ==="
test -d .adr/var && echo "DIR_EXISTS"

echo "=== file line count ==="
wc -l < .adr/var/work-items.jsonl | tr -d ' '
