# 🔒 Decentralized Time-Lock Vault

A production-ready Soroban smart contract on the Stellar blockchain that locks XLM or any Stellar asset until a future timestamp is reached.

---

## Overview

| Property | Value |
|---|---|
| Network | Stellar (Soroban) |
| Language | Rust |
| SDK | soroban-sdk v22 |
| Storage | Persistent (per-depositor) |
| Max deposit | 10^15 units (1 quadrillion) |
| Max lock duration | 5 years |

---

## How It Works

1. A user calls `deposit(token, amount, unlock_time)` — tokens transfer from their wallet into the contract.
2. The contract stores a `VaultEntry` in **Persistent Storage** keyed by the depositor's address.
3. When the user calls `withdraw()`, the contract checks `env.ledger().timestamp() >= unlock_time`.
4. If the time has passed, tokens are returned. Otherwise the call fails with `FundsStillLocked`.
5. An admin can perform emergency withdrawals (funds always return to the depositor, never to the admin).
6. Admin rights can be transferred via a two-step process, or permanently renounced to make the vault fully trustless.

---

## Architecture

### Deposit / Withdraw Flow

```
Depositor
   │
   ├─► deposit(token, amount, unlock_time)
   │       │
   │       ├─ validate amount & unlock_time
   │       ├─ token.transfer(depositor → contract)
   │       ├─ storage::set_deposit(VaultKey::Deposit(depositor) → VaultEntry)
   │       └─ emit "deposit" event
   │
   └─► withdraw(depositor)
           │
           ├─ load VaultEntry
           ├─ assert now >= unlock_time
           ├─ storage::remove_deposit(depositor)   ← state cleared first (CEI)
           ├─ token.transfer(contract → depositor)
           └─ emit "withdraw" event
```

### Storage Layout

```
Persistent Storage
├── VaultKey::Admin                    → Address
│       (set once on initialize; removed on renounce_admin)
│
├── VaultKey::PendingAdmin             → Address
│       (set by transfer_admin; cleared by accept_admin / cancel_transfer_admin)
│
└── VaultKey::Deposit(depositor: Address) → VaultEntry
        ├── token:       Address   (SEP-41 token contract)
        ├── amount:      i128      (locked units)
        ├── unlock_time: u64       (Unix seconds)
        └── depositor:   Address   (owner; stored for event emission)
```

All entries use TTL bump threshold ≈ 30 days and target ≈ 5.2 years so a max-duration deposit cannot expire before its unlock time.

---

## Project Structure

```
.
├── Cargo.toml                          # Workspace manifest
├── Makefile                            # Build / test / lint / deploy helpers
├── rust-toolchain.toml                 # Pins stable Rust + wasm32 target
├── .cargo/
│   └── config.toml                     # Documents --target trade-off (default target intentionally unset)
├── .gitignore
├── README.md
├── .github/
│   └── workflows/
│       └── ci.yml                      # CI: lint → test → build WASM
├── scripts/
│   └── deploy_testnet.sh               # Automated testnet deploy + smoke test
└── contracts/time-lock-vault/
    ├── Cargo.toml
    └── src/
        ├── lib.rs          # Crate root & module declarations
        ├── contract.rs     # All public entry points
        ├── types.rs        # VaultKey, VaultEntry, protocol constants
        ├── errors.rs       # VaultError enum (9 typed codes)
        ├── events.rs       # Event emission helpers
        ├── storage.rs      # Persistent storage helpers + TTL bump logic
        └── test.rs         # Full unit test suite (48+ tests)
```

---

## Contract API

### Initialization

#### `initialize(admin: Address, max_deposit: Option<i128>, max_lock_secs: Option<u64>)`
Sets the admin address. Optionally overrides the compile-time limits for this deployment. Pass `None` to use the defaults (`10^15` and `5 years`). Must be called once after deployment.

---

### Core

#### `deposit(depositor, token, amount, unlock_time, penalty_bps)`
Locks `amount` of `token` until `unlock_time` (Unix seconds).

