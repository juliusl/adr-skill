#!/bin/bash
# Test: rename a TBD ADR to a final title
nygard-agent-format.sh init
new.sh nygard-agent tbd
echo "--- before rename ---"
ls docs/adr/
echo "--- rename ---"
nygard-agent-format.sh rename 2 "Use PostgreSQL for persistence"
echo "--- after rename ---"
ls docs/adr/
echo "--- content ---"
cat docs/adr/0002-use-postgresql-for-persistence.md
echo "--- no-op rename ---"
nygard-agent-format.sh rename 2 "Use PostgreSQL for persistence"
echo "--- error: not found ---"
nygard-agent-format.sh rename 99 "Nonexistent" 2>&1 || true
