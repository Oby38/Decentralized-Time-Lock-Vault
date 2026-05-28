use soroban_sdk::contracterror;

/// All contract-level errors.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    /// Caller tried to deposit zero or negative amount.
    InvalidAmount = 1,

    /// The requested unlock_time is not in the future.
    UnlockTimeNotInFuture = 2,

    /// No active deposit found for this address.
    NoDepositFound = 3,

    /// The lock period has not yet expired.
    FundsStillLocked = 4,

    /// A deposit already exists for this address.
    DepositAlreadyExists = 5,

    /// The requested lock duration exceeds the maximum allowed.
    LockDurationTooLong = 6,

    /// Caller is not authorized to perform this action.
    Unauthorized = 7,

    /// The deposit amount exceeds the maximum allowed per vault.
    AmountTooLarge = 8,

    /// penalty_bps exceeds 10000 (100%).
    InvalidPenaltyBps = 9,

    /// The requested lock duration is shorter than the minimum allowed.
    LockDurationTooShort = 10,

    /// The nominated admin address is invalid (e.g., same as current admin).
    InvalidAdmin = 11,

    /// `batch_emergency_withdraw` was called with more than `MAX_BATCH_SIZE` depositors.
    BatchTooLarge = 12,
}
