# Changelog

All notable changes to this project will be documented in this file.

## [Unreleased]

## [0.1.0] - 2026-05-28

### Added
- `get_version() → String` — returns `CARGO_PKG_VERSION` at compile time
- `has_deposit(depositor) → bool` — lightweight check without deserializing `VaultEntry`
- `is_admin(address) → bool` — ergonomic admin check for UI clients
- `get_vault_with_time_remaining(depositor)` — combines vault lookup and time remaining in one call
- `cancel_deposit(depositor)` — early-exit with configurable `penalty_bps`
- `get_depositor_count()` and `get_depositors(offset, limit)` — paginated depositor enumeration
- Two-step admin transfer (`transfer_admin` / `accept_admin` / `cancel_transfer_admin`)
- `renounce_admin` — permanently removes admin for fully trustless operation
- Runtime-configurable `max_deposit` and `max_lock_secs` via `initialize`

### Fixed
- README: removed duplicate `initialize` signature, duplicate `get_fee_recipient`, `get_depositor_count`, and `get_depositors` entries
