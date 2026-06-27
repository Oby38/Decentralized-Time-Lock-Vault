#!/usr/bin/env bash
# =============================================================
#  deploy_testnet.sh
#  Builds, deploys, and initializes the Time-Lock Vault on
#  Stellar Testnet, then runs post-deployment smoke tests and
#  prints usage examples for every contract entry point.
#
#  Prerequisites:
#    - Rust + cargo installed (https://rustup.rs)
#    - soroban-cli installed:
#        cargo install --locked soroban-cli
#    - SOROBAN_SECRET_KEY env var — the S... Stellar secret key
#      of the testnet account that will become the contract admin.
#      Generate one with: soroban keys generate --network testnet mykey
#      then export: export SOROBAN_SECRET_KEY=$(soroban keys show mykey)
#
#  Optional env var overrides (all have working defaults):
#    WASM_PATH          Path for the optimized WASM output.
#                       Default: target/time_lock_vault.optimized.wasm
#    FEE_RECIPIENT      Stellar address that receives early-exit penalties.
#                       Default: the deployer address (DEPLOYER_ADDRESS).
#    MAX_DEPOSIT        Maximum deposit amount in stroops (1 XLM = 10_000_000).
#                       Default: 1_000_000_000_000_000 (100,000,000 XLM).
#                       Pass "null" to leave unset (contract uses built-in default).
#    MAX_LOCK_SECS      Maximum lock duration in seconds.
#                       Default: 157_788_000 (~5 years).
#                       Pass "null" to leave unset (contract uses built-in default).
#    DEPLOY_LOG         Path for the deployment log file.
#                       Default: deploy_testnet.log
#
#  Usage:
#    chmod +x scripts/deploy_testnet.sh
#
#    # Minimal — deployer becomes fee recipient, contract defaults apply:
#    SOROBAN_SECRET_KEY=S... ./scripts/deploy_testnet.sh
#
#    # Custom fee recipient and a 30-day max lock:
#    SOROBAN_SECRET_KEY=S... \
#      FEE_RECIPIENT=GABC...XYZ \
#      MAX_LOCK_SECS=2592000 \
#      ./scripts/deploy_testnet.sh
#
#    # Custom max deposit (50,000 XLM = 500_000_000_000 stroops):
#    SOROBAN_SECRET_KEY=S... \
#      MAX_DEPOSIT=500000000000 \
#      ./scripts/deploy_testnet.sh
#
#  Default environment assumptions:
#    Network:            Stellar Testnet (SDF-hosted)
#    RPC endpoint:       https://soroban-testnet.stellar.org
#    Network passphrase: "Test SDF Network ; September 2015"
#    Friendbot URL:      https://friendbot.stellar.org  (free testnet XLM)
#    Min lock duration:  60 seconds  (MIN_LOCK_DURATION_SECS in constants.rs)
#    Max lock duration:  157_788_000 seconds / ~5 years (MAX_LOCK_DURATION_SECS)
#    Max deposit:        1_000_000_000_000_000 stroops  (MAX_DEPOSIT_AMOUNT)
#    Max batch size:     20 depositors per batch call   (MAX_BATCH_SIZE)
#    Amount unit:        stroops  (1 XLM = 10_000_000 stroops)
#    penalty_bps unit:   basis points (100 bps = 1%; 10_000 bps = 100%)
# =============================================================

set -euo pipefail

# ---- Config (override via env vars) --------------------------
NETWORK="testnet"
RPC_URL="https://soroban-testnet.stellar.org"
NETWORK_PASSPHRASE="Test SDF Network ; September 2015"
WASM_PATH="${WASM_PATH:-target/time_lock_vault.optimized.wasm}"
DEPLOY_LOG="${DEPLOY_LOG:-deploy_testnet.log}"

# ---- Validate required env -----------------------------------
if [[ -z "${SOROBAN_SECRET_KEY:-}" ]]; then
  echo "ERROR: SOROBAN_SECRET_KEY is not set."
  echo ""
  echo "Generate a testnet key and fund it:"
  echo "  soroban keys generate --network testnet mykey"
  echo "  export SOROBAN_SECRET_KEY=\$(soroban keys show mykey)"
  echo ""
  echo "Then re-run:"
  echo "  SOROBAN_SECRET_KEY=\$SOROBAN_SECRET_KEY ./scripts/deploy_testnet.sh"
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

# FEE_RECIPIENT defaults to the deployer if not set externally.
FEE_RECIPIENT="${FEE_RECIPIENT:-$DEPLOYER_ADDRESS}"

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
# initialize(admin, fee_recipient, max_deposit: Option<i128>, max_lock_secs: Option<u64>)
#
# max_deposit and max_lock_secs are Option types:
#   pass a numeric value to override the built-in constant, or "null" to use the default.
#
# Built-in defaults (from constants.rs):
#   max_deposit  = 1_000_000_000_000_000 stroops (100,000,000 XLM)
#   max_lock_secs = 157_788_000 seconds (~5 years)
#   min lock     = 60 seconds (enforced on every deposit; not configurable via initialize)
MAX_DEPOSIT_ARG="${MAX_DEPOSIT:-null}"
MAX_LOCK_SECS_ARG="${MAX_LOCK_SECS:-null}"

