# ADR Skill Makefile
# Abstracts adr-tools and madr-tools behind a unified interface.
#
# Set ADR_AGENT_SKILL_FORMAT to choose the ADR format:
#   make init ADR_AGENT_SKILL_FORMAT=nygard   (default)
#   make init ADR_AGENT_SKILL_FORMAT=madr
#
# Or export it:
#   export ADR_AGENT_SKILL_FORMAT=madr
#   make new TITLE="Use PostgreSQL"

SKILL_ROOT := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
ADR_AGENT_SKILL_FORMAT ?= nygard

# Runtime paths
NYGARD_BIN := $(SKILL_ROOT)scripts/adr-tools-3.0.0/src
MADR_BIN   := $(SKILL_ROOT)scripts/madr-tools/src

ifeq ($(ADR_AGENT_SKILL_FORMAT),madr)
  ADR_CMD := $(MADR_BIN)/madr
else
  ADR_CMD := $(NYGARD_BIN)/adr
endif

# --- Targets ---

.PHONY: help init init-meta new list status generate link check-format install-agents

help: ## Show available targets
	@echo "ADR Skill Makefile (format: $(ADR_AGENT_SKILL_FORMAT))"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'
	@echo ""
	@echo "Set ADR_AGENT_SKILL_FORMAT=nygard|madr to switch formats."

init: check-format ## Initialize ADR directory (DIR=path)
ifeq ($(ADR_AGENT_SKILL_FORMAT),madr)
	$(ADR_CMD) init $(DIR)
else
	$(ADR_CMD) init $(DIR)
endif
	@$(MAKE) -f $(lastword $(MAKEFILE_LIST)) init-meta

init-meta: ## Create .meta/ directory for behavioral policies
	@adr_dir=$$(cat .adr-dir 2>/dev/null || echo "docs/adr"); \
	if [ -d "$$adr_dir" ]; then \
		mkdir -p "$$adr_dir/.meta"; \
		echo "Created $$adr_dir/.meta/ for behavioral policies."; \
	else \
		echo "ADR directory $$adr_dir does not exist. Run 'make init' first."; \
		exit 1; \
	fi

new: check-format ## Create a new ADR (TITLE="..." [SUPERSEDE=N] [TEMPLATE=minimal|full])
ifeq ($(ADR_AGENT_SKILL_FORMAT),madr)
	$(ADR_CMD) new $(if $(SUPERSEDE),-s $(SUPERSEDE)) $(if $(TEMPLATE),-t $(TEMPLATE)) $(TITLE)
else
	$(ADR_CMD) new $(if $(SUPERSEDE),-s $(SUPERSEDE)) $(TITLE)
endif

list: check-format ## List all ADRs
	$(ADR_CMD) list

status: check-format ## Show or update ADR status (NUM=N [STATUS=Accepted])
ifeq ($(ADR_AGENT_SKILL_FORMAT),madr)
	$(ADR_CMD) status $(NUM) $(STATUS)
else
	@echo "Status management is manual in Nygard format. Edit the ADR file directly."
endif

generate: check-format ## Generate table of contents
ifeq ($(ADR_AGENT_SKILL_FORMAT),madr)
	$(ADR_CMD) generate toc $(if $(PREFIX),-p $(PREFIX))
else
	$(ADR_CMD) generate toc $(if $(PREFIX),-p $(PREFIX))
endif

link: check-format ## Link two ADRs (SOURCE=N LINK="text" TARGET=N REVERSE="text") [nygard only]
ifeq ($(ADR_AGENT_SKILL_FORMAT),madr)
	@echo "Use 'make new SUPERSEDE=N' for MADR supersedes relationships."
	@echo "Manual linking: edit the ADR files directly."
else
	$(ADR_CMD) link $(SOURCE) "$(LINK)" $(TARGET) "$(REVERSE)"
endif

check-format:
	@if [ "$(ADR_AGENT_SKILL_FORMAT)" != "nygard" ] && [ "$(ADR_AGENT_SKILL_FORMAT)" != "madr" ]; then \
		echo "ERROR: ADR_AGENT_SKILL_FORMAT must be 'nygard' or 'madr' (got '$(ADR_AGENT_SKILL_FORMAT)')"; \
		exit 1; \
	fi

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
