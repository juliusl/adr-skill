#!/bin/bash
# Test: show status of a specific ADR
nygard-agent-format.sh init
new.sh nygard-agent "Use PostgreSQL"
echo "---"
nygard-agent-format.sh status 1
echo "---"
nygard-agent-format.sh status 2
