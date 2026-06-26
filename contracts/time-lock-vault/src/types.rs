use soroban_sdk::{contracttype, Address};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VaultKey {
    Deposit(Address, u32),
    DepositByLedger(Address, u32),
    DepositCounter(Address),
    /// Tracks the set of active deposit IDs for a depositor (replaces the
    /// O(counter) scan in the old implementation).
    ActiveDepositIds(Address),
    Admin,
    PendingAdmin,
    Initialized,
    DepositorList,
    /// Per-depositor membership flag — enables O(1) duplicate check in
    /// `add_depositor` without scanning the full `DepositorList`.
    DepositorMember(Address),
    FeeRecipient,
    MaxDeposit,
    MaxLockSecs,
    Paused,
}

/// Persistent record of a time-locked deposit keyed by Unix timestamp.
///
/// Stored under `VaultKey::Deposit(depositor, deposit_id)`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VaultEntry {
    /// SEP-41 token contract address for the locked asset.
    pub token: Address,
    /// Number of token units locked. Always > 0 and ≤ `MAX_DEPOSIT_AMOUNT` (10^15).
    /// Uses the token's own decimal base — e.g. 10_000_000 stroops for 1 XLM (7 decimals).
    pub amount: i128,
    /// Unix timestamp (seconds since epoch) after which `withdraw` succeeds.
    /// Set by the depositor at deposit time; must satisfy
    /// `now < unlock_time ≤ now + MAX_LOCK_DURATION_SECS`.
    pub unlock_time: u64,
    /// Address that originally made the deposit and is the sole authorised recipient
    /// on withdrawal or emergency-withdrawal. Never changed after creation.
    pub depositor: Address,
    /// Early-exit penalty in basis points (0–10 000).
    /// Applied only by `cancel_deposit`; `withdraw` after unlock incurs no penalty.
    /// Example: 500 = 5 % of `amount` sent to the fee recipient.
    pub penalty_bps: u32,
}

/// Persistent record of a time-locked deposit keyed by ledger sequence number.
///
/// Functionally identical to [`VaultEntry`] but uses a ledger number instead of
/// a Unix timestamp for the unlock condition. Stored under
/// `VaultKey::DepositByLedger(depositor, deposit_id)`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LedgerVaultEntry {
    /// SEP-41 token contract address for the locked asset.
    pub token: Address,
    /// Number of token units locked. Always > 0 and ≤ `MAX_DEPOSIT_AMOUNT` (10^15).
    /// Uses the token's own decimal base — e.g. 10_000_000 stroops for 1 XLM (7 decimals).
    pub amount: i128,
    /// Ledger sequence number at or after which withdrawal is permitted.
    /// Soroban ledgers close approximately every 5 seconds; convert from seconds with
    /// `current_ledger + duration_secs / LEDGER_SECONDS`.
    pub unlock_ledger: u32,
    /// Address that originally made the deposit and is the sole authorised recipient
    /// on withdrawal or emergency-withdrawal. Never changed after creation.
    pub depositor: Address,
    /// Early-exit penalty in basis points (0–10 000).
    /// Applied only by `cancel_deposit`; `withdraw` after unlock incurs no penalty.
    /// Example: 500 = 5 % of `amount` sent to the fee recipient.
    pub penalty_bps: u32,
}

/// Result entry for `batch_emergency_withdraw`.
#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WithdrawResult {
    pub depositor: Address,
    pub deposit_id: u32,
    pub success: bool,
}