| Param | Type | Constraint |
|---|---|---|
| `depositor` | `Address` | Must sign |
| `token` | `Address` | SEP-41 token contract |
| `amount` | `i128` | `0 < amount ≤ 10^15` |
| `unlock_time` | `u64` | `now < unlock_time ≤ now + 5 years` |
| `penalty_bps` | `u32` | `0–10000` (basis points for early-exit penalty) |

#### `cancel_deposit(depositor)`
Cancels an active deposit before the unlock time. The penalty (`penalty_bps` set at deposit time) is sent to the `fee_recipient`; the remainder is returned to the depositor. Fails with `FundsStillLocked` if the vault is already past its unlock time (use `withdraw` instead).

#### `withdraw(depositor)`
Withdraws funds if `now >= unlock_time`. Fails with `FundsStillLocked` otherwise.

---

### Admin

#### `emergency_withdraw(admin, depositor)`
Admin-only. Returns funds to the depositor regardless of lock time. Funds always go to the depositor — never to the admin.

#### `transfer_admin(admin, new_admin)`
Step 1 of a two-step admin transfer. Nominates `new_admin` as pending admin.

#### `accept_admin(new_admin)`
Step 2. The pending admin accepts and becomes the active admin.

#### `cancel_transfer_admin(admin)`
Cancels a pending admin transfer. Only the current admin can cancel.

#### `renounce_admin(admin)`
Permanently removes admin privileges. After this call, `emergency_withdraw` and all admin functions are disabled forever. Makes the vault fully trustless.

---

### Read-only Queries

#### `get_vault(depositor) → Option<VaultEntry>`
Returns the current vault entry. Does **not** bump storage TTL (no extra fees).

#### `get_vault_with_time_remaining(depositor) → Option<(VaultEntry, u64)>`
Returns `Some((entry, seconds_remaining))` if a deposit exists, or `None`. Combines `get_vault` and `time_remaining` into a single RPC call.

#### `time_remaining(depositor) → u64`
Returns seconds until unlock. Returns `0` if unlocked or no deposit exists. Does **not** bump TTL.

#### `has_deposit(depositor) → bool`
Returns `true` if `depositor` has an active deposit. Cheaper than `get_vault` — no `VaultEntry` deserialization.

#### `get_time() → u64`
Returns the current ledger timestamp.

#### `get_admin() → Option<Address>`
Returns the current admin, or `None` if renounced.

#### `get_pending_admin() → Option<Address>`
Returns the pending admin during a transfer, or `None`.

#### `is_admin(address) → bool`
Returns `true` if `address` is the current admin. Returns `false` if admin has been renounced.

#### `get_fee_recipient() → Option<Address>`
Returns the fee recipient address set at initialization.

#### `get_constants() → (i128, u64)`
Returns the effective `(MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS)` for this deployment — runtime-configured values if set at `initialize`, otherwise the compile-time defaults.

#### `get_depositor_count() → u32`
Returns the total number of addresses with an active deposit.

#### `get_depositors(offset: u32, limit: u32) → Vec<Address>`
Returns a paginated slice of active depositor addresses.

| Param | Type | Description |
|---|---|---|
| `offset` | `u32` | Zero-based start index |
| `limit` | `u32` | Maximum number of addresses to return |

Use `offset=0, limit=N` for the first page, then increment `offset` by `N` for subsequent pages.

#### `get_version() → String`
Returns the contract version string (e.g. `"0.1.0"`), set at compile time from `CARGO_PKG_VERSION`.

---

## Error Codes

