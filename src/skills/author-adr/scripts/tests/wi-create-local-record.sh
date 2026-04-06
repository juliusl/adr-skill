#!/bin/bash
# Test: create ADR with local remote via wi-nygard-agent format
nygard-agent-format.sh init
new.sh wi-nygard-agent local a1b2c3d4 "Use PostgreSQL for persistence"
echo "---"
cat docs/adr/local-a1b2c3d4-use-postgresql-for-persistence.md
