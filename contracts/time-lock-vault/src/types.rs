use soroban_sdk::{contracttype, Address};

// ----------------------------------------------------------------
//  Protocol constants
// ----------------------------------------------------------------

/// Maximum amount that can be locked in a single vault deposit.
/// Set to 1 quadrillion units (10^15) — safely below i128::MAX,
/// prevents accidental locking of astronomically large amounts.
pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;

/// Maximum lock duration: 5 years in seconds (5 * 365.25 * 24 * 3600).
/// Prevents deposits that would require TTL bumps indefinitely or
/// risk expiry before the user can withdraw.
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;

/// Minimum lock duration: prevent trivial, pointless vaults that waste storage.
/// Set to 60 seconds to avoid very short-lived deposits.
pub const MIN_LOCK_DURATION_SECS: u64 = 60;

// ----------------------------------------------------------------
//  Storage Keys
// ----------------------------------------------------------------

/// Top-level key enum for Persistent storage.
/// Each depositor gets their own VaultEntry keyed by their address.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultKey {
    Deposit(Address),
    Admin,
    PendingAdmin,
    /// Global list of all active depositor addresses (Vec<Address>)
    DepositorList,
    /// Address that receives penalty fees on early cancellation
    FeeRecipient,
    /// Runtime-configurable max deposit amount (overrides compile-time constant).
    MaxDeposit,
    /// Runtime-configurable max lock duration in seconds (overrides compile-time constant).
    MaxLockSecs,
}

// ----------------------------------------------------------------
//  Data Structures
// ----------------------------------------------------------------

/// Represents a single vault deposit.
/// The depositor address is not stored here — it is already the storage key
/// (VaultKey::Deposit(Address)), so duplicating it wastes persistent storage.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultEntry {
    /// The Stellar asset contract address (use the XLM wrapped token address
    /// or any SEP-41 compliant token contract).
    pub token: Address,

    /// Amount locked (in stroops for XLM, or the token's smallest unit).
    pub amount: i128,

    /// Unix timestamp (seconds) after which withdrawal is permitted.
    pub unlock_time: u64,

    /// The depositor's address — stored for convenience and event emission.
    pub depositor: Address,

    /// Early-exit penalty in basis points (0–10000). Charged on cancel_deposit.
    /// 0 = free cancellation, 10000 = 100% penalty (all funds go to fee_recipient).
    pub penalty_bps: u32,
}
