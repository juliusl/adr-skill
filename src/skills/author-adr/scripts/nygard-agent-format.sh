#!/usr/bin/env bash
set -euo pipefail

# nygard-agent-format.sh — Self-contained format script for nygard-agent ADRs
# Template from ADR-0017 is baked in. No external template files.
# Subcommands: new, init, list, status

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

# Parse status from an ADR file. Handles both inline "Status:" metadata
# (nygard-agent format) and "## Status" heading (standard Nygard format).
parse_status() {
  local file="$1"
  local status=""

  # Try inline Status: first
  status=$(grep -m1 '^Status:' "$file" 2>/dev/null | sed 's/^Status: *//' || true)

  # Fall back to ## Status heading
  if [ -z "$status" ]; then
    status=$(awk '/^## Status/{getline; while(/^$/){getline}; print; exit}' "$file" 2>/dev/null || true)
  fi

  echo "$status"
}

# Parse title from first heading line
parse_title() {
  local file="$1"
  head -1 "$file" | sed 's/^# *//'
}

# --- Template ---

generate_template() {
  local number="$1" title="$2"
  local date display_number
  date=$(adr_date)
  display_number=$(echo "$number" | sed 's/^0*//')

  cat <<EOF
# ${display_number}. ${title}

Date: ${date}
Status: Prototype
Last Updated: ${date}
Links:

## Context

## Options

## Decision

## Consequences

## Quality Strategy

- [ ] Introduces major semantic changes
- [ ] Introduces minor semantic changes
- [ ] Fuzz testing
- [ ] Unit testing
- [ ] Load testing
- [ ] Performance testing
- [ ] Backwards Compatible
- [ ] Integration tests
- [ ] User documentation

### Additional Quality Concerns

---

## Comments
EOF
}

# --- Subcommands ---

cmd_new() {
  local number="$1" title="$2" dir="$3"
  local slug
  slug=$(slugify "$title")
  local file="$dir/${number}-${slug}.md"

  generate_template "$number" "$title" > "$file"
  echo "$file"
}

cmd_init() {
  local dir="${1:-$(resolve_dir)}"
  mkdir -p "$dir"

  # Write .adr-dir if non-default
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
- ~~User documentation~~

### Additional Quality Concerns

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

  for f in "$dir"/[0-9]*.md; do
    [ -f "$f" ] || continue
    local title status
    title=$(parse_title "$f")
    status=$(parse_status "$f")
    printf "%s\t[%s]\n" "$title" "$status"
  done
}

cmd_rename() {
  local dir num new_title slug old_file new_file display_number
  dir=$(resolve_dir)

  if [ $# -lt 2 ]; then
    echo "Usage: nygard-agent-format.sh rename <num> <new-title>" >&2
    exit 1
  fi

  num=$(printf "%04d" "$1")
  shift
  new_title="$*"
  slug=$(slugify "$new_title")
  display_number=$(echo "$num" | sed 's/^0*//')

  old_file=""
  for f in "$dir"/${num}-*.md; do
    [ -f "$f" ] && old_file="$f" && break
  done

  if [ -z "$old_file" ]; then
    echo "ERROR: ADR $num not found in $dir" >&2
    exit 1
  fi

  new_file="$dir/${num}-${slug}.md"

  if [ "$old_file" = "$new_file" ]; then
    echo "No rename needed: $(basename "$old_file")"
    return
  fi

  if [ -f "$new_file" ]; then
    echo "ERROR: target file already exists: $new_file" >&2
    exit 1
  fi

  # Update heading (first line)
  local today
  today=$(adr_date)
  sed -i '' "1s/.*/# ${display_number}. ${new_title}/" "$old_file"

  # Update Last Updated date
  if grep -q '^Last Updated:' "$old_file"; then
    sed -i '' "s/^Last Updated:.*/Last Updated: ${today}/" "$old_file"
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

  local num
  num=$(printf "%04d" "$1")
  local file=""
  for f in "$dir"/${num}-*.md; do
    [ -f "$f" ] && file="$f" && break
  done

  if [ -z "$file" ]; then
    echo "ERROR: ADR $num not found in $dir" >&2
    exit 1
  fi

  if [ $# -eq 1 ]; then
    parse_status "$file"
  else
    local new_status="$2"

    if grep -q '^Status:' "$file"; then
      # Inline format — use awk to replace
      awk -v status="$new_status" '{
        if ($0 ~ /^Status:/) { print "Status: " status }
        else { print }
      }' "$file" > "${file}.tmp" && mv "${file}.tmp" "$file"
    elif grep -q '^## Status' "$file"; then
      # Heading format — replace the first non-blank line after ## Status
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
    echo "Usage: nygard-agent-format.sh {new|init|list|rename|status}" >&2
    echo "" >&2
    echo "Subcommands:" >&2
    echo "  new <number> <title> <dir>   Generate ADR from baked-in template" >&2
    echo "  init [dir]                   Bootstrap ADR directory" >&2
    echo "  list                         List ADRs with title and status" >&2
    echo "  rename <num> <new-title>     Rename an ADR file and update heading" >&2
    echo "  status [num] [new-status]    Show or update ADR status" >&2
    exit 1
    ;;
esac
