
include ../defaults.env
include ../.env
PATH := $(PWD)/../bin:$(PATH)
SHELL := env PATH=$(PATH) /bin/bash

source := $(shell find ./src -iname *.rs)

debug_target = target/debug/variables
release_target = target/release/variables

.PHONY: all
all: build run

.PHONY: list
list:
	@echo source
	@echo $(source)
	@echo debug_target
	@echo $(debug_target)
	@echo release_target
	@echo $(release_target)

.PHONY: clean
clean:
	$(CARGO) clean

.PHONY: build
build: $(debug_target)

.PHONY: run
run:
	$(CARGO) run

.PHONY: check
check: 
	$(CARGO) check

.PHONY: build_release
build_release: $(release_target)

.PHONY: run_release
run_release: $(release_target)
	$<

$(debug_target): $(source)
	$(CARGO) build

$(release_target): $(source)
	$(CARGO) build --release
