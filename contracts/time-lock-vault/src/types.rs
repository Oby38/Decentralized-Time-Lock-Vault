use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultKey {
    Deposit(Address, u32),
    DepositByLedger(Address, u32),
    DepositCounter(Address),
    ActiveDepositIds(Address),
    ActiveDepositCount(Address),
    Admin,
    PendingAdmin,
    Initialized,
    DepositorMember(Address),
    DepositorCount,
    DepositorAt(u32),
    DepositorIndex(Address),
    FeeRecipient,
    MaxDeposit,
    MaxLockSecs,
    Paused,
    DepositorFrozen(Address),
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

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawResult {
    pub depositor: Address,
    pub deposit_id: u32,
    pub success: bool,
}
