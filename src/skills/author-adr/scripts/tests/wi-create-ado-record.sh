#!/bin/bash
# Test: create ADR with ado remote via wi-nygard-agent format
export ADR_TEST_ADAPTER=ado
nygard-agent-format.sh init
new.sh wi-nygard-agent ado 1234 "Use PostgreSQL for persistence"
echo "---"
cat docs/adr/ado-1234-use-postgresql-for-persistence.md
