.ONESHELL:
# @see http://marmelab.com/blog/2016/02/29/auto-documented-makefile.html
.DEFAULT_GOAL := help
SHELL := /bin/bash
DIR:=$(strip $(shell dirname $(realpath $(lastword $(MAKEFILE_LIST)))))

export DATABASE_URL := postgres://user:password@localhost:5432/rust-sample

.PHONY: help
help: ## provides cli help for this makefile (default) 📖
	@grep -E '^[a-zA-Z_0-9-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: build
build: ## build the web app
	cargo build


.PHONY: clean
clean: ## clean all compiled code ✓
	cargo clean

.PHONY: start
start: ## start the web app ✓
	$(DIR)/target/debug/ssr_rust.exe
