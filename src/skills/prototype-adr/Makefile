# prototype-adr Makefile
# Downstream agent interface for the prototype-adr skill

SKILL_ROOT := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
ADR_DIR := $(shell \
  if [ -f .adr/adr-dir ]; then cat .adr/adr-dir; \
  elif [ -f .adr-dir ]; then cat .adr-dir; \
  else echo docs/adr; fi)

.PHONY: help list-adrs scan-validation-needs show-profile-template

help:
	@echo "prototype-adr targets:"
	@echo "  list-adrs               List ADRs with status"
	@echo "  scan-validation-needs   Find ADRs with populated Validation needs"
	@echo "  show-profile-template   Display the default profile template"

list-adrs:
	@if [ -d "$(ADR_DIR)" ]; then \
		for f in $(ADR_DIR)/[0-9]*.md; do \
			[ -f "$$f" ] || continue; \
			title=$$(head -1 "$$f" | sed 's/^# *//'); \
			status=$$(grep -m1 '^Status:' "$$f" | sed 's/^Status: *//' || echo "unknown"); \
			printf "%s\t[%s]\n" "$$title" "$$status"; \
		done; \
	else \
		echo "No ADR directory found at $(ADR_DIR)"; \
	fi

scan-validation-needs:
	@if [ -d "$(ADR_DIR)" ]; then \
		for f in $(ADR_DIR)/[0-9]*.md; do \
			[ -f "$$f" ] || continue; \
			if grep -q 'Pause for validation' "$$f" 2>/dev/null; then \
				title=$$(head -1 "$$f" | sed 's/^# *//'); \
				echo "$$title — has validation needs"; \
			fi; \
		done; \
	else \
		echo "No ADR directory found at $(ADR_DIR)"; \
	fi

show-profile-template:
	@cat $(SKILL_ROOT)assets/templates/default-profile.toml
