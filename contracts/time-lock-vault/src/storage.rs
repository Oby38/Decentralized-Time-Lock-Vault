use soroban_sdk::{Address, Env, Vec};

use crate::errors::VaultError;
use crate::types::{VaultEntry, VaultKey};

// ----------------------------------------------------------------
//  Persistent storage TTL constants
// ----------------------------------------------------------------

/// Minimum remaining TTL (in ledgers) before a bump is applied (≈ 30 days at 5s/ledger).
pub const BUMP_THRESHOLD: u32 = 518_400;

/// Target TTL after a bump (≈ 5.2 years at 5s/ledger).
/// Must exceed MAX_LOCK_DURATION_SECS in ledger units so a max-duration deposit
/// cannot expire before its unlock time.
pub const BUMP_TARGET: u32 = 33_000_000;

// ----------------------------------------------------------------
//  Deposit helpers
// ----------------------------------------------------------------

pub fn set_deposit(env: &Env, depositor: &Address, entry: &VaultEntry) {
    let key = VaultKey::Deposit(depositor.clone());
//  Deposit counter helpers
// ----------------------------------------------------------------

/// Returns the next sequential deposit ID for `depositor` and increments the persistent counter.
/// The counter itself is TTL-bumped on every call.
pub fn next_deposit_id(env: &Env, depositor: &Address) -> u32 {
    let key = VaultKey::DepositCounter(depositor.clone());
    let id: u32 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(id + 1));
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    id
}

/// Returns all active deposit IDs for `depositor` by scanning the counter range.
/// IDs whose storage entries have been removed (withdrawn) are excluded.
pub fn get_deposit_ids(env: &Env, depositor: &Address) -> Vec<u32> {
    let counter_key = VaultKey::DepositCounter(depositor.clone());
    let count: u32 = env.storage().persistent().get(&counter_key).unwrap_or(0);

    let mut ids = Vec::new(env);
    for id in 0..count {
        let key = VaultKey::Deposit(depositor.clone(), id);
        if env.storage().persistent().has(&key) {
            ids.push_back(id);
        }
    }
    ids
}

// ----------------------------------------------------------------
//  Deposit helpers
// ----------------------------------------------------------------

/// Persists `entry` under `VaultKey::Deposit(depositor, deposit_id)` and bumps its TTL.
pub fn set_deposit(env: &Env, depositor: &Address, deposit_id: u32, entry: &VaultEntry) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Load deposit and bump TTL (use for mutating paths that keep the entry).
pub fn get_deposit(env: &Env, depositor: &Address) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone());
/// Loads a deposit entry and bumps its TTL if found. Use for mutating call paths.
pub fn get_deposit(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    let entry: Option<VaultEntry> = env.storage().persistent().get(&key);
    if entry.is_some() {
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    }
    entry
}

/// Load deposit without bumping TTL (use when the entry will be deleted or for reads).
pub fn get_deposit_readonly(env: &Env, depositor: &Address) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone());
    env.storage().persistent().get(&key)
}

pub fn remove_deposit(env: &Env, depositor: &Address) {
    env.storage()
        .persistent()
        .remove(&VaultKey::Deposit(depositor.clone()));
/// Loads a deposit entry without bumping TTL. Use for read-only queries to avoid extra fees.
pub fn get_deposit_readonly(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().get(&key)
}

/// Removes the deposit entry from persistent storage (called after a successful withdrawal).
pub fn remove_deposit(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().remove(&key);
}

// ----------------------------------------------------------------
//  Admin helpers
// ----------------------------------------------------------------

/// Persists `admin` under `VaultKey::Admin` and bumps TTL.
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&VaultKey::Admin, admin);
}

/// Returns the current admin address, or `None` if admin has been renounced.
pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&VaultKey::Admin)
}

/// Stores the nominated `pending` admin during a two-step transfer and bumps TTL.
pub fn set_pending_admin(env: &Env, pending: &Address) {
    env.storage().instance().set(&VaultKey::PendingAdmin, pending);
}

/// Returns the pending admin address, or `None` if no transfer is in progress.
pub fn get_pending_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&VaultKey::PendingAdmin)
}

/// Clears the pending admin entry (called on accept or cancel of a transfer).
pub fn remove_pending_admin(env: &Env) {
    env.storage().instance().remove(&VaultKey::PendingAdmin);
}

