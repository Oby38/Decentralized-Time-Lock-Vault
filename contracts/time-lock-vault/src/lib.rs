// ============================================================
//  Time-Lock Vault — Soroban Smart Contract
//  Stellar Blockchain | Soroban SDK v22
// ============================================================
//
//  A user deposits XLM (or any Stellar asset) into this vault
//  and specifies a future unlock timestamp. Funds cannot be
//  withdrawn until env.ledger().timestamp() >= unlock_time.
//
//  Storage layout (Persistent):
//    VaultKey::Deposit(Address) → VaultEntry { amount, unlock_time, token }
//
// ============================================================

#![no_std]

mod contract;
mod errors;
mod events;
mod storage;
mod types;

pub use contract::TimeLockVaultClient;

// Re-export the contract for Soroban registration
pub use contract::TimeLockVault;

#[cfg(test)]
mod test;
