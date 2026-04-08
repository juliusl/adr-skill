#!/usr/bin/env bash
set -euo pipefail

# wi-nygard-agent-format.sh — Work-item-referenced ADR format script
# Naming: {adapter}-{id}-{slug}.md (ADR-0034, ADR-0062)
# Template body reuses nygard-agent template (ADR-0017)
# Subcommands: new, init, list, rename, status, lifecycle

# --- Helpers ---

slugify() {
  echo "$*" | tr '[:upper:]' '[:lower:]' | tr -cs '[:alnum:]' '-' | sed 's/^-//;s/-$//'
}

resolve_dir() {
  if [ -f ".adr/adr-dir" ]; then
    cat .adr/adr-dir
  elif [ -f ".adr-dir" ]; then
    cat .adr-dir
  else
    echo "docs/adr"
  fi
}

adr_date() {
  echo "${ADR_DATE:-$(date +%Y-%m-%d)}"
}

# Extract host from a git URL (SSH shorthand, ssh://, or https://)
extract_host() {
  local url="$1"
  # SSH shorthand: git@host:path
  if printf '%s' "$url" | grep -q '^git@'; then
    printf '%s' "$url" | sed 's/^git@//; s/:.*//' | tr '[:upper:]' '[:lower:]'
    return
  fi
  # Scheme-based: https://host/path or ssh://git@host/path
  printf '%s' "$url" | sed 's|.*://||; s|.*@||; s|/.*||; s|:.*||' | tr '[:upper:]' '[:lower:]'
}

# Detect adapter type from a git remote URL
detect_adapter_from_url() {
  local url="$1"
  local host
  host=$(extract_host "$url")

  case "$host" in
    *github.com*) printf 'gh' ;;
    *dev.azure.com*|*visualstudio.com*) printf 'ado' ;;
    *)
      echo "ERROR: Could not detect adapter type for URL '$url'. Supported hosts: github.com, dev.azure.com. For other forges, a future configuration mechanism will allow explicit adapter mapping." >&2
      return 1
      ;;
  esac
}

# Detect adapter type from a git remote name or 'local' keyword.
# If ADR_TEST_ADAPTER is set, return its value (testing seam).
# Echoes adapter type to stdout. Returns 1 on failure.
detect_adapter() {
  local remote="$1"

  # Testing seam: allow tests to override detection (testing only)
  if [ -n "${ADR_TEST_ADAPTER:-}" ]; then
    case "$ADR_TEST_ADAPTER" in
      gh|ado|gitea|local) ;;
      *) echo "ERROR: ADR_TEST_ADAPTER='$ADR_TEST_ADAPTER' is not a known adapter type" >&2; return 1 ;;
    esac
    printf '%s' "$ADR_TEST_ADAPTER"
    return 0
  fi

  if [ "$remote" = "local" ]; then
    printf 'local'
    return 0
  fi

  local url
  url=$(git remote get-url "$remote" 2>/dev/null) || {
    echo "ERROR: git remote '$remote' not found. Run 'git remote -v' to list available remotes." >&2
    return 1
  }

  detect_adapter_from_url "$url"
}

# Validate id: alphanumeric and hyphens only, no empty, no path separators
validate_id() {
  local id="$1"
  if [ -z "$id" ]; then
    echo "ERROR: id is required" >&2
    exit 1
  fi
  if ! echo "$id" | grep -qE '^[a-zA-Z0-9]+$'; then
    echo "ERROR: id must be alphanumeric, got '$id'" >&2
    exit 1
  fi
}

parse_status() {
  local file="$1"
  local status=""
  status=$(grep -m1 '^Status:' "$file" 2>/dev/null | sed 's/^Status: *//' || true)
  if [ -z "$status" ]; then
    status=$(awk '/^## Status/{getline; while(/^$/){getline}; print; exit}' "$file" 2>/dev/null || true)
  fi
  echo "$status"
}

