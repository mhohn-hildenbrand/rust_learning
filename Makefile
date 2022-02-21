
include defaults.env
include .env
PATH := $(PWD)/bin:$(PATH)
SHELL := env PATH=$(PATH) /bin/bash

.PHONY: all
all: status helloWorld/ \
     hello_cargo/ \
     guessing_game/ \
		 variables/ \
     branches/ \
		 ownership/

.PHONY: clean
clean:
	make -C helloWorld/ clean

.PHONY: status
status:
	git status

.PHONY: helloWorld/
helloWorld/:
	@echo Running helloWorld/
	make -C $@

.PHONY: hello_cargo/
hello_cargo/:
	@echo Running hello_cargo/
	make -C $@

.PHONY: guessing_game/
guessing_game/:
	@echo Running hello_cargo/
	make -C $@

.PHONY: variables/
variables/:
	@echo Running variables/
	make -C $@

.PHONY: branches/
branches/:
	@echo Running branches/
	make -C $@

.PHONY: ownership/
ownership/:
	@echo Running ownership/
	make -C $@

.env:
	touch $@


