use soroban_sdk::contracterror;

/// All contract-level errors.
#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
#[repr(u32)]
pub enum VaultError {
    InvalidAmount = 1,
    UnlockTimeNotInFuture = 2,
    NoDepositFound = 3,
    FundsStillLocked = 4,

    /// A deposit already exists for this address.
    DepositAlreadyExists = 5,
    LockDurationTooLong = 6,
    Unauthorized = 7,
    AmountTooLarge = 8,
    InvalidPenaltyBps = 9,

    /// The requested lock duration is shorter than the minimum allowed.
    LockDurationTooShort = 10,

    /// The nominated admin address is invalid (e.g., same as current admin).
    InvalidAdmin = 11,

    /// `batch_emergency_withdraw` was called with more than `MAX_BATCH_SIZE` depositors.
    BatchTooLarge = 12,
}
