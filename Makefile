# ============================================================
#  Time-Lock Vault — Developer Makefile
# ============================================================

WASM_TARGET  := wasm32-unknown-unknown
WASM_OUT     := target/wasm32-unknown-unknown/release/time_lock_vault.wasm
OPTIMIZED    := target/time_lock_vault.optimized.wasm

.PHONY: all build test fmt fmt-check lint check audit deny doc clean optimize deploy-testnet size check-wasm-size smoke-test-local help
.PHONY: all build test fmt fmt-fix lint clean optimize deploy-testnet size check audit deny
.PHONY: all build test fmt fmt-fix lint clean optimize deploy-testnet size check doc smoke-test-local
.PHONY: all build test fmt lint clean optimize deploy-testnet size check audit deny
.PHONY: all build test fmt lint clean optimize deploy-testnet size check doc smoke-test-local install-tools

all: lint test ## Default: lint + test

help: ## Show this help message
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'

build: ## Compile the contract to WASM
	cargo build --target $(WASM_TARGET) --release

test: ## Run all unit tests (native, no WASM needed)
	cargo test --features testutils

fmt: ## Format all Rust source files
	cargo fmt --all

fmt-check: ## Check formatting without modifying files (used in CI)
## Format all Rust source files
fmt-fix:
	cargo fmt --all

## Backwards-compat alias for fmt-fix
fmt: fmt-fix

## Check formatting without modifying files (used in CI)
fmt-check:
	cargo fmt --all -- --check

lint: ## Run Clippy linter (fail on warnings)
	cargo clippy --all-targets --features testutils -- -D warnings

check: fmt-check lint test audit deny ## Run fmt-check + lint + test + audit + deny in sequence (mirrors CI)

audit: ## Check dependencies for known security vulnerabilities
	cargo audit

deny: ## Check dependencies for license and ban policy compliance
	cargo deny check

doc: ## Generate and open Rust API docs
	cargo doc --no-deps --open

clean: ## Remove build artifacts
	cargo clean

optimize: build ## Optimize WASM binary with soroban CLI
	soroban contract optimize --wasm $(WASM_OUT) --wasm-out $(OPTIMIZED)
	@echo "Optimized WASM: $(OPTIMIZED)"
	@ls -lh $(OPTIMIZED)

deploy-testnet: optimize ## Deploy to Stellar Testnet (requires SOROBAN_SECRET_KEY env var)
	bash scripts/deploy_testnet.sh

size: build ## Show raw WASM size
	@ls -lh $(WASM_OUT)

MAX_WASM_BYTES ?= 65536
check-wasm-size: optimize ## Fail if optimized WASM exceeds MAX_WASM_BYTES (default 65536 = 64 KB)
	@ACTUAL=$$(wc -c < $(OPTIMIZED)); \
	echo "Optimized WASM size: $${ACTUAL} bytes (limit: $(MAX_WASM_BYTES))"; \
	if [ "$$ACTUAL" -gt "$(MAX_WASM_BYTES)" ]; then \
		echo "ERROR: WASM too large: $${ACTUAL} bytes exceeds limit of $(MAX_WASM_BYTES) bytes"; \
		exit 1; \
	fi

smoke-test-local: build ## Run smoke tests against a local Soroban standalone node (requires stellar CLI)
	bash scripts/smoke_test_local.sh

## Install all required dev tools (stellar-cli, cargo-watch, cargo-audit, cargo-deny)
install-tools:
	cargo install --locked stellar-cli
	cargo install cargo-watch
	cargo install cargo-audit
	cargo install cargo-deny