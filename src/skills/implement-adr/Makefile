# Implement-ADR Skill Makefile
# Provides targets for the implement-adr agent workflow.
#
# Usage:
#   make -f <skill-root>/Makefile list-adrs
#   make -f <skill-root>/Makefile show-template
#   make -f <skill-root>/Makefile help

SKILL_ROOT := $(dir $(abspath $(lastword $(MAKEFILE_LIST))))
ADR_DIR    ?= docs/adr

.PHONY: help list-adrs list-plans show-template install-agents extract-summary review-plan

help: ## Show available targets
	@echo "Implement-ADR Skill Makefile"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-18s %s\n", $$1, $$2}'
	@echo ""
	@echo "Variables:"
	@echo "  ADR_DIR=path      ADR directory (default: docs/adr)"

list-adrs: ## List all ADRs in the repository
	@if [ -d "$(ADR_DIR)" ]; then \
		for f in "$(ADR_DIR)"/*.md; do \
			[ -f "$$f" ] || continue; \
			num=$$(basename "$$f" | grep -oE '^[0-9]+'); \
			title=$$(head -1 "$$f" | sed 's/^#\+\s*//; s/^[0-9]*\.\s*//'); \
			status=$$(grep -A2 '## Status' "$$f" | tail -1 | sed 's/^[[:space:]]*//'); \
			printf "  ADR-%s: %-50s [%s]\n" "$$num" "$$title" "$$status"; \
		done; \
	else \
		echo "No ADR directory found at $(ADR_DIR)."; \
		echo "Use the author-adr skill to create decision records first."; \
		exit 1; \
	fi

show-template: ## Display the plan.md template
	@cat "$(SKILL_ROOT)assets/templates/plan-template.md"

list-plans: ## List implementation plans in docs/plans/
	@if [ -d docs/plans ]; then \
		found=0; \
		for f in docs/plans/*.plan.md; do \
			[ -f "$$f" ] || continue; \
			basename "$$f"; \
			found=1; \
		done; \
		if [ "$$found" = "0" ]; then \
			echo "No plans found in docs/plans/"; \
		fi; \
	else \
		echo "No docs/plans/ directory found."; \
	fi

install-agents: ## Install custom agents to the agents/ directory (ADR_AGENTS_DIR overrides target)
	@agents_src="$(SKILL_ROOT)assets"; \
	if [ -n "$(ADR_AGENTS_DIR)" ]; then \
		target_dir="$(ADR_AGENTS_DIR)"; \
	else \
		target_dir="$(abspath $(SKILL_ROOT)../agents)"; \
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
	fi; \
	echo "Done."

extract-summary: ## Extract implementation summary as JSONL from a plan file (PLAN_FILE=path)
	@if [ -z "$(PLAN_FILE)" ]; then \
		echo "Usage: make extract-summary PLAN_FILE=docs/plans/NNNN.0.plan.md"; \
		exit 1; \
	fi
	@awk -f "$(SKILL_ROOT)scripts/extract-summary.awk" "$(PLAN_FILE)"

review-plan: ## Display the plan-review protocol reference
	@cat "$(SKILL_ROOT)references/plan-review.md"
