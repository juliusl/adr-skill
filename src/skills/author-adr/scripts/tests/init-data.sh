#!/bin/bash
# Test: init-data creates .adr/ project-scoped directory
# init-data lives in the Makefile, not the format script — derive path from scripts on PATH
SKILL_MAKEFILE="$(dirname "$(which nygard-agent-format.sh)")/../Makefile"
make -f "$SKILL_MAKEFILE" init-data
echo "---"
ls -1A .adr/
echo "---"
cat .adr/.gitignore
echo "---"
# Idempotent: run again, no errors
make -f "$SKILL_MAKEFILE" init-data
echo "---"
cat .adr/.gitignore
