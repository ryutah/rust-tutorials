.PHONY: all

CURDIR := $(shell pwd)

help: ## Print this help
	@echo 'Usage: make [target]'
	@echo ''
	@echo 'Targets:'
	@awk 'BEGIN {FS = ":.*?## "} /^[a-zA-Z_-]+:.*?## / {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}' $(MAKEFILE_LIST)

build: ## Build wasm
	cargo build --target wasm32-unknown-unknown --release
	wasm-gc target/wasm32-unknown-unknown/release/wasmhelloworld.wasm -o hello_world.gc.wasm

serve: ## Start local server
	python -m http.server
