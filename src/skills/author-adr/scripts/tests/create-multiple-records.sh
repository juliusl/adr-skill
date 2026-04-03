#!/bin/bash
# Test: create multiple records and verify numbering
nygard-agent-format.sh init
new.sh nygard-agent "Use PostgreSQL"
new.sh nygard-agent "Add Redis caching"
new.sh nygard-agent "Deploy to Kubernetes"
echo "---"
ls docs/adr/
