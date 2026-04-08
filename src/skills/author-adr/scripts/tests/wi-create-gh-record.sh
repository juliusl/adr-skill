#!/bin/bash
# Test: create ADR with gh remote via wi-nygard-agent format
export ADR_TEST_ADAPTER=gh
nygard-agent-format.sh init
new.sh wi-nygard-agent gh 42 "Use PostgreSQL for persistence"
echo "---"
cat docs/adr/gh-42-use-postgresql-for-persistence.md
