#!/usr/bin/env bash
# work-item-adapters.sh — Normalized work item adapters (ADR-0035)
# Source this file to use adapter functions. Do not invoke directly.
#
# Adapters transform remote-specific work item data into a normalized JSON model.
# Each adapter reads JSON from stdin (or arguments) and writes normalized JSON to stdout.
#
# Usage:
#   source work-item-adapters.sh
#   echo '{"number":42,...}' | gh_adapter
#   echo '{"id":1234,...}' | ado_adapter
#   local_adapter "My ADR title"

# Require jq
if ! command -v jq &>/dev/null; then
  echo "ERROR: jq is required for work-item-adapters.sh" >&2
  return 1 2>/dev/null || exit 1
fi

# --- Validation ---

# Validate a normalized work item JSON object.
# Returns 0 if valid, 1 if invalid (with error on stderr).
validate_work_item() {
  local json="${1:-$(cat)}"

  local errors
  errors=$(echo "$json" | jq -r '
    def check_field(name; type_check):
      if has(name) then
        if (.[name] | type_check) then empty
        else "field \(name): wrong type"
        end
      else "missing required field: \(name)"
      end;

    def check_enum(name; values):
      if has(name) then
        if ([.[name]] | inside(values)) then empty
        else "field \(name): invalid value \"\(.[name])\", expected one of \(values)"
        end
      else empty
      end;

    [
      check_field("remote"; type == "string"),
      check_field("id"; type == "string"),
      check_field("title"; type == "string"),
      check_field("type"; type == "string"),
      check_field("state"; type == "string"),
      check_field("labels"; type == "array"),
      check_field("created"; type == "string"),
      check_field("updated"; type == "string"),
      check_enum("remote"; ["gh","ado","local","gitea"]),
      check_enum("type"; ["issue","bug","story","task","feature","epic","other"]),
      check_enum("state"; ["open","active","resolved","closed"])
    ] | if length == 0 then "valid" else .[] end
  ' 2>&1)

  if [ "$errors" = "valid" ]; then
    return 0
  else
    echo "Validation errors:" >&2
    echo "$errors" >&2
    return 1
  fi
}

# --- GitHub Adapter ---

# Transform GitHub issue JSON (from github-mcp-server-issue_read) to normalized model.
# Reads JSON from stdin.
gh_adapter() {
  jq -c '{
    remote: "gh",
    id: (.number // .issue_number | tostring),
    title: .title,
    type: (
      if (.labels // [] | map(if type == "object" then .name else . end) | index("bug")) then "bug"
      else "issue"
      end
    ),
    state: (
      if .state == "closed" then "closed"
      else "open"
      end
    ),
    url: (.html_url // .url // ""),
    description: ((.body // "")[0:500]),
    labels: [(.labels // [] | .[] | if type == "object" then .name else . end)],
    created: (.created_at // ""),
    updated: (.updated_at // "")
  }'
}

# --- Azure DevOps Adapter ---

# Transform ADO work item JSON (from ado-wit_get_work_item) to normalized model.
# Reads JSON from stdin. Handles both flat and nested (fields) formats.
ado_adapter() {
  jq -c '
    # Extract fields — handle both flat and nested formats
    def get_field(name):
      if has("fields") then .fields[name]
      else .[name]
      end;

    # Normalize ADO work item type
    def normalize_type:
      (get_field("System.WorkItemType") // "other") |
      if . == "Bug" then "bug"
      elif . == "User Story" then "story"
      elif . == "Task" then "task"
      elif . == "Feature" then "feature"
      elif . == "Epic" then "epic"
      else "other"
      end;

    # Normalize ADO state
    def normalize_state:
      (get_field("System.State") // "New") |
      if . == "New" then "open"
      elif . == "Active" then "active"
      elif . == "Resolved" then "resolved"
      elif . == "Closed" then "closed"
      else "open"
      end;

    # Split ADO tags string into array
    def parse_tags:
      (get_field("System.Tags") // "") |
      if . == "" then []
      else split("; ") | map(gsub("^\\s+|\\s+$"; ""))
      end;

    {
      remote: "ado",
      id: (.id // get_field("System.Id") | tostring),
      title: (get_field("System.Title") // ""),
      type: normalize_type,
      state: normalize_state,
      url: (._links.html.href // .url // ""),
      description: ((get_field("System.Description") // "")[0:500]),
      labels: parse_tags,
      created: (get_field("System.CreatedDate") // ""),
      updated: (get_field("System.ChangedDate") // "")
    }
  '
}

# --- Local Adapter ---

# Generate a normalized work item for local/offline use.
# Accepts title as argument. Generates an 8-char hex ID from timestamp.
local_adapter() {
  local title="${1:-}"
  if [ -z "$title" ]; then
    echo "ERROR: title is required for local_adapter" >&2
    return 1
  fi

  local id now
  # Use ADR_LOCAL_ID if set (for testing determinism), otherwise generate
  if [ -n "${ADR_LOCAL_ID:-}" ]; then
    id="$ADR_LOCAL_ID"
  else
    id=$(printf '%s' "$(date +%s%N 2>/dev/null || date +%s)$$" | shasum | head -c 8)
  fi
  now="${ADR_DATE:-$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date +%Y-%m-%dT%H:%M:%SZ)}"

  jq -n -c \
    --arg id "$id" \
    --arg title "$title" \
    --arg now "$now" \
    '{
      remote: "local",
      id: $id,
      title: $title,
      type: "other",
      state: "open",
      url: "",
      description: "",
      labels: [],
      created: $now,
      updated: $now
    }'
}

# --- Cache Functions (ADR-0036) ---

# Cache a normalized work item by appending to .adr/var/work-items.jsonl
# Reads JSON from stdin, adds cached_at timestamp, appends to cache file.
cache_work_item() {
  local cache_file=".adr/var/work-items.jsonl"
  mkdir -p "$(dirname "$cache_file")"
  local now
  now="${ADR_CACHE_TS:-$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date +%Y-%m-%dT%H:%M:%SZ)}"
  jq -c --arg ts "$now" '. + {cached_at: $ts}' >> "$cache_file"
}

# Look up the latest cached snapshot for a given remote and id.
# Returns the JSON on stdout and exit 0, or exit 1 if not found.
lookup_work_item() {
  local remote="${1:-}" id="${2:-}"
  if [ -z "$remote" ] || [ -z "$id" ]; then
    echo "Usage: lookup_work_item <remote> <id>" >&2
    return 1
  fi

  local cache_file=".adr/var/work-items.jsonl"
  if [ ! -f "$cache_file" ]; then
    return 1
  fi

  local result
  result=$(grep "\"remote\":\"${remote}\"" "$cache_file" | grep "\"id\":\"${id}\"" | tail -1)
  if [ -z "$result" ]; then
    return 1
  fi
  echo "$result"
}
