// ----------------------------------------------------------------
//  Protocol Constants
// ----------------------------------------------------------------

/// Maximum deposit amount (in stroops or token base units).
pub const MAX_DEPOSIT_AMOUNT: i128 = 1_000_000_000_000_000;

/// Maximum lock duration in seconds (~5 years).
pub const MAX_LOCK_DURATION_SECS: u64 = 157_788_000;

/// Minimum lock duration: prevent trivial, pointless vaults that waste storage.
pub const MIN_LOCK_DURATION_SECS: u64 = 60;

/// Maximum depositors per `batch_emergency_withdraw` call.
///
/// Soroban's per-transaction instruction budget is ~100M instructions.
/// Each iteration performs two persistent-storage removes, one token transfer,
/// and one event publish — roughly 1–2M instructions each.
/// 25 leaves comfortable headroom for the common migration use-case.
pub const MAX_BATCH_SIZE: u32 = 25;

/// Number of seconds per ledger — Soroban ledgers are ~5 seconds apart.
pub const LEDGER_SECONDS: u64 = 5;

// ----------------------------------------------------------------
//  Storage TTL Constants
// ----------------------------------------------------------------

/// Minimum remaining ledgers before a TTL bump is triggered (~30 days).
pub const BUMP_THRESHOLD: u32 = 518_400;

/// Target ledger count to extend TTL to (~5.2 years), covering the max lock duration.
pub const BUMP_TARGET: u32 = ((MAX_LOCK_DURATION_SECS + LEDGER_SECONDS - 1) / LEDGER_SECONDS) as u32;
