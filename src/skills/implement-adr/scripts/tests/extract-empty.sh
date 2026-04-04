#!/bin/bash
# Test: extract summary from a plan with no implementation summary block
cat <<'PLAN' > plan.md
# Implementation Plan: Test Plan Without Summary

**Source ADRs:**
- [ADR-0099: Test decision](docs/adr/0099-test-decision.md)

**Generated:** 2026-01-15

---

## Summary

| Stage | Task | Cost | Dependencies |
|-------|------|------|--------------|
| 1. Setup | 1.1 Create config | small | — |

**Total estimated cost:** 1 small
PLAN
awk -f "$SCRIPT_DIR/extract-summary.awk" plan.md
