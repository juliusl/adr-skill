#!/bin/bash
# Test: list records showing title and status
nygard-agent-format.sh init
new.sh nygard-agent "Use PostgreSQL"
new.sh nygard-agent "Add Redis caching"
echo "---"
nygard-agent-format.sh list
