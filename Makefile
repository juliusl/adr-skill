# ADR Skill — Development Makefile
# Targets for maintaining and testing the skill itself.

SKILL_DIR := $(CURDIR)/author-adr

.PHONY: help test test-nygard test-madr install-agents dogfood-copilot validate-setup validate

help: ## Show available targets
	@echo "ADR Skill Development Makefile"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

test: test-nygard test-madr ## Run tests for both formats

test-nygard: ## Run adr-tools tests
	$(MAKE) -C $(SKILL_DIR)/scripts/adr-tools-3.0.0 clean check

test-madr: ## Run madr-tools tests
	$(MAKE) -C $(SKILL_DIR)/scripts/madr-tools clean check

install-agents: ## Install custom agents (ADR_AGENTS_DIR overrides target)
	$(MAKE) -C $(SKILL_DIR) install-agents $(if $(ADR_AGENTS_DIR),ADR_AGENTS_DIR=$(ADR_AGENTS_DIR))

dogfood-copilot: ## Install skill to ~/.copilot/skills for local testing
	@mkdir -p $(HOME)/.copilot/skills
	@rm -rf $(HOME)/.copilot/skills/author-adr
	cp -r $(SKILL_DIR) $(HOME)/.copilot/skills/author-adr
	@echo "Installed to ~/.copilot/skills/author-adr"

SKILLS_REF_DIR := /tmp/agentskills/skills-ref
SKILLS_REF := $(SKILLS_REF_DIR)/.venv/bin/skills-ref

validate-setup: ## Install skills-ref validator (one-time)
	@if [ -x "$(SKILLS_REF)" ]; then \
		echo "skills-ref already installed at $(SKILLS_REF)"; \
	else \
		echo "Cloning agentskills repo..."; \
		rm -rf /tmp/agentskills; \
		git clone --quiet https://github.com/agentskills/agentskills.git /tmp/agentskills; \
		echo "Installing skills-ref..."; \
		cd $(SKILLS_REF_DIR) && uv sync --quiet; \
		echo "Done. skills-ref installed."; \
	fi

validate: ## Validate skill against agentskills.io spec
	@if [ ! -x "$(SKILLS_REF)" ]; then \
		echo "skills-ref not found. Run 'make validate-setup' first."; \
		exit 1; \
	fi
	$(SKILLS_REF) validate $(SKILL_DIR)
