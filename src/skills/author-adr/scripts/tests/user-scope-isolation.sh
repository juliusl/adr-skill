#!/bin/bash
# Test: user-mode and project-mode don't interfere
mkdir -p docs/adr .adr
printf "var/\nusr/\n" > .adr/.gitignore

# Create project-mode ADR
new.sh nygard-agent "Project decision" > /dev/null

# Create user-mode ADR
ADR_SCOPE=user ADR_USERNAME=testuser new.sh nygard-agent "Personal draft" > /dev/null

echo "--- project list ---"
nygard-agent-format.sh list
echo "--- user list ---"
ADR_SCOPE=user ADR_USERNAME=testuser nygard-agent-format.sh list
echo "--- project dir ---"
ls docs/adr/
echo "--- user dir ---"
ls .adr/usr/docs/adr/
