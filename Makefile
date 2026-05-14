# ============================================================
#  Time-Lock Vault — Developer Makefile
# ============================================================

WASM_TARGET  := wasm32-unknown-unknown
WASM_OUT     := target/wasm32-unknown-unknown/release/time_lock_vault.wasm
OPTIMIZED    := target/time_lock_vault.optimized.wasm

.PHONY: all build test fmt lint clean optimize deploy-testnet size check

## Default: lint + test
all: lint test

## Compile the contract to WASM
build:
	cargo build --target $(WASM_TARGET) --release

## Run all unit tests (native, no WASM needed)
test:
	cargo test --features testutils

## Format all Rust source files
fmt:
	cargo fmt --all

## Check formatting without modifying files (used in CI)
fmt-check:
	cargo fmt --all -- --check

## Run Clippy linter (fail on warnings)
lint:
	cargo clippy --all-targets --features testutils -- -D warnings

## Run fmt-check + lint + test in sequence (mirrors CI)
check: fmt-check lint test

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
