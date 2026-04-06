# ADR Skill Makefile
# Unified script interface for ADR management.
#
# Set ADR_AGENT_SKILL_FORMAT to choose the ADR format:
#   make init ADR_AGENT_SKILL_FORMAT=nygard-agent   (default)
#
# Or export it:
#   export ADR_AGENT_SKILL_FORMAT=nygard-agent
#   make new TITLE="Use PostgreSQL"

SKILL_ROOT := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
ADR_AGENT_SKILL_FORMAT ?= nygard-agent

# Runtime paths — unified scripts
SCRIPTS_DIR    := $(SKILL_ROOT)scripts
FORMAT_SCRIPT  := $(SCRIPTS_DIR)/$(ADR_AGENT_SKILL_FORMAT)-format.sh
ORCHESTRATOR   := $(SCRIPTS_DIR)/new.sh

# --- Targets ---

.PHONY: help init init-data new rename list status install-agents

help: ## Show available targets
	@echo "ADR Skill Makefile (format: $(ADR_AGENT_SKILL_FORMAT))"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'
	@echo ""
	@echo "Set ADR_AGENT_SKILL_FORMAT to switch formats (default: nygard-agent)."

init: ## Initialize ADR directory (DIR=path)
	$(FORMAT_SCRIPT) init $(DIR)

init-data: ## Bootstrap .adr/ project-scoped directory
	@mkdir -p .adr/var
	@if [ ! -f .adr/.gitignore ]; then printf "var/\nusr/\n" > .adr/.gitignore; fi
	@echo "Initialized .adr/ project-scoped directory"

new: ## Create a new ADR (TITLE="..." or REMOTE=local TITLE="..." or REMOTE=gh ID=42 TITLE="...")
ifeq ($(ADR_AGENT_SKILL_FORMAT),wi-nygard-agent)
	$(ORCHESTRATOR) $(ADR_AGENT_SKILL_FORMAT) $(REMOTE) $(ID) $(TITLE)
else
	$(ORCHESTRATOR) $(ADR_AGENT_SKILL_FORMAT) $(TITLE)
endif

rename: ## Rename an ADR (NUM=N or REMOTE=gh ID=42 TITLE="New Title")
ifeq ($(ADR_AGENT_SKILL_FORMAT),wi-nygard-agent)
	$(FORMAT_SCRIPT) rename $(REMOTE) $(ID) $(TITLE)
else
	$(FORMAT_SCRIPT) rename $(NUM) $(TITLE)
endif

list: ## List all ADRs
	$(FORMAT_SCRIPT) list

status: ## Show or update ADR status (NUM=N or REMOTE=gh ID=42 [STATUS=Accepted])
ifeq ($(ADR_AGENT_SKILL_FORMAT),wi-nygard-agent)
	$(FORMAT_SCRIPT) status $(REMOTE) $(ID) $(STATUS)
else
	$(FORMAT_SCRIPT) status $(NUM) $(STATUS)
endif

install-agents: ## Install custom agents to the agents/ directory (ADR_AGENTS_DIR overrides target)
	@agents_src="$(SKILL_ROOT)assets"; \
	if [ -n "$(ADR_AGENTS_DIR)" ]; then \
		target_dir="$(ADR_AGENTS_DIR)"; \
	else \
		target_dir="$(abspath $(SKILL_ROOT)../../agents)"; \
	fi; \
	mkdir -p "$$target_dir"; \
	found=0; \
	for agent_file in "$$agents_src"/*.agent.md; do \
		if [ -f "$$agent_file" ]; then \
			agent_basename=$$(basename "$$agent_file"); \
			echo "Installing: $$agent_basename -> $$target_dir/$$agent_basename"; \
			cp "$$agent_file" "$$target_dir/$$agent_basename"; \
			found=1; \
		fi; \
	done; \
	if [ "$$found" = "0" ]; then \
		echo "No .agent.md files found in assets/."; \
		exit 1; \
	fi; \
	echo "Done. Agents installed to $$target_dir/"
