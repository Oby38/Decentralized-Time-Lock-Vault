use soroban_sdk::{contracttype, Address};

pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;
pub const MIN_LOCK_DURATION_SECS: u64 = 60;
pub const MAX_BATCH_SIZE: u32 = 20;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultKey {
    Deposit(Address, u32),
    DepositByLedger(Address, u32),
    DepositCounter(Address),
    Admin,
    PendingAdmin,
    Initialized,
    DepositorList,
    FeeRecipient,
    MaxDeposit,
    MaxLockSecs,
    /// Boolean flag: when true, deposits are paused (admin-controlled)
    Paused,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultEntry {
    pub token: Address,
    pub amount: i128,
    pub unlock_time: u64,
    pub depositor: Address,
    pub penalty_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerVaultEntry {
    pub token: Address,
    pub amount: i128,
    pub unlock_ledger: u32,
    pub depositor: Address,
    pub penalty_bps: u32,
}

/// Summary of the contract's current operational state, returned by `vault_status`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultStatus {
    /// `true` if an admin exists and has not been renounced.
    pub has_admin: bool,
    /// The current admin address, or `None` if renounced.
    pub admin: Option<Address>,
    /// `true` if new deposits are currently paused.
    pub paused: bool,
    /// Total number of active depositors.
    pub depositor_count: u32,
}
