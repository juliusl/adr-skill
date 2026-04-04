#!/bin/bash
# Test: resolve_dir prefers .adr/adr-dir over .adr-dir
# Setup: create both .adr/adr-dir and .adr-dir with different values
nygard-agent-format.sh init my-decisions
echo "---"
# Legacy .adr-dir points to my-decisions
cat .adr-dir
echo "---"
# Now create .adr/adr-dir pointing to a different directory
mkdir -p .adr
echo "new-decisions" > .adr/adr-dir
mkdir -p new-decisions
# Create an ADR in the new directory via init
nygard-agent-format.sh init new-decisions
echo "---"
# list should read from .adr/adr-dir (new-decisions), not .adr-dir (my-decisions)
nygard-agent-format.sh list
