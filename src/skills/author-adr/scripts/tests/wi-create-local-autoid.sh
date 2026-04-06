#!/bin/bash
# Test: create ADR with local remote and auto-generated ID
nygard-agent-format.sh init
ADR_LOCAL_ID=deadbeef new.sh wi-nygard-agent local "Use Redis for caching"
echo "---"
cat docs/adr/local-deadbeef-use-redis-for-caching.md
