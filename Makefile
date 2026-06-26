# ============================================================
#  Time-Lock Vault — Developer Makefile
# ============================================================

WASM_TARGET  := wasm32-unknown-unknown
WASM_OUT     := target/wasm32-unknown-unknown/release/time_lock_vault.wasm
OPTIMIZED    := target/time_lock_vault.optimized.wasm

.PHONY: all build test fmt lint clean optimize deploy-testnet size check audit deny
.PHONY: all build test watch fmt lint clean optimize deploy-testnet size check doc smoke-test-local

## Default: lint + test
all: lint test

## Compile the contract to WASM
build:
	cargo build --target $(WASM_TARGET) --release

## Run all unit tests (native, no WASM needed)
test:
	cargo test --features testutils

## Auto-run tests on file changes (requires cargo-watch)
watch:
	cargo watch -x 'test --features testutils'

## Format all Rust source files
fmt:
	cargo fmt --all

## Check formatting without modifying files (used in CI)
fmt-check:
	cargo fmt --all -- --check

## Run Clippy linter (fail on warnings)
lint:
	cargo clippy --all-targets --features testutils -- -D warnings

## Run fmt-check + lint + test + audit + deny in sequence (mirrors CI)
check: fmt-check lint test audit deny

## Check dependencies for known security vulnerabilities
audit:
	cargo audit

## Check dependencies for license and ban policy compliance
deny:
	cargo deny check

## Generate and open Rust API docs
doc:
	cargo doc --no-deps --open

## Remove build artifacts
clean:
	cargo clean

## Optimize WASM binary with soroban CLI
optimize: build
	soroban contract optimize --wasm $(WASM_OUT) --wasm-out $(OPTIMIZED)
	@echo "Optimized WASM: $(OPTIMIZED)"
	@ls -lh $(OPTIMIZED)

## Deploy to Stellar Testnet (requires SOROBAN_SECRET_KEY env var)
deploy-testnet: optimize
	bash scripts/deploy_testnet.sh

## Show raw WASM size
size: build
	@ls -lh $(WASM_OUT)

## Fail if optimized WASM exceeds MAX_WASM_BYTES (default 65536 = 64 KB)
MAX_WASM_BYTES ?= 65536
check-wasm-size: optimize
	@ACTUAL=$$(wc -c < $(OPTIMIZED)); \
	echo "Optimized WASM size: $${ACTUAL} bytes (limit: $(MAX_WASM_BYTES))"; \
	if [ "$$ACTUAL" -gt "$(MAX_WASM_BYTES)" ]; then \
		echo "ERROR: WASM too large: $${ACTUAL} bytes exceeds limit of $(MAX_WASM_BYTES) bytes"; \
		exit 1; \
	fi

## Run smoke tests against a local Soroban standalone node (requires stellar CLI)
smoke-test-local: build
	bash scripts/smoke_test_local.sh
