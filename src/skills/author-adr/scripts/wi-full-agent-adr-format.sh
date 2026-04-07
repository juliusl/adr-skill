#!/usr/bin/env bash
set -euo pipefail

# wi-full-agent-adr-format.sh — Thin wrapper delegating to adr-format binary
# The adr-format binary handles all TOML operations for the wi-full-agent-adr format.

# Locate adr-format binary: $PATH first, then workspace build output
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
CRATES_BIN="$SCRIPT_DIR/../../crates/target/release/adr-format"

if command -v adr-format &>/dev/null; then
  exec adr-format "$@"
elif [ -x "$CRATES_BIN" ]; then
  exec "$CRATES_BIN" "$@"
else
  echo "ERROR: adr-format binary not found. Run 'make build-tools' first." >&2
  exit 1
fi