echo ">>> Initializing contract..."
echo "    admin        = $DEPLOYER_ADDRESS"
echo "    fee_recipient= $FEE_RECIPIENT"
echo "    max_deposit  = $MAX_DEPOSIT_ARG (null → built-in default: 1_000_000_000_000_000)"
echo "    max_lock_secs= $MAX_LOCK_SECS_ARG (null → built-in default: 157_788_000)"
soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOROBAN_SECRET_KEY" \
  --network "$NETWORK" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$NETWORK_PASSPHRASE" \
  -- initialize \
  --admin "$DEPLOYER_ADDRESS" \
  --fee_recipient "$FEE_RECIPIENT" \
  --max_deposit "$MAX_DEPOSIT_ARG" \
  --max_lock_secs "$MAX_LOCK_SECS_ARG"

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

echo "    Constants (max_amount stroops, max_duration secs): $CONSTANTS"

# ---- Smoke test: is_initialized ------------------------------
echo ">>> Smoke test: is_initialized..."
IS_INIT=$(soroban contract invoke \
  --id "$CONTRACT_ID" \
  --source "$SOROBAN_SECRET_KEY" \
  --network "$NETWORK" \
  --rpc-url "$RPC_URL" \
  --network-passphrase "$NETWORK_PASSPHRASE" \
  -- is_initialized)

echo "    is_initialized: $IS_INIT"

# ---- Write deployment log ------------------------------------
cat > "$DEPLOY_LOG" <<EOF
Deployment Log — $(date -u +"%Y-%m-%dT%H:%M:%SZ")
Network:        $NETWORK
Contract ID:    $CONTRACT_ID
Admin:          $DEPLOYER_ADDRESS
Fee recipient:  $FEE_RECIPIENT
Ledger Time:    $LEDGER_TIME
Constants:      $CONSTANTS
Is initialized: $IS_INIT
EOF

echo ""
echo "============================================"
echo "  Deployment successful!"
echo "  Contract ID : $CONTRACT_ID"
echo "  Log written : $DEPLOY_LOG"
echo "============================================"
echo ""

# ---- Post-deployment usage examples --------------------------
# The examples below show every major entry point.
# Replace placeholder values (TOKEN_ID, DEPOSITOR, etc.) with real addresses.
# All amounts are in stroops: 1 XLM = 10_000_000 stroops.
# penalty_bps is in basis points: 100 bps = 1%, 10_000 bps = 100%.

cat <<USAGE_EXAMPLES
------------------------------------------------------------
 Post-deployment usage examples  (CONTRACT_ID=$CONTRACT_ID)
------------------------------------------------------------

# Common CLI flags (reuse across all calls below):
INVOKE="soroban contract invoke \\
  --id $CONTRACT_ID \\
  --source \$SOROBAN_SECRET_KEY \\
  --network $NETWORK \\
  --rpc-url $RPC_URL \\
  --network-passphrase '$NETWORK_PASSPHRASE'"

# ── Deposit (timestamp-based unlock) ────────────────────────
# Lock 10 XLM for 1 hour, 5% early-exit penalty.
# unlock_time must be > current ledger timestamp and within max_lock_secs.
# penalty_bps: 0–10000; 500 = 5%; sent to fee_recipient on cancel_deposit.
UNLOCK_TIME=\$(( \$(date +%s) + 3600 ))
\$INVOKE -- deposit \\
  --depositor $DEPLOYER_ADDRESS \\
  --token TOKEN_ID \\
  --amount 100000000 \\
  --unlock_time \$UNLOCK_TIME \\
  --penalty_bps 500
# Returns: deposit_id (u32) — save this to reference the deposit later.

# ── Deposit on behalf of another account (deposit_for) ───────
# Payer funds a vault for a different depositor.
# payer signs and pays; depositor owns the vault and can withdraw.
\$INVOKE -- deposit_for \\
  --payer $DEPLOYER_ADDRESS \\
  --depositor DEPOSITOR_ADDRESS \\
  --token TOKEN_ID \\
  --amount 100000000 \\
  --unlock_time \$UNLOCK_TIME \\
  --penalty_bps 0

