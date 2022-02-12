
include ../defaults.env
include ../.env
PATH := $(PWD)/../bin:$(PATH)
SHELL := env PATH=$(PATH) /bin/bash

.PHONY: all
all: build run check run_release

.PHONY: clean
clean:
	@echo cleaning project
	$(CARGO) clean

.PHONY: build
build: 
	make clean
	@echo building project
	$(CARGO) build
	@echo note output
	ls target/debug/hello_cargo

# builds and runs in one step
.PHONY: run
run: build
	make clean
	@echo building + running in one step
	$(CARGO) run
	@echo note output
	ls target/debug/hello_cargo

# Checks src would compile but dosen't output executable
.PHONY: check
check: 
	make clean
	@echo checking code would compile
	$(CARGO) check
	@echo note no output
	ls target/debug/hello_cargo || true
	

# Build a release version of the executable, which includes optimiations
.PHONY: build_release
build_release:
	make clean
	@echo building project in release mode
	$(CARGO) build --release

# Run the release build
.PHONY: run_release
run_release: build_release
	@echo running release target
	target/release/hello_cargo