pub fn require_admin(env: &Env, caller: &Address) -> Result<(), VaultError> {
    let stored = get_admin(env).ok_or(VaultError::Unauthorized)?;
    if caller != &stored {
        return Err(VaultError::Unauthorized);
    }
    Ok(())
}

// ----------------------------------------------------------------
//  Initialized flag
// ----------------------------------------------------------------

/// Marks the contract as initialized. Called once during `initialize`.
pub fn set_initialized(env: &Env) {
    env.storage()
        .persistent()
        .set(&VaultKey::Initialized, &true);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::Initialized, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Returns `true` if `initialize` has already been called.
pub fn is_initialized(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get::<VaultKey, bool>(&VaultKey::Initialized)
        .unwrap_or(false)
}

// ----------------------------------------------------------------
//  Runtime limits helpers
// ----------------------------------------------------------------

/// Persists a runtime override for the maximum deposit amount and bumps TTL.
pub fn set_max_deposit(env: &Env, v: i128) {
    env.storage().persistent().set(&VaultKey::MaxDeposit, &v);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::MaxDeposit, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Returns the runtime-configured max deposit amount, or `None` to use the compile-time default.
pub fn get_max_deposit(env: &Env) -> Option<i128> {
    env.storage().persistent().get(&VaultKey::MaxDeposit)
}

/// Persists a runtime override for the maximum lock duration (seconds) and bumps TTL.
pub fn set_max_lock_secs(env: &Env, v: u64) {
    env.storage().persistent().set(&VaultKey::MaxLockSecs, &v);
    env.storage().persistent().extend_ttl(&VaultKey::MaxLockSecs, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Returns the runtime-configured max lock duration, or `None` to use the compile-time default.
pub fn get_max_lock_secs(env: &Env) -> Option<u64> {
    env.storage().persistent().get(&VaultKey::MaxLockSecs)
}

// ----------------------------------------------------------------
//  Fee recipient helpers
// ----------------------------------------------------------------

/// Persists the `fee_recipient` address and bumps TTL. Called once during `initialize`.
pub fn set_fee_recipient(env: &Env, recipient: &Address) {
    env.storage()
        .persistent()
        .set(&VaultKey::FeeRecipient, recipient);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::MaxLockSecs, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_max_lock_secs(env: &Env) -> Option<u64> {
    env.storage().persistent().get(&VaultKey::MaxLockSecs)
}

// ----------------------------------------------------------------
//  Fee recipient helpers
// ----------------------------------------------------------------

pub fn set_fee_recipient(env: &Env, recipient: &Address) {
    env.storage()
        .persistent()
        .set(&VaultKey::FeeRecipient, recipient);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::FeeRecipient, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Returns the fee recipient address, or `None` if not set.
pub fn get_fee_recipient(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::FeeRecipient)
}

// ----------------------------------------------------------------
//  Depositor list helpers
// ----------------------------------------------------------------

fn get_depositor_list(env: &Env) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&VaultKey::DepositorList)
        .unwrap_or_else(|| Vec::new(env))
}

fn save_depositor_list(env: &Env, list: &Vec<Address>) {
    env.storage()
        .persistent()
        .set(&VaultKey::DepositorList, list);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::DepositorList, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn add_depositor(env: &Env, depositor: &Address) {
    let mut list = get_depositor_list(env);
    list.push_back(depositor.clone());
    save_depositor_list(env, &list);
}

pub fn remove_depositor(env: &Env, depositor: &Address) {
    let list = get_depositor_list(env);
    let mut new_list: Vec<Address> = Vec::new(env);
    for addr in list.iter() {
        if &addr != depositor {
            new_list.push_back(addr);
        }
    }
    save_depositor_list(env, &new_list);
}

pub fn get_depositor_count(env: &Env) -> u32 {
    get_depositor_list(env).len()
}

pub fn get_depositors_page(env: &Env, offset: u32, limit: u32) -> Vec<Address> {
    let list = get_depositor_list(env);
    let len = list.len();
    let mut page: Vec<Address> = Vec::new(env);
    let end = (offset + limit).min(len);
    for i in offset..end {
        page.push_back(list.get(i).unwrap());
    }
    page
}
