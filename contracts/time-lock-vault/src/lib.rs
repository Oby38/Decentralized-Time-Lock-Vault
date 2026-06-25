// ============================================================
//  Time-Lock Vault — Soroban Smart Contract
//  Stellar Blockchain | Soroban SDK v22
// ============================================================

#![no_std]
// Deny silent integer overflow in all arithmetic operations.
// All arithmetic must use checked, saturating, or wrapping variants.
// This catches potential overflow bugs at compile time rather than silently
// wrapping at runtime in the deterministic Soroban WASM environment.
#![deny(clippy::arithmetic_side_effects)]

// Compile-time assertion: ensure u64 is 8 bytes (closes #82)
const _: () = assert!(std::mem::size_of::<u64>() == 8);

mod constants;
mod contract;
mod errors;
mod events;
mod storage;
mod types;

pub use constants::{
    MAX_BATCH_SIZE, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS, MIN_LOCK_DURATION_SECS,
};

pub use contract::TimeLockVault;
pub use contract::TimeLockVaultClient;

#[cfg(test)]
mod test;
