test:
	env XDG_CONFIG_HOME=$(CURDIR)/test venv/bin/fbs run
.PHONY: test

run:
	venv/bin/fbs run
.PHONY: run

build:
	scripts/build.sh
.PHONY: build
