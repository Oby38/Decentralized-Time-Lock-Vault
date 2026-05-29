use soroban_sdk::{contracttype, Address};

// ----------------------------------------------------------------
//  Storage Keys
// ----------------------------------------------------------------

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultKey {
    /// Maps depositor → VaultEntry
    Deposit(Address),
    /// Contract-level admin address
    Admin,
    PendingAdmin,
    /// Set to true once initialize() has been called; never removed
    Initialized,
    /// Global list of all active depositor addresses (Vec<Address>)
    DepositorList,
    /// Address that receives penalty fees on early cancellation
    FeeRecipient,
    /// Runtime-configurable max deposit amount (overrides compile-time constant)
    MaxDeposit,
    /// Runtime-configurable max lock duration in seconds (overrides compile-time constant)
    MaxLockSecs,
}

// ----------------------------------------------------------------
//  Data Structures
// ----------------------------------------------------------------

/// Represents a single vault deposit.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultEntry {
    pub token: Address,
    pub amount: i128,
    pub unlock_time: u64,
    /// Early-exit penalty in basis points (0–10000). Charged on cancel_deposit.
    pub penalty_bps: u32,
}

/// Per-depositor result returned by `batch_emergency_withdraw`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawResult {
    pub depositor: Address,
    /// `true` if funds were successfully transferred; `false` if skipped (no deposit).
    pub success: bool,
}
