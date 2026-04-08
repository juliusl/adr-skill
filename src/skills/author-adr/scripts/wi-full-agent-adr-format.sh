#!/usr/bin/env bash
set -euo pipefail

# wi-full-agent-adr-format.sh — Thin wrapper delegating to adr-db author subcommand
# The adr-db binary handles all TOML operations via the 'format' subcommand.

# Resolve symlinks to find the actual script location
SCRIPT_SOURCE="${BASH_SOURCE[0]}"
while [ -L "$SCRIPT_SOURCE" ]; do
  SCRIPT_DIR="$(cd "$(dirname "$SCRIPT_SOURCE")" && pwd)"
  SCRIPT_SOURCE="$(readlink "$SCRIPT_SOURCE")"
  [[ "$SCRIPT_SOURCE" != /* ]] && SCRIPT_SOURCE="$SCRIPT_DIR/$SCRIPT_SOURCE"
done
SCRIPT_DIR="$(cd "$(dirname "$SCRIPT_SOURCE")" && pwd)"
CRATES_BIN="$SCRIPT_DIR/../../../crates/target/release/adr-db"

if command -v adr-db &>/dev/null; then
  exec adr-db author "$@"
elif [ -x "$CRATES_BIN" ]; then
  exec "$CRATES_BIN" format "$@"
else
  echo "ERROR: adr-db binary not found. Run 'make build-tools' first." >&2
  exit 1
fi
