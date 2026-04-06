#!/bin/bash
# Test: ADR_USERNAME with path traversal characters is rejected
mkdir -p .adr
printf "var/\nusr/\n" > .adr/.gitignore

echo "--- path traversal ---"
ADR_SCOPE=user ADR_USERNAME='../etc' new.sh nygard-agent "Evil ADR" 2>&1 || true
echo "--- special chars ---"
ADR_SCOPE=user ADR_USERNAME='user;rm' new.sh nygard-agent "Evil ADR" 2>&1 || true
echo "--- valid username ---"
ADR_SCOPE=user ADR_USERNAME='valid-user_1' new.sh nygard-agent "Safe ADR"
