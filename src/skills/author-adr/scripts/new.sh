#!/usr/bin/env bash
set -euo pipefail

# new.sh — ADR orchestrator
# Resolves ADR directory, delegates to <format>-format.sh
# Sequential formats: computes next number, passes to format script
# Work-item formats: passes remote+id from CLI args to format script

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

format="${1:-}"
if [ -z "$format" ]; then
  echo "Usage: new.sh <format> <title...>" >&2
  echo "       new.sh wi-nygard-agent <remote> <id> <title...>" >&2
  echo "       new.sh wi-nygard-agent local <title...>  (ID auto-generated)" >&2
  exit 1
fi
shift

# Resolve ADR directory
if [ -f ".adr/adr-dir" ]; then
  adr_dir="$(cat .adr/adr-dir)"
elif [ -f ".adr-dir" ]; then
  adr_dir="$(cat .adr-dir)"
else
  adr_dir="docs/adr"
fi

# Ensure directory exists
mkdir -p "$adr_dir"

# Delegate to format script
format_script="$SCRIPT_DIR/${format}-format.sh"
if [ ! -x "$format_script" ]; then
  echo "ERROR: format script not found or not executable: $format_script" >&2
  exit 1
fi

case "$format" in
  wi-nygard-agent)
    # Format handles its own naming — parse remote + id from args
    remote="${1:-}"
    if [ -z "$remote" ]; then
      echo "ERROR: remote is required for wi-nygard-agent format" >&2
      echo "Usage: new.sh wi-nygard-agent <remote> <id> <title...>" >&2
      echo "       new.sh wi-nygard-agent local <title...>  (ID auto-generated)" >&2
      exit 1
    fi
    shift

    if [ "$remote" = "local" ]; then
      # Local adapter: ID is optional. Auto-generate when not provided.
      if [ -n "${ADR_LOCAL_ID:-}" ]; then
        id="$ADR_LOCAL_ID"
      elif [ $# -ge 2 ] && echo "$1" | grep -qE '^[a-zA-Z0-9]+$'; then
        # First arg looks like an explicit ID and more args follow (title)
        id="$1"
        shift
      else
        id=$(printf '%s' "$(date +%s%N 2>/dev/null || date +%s)$$" | shasum | head -c 8)
      fi
    else
      # Remote adapters: ID is required as next arg
      id="${1:-}"
      if [ -z "$id" ]; then
        echo "ERROR: id is required for wi-nygard-agent format with remote '$remote'" >&2
        echo "Usage: new.sh wi-nygard-agent <remote> <id> <title...>" >&2
        exit 1
      fi
      shift
    fi

    title="$*"
    if [ -z "$title" ]; then
      echo "ERROR: title is required" >&2
      echo "Usage: new.sh wi-nygard-agent <remote> <id> <title...>" >&2
      exit 1
    fi

    exec "$format_script" new "$remote" "$id" "$title" "$adr_dir"
    ;;
  *)
    # Sequential naming — compute next number
    title="$*"
    if [ -z "$title" ]; then
      echo "ERROR: title is required" >&2
      echo "Usage: new.sh <format> <title...>" >&2
      exit 1
    fi

    # User-mode: override directory and scope numbering to username-prefixed files
    if [ "${ADR_SCOPE:-}" = "user" ]; then
      adr_dir=".adr/usr/docs/adr"
      mkdir -p "$adr_dir"
      local_username="${ADR_USERNAME:-$(whoami)}"
      last=0
      for f in "$adr_dir"/${local_username}-[0-9]*.md; do
        [ -f "$f" ] || continue
        num=$(basename "$f" | sed -E "s/^${local_username}-0*([0-9]+)-.*/\1/")
        if [ "$num" -gt "$last" ] 2>/dev/null; then
          last="$num"
        fi
      done
    else
      last=0
      for f in "$adr_dir"/[0-9]*.md; do
        [ -f "$f" ] || continue
        num=$(basename "$f" | sed -E 's/^0*([0-9]+)-.*/\1/')
        if [ "$num" -gt "$last" ] 2>/dev/null; then
          last="$num"
        fi
      done
    fi
    next=$(printf "%04d" $(( last + 1 )))

    exec "$format_script" new "$next" "$title" "$adr_dir"
    ;;
esac
