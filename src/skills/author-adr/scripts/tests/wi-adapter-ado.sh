#!/bin/bash
# Test: ADO adapter produces normalized work item JSON
SCRIPT_DIR="$(cd "$(dirname "$0")/.." && pwd)"
source "$SCRIPT_DIR/work-item-adapters.sh"

echo "=== user story ==="
echo '{"id":1234,"fields":{"System.Title":"Implement auth module","System.WorkItemType":"User Story","System.State":"Active","System.Description":"Auth module needed for API","System.Tags":"adr; security","System.CreatedDate":"2026-04-01T10:00:00Z","System.ChangedDate":"2026-04-05T07:00:00Z"},"url":"https://dev.azure.com/org/project/_workitems/edit/1234"}' | ado_adapter | jq -S .

echo "=== bug ==="
echo '{"id":5678,"fields":{"System.Title":"Login fails on timeout","System.WorkItemType":"Bug","System.State":"Resolved","System.Description":"","System.Tags":"","System.CreatedDate":"2026-04-02T10:00:00Z","System.ChangedDate":"2026-04-04T07:00:00Z"},"url":"https://dev.azure.com/org/project/_workitems/edit/5678"}' | ado_adapter | jq -S .

echo "=== validation ==="
echo '{"id":1234,"fields":{"System.Title":"Test","System.WorkItemType":"Task","System.State":"New","System.Tags":"","System.CreatedDate":"2026-04-01T10:00:00Z","System.ChangedDate":"2026-04-01T10:00:00Z"},"url":""}' | ado_adapter | validate_work_item && echo "VALID"
