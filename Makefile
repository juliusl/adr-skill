# ADR Skills — Development Makefile
# Targets for maintaining and testing the skills in this repo.

AUTHOR_SKILL_DIR    := $(CURDIR)/src/skills/author-adr
IMPLEMENT_SKILL_DIR := $(CURDIR)/src/skills/implement-adr
PROTOTYPE_SKILL_DIR := $(CURDIR)/src/skills/prototype-adr
SOLVE_SKILL_DIR     := $(CURDIR)/src/skills/solve-adr

# Legacy alias so existing references keep working
SKILL_DIR := $(AUTHOR_SKILL_DIR)

.PHONY: help test build-tools install-agents install-skills validate-setup validate validate-all check-refs

help: ## Show available targets
	@echo "ADR Skill Development Makefile"
	@echo ""
	@echo "Targets:"
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "  %-15s %s\n", $$1, $$2}'

test: ## Run all script tests (author-adr + implement-adr)
	$(MAKE) -C $(SKILL_DIR)/scripts clean check
	$(MAKE) -C $(IMPLEMENT_SKILL_DIR)/scripts clean check

build-tools: ## Build Rust tooling (requires Rust toolchain)
	cargo build --release --manifest-path $(CURDIR)/src/crates/Cargo.toml

check-refs: ## Check for broken markdown references in all skills
	@$(CURDIR)/scripts/check-refs $(AUTHOR_SKILL_DIR) $(IMPLEMENT_SKILL_DIR) $(PROTOTYPE_SKILL_DIR) $(SOLVE_SKILL_DIR)

AGENTS_SRC_DIR := $(CURDIR)/src/agents

install-agents: ## Install custom agents (ADR_AGENTS_DIR overrides target)
	@if [ -n "$(ADR_AGENTS_DIR)" ]; then \
		target_dir="$(ADR_AGENTS_DIR)"; \
	else \
		target_dir="$(HOME)/.copilot/agents"; \
	fi; \
	mkdir -p "$$target_dir"; \
	found=0; \
	for agent_file in $(AGENTS_SRC_DIR)/*.agent.md; do \
		if [ -f "$$agent_file" ]; then \
			agent_basename=$$(basename "$$agent_file"); \
			echo "Installing: $$agent_basename -> $$target_dir/$$agent_basename"; \
			cp "$$agent_file" "$$target_dir/$$agent_basename"; \
			found=1; \
		fi; \
	done; \
	if [ "$$found" = "0" ]; then \
		echo "No .agent.md files found in src/agents/."; \
		exit 1; \
	fi; \
	echo "Done. Agents installed to $$target_dir/"

install-skills: ## Install all skills to ~/.copilot/skills
	@echo 'Installing author-adr, implement-adr, prototype-adr, and solve-adr to ~/.copilot/skills'
	@mkdir -p $(HOME)/.copilot/skills
	@rm -rf $(HOME)/.copilot/skills/author-adr
	cp -r $(AUTHOR_SKILL_DIR) $(HOME)/.copilot/skills/author-adr
	@echo "Installed to ~/.copilot/skills/author-adr"
	@rm -rf $(HOME)/.copilot/skills/implement-adr
	cp -r $(IMPLEMENT_SKILL_DIR) $(HOME)/.copilot/skills/implement-adr
	@echo "Installed to ~/.copilot/skills/implement-adr"
	@rm -rf $(HOME)/.copilot/skills/prototype-adr
	cp -r $(PROTOTYPE_SKILL_DIR) $(HOME)/.copilot/skills/prototype-adr
	@echo "Installed to ~/.copilot/skills/prototype-adr"
	@rm -rf $(HOME)/.copilot/skills/solve-adr
	cp -r $(SOLVE_SKILL_DIR) $(HOME)/.copilot/skills/solve-adr
	@echo "Installed to ~/.copilot/skills/solve-adr"

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

validate: ## Validate author-adr skill against agentskills.io spec
	@if [ ! -x "$(SKILLS_REF)" ]; then \
		echo "skills-ref not found. Run 'make validate-setup' first."; \
		exit 1; \
	fi
	$(SKILLS_REF) validate $(AUTHOR_SKILL_DIR)

validate-implement: ## Validate implement-adr skill against agentskills.io spec
	@if [ ! -x "$(SKILLS_REF)" ]; then \
		echo "skills-ref not found. Run 'make validate-setup' first."; \
		exit 1; \
	fi
	$(SKILLS_REF) validate $(IMPLEMENT_SKILL_DIR)

validate-prototype: ## Validate prototype-adr skill against agentskills.io spec
	@if [ ! -x "$(SKILLS_REF)" ]; then \
		echo "skills-ref not found. Run 'make validate-setup' first."; \
		exit 1; \
	fi
	$(SKILLS_REF) validate $(PROTOTYPE_SKILL_DIR)

validate-solve: ## Validate solve-adr skill against agentskills.io spec
	@if [ ! -x "$(SKILLS_REF)" ]; then \
		echo "skills-ref not found. Run 'make validate-setup' first."; \
		exit 1; \
	fi
	$(SKILLS_REF) validate $(SOLVE_SKILL_DIR)
	@lines=$$(wc -l < $(SOLVE_SKILL_DIR)/SKILL.md); \
	if [ "$$lines" -gt 500 ]; then \
		echo "ERROR: $(SOLVE_SKILL_DIR)/SKILL.md has $$lines lines (limit: 500)"; \
		exit 1; \
	else \
		echo "OK: SKILL.md line count $$lines/500"; \
	fi

validate-all: validate validate-implement validate-prototype validate-solve ## Validate all skills
