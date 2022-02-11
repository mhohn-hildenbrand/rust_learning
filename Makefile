
include defaults.env
include .env
PATH := $(PWD)/bin:$(PATH)
SHELL := env PATH=$(PATH) /bin/bash

.PHONY: all
all: status helloWorld/

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

.env:
	touch $@


