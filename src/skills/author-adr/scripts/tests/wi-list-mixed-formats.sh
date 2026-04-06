#!/bin/bash
# Test: list ADRs with mixed sequential and work-item-prefixed formats
nygard-agent-format.sh init
new.sh nygard-agent "Sequential record"
new.sh wi-nygard-agent gh 42 "Work item record"
echo "---"
wi-nygard-agent-format.sh list
