#!/bin/bash
# Test: init-data creates .adr/ project-scoped directory
nygard-agent-format.sh init-data
echo "---"
ls -1A .adr/
echo "---"
cat .adr/.gitignore
echo "---"
# Idempotent: run again, no errors
nygard-agent-format.sh init-data
echo "---"
cat .adr/.gitignore
