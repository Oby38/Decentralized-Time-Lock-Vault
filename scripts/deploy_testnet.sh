#!/usr/bin/env bash
# =============================================================
#  deploy_testnet.sh
#  Deploys and initializes the Time-Lock Vault on Stellar Testnet.
#
#  Prerequisites:
#    - soroban-cli installed  (cargo install --locked soroban-cli)
#    - SOROBAN_SECRET_KEY env var set to your testnet secret key
#
#  Usage:
#    chmod +x scripts/deploy_testnet.sh
#    SOROBAN_SECRET_KEY=S... ./scripts/deploy_testnet.sh
# =============================================================

set -euo pipefail

# ---- Config --------------------------------------------------
NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
WASM_PATH="target/time_lock_vault.optimized.wasm"
DEPLOY_LOG="deploy_testnet.log"

# ---- Validate env --------------------------------------------
if [[ -z "${SOROBAN_SECRET_KEY:-}" ]]; then
  echo "ERROR: SOROBAN_SECRET_KEY is not set."
  echo "Export your testnet secret key: export SOROBAN_SECRET_KEY=S..."
  exit 1
fi

# ---- Build & optimize ----------------------------------------
echo ">>> Building WASM..."
cargo build --target wasm32-unknown-unknown --release

echo ">>> Optimizing WASM..."
soroban contract optimize \
  --wasm target/wasm32-unknown-unknown/release/time_lock_vault.wasm \
  --wasm-out "$WASM_PATH"

echo ">>> Optimized WASM size: $(du -sh "$WASM_PATH" | cut -f1)"

# ---- Fund deployer account on testnet ------------------------
echo ">>> Funding deployer account via Friendbot..."
DEPLOYER_ADDRESS=$(soroban keys address "$SOROBAN_SECRET_KEY" 2>/dev/null || \
  soroban keys generate --network "$NETWORK" deployer && \
  soroban keys address deployer)

curl -s "https://friendbot.stellar.org?addr=${DEPLOYER_ADDRESS}" > /dev/null
echo "    Deployer: $DEPLOYER_ADDRESS"

# ---- Deploy contract -----------------------------------------
echo ">>> Deploying contract..."
CONTRACT_ID=$(soroban contract deploy \
  --wasm "$WASM_PATH" \
  --source "$SOROBAN_SECRET_KEY" \
  --network "$NETWORK" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$NETWORK_PASSPHRASE")

echo "    Contract ID: $CONTRACT_ID"

# ---- Initialize contract -------------------------------------
echo ">>> Initializing contract with admin = $DEPLOYER_ADDRESS ..."
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOROBAN_SECRET_KEY" \
  --network "$NETWORK" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$NETWORK_PASSPHRASE" \
  -- initialize \
  --admin "$DEPLOYER_ADDRESS"

echo "    Contract initialized."

# ---- Smoke test: read admin back -----------------------------
echo ">>> Smoke test: get_admin..."
STORED_ADMIN=$(soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOROBAN_SECRET_KEY" \
  --network "$NETWORK" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$NETWORK_PASSPHRASE" \
  -- get_admin)

echo "    Stored admin: $STORED_ADMIN"

# ---- Smoke test: get_time ------------------------------------
echo ">>> Smoke test: get_time..."
LEDGER_TIME=$(soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOROBAN_SECRET_KEY" \
  --network "$NETWORK" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$NETWORK_PASSPHRASE" \
  -- get_time)

echo "    Ledger timestamp: $LEDGER_TIME"

# ---- Smoke test: get_constants -------------------------------
echo ">>> Smoke test: get_constants..."
CONSTANTS=$(soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOROBAN_SECRET_KEY" \
  --network "$NETWORK" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$NETWORK_PASSPHRASE" \
  -- get_constants)

echo "    Constants (max_amount, max_duration): $CONSTANTS"

# ---- Write deployment log ------------------------------------
cat > "$DEPLOY_LOG" <<EOF
Deployment Log — $(date -u +"%Y-%m-%dT%H:%M:%SZ")
Network:      $NETWORK
Contract ID:  $CONTRACT_ID
Admin:        $DEPLOYER_ADDRESS
Ledger Time:  $LEDGER_TIME
Constants:    $CONSTANTS
EOF

echo ""
echo "============================================"
echo "  Deployment successful!"
echo "  Contract ID : $CONTRACT_ID"
echo "  Log written : $DEPLOY_LOG"
echo "============================================"
