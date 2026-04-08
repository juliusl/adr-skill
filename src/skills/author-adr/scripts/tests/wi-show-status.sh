#!/bin/bash
# Test: show and update status for work-item-prefixed ADR
export ADR_TEST_ADAPTER=gh
nygard-agent-format.sh init
new.sh wi-nygard-agent gh 42 "Use PostgreSQL"
echo "---"
wi-nygard-agent-format.sh status gh 42
echo "---"
wi-nygard-agent-format.sh status gh 42 Accepted
wi-nygard-agent-format.sh status gh 42
