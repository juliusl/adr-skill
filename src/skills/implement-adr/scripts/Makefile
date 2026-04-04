# Implement-ADR Scripts — Test Suite
# Pattern: tests/*.sh (commands) vs tests/*.expected (output)

SHELL := /bin/bash
SCRIPT_DIR := $(CURDIR)

TESTS := $(sort $(basename $(notdir $(wildcard tests/*.sh))))

.PHONY: check clean

check: $(TESTS:%=test-%)

test-%: tests/%.sh tests/%.expected
	@echo "Running: $*"
	@builddir=$$(mktemp -d /tmp/adr-test-XXXXXX); \
	trap 'rm -rf "$$builddir"' EXIT; \
	cd "$$builddir" && \
	SCRIPT_DIR="$(SCRIPT_DIR)" \
	bash "$(CURDIR)/tests/$*.sh" > actual.out 2>&1; \
	if diff -u "$(CURDIR)/tests/$*.expected" actual.out > diff.out 2>&1; then \
		echo "  PASS"; \
	else \
		echo "  FAIL"; \
		cat diff.out; \
		exit 1; \
	fi

clean:
	@echo "Nothing to clean"
