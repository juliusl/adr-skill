#!/bin/bash
# Test: create a TBD ADR (interim title for solve workflow)
nygard-agent-format.sh init
new.sh nygard-agent tbd
echo "---"
cat docs/adr/0002-tbd.md
