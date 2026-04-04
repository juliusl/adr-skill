#!/usr/bin/env bash
set -euo pipefail

# new.sh — ADR orchestrator
# Resolves ADR directory, computes next number, slugifies title,
# delegates to <format>-format.sh

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"

format="${1:-}"
if [ -z "$format" ]; then
  echo "Usage: new.sh <format> <title...>" >&2
  exit 1
fi
shift

title="$*"
if [ -z "$title" ]; then
  echo "ERROR: title is required" >&2
  echo "Usage: new.sh <format> <title...>" >&2
  exit 1
fi

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

# Compute next sequential number
last=0
for f in "$adr_dir"/[0-9]*.md; do
  [ -f "$f" ] || continue
  num=$(basename "$f" | sed -E 's/^0*([0-9]+)-.*/\1/')
  if [ "$num" -gt "$last" ] 2>/dev/null; then
    last="$num"
  fi
done
next=$(printf "%04d" $(( last + 1 )))

# Delegate to format script
format_script="$SCRIPT_DIR/${format}-format.sh"
if [ ! -x "$format_script" ]; then
  echo "ERROR: format script not found or not executable: $format_script" >&2
  exit 1
fi

exec "$format_script" new "$next" "$title" "$adr_dir"