parse_title() {
  local file="$1"
  head -1 "$file" | sed 's/^# *//'
}

parse_date() {
  local file="$1"
  grep -m1 '^Date:' "$file" 2>/dev/null | sed 's/^Date: *//' || true
}

# --- Template ---

generate_template() {
  local adapter="$1" id="$2" title="$3"
  local date
  date=$(adr_date)

  cat <<EOF
# ${adapter}-${id}. ${title}

Date: ${date}
Status: Prototype
Last Updated: ${date}
Work-Item: ${adapter}#${id}
Links:

## Context

## Options

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** [Proceed | Pause for validation | Skipped — <rationale>]

- [ ] All options evaluated at comparable depth
- [ ] Decision drivers are defined and referenced in option analysis
- [ ] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

## Consequences

## Deliverables

<!-- Planning phase: define expected deliverables as a checklist. -->
<!-- Delivery phase: check items off and add artifact references. -->

- [ ] [Expected artifact or outcome]

## Quality Strategy

- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [ ] Backwards Compatible
- [ ] Integration tests
- [ ] Tooling
- [ ] User documentation

### Additional Quality Concerns

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** [Ready for review | Needs work | Skipped — <rationale>]

- [ ] Decision justified (Y-statement or equivalent)
- [ ] Consequences include positive, negative, and neutral outcomes
- [ ] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [ ] Links to related ADRs populated

**Pre-review notes:**

---

## Comments
EOF
}

# --- Subcommands ---

