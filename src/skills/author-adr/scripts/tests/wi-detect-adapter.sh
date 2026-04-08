#!/bin/bash
# Test: detect_adapter URL→adapter mapping
# Tests the detection functions by calling the format script through its
# subcommands with ADR_TEST_ADAPTER to verify the seam works, then tests
# extract_host and detect_adapter_from_url inline.

# --- Test extract_host (inline copy for unit testing) ---
extract_host() {
  local url="$1"
  if printf '%s' "$url" | grep -q '^git@'; then
    printf '%s' "$url" | sed 's/^git@//; s/:.*//' | tr '[:upper:]' '[:lower:]'
    return
  fi
  printf '%s' "$url" | sed 's|.*://||; s|.*@||; s|/.*||; s|:.*||' | tr '[:upper:]' '[:lower:]'
}

detect_adapter_from_url() {
  local url="$1"
  local host
  host=$(extract_host "$url")
  case "$host" in
    *github.com*) printf 'gh' ;;
    *dev.azure.com*|*visualstudio.com*) printf 'ado' ;;
    *)
      echo "ERROR: Could not detect adapter type for URL '$url'. Supported hosts: github.com, dev.azure.com." >&2
      return 1
      ;;
  esac
}

echo "=== extract_host ==="
echo "SSH: $(extract_host 'git@github.com:org/repo.git')"
echo "HTTPS: $(extract_host 'https://github.com/org/repo')"
echo "SSH-scheme: $(extract_host 'ssh://git@github.com/org/repo')"
echo "ADO: $(extract_host 'https://dev.azure.com/org/project/_git/repo')"
echo "Case: $(extract_host 'https://GitHub.COM:443/org/repo')"

echo "=== detect_adapter_from_url ==="
echo "GitHub SSH: $(detect_adapter_from_url 'git@github.com:org/repo.git')"
echo "GitHub HTTPS: $(detect_adapter_from_url 'https://github.com/org/repo')"
echo "ADO HTTPS: $(detect_adapter_from_url 'https://dev.azure.com/org/project/_git/repo')"
echo "ADO SSH: $(detect_adapter_from_url 'git@ssh.dev.azure.com:v3/org/project/repo')"
echo "ADO VS: $(detect_adapter_from_url 'https://org.visualstudio.com/project/_git/repo')"
echo "GitHub ssh://: $(detect_adapter_from_url 'ssh://git@github.com/org/repo')"

echo "=== unknown host ==="
detect_adapter_from_url "https://gitlab.example.com/org/repo" 2>&1 || true

echo "=== ADR_TEST_ADAPTER seam ==="
export ADR_TEST_ADAPTER=gh
nygard-agent-format.sh init
new.sh wi-nygard-agent testremote 99 "Seam test"
echo "File created: $(ls docs/adr/gh-99-seam-test.md 2>/dev/null && echo yes || echo no)"
