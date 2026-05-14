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

## Project Structure

```
.
├── Cargo.toml                          # Workspace manifest
├── Makefile                            # Build / test / lint / deploy helpers
├── rust-toolchain.toml                 # Pins stable Rust + wasm32 target
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
        ├── errors.rs       # VaultError enum (8 typed codes)
        ├── events.rs       # Event emission helpers
        ├── storage.rs      # Persistent storage helpers + TTL bump logic
        └── test.rs         # Full unit test suite (35+ tests)
```

---

## Contract API

### Initialization

#### `initialize(admin: Address)`
Sets the admin address. Must be called once after deployment.

---

### Core

#### `deposit(depositor, token, amount, unlock_time)`
Locks `amount` of `token` until `unlock_time` (Unix seconds).

| Param | Type | Constraint |
|---|---|---|
| `depositor` | `Address` | Must sign |
| `token` | `Address` | SEP-41 token contract |
| `amount` | `i128` | `0 < amount ≤ 10^15` |
| `unlock_time` | `u64` | `now < unlock_time ≤ now + 5 years` |

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

#### `time_remaining(depositor) → u64`
Returns seconds until unlock. Returns `0` if unlocked or no deposit exists. Does **not** bump TTL.

#### `get_time() → u64`
Returns the current ledger timestamp.

#### `get_admin() → Option<Address>`
Returns the current admin, or `None` if renounced.

#### `get_pending_admin() → Option<Address>`
Returns the pending admin during a transfer, or `None`.

#### `get_constants() → (i128, u64)`
Returns `(MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS)` for client-side validation.

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

### Test

```bash
make test
```

### Full CI check (fmt + lint + test)

```bash
make check
```

### Optimize WASM

```bash
make optimize
```

### Deploy to Testnet

```bash
export SOROBAN_SECRET_KEY=S...
make deploy-testnet
```

---

## Use Cases

- **Savings accounts** — Lock funds for a fixed period to enforce saving discipline.
- **Token vesting** — Team or investor tokens released on a schedule.
- **HODL challenges** — Commit to not selling until a future date.
- **Escrow** — Time-gated release of payment.

---

## License

MIT
