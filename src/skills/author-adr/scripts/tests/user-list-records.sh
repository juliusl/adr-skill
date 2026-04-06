#!/bin/bash
# Test: user-mode list shows only user's ADRs
mkdir -p .adr
printf "var/\nusr/\n" > .adr/.gitignore

# Create user-mode ADRs
ADR_SCOPE=user ADR_USERNAME=alice new.sh nygard-agent "Alice idea" > /dev/null
ADR_SCOPE=user ADR_USERNAME=bob new.sh nygard-agent "Bob idea" > /dev/null

echo "--- alice list ---"
ADR_SCOPE=user ADR_USERNAME=alice nygard-agent-format.sh list
echo "--- bob list ---"
ADR_SCOPE=user ADR_USERNAME=bob nygard-agent-format.sh list
