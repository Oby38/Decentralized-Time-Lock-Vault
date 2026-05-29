// ============================================================
//  Time-Lock Vault — Soroban Smart Contract
//  Stellar Blockchain | Soroban SDK v22
// ============================================================

#![no_std]

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
