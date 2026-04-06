#!/bin/bash
# Test: validate_work_item function
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
source "$SCRIPT_DIR/work-item-adapters.sh"

echo "=== valid ==="
echo '{"remote":"gh","id":"42","title":"Test","type":"issue","state":"open","url":"","description":"","labels":[],"created":"2026-04-01","updated":"2026-04-01"}' | validate_work_item && echo "VALID"

echo "=== invalid remote ==="
echo '{"remote":"github","id":"42","title":"Test","type":"issue","state":"open","url":"","description":"","labels":[],"created":"2026-04-01","updated":"2026-04-01"}' | validate_work_item 2>&1 && echo "VALID" || echo "INVALID"

echo "=== missing fields ==="
echo '{"remote":"gh"}' | validate_work_item 2>&1 && echo "VALID" || echo "INVALID"
