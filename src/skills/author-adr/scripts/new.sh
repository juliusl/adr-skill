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
      exit 1
    fi
    shift

    id="${1:-}"
    if [ -z "$id" ]; then
      echo "ERROR: id is required for wi-nygard-agent format" >&2
      echo "Usage: new.sh wi-nygard-agent <remote> <id> <title...>" >&2
      exit 1
    fi
    shift

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

    last=0
    for f in "$adr_dir"/[0-9]*.md; do
      [ -f "$f" ] || continue
      num=$(basename "$f" | sed -E 's/^0*([0-9]+)-.*/\1/')
      if [ "$num" -gt "$last" ] 2>/dev/null; then
        last="$num"
      fi
    done
    next=$(printf "%04d" $(( last + 1 )))

    exec "$format_script" new "$next" "$title" "$adr_dir"
    ;;
esac