# ── Deposit (ledger-sequence-based unlock) ────────────────────
# Alternative to timestamp-based: unlock at a specific ledger number.
# Stellar produces ~1 ledger every 5–6 seconds on testnet.
# unlock_ledger must be > current ledger and within max_lock_secs/LEDGER_SECONDS.
CURRENT_LEDGER=\$(soroban network status --network $NETWORK | jq '.sequence')
UNLOCK_LEDGER=\$(( CURRENT_LEDGER + 720 ))  # ~1 hour at 5s/ledger
\$INVOKE -- deposit_by_ledger \\
  --depositor $DEPLOYER_ADDRESS \\
  --token TOKEN_ID \\
  --amount 100000000 \\
  --unlock_ledger \$UNLOCK_LEDGER \\
  --penalty_bps 0

# ── Query a timestamp-based vault ────────────────────────────
\$INVOKE -- get_vault \\
  --depositor $DEPLOYER_ADDRESS \\
  --deposit_id 0
# Returns: VaultEntry { token, amount, unlock_time, depositor, penalty_bps } or null.

# ── Query a ledger-based vault ────────────────────────────────
\$INVOKE -- get_vault_by_ledger \\
  --depositor $DEPLOYER_ADDRESS \\
  --deposit_id 0
# Returns: LedgerVaultEntry { token, amount, unlock_ledger, depositor, penalty_bps } or null.

# ── Time remaining (seconds) on a timestamp-based vault ───────
\$INVOKE -- time_remaining \\
  --depositor $DEPLOYER_ADDRESS \\
  --deposit_id 0
# Returns: seconds until unlock (u64). Returns 0 if unlocked or no deposit found.

# ── Ledgers remaining on a ledger-based vault ─────────────────
\$INVOKE -- ledgers_remaining \\
  --depositor $DEPLOYER_ADDRESS \\
  --deposit_id 0
# Returns: ledgers until unlock (u32). Returns 0 if unlocked or not found.

# ── List all deposit IDs for a depositor ─────────────────────
\$INVOKE -- get_deposit_ids \\
  --depositor $DEPLOYER_ADDRESS

# ── Withdraw (after unlock time has passed) ───────────────────
# Fails with FundsStillLocked if called before the unlock time/ledger.
\$INVOKE -- withdraw \\
  --depositor $DEPLOYER_ADDRESS \\
  --deposit_id 0

# ── Withdraw to a different recipient ────────────────────────
# depositor must sign; funds go to recipient instead.
\$INVOKE -- withdraw_to \\
  --depositor $DEPLOYER_ADDRESS \\
  --deposit_id 0 \\
  --recipient RECIPIENT_ADDRESS

# ── Cancel deposit early (before unlock) ─────────────────────
# Refunds (amount - penalty) to depositor; penalty goes to fee_recipient.
# Fails with FundsAlreadyUnlocked if the lock period has already expired.
\$INVOKE -- cancel_deposit \\
  --depositor $DEPLOYER_ADDRESS \\
  --deposit_id 0

# ── Admin: pause / unpause new deposits ──────────────────────
\$INVOKE -- pause --admin $DEPLOYER_ADDRESS
\$INVOKE -- unpause --admin $DEPLOYER_ADDRESS
\$INVOKE -- is_paused

# ── Admin: two-step admin transfer ───────────────────────────
# Step 1 — current admin nominates a new admin:
\$INVOKE -- transfer_admin \\
  --admin $DEPLOYER_ADDRESS \\
  --new_admin NEW_ADMIN_ADDRESS
# Step 2 — new admin accepts (must sign with NEW_ADMIN's key):
\$INVOKE -- accept_admin --new_admin NEW_ADMIN_ADDRESS
# Cancel a pending transfer:
\$INVOKE -- cancel_transfer_admin --admin $DEPLOYER_ADDRESS

# ── Admin: emergency withdraw (bypasses lock, admin signs) ────
\$INVOKE -- emergency_withdraw \\
  --admin $DEPLOYER_ADDRESS \\
  --depositor DEPOSITOR_ADDRESS \\
  --deposit_id 0

# ── Admin: batch emergency withdraw (up to 20 entries) ───────
\$INVOKE -- batch_emergency_withdraw \\
  --admin $DEPLOYER_ADDRESS \\
  --depositors '[[\"DEPOSITOR1_ADDRESS\",0],[\"DEPOSITOR2_ADDRESS\",1]]'

# ── Read-only queries ─────────────────────────────────────────
\$INVOKE -- get_admin
\$INVOKE -- get_pending_admin
\$INVOKE -- get_fee_recipient
\$INVOKE -- get_constants      # returns (max_deposit_stroops, max_lock_secs)
\$INVOKE -- get_time           # current ledger Unix timestamp
\$INVOKE -- is_initialized
\$INVOKE -- get_depositor_count
# Paginated depositor list (offset=0, limit=20):
\$INVOKE -- get_depositors --offset 0 --limit 20
# Batch vault query for a single deposit_id across multiple depositors:
\$INVOKE -- get_vault_batch \\
  --depositors '["DEPOSITOR1_ADDRESS","DEPOSITOR2_ADDRESS"]' \\
  --deposit_id 0
USAGE_EXAMPLES
