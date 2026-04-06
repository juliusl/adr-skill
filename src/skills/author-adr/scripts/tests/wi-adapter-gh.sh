#!/bin/bash
# Test: GitHub adapter produces normalized work item JSON
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
source "$SCRIPT_DIR/work-item-adapters.sh"

echo "=== basic issue ==="
echo '{"number":42,"title":"Evaluate PostgreSQL","state":"open","html_url":"https://github.com/org/repo/issues/42","body":"Some description","labels":["adr","architecture"],"created_at":"2026-04-01T10:00:00Z","updated_at":"2026-04-05T07:00:00Z"}' | gh_adapter | jq -S .

echo "=== bug label ==="
echo '{"number":7,"title":"Fix crash on startup","state":"closed","html_url":"https://github.com/org/repo/issues/7","body":"App crashes","labels":[{"name":"bug"},{"name":"urgent"}],"created_at":"2026-04-01T10:00:00Z","updated_at":"2026-04-05T07:00:00Z"}' | gh_adapter | jq -S .

echo "=== validation ==="
echo '{"number":42,"title":"Test","state":"open","html_url":"","body":"","labels":[],"created_at":"2026-04-01T10:00:00Z","updated_at":"2026-04-01T10:00:00Z"}' | gh_adapter | validate_work_item && echo "VALID"
