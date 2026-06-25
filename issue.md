# Issue Template — Decentralized Time-Lock Vault

---

## [BUG] `deposit_by_ledger` bypasses pause state and lock-duration validation

**Labels:** `bug` `security` `contract`
**Priority:** 🔴 Critical
**Difficulty:** Advanced
**Tags:** `contract` `pause` `validation` `ledger`

---

### Description

`deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs` accepts ledger-based deposits without enforcing the same pause and lock-duration rules as timestamp-based deposit methods.

This creates two concrete issues:
- a paused contract can still accept ledger-based deposits,
- ledger-based deposits can be created with durations shorter than `MIN_LOCK_DURATION_SECS` or longer than the configured maximum.

---

### Reproduction Steps

1. Deploy the contract and call `initialize(admin, fee_recipient, None, None)`.
2. Call `pause(admin)`.
3. Call `deposit_by_ledger(depositor, token, amount, current_ledger + 10, 0)`.
4. Observe the call succeeds even though the contract is paused.
5. Call `deposit_by_ledger(depositor, token, amount, current_ledger + 1, 0)`.
6. Observe the call succeeds despite an effectively too-short lock period.

---

### Expected Behavior

- `deposit_by_ledger` returns `VaultError::ContractPaused` when the contract is paused.
- `deposit_by_ledger` rejects lock durations shorter than `MIN_LOCK_DURATION_SECS`.
- `deposit_by_ledger` rejects lock durations longer than the configured runtime `max_lock_secs` or `MAX_LOCK_DURATION_SECS`.
- Ledger-based deposits follow the same contract invariants as timestamp-based deposits.

---

### Actual Behavior

- `deposit_by_ledger` does not check for pause state.
- `deposit_by_ledger` does not validate lock duration at all.
- The ledger deposit path can therefore bypass emergency pause and create invalid locks.

---

### Technical Notes

- `deposit()` and `deposit_for()` both call `storage::is_paused(&env)`.
- `deposit_by_ledger()` only checks `amount` and `penalty_bps` before transferring tokens.
- `deposit_by_ledger()` lacks any validation of `unlock_ledger` relative to `env.ledger().sequence()` other than `unlock_ledger > current_ledger`.
- The current public API also does not properly expose ledger deposits through `get_vault`, `time_remaining`, or `get_deposit_ids`.

---

### Acceptance Criteria

- [ ] `deposit_by_ledger` checks `storage::is_paused(&env)` and returns `VaultError::ContractPaused` when paused.
- [ ] `deposit_by_ledger` validates `unlock_ledger` against minimum and maximum lock durations.
- [ ] Add unit tests for paused-state rejection and ledger lock-duration boundary cases.
- [ ] Update documentation to state pause semantics for ledger-based deposits.

---

### Suggested Implementation

1. Add pause validation at the start of `deposit_by_ledger()`:

```rust
if storage::is_paused(&env) {
    return Err(VaultError::ContractPaused);
}
```

2. Compute ledger-based lock duration:

```rust
let current_ledger = env.ledger().sequence();
let lock_ledgers = unlock_ledger.saturating_sub(current_ledger);
let lock_seconds = lock_ledgers.saturating_mul(LEDGER_SECONDS);
```

3. Reuse the existing time-based deposit bounds checks:

```rust
let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
if lock_seconds > max_lock {
    return Err(VaultError::LockDurationTooLong);
}
if lock_seconds < MIN_LOCK_DURATION_SECS {
    return Err(VaultError::LockDurationTooShort);
}
```

4. Add regression tests for paused contracts and invalid ledger lock durations.

---

*Filed as part of Wave Program Sprint — Decentralized Time-Lock Vault*
