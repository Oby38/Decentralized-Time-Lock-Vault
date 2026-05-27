#!/usr/bin/env bash
# Usage: bash scripts/smoke_test_local.sh
#
# Smoke-tests the Time-Lock Vault contract against a local Soroban
# standalone node (stellar network start local).
#
# Prerequisites:
#   - stellar CLI installed (https://developers.stellar.org/docs/tools/developer-tools/cli/install-cli)
#   - Contract WASM built: make build
#
# The script:
#   1. Starts a local Soroban node
#   2. Generates a funded test identity
#   3. Deploys the contract
#   4. Calls initialize, deposit, get_vault, time_remaining, withdraw
#   5. Asserts expected outputs
#   6. Stops the local node

set -euo pipefail

WASM="target/wasm32-unknown-unknown/release/time_lock_vault.wasm"
NETWORK="local"
IDENTITY="smoke-test-user"

# ── helpers ──────────────────────────────────────────────────────────────────

pass() { echo "  ✓ $*"; }
fail() { echo "  ✗ $*" >&2; exit 1; }

assert_contains() {
    local label="$1" expected="$2" actual="$3"
    if echo "$actual" | grep -qF "$expected"; then
        pass "$label"
    else
        fail "$label — expected to contain '$expected', got: $actual"
    fi
}

# ── 1. Build check ────────────────────────────────────────────────────────────

echo "==> Checking WASM..."
[ -f "$WASM" ] || { echo "WASM not found. Run 'make build' first."; exit 1; }
pass "WASM found: $WASM"

# ── 2. Start local node ───────────────────────────────────────────────────────

echo "==> Starting local Soroban node..."
stellar network start "$NETWORK" --background 2>/dev/null || true
sleep 2
pass "Local node started"

cleanup() {
    echo "==> Stopping local node..."
    stellar network stop "$NETWORK" 2>/dev/null || true
}
trap cleanup EXIT

# ── 3. Identity & funding ─────────────────────────────────────────────────────

echo "==> Setting up identity..."
stellar keys generate "$IDENTITY" --network "$NETWORK" --fund 2>/dev/null || true
ADMIN_ADDR=$(stellar keys address "$IDENTITY")
pass "Identity: $ADMIN_ADDR"

# ── 4. Deploy ─────────────────────────────────────────────────────────────────

echo "==> Deploying contract..."
CONTRACT_ID=$(stellar contract deploy \
    --wasm "$WASM" \
    --source "$IDENTITY" \
    --network "$NETWORK")
pass "Contract deployed: $CONTRACT_ID"

# ── 5. Initialize ─────────────────────────────────────────────────────────────

echo "==> Calling initialize..."
stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- initialize \
    --admin "$ADMIN_ADDR" > /dev/null
pass "initialize OK"

# ── 6. Wrap native XLM as a token ────────────────────────────────────────────

echo "==> Wrapping native XLM..."
TOKEN_ID=$(stellar contract asset deploy \
    --asset native \
    --source "$IDENTITY" \
    --network "$NETWORK")
pass "Token: $TOKEN_ID"

# ── 7. Deposit ────────────────────────────────────────────────────────────────

echo "==> Calling deposit..."
# unlock_time = now + 120 seconds
UNLOCK_TIME=$(( $(date +%s) + 120 ))
stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- deposit \
    --depositor "$ADMIN_ADDR" \
    --token "$TOKEN_ID" \
    --amount 1000 \
    --unlock_time "$UNLOCK_TIME" > /dev/null
pass "deposit OK"

# ── 8. get_vault ──────────────────────────────────────────────────────────────

echo "==> Calling get_vault..."
VAULT_OUT=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- get_vault \
    --depositor "$ADMIN_ADDR")
assert_contains "get_vault returns amount 1000" "1000" "$VAULT_OUT"

# ── 9. time_remaining ────────────────────────────────────────────────────────

echo "==> Calling time_remaining..."
TIME_OUT=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- time_remaining \
    --depositor "$ADMIN_ADDR")
# Should be > 0 since we just deposited with a 120s lock
if [ "$TIME_OUT" -gt 0 ] 2>/dev/null; then
    pass "time_remaining > 0 ($TIME_OUT)"
else
    fail "time_remaining should be > 0, got: $TIME_OUT"
fi

# ── 10. withdraw (should fail — still locked) ─────────────────────────────────

echo "==> Calling withdraw (expect FundsStillLocked)..."
WITHDRAW_ERR=$(stellar contract invoke \
    --id "$CONTRACT_ID" \
    --source "$IDENTITY" \
    --network "$NETWORK" \
    -- withdraw \
    --depositor "$ADMIN_ADDR" 2>&1 || true)
assert_contains "withdraw fails while locked" "FundsStillLocked" "$WITHDRAW_ERR"

# ── Done ──────────────────────────────────────────────────────────────────────

echo ""
echo "All smoke tests passed."
