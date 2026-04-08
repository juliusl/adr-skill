#!/bin/bash
# Test: rename a work-item-prefixed ADR
export ADR_TEST_ADAPTER=gh
nygard-agent-format.sh init
new.sh wi-nygard-agent gh 42 "Use PostgreSQL"
echo "---"
wi-nygard-agent-format.sh rename gh 42 "Use MySQL instead"
echo "---"
head -1 docs/adr/gh-42-use-mysql-instead.md
