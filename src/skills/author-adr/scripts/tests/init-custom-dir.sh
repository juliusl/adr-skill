#!/bin/bash
# Test: init with custom directory
nygard-agent-format.sh init my-decisions
echo "---"
cat .adr-dir
echo "---"
ls my-decisions/
