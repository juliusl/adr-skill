TESTS:=$(wildcard tests/*.sh)
SRC:=$(wildcard src/*)
SKILL_ROOT:=$(abspath ../..)

BUILDDIR:=/tmp/madr-tools-build

check: $(TESTS:tests/%.sh=$(BUILDDIR)/tests/%.diff)
	@echo SUCCESS

$(BUILDDIR)/tests/%.diff: $(BUILDDIR)/tests/%.output tests/%.expected
	@diff --side-by-side $^ > $@ || ! cat $@

$(BUILDDIR)/tests/%.output: tests/%.sh tests/%.expected $(SRC)
	@echo TEST: $*
	@rm -rf $(dir $@)/$*
	@mkdir -p $(dir $@)/$*
	@(cd $(dir $@)/$* && \
	  PATH=/bin:/usr/bin:$(abspath src) \
	  MADR_DATE=2025-01-15 \
	  VISUAL=true \
	  EDITOR= \
	  PS4='+ ' \
	    /bin/bash -v $(abspath $<) > $(abspath $@) 2>&1) || ! cat $@

clean:
	rm -rf /tmp/madr-tools-build

.PHONY: all clean check
.PRECIOUS: $(BUILDDIR)/tests/%.output
.DELETE_ON_ERROR:
