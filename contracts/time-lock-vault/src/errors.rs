use soroban_sdk::contracterror;

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    InvalidAmount = 1,
    UnlockTimeNotInFuture = 2,
    NoDepositFound = 3,
    FundsStillLocked = 4,
    DepositAlreadyExists = 5,
    LockDurationTooLong = 6,
    Unauthorized = 7,
    AmountTooLarge = 8,
    InvalidPenaltyBps = 9,
    InvalidAdmin = 10,
    LockDurationTooShort = 11,
    ContractPaused = 12,
    /// Returned by `cancel_deposit` when the vault is already past its unlock
    /// time — the caller should use `withdraw` instead.
    FundsAlreadyUnlocked = 13,
    /// Returned by `batch_emergency_withdraw` when `depositors.len()` exceeds
    /// `MAX_BATCH_SIZE`.
    BatchTooLarge = 14,
    DepositorFrozen = 15,
    MigrationNotAllowed = 16,
}
