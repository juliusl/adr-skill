# ADR Skill — Development Makefile
# Targets for maintaining and testing the skill itself.

SKILL_DIR := $(CURDIR)/architectural-decision-records

.PHONY: help test test-nygard test-madr install-agents dogfood-copilot

help: ## Show available targets
	@echo "ADR Skill Development Makefile"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

test: test-nygard test-madr ## Run tests for both runtimes

test-nygard: ## Run adr-tools tests
	$(MAKE) -C $(SKILL_DIR)/scripts/adr-tools-3.0.0 clean check

test-madr: ## Run madr-tools tests
	$(MAKE) -C $(SKILL_DIR)/scripts/madr-tools clean check

install-agents: ## Install custom agents (ADR_AGENTS_DIR overrides target)
	$(MAKE) -C $(SKILL_DIR) install-agents $(if $(ADR_AGENTS_DIR),ADR_AGENTS_DIR=$(ADR_AGENTS_DIR))

dogfood-copilot: ## Install skill to .copilot/skills for local testing
	@mkdir -p .copilot/skills
	@rm -rf .copilot/skills/architectural-decision-records
	cp -r $(SKILL_DIR) .copilot/skills/architectural-decision-records
	@echo "Installed to .copilot/skills/architectural-decision-records"