| Code | Name | Meaning |
|---|---|---|
| 1 | `InvalidAmount` | Amount ≤ 0 |
| 2 | `UnlockTimeNotInFuture` | `unlock_time` ≤ current ledger time |
| 3 | `NoDepositFound` | No active deposit for this address |
| 4 | `FundsStillLocked` | Lock period not yet expired |
| 5 | `DepositAlreadyExists` | Must withdraw before re-depositing |
| 6 | `LockDurationTooLong` | Lock period exceeds 5 years |
| 7 | `Unauthorized` | Caller is not the admin |
| 8 | `AmountTooLarge` | Amount exceeds 10^15 |
| 9 | `InvalidPenaltyBps` | `penalty_bps` > 10000 |

---

## Security Properties

| Property | Implementation |
|---|---|
| Checks-Effects-Interactions | Storage cleared before token transfer on every withdrawal |
| Auth-first ordering | `require_auth()` is always the first statement in every mutating function |
| No re-entrancy surface | State removed before any external token call |
| Bounded inputs | Amount capped at 10^15; lock duration capped at 5 years |
| No admin fund theft | Emergency withdraw always sends to depositor, never to admin |
| Trustless mode | Admin can permanently renounce via `renounce_admin()` |
| Safe admin transfer | Two-step transfer prevents accidental key loss |
| TTL management | Persistent entries bumped to ~1 year on every write; view functions skip TTL bump |
| No testutils in production | `features = ["testutils"]` only in `[dev-dependencies]` |

---

## Getting Started

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Add WASM target
rustup target add wasm32-unknown-unknown

# Install Soroban CLI
cargo install --locked soroban-cli
```

### Build

```bash
make build
```

> **Why not just `cargo build`?**
> Running `cargo build` without `--target wasm32-unknown-unknown` produces a native binary, not a WASM contract. The Makefile's `build` target always passes the correct flag. A `.cargo/config.toml` is included in the repo that documents this trade-off — the default target is intentionally left commented out because setting it would break `cargo test` (tests must run natively to use Soroban testutils).

### Test

```bash
make test
```

> Tests run natively (no `--target` flag) so that `soroban-sdk`'s `testutils` feature works. Never run `cargo test --target wasm32-unknown-unknown`.

### Full CI check (fmt + lint + test + audit + deny)

```bash
make check
```

### Security audit

```bash
make audit
```

Runs `cargo audit` to check all dependencies against the [RustSec Advisory Database](https://rustsec.org/).

### License & dependency policy

```bash
make deny
```

Runs `cargo deny check` to enforce license allowlists and ban policies defined in `deny.toml`.

### Optimize WASM

```bash
make optimize
```

### Check WASM size

```bash
make check-wasm-size
```

Fails if the optimized WASM exceeds `MAX_WASM_BYTES` (default **65 536 bytes / 64 KB**).
Override the threshold at the command line:

```bash
make check-wasm-size MAX_WASM_BYTES=81920   # 80 KB
```

The same threshold is enforced in CI via the `Check WASM size` step in `.github/workflows/ci.yml`.
To update the limit, change `MAX_WASM_BYTES` in both places (or only in `ci.yml` if you don't use the Makefile target locally).

### Deploy to Testnet

```bash
export SOROBAN_SECRET_KEY=S...
make deploy-testnet
```

### Smoke Test (local node)

Runs a quick end-to-end test against a local Soroban standalone node — no funded account or testnet access required.

```bash
# Build the WASM first, then run the smoke test
make smoke-test-local
```

The script (`scripts/smoke_test_local.sh`):
1. Starts a local node via `stellar network start local`
2. Generates a funded test identity
3. Deploys the contract and calls `initialize`, `deposit`, `get_vault`, `time_remaining`, and `withdraw`
4. Asserts expected outputs at each step
5. Stops the local node on exit

---

## Use Cases

- **Savings accounts** — Lock funds for a fixed period to enforce saving discipline.
- **Token vesting** — Team or investor tokens released on a schedule.
- **HODL challenges** — Commit to not selling until a future date.
- **Escrow** — Time-gated release of payment.

---

## Contributing

See [CONTRIBUTING.md](./CONTRIBUTING.md) for branch naming, commit conventions, and the PR checklist.

---

## License

MIT
