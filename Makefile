
include defaults.env
include .env

.PHONY: all
all: status

.PHONY: status
status:
	git status

.env:
	touch $@