cmd_new() {
  if [ $# -lt 4 ]; then
    echo "Usage: wi-nygard-agent-format.sh new <remote> <id> <title> <dir>" >&2
    exit 1
  fi

  local remote="$1" id="$2" title="$3" dir="$4"

  local adapter
  adapter=$(detect_adapter "$remote") || exit 1
  validate_id "$id"

  if [ -z "$title" ]; then
    echo "ERROR: title is required" >&2
    exit 1
  fi

  local slug
  slug=$(slugify "$title")
  local file="$dir/${adapter}-${id}-${slug}.md"

  # Check for any existing ADR with same adapter-id (different slug is still a duplicate)
  for existing in "$dir"/${adapter}-${id}-*.md; do
    if [ -f "$existing" ]; then
      echo "ERROR: ADR for ${adapter}-${id} already exists: $existing" >&2
      exit 1
    fi
  done

  generate_template "$adapter" "$id" "$title" > "$file"
  echo "$file"
}

cmd_init() {
  local dir="${1:-$(resolve_dir)}"
  mkdir -p "$dir"

  if [ "$dir" != "docs/adr" ]; then
    echo "$dir" > .adr-dir
  fi

  local date
  date=$(adr_date)

  cat > "$dir/0001-record-architecture-decisions.md" <<EOF
# 1. Record architecture decisions

Date: ${date}
Status: Accepted
Last Updated: ${date}
Links:

## Context

We need to record the architectural decisions made on this project.

## Options

### Option 1: No documentation

Keep decisions informal, undocumented.

### Option 2: Use ADRs

Record decisions as Architecture Decision Records.

## Evaluation Checkpoint (Optional)
<!-- Gate: Options → Decision. Agent assesses and recommends. -->

**Assessment:** Proceed

- [x] All options evaluated at comparable depth
- [x] Decision drivers are defined and referenced in option analysis
- [x] No unacknowledged experimentation gaps (ADR-0022 tolerance check)

**Validation needs:**

## Decision

We will use Architecture Decision Records, as described by Michael Nygard.

## Consequences

See Michael Nygard's article, linked above. For a lightweight ADR toolset,
see Nat Pryce's adr-tools.

## Quality Strategy

- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- ~~Fuzz testing~~
- ~~Unit testing~~
- ~~Load testing~~
- ~~Performance testing~~
- [x] Backwards Compatible
- ~~Integration tests~~
- ~~Tooling~~
- ~~User documentation~~

### Additional Quality Concerns

## Conclusion Checkpoint (Optional)
<!-- Gate: Quality Strategy → Review. Verify before requesting review. -->

**Assessment:** Ready for review

- [x] Decision justified (Y-statement or equivalent)
- [x] Consequences include positive, negative, and neutral outcomes
- [x] Quality Strategy reviewed — relevant items checked, irrelevant struck through
- [x] Links to related ADRs populated

**Pre-review notes:**

---

## Comments
EOF

  echo "$dir/0001-record-architecture-decisions.md"
}

cmd_list() {
  local dir
  dir=$(resolve_dir)

  if [ ! -d "$dir" ]; then
    echo "ERROR: ADR directory not found: $dir" >&2
    exit 1
  fi

  # Collect all ADR files: sequential + work-item-prefixed
  local files=()
  local seen=()

  for f in "$dir"/[0-9]*.md; do
    [ -f "$f" ] || continue
    files+=("$f")
    seen+=("$(basename "$f")")
  done

  for f in "$dir"/[a-z]*-[0-9a-zA-Z]*-*.md; do
    [ -f "$f" ] || continue
    local base
    base=$(basename "$f")
    # Skip if already seen (shouldn't happen, but guard against duplicates)
    local dup=0
    for s in "${seen[@]+"${seen[@]}"}"; do
      [ "$s" = "$base" ] && dup=1 && break
    done
    [ "$dup" -eq 1 ] && continue
    files+=("$f")
  done

  # Sort work-item files by Date: metadata
  # Sequential files come first (they sort by filename), then wi files by date
  local seq_files=()
  local wi_entries=()

  for f in "${files[@]+"${files[@]}"}"; do
    local base
    base=$(basename "$f")
    if echo "$base" | grep -qE '^[0-9]'; then
      seq_files+=("$f")
    else
      local d
      d=$(parse_date "$f")
      wi_entries+=("${d}|${f}")
    fi
  done

  # Print sequential files in filename order
  for f in "${seq_files[@]+"${seq_files[@]}"}"; do
    local title status
    title=$(parse_title "$f")
    status=$(parse_status "$f")
    printf "%s\t[%s]\n" "$title" "$status"
  done

  # Print work-item files sorted by date
  if [ ${#wi_entries[@]} -gt 0 ]; then
    printf '%s\n' "${wi_entries[@]}" | sort -t'|' -k1 | while IFS='|' read -r _ wf; do
      local title status
      title=$(parse_title "$wf")
      status=$(parse_status "$wf")
      printf "%s\t[%s]\n" "$title" "$status"
    done
  fi
}

cmd_rename() {
  local dir
  dir=$(resolve_dir)

  if [ $# -lt 3 ]; then
    echo "Usage: wi-nygard-agent-format.sh rename <remote> <id> <new-title>" >&2
    exit 1
  fi

  local remote="$1" id="$2"
  shift 2
  local new_title="$*"

  local adapter
  adapter=$(detect_adapter "$remote") || exit 1
  validate_id "$id"

  local slug
  slug=$(slugify "$new_title")

  local old_file=""
  for f in "$dir"/${adapter}-${id}-*.md; do
    [ -f "$f" ] && old_file="$f" && break
  done

  if [ -z "$old_file" ]; then
    echo "ERROR: ADR ${adapter}-${id} not found in $dir" >&2
    exit 1
  fi

  local new_file="$dir/${adapter}-${id}-${slug}.md"

  if [ "$old_file" = "$new_file" ]; then
    echo "No rename needed: $(basename "$old_file")"
    return
  fi

  if [ -f "$new_file" ]; then
    echo "ERROR: target file already exists: $new_file" >&2
    exit 1
  fi

  # Update heading — use awk to avoid sed metacharacter issues
  local today
  today=$(adr_date)
  local new_heading="# ${adapter}-${id}. ${new_title}"

  awk -v heading="$new_heading" 'NR==1 { print heading; next } { print }' "$old_file" > "${old_file}.tmp"
  mv "${old_file}.tmp" "$old_file"

  # Update Last Updated date
  if grep -q '^Last Updated:' "$old_file"; then
    awk -v d="$today" '{ if ($0 ~ /^Last Updated:/) print "Last Updated: " d; else print }' "$old_file" > "${old_file}.tmp"
    mv "${old_file}.tmp" "$old_file"
  fi

  mv "$old_file" "$new_file"
  echo "Renamed: $(basename "$old_file") → $(basename "$new_file")"
}

cmd_status() {
  local dir
  dir=$(resolve_dir)

  if [ ! -d "$dir" ]; then
    echo "ERROR: ADR directory not found: $dir" >&2
    exit 1
  fi

  if [ $# -eq 0 ]; then
    cmd_list
    return
  fi

  if [ $# -lt 2 ]; then
    echo "Usage: wi-nygard-agent-format.sh status <remote> <id> [new-status]" >&2
    exit 1
  fi

  local remote="$1" id="$2"
  local adapter
  adapter=$(detect_adapter "$remote") || exit 1
  validate_id "$id"

  local file=""
  for f in "$dir"/${adapter}-${id}-*.md; do
    [ -f "$f" ] && file="$f" && break
  done

  if [ -z "$file" ]; then
    echo "ERROR: ADR ${adapter}-${id} not found in $dir" >&2
    exit 1
  fi

  if [ $# -eq 2 ]; then
    parse_status "$file"
  else
    local new_status="$3"

    if grep -q '^Status:' "$file"; then
      awk -v status="$new_status" '{
        if ($0 ~ /^Status:/) { print "Status: " status }
        else { print }
      }' "$file" > "${file}.tmp" && mv "${file}.tmp" "$file"
    elif grep -q '^## Status' "$file"; then
      awk -v status="$new_status" '
        /^## Status/ { print; found=1; next }
        found && /^$/ { print; next }
        found && !/^$/ { print status; found=0; next }
        { print }
      ' "$file" > "${file}.tmp" && mv "${file}.tmp" "$file"
    fi

    echo "Updated: $(basename "$file") → $new_status"
  fi
}

# --- Lifecycle (ADR-0038) ---

wi_state_to_adr_status() {
  case "$1" in
    open)     echo "Prototype" ;;
    active)   echo "Proposed" ;;
    resolved) echo "Accepted" ;;
    closed)   echo "Delivered" ;;
    *)        echo "unknown" ;;
  esac
}

log_lifecycle() {
  local remote="$1" id="$2" from="$3" to="$4" action="$5" trigger="$6"
  local log_file=".adr/var/lifecycle.jsonl"
  mkdir -p "$(dirname "$log_file")"
  local now
  now="${ADR_CACHE_TS:-$(date -u +%Y-%m-%dT%H:%M:%SZ 2>/dev/null || date +%Y-%m-%dT%H:%M:%SZ)}"
  if command -v jq &>/dev/null; then
    jq -n -c \
      --arg remote "$remote" --arg id "$id" --arg from "$from" \
      --arg to "$to" --arg action "$action" --arg trigger "$trigger" --arg ts "$now" \
      '{remote:$remote,id:$id,from_status:$from,to_status:$to,action:$action,trigger:$trigger,timestamp:$ts}' \
      >> "$log_file"
  fi
}

cmd_lifecycle() {
  local auto=0 sync=0

  if [ $# -lt 2 ]; then
    echo "Usage: wi-nygard-agent-format.sh lifecycle <remote> <id> [--auto] [--sync]" >&2
    exit 1
  fi

  local remote="$1" id="$2"
  shift 2

  while [ $# -gt 0 ]; do
    case "$1" in
      --auto) auto=1 ;;
      --sync) sync=1 ;;
      *) echo "ERROR: unknown flag '$1'" >&2; exit 1 ;;
    esac
    shift
  done

  local adapter
  adapter=$(detect_adapter "$remote") || exit 1
  validate_id "$id"

  local dir
  dir=$(resolve_dir)

  local adr_file=""
  for f in "$dir"/${adapter}-${id}-*.md; do
    [ -f "$f" ] && adr_file="$f" && break
  done

  if [ -z "$adr_file" ]; then
    echo "ERROR: ADR ${adapter}-${id} not found in $dir" >&2
    exit 1
  fi

  # Source adapters for cache lookup
  local script_dir
  script_dir="$(cd "$(dirname "$0")" && pwd)"
  # shellcheck disable=SC1091
  source "$script_dir/work-item-adapters.sh"

  local adr_status
  adr_status=$(parse_status "$adr_file")

  local cached_wi
  cached_wi=$(lookup_work_item "$adapter" "$id" 2>/dev/null || true)

  if [ -z "$cached_wi" ]; then
    echo "No cached work item for ${adapter}-${id}."
    echo "Current ADR status: $adr_status"
    return 0
  fi

  local wi_state expected_adr_status
  wi_state=$(echo "$cached_wi" | jq -r '.state')
  expected_adr_status=$(wi_state_to_adr_status "$wi_state")

  echo "ADR: $(basename "$adr_file")"
  echo "  ADR status:       $adr_status"
  echo "  Work item state:  $wi_state"
  echo "  Expected status:  $expected_adr_status"

  if [ "$adr_status" = "$expected_adr_status" ]; then
    echo "  → In sync. No action needed."
    return 0
  fi

  local action=""
  case "$expected_adr_status" in
    Accepted)  action="Transition to Accepted (implement-adr eligible)" ;;
    Delivered) action="Transition to Delivered (verify Deliverables checklist)" ;;
    *)         action="Transition to $expected_adr_status" ;;
  esac

  echo "  → Recommended: $action"

  if [ "$auto" -eq 0 ]; then
    echo "  Run with --auto to execute this transition."
    return 0
  fi

  echo "  → Executing: $adr_status → $expected_adr_status"

  if grep -q '^Status:' "$adr_file"; then
    awk -v status="$expected_adr_status" '{
      if ($0 ~ /^Status:/) { print "Status: " status }
      else { print }
    }' "$adr_file" > "${adr_file}.tmp" && mv "${adr_file}.tmp" "$adr_file"
  fi

  local trigger="lifecycle --auto"
  [ "$sync" -eq 1 ] && trigger="$trigger --sync"
  log_lifecycle "$adapter" "$id" "$adr_status" "$expected_adr_status" "$action" "$trigger"

  echo "  → Updated: $(basename "$adr_file") → $expected_adr_status"
  echo "  → Logged to .adr/var/lifecycle.jsonl"
}

# --- Dispatch ---

case "${1:-help}" in
  new)       shift; cmd_new "$@" ;;
  init)      shift; cmd_init "$@" ;;
  list)      shift; cmd_list "$@" ;;
  rename)    shift; cmd_rename "$@" ;;
  status)    shift; cmd_status "$@" ;;
  lifecycle) shift; cmd_lifecycle "$@" ;;
  *)
    echo "Usage: wi-nygard-agent-format.sh {new|init|list|rename|status|lifecycle}" >&2
    echo "" >&2
    echo "Subcommands:" >&2
    echo "  new <remote> <id> <title> <dir>      Generate ADR with work-item-referenced naming" >&2
    echo "  init [dir]                            Bootstrap ADR directory" >&2
    echo "  list                                  List ADRs with title and status" >&2
    echo "  rename <remote> <id> <new-title>      Rename an ADR file and update heading" >&2
    echo "  status [remote] [id] [new-status]     Show or update ADR status" >&2
    echo "  lifecycle <remote> <id> [--auto]      Check/execute lifecycle transition" >&2
    exit 1
    ;;
esac
