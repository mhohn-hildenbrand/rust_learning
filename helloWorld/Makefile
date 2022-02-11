
include ../defaults.env
include ../.env
PATH := $(PWD)/../bin:$(PATH)
SHELL := env PATH=$(PATH) /bin/bash

.PHONY: all
all: test

.PHONY: clean
clean:
	rm -fr dist/

.PHONY: test
test: dist/helloWorld
	./dist/helloWorld

dist/:
	mkdir -p $@

dist/helloWorld: main.rs | dist/
	$(RUSTC) -o $@ $<
