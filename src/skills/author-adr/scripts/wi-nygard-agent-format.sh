#!/usr/bin/env bash
set -euo pipefail

# wi-nygard-agent-format.sh — Work-item-referenced ADR format script
# Naming: {remote}-{id}-{slug}.md (ADR-0034)
# Template body reuses nygard-agent template (ADR-0017)
# Subcommands: new, init, list, rename, status

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

# Validate remote against allowlist
validate_remote() {
  local remote="$1"
  case "$remote" in
    gh|ado|local) return 0 ;;
    *)
      echo "ERROR: unknown remote '$remote'. Allowed: gh, ado, local" >&2
      exit 1
      ;;
  esac
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
  local remote="$1" id="$2" title="$3"
  local date
  date=$(adr_date)

  cat <<EOF
# ${remote}-${id}. ${title}

Date: ${date}
Status: Prototype
Last Updated: ${date}
Work-Item: ${remote}#${id}
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

  validate_remote "$remote"
  validate_id "$id"

  if [ -z "$title" ]; then
    echo "ERROR: title is required" >&2
    exit 1
  fi

  local slug
  slug=$(slugify "$title")
  local file="$dir/${remote}-${id}-${slug}.md"

  # Check for any existing ADR with same remote-id (different slug is still a duplicate)
  for existing in "$dir"/${remote}-${id}-*.md; do
    if [ -f "$existing" ]; then
      echo "ERROR: ADR for ${remote}-${id} already exists: $existing" >&2
      exit 1
    fi
  done

  generate_template "$remote" "$id" "$title" > "$file"
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

  validate_remote "$remote"
  validate_id "$id"

  local slug
  slug=$(slugify "$new_title")

  local old_file=""
  for f in "$dir"/${remote}-${id}-*.md; do
    [ -f "$f" ] && old_file="$f" && break
  done

  if [ -z "$old_file" ]; then
    echo "ERROR: ADR ${remote}-${id} not found in $dir" >&2
    exit 1
  fi

  local new_file="$dir/${remote}-${id}-${slug}.md"

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
  local new_heading="# ${remote}-${id}. ${new_title}"

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
  validate_remote "$remote"
  validate_id "$id"

  local file=""
  for f in "$dir"/${remote}-${id}-*.md; do
    [ -f "$f" ] && file="$f" && break
  done

  if [ -z "$file" ]; then
    echo "ERROR: ADR ${remote}-${id} not found in $dir" >&2
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

# --- Dispatch ---

case "${1:-help}" in
  new)    shift; cmd_new "$@" ;;
  init)   shift; cmd_init "$@" ;;
  list)   shift; cmd_list "$@" ;;
  rename) shift; cmd_rename "$@" ;;
  status) shift; cmd_status "$@" ;;
  *)
    echo "Usage: wi-nygard-agent-format.sh {new|init|list|rename|status}" >&2
    echo "" >&2
    echo "Subcommands:" >&2
    echo "  new <remote> <id> <title> <dir>      Generate ADR with work-item-referenced naming" >&2
    echo "  init [dir]                            Bootstrap ADR directory" >&2
    echo "  list                                  List ADRs with title and status" >&2
    echo "  rename <remote> <id> <new-title>      Rename an ADR file and update heading" >&2
    echo "  status [remote] [id] [new-status]     Show or update ADR status" >&2
    exit 1
    ;;
esac
