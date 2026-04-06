#!/bin/bash
# Test: user-mode creates ADR in .adr/usr/docs/adr/ with username prefix
mkdir -p .adr
printf "var/\nusr/\n" > .adr/.gitignore

ADR_SCOPE=user ADR_USERNAME=testuser new.sh nygard-agent "Explore caching"
echo "---"
ls .adr/usr/docs/adr/
echo "---"
head -1 .adr/usr/docs/adr/testuser-0001-explore-caching.md
echo "---"
# Second record increments
ADR_SCOPE=user ADR_USERNAME=testuser new.sh nygard-agent "Evaluate framework"
ls .adr/usr/docs/adr/
