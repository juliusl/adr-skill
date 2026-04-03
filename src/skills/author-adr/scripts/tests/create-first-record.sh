#!/bin/bash
# Test: create first record after init
nygard-agent-format.sh init
new.sh nygard-agent "Use PostgreSQL for persistence"
echo "---"
cat docs/adr/0002-use-postgresql-for-persistence.md
