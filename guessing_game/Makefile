
include ../defaults.env
include ../.env
PATH := $(PWD)/../bin:$(PATH)
SHELL := env PATH=$(PATH) /bin/bash

.PHONY: all
all: doc

.PHONY: clean
clean:
	$(CARGO) clean

# Excluded from 'all' because requesting user input is blocking
.PHONY: run
run:
	@echo Note that builds, which is implicitly in run, will install dependencies.
	$(CARGO) run

.PHONY: update
update:
	@echo Updating dependencies listed in Cargo.toml
	$(CARGO) update

.PHONY: doc
doc:
	@echo Generate documentation for source and dependency packages
	$(CARGO) doc
	@echo open file://$(PWD)/target/doc/guessing_game/index.html to review	
