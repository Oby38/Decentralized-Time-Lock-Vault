use soroban_sdk::{Address, Env, Vec};

use crate::types::{VaultEntry, VaultKey};

// ----------------------------------------------------------------
//  Persistent storage TTL constants
// ----------------------------------------------------------------
// Soroban persistent storage requires explicit TTL (time-to-live)
// bump calls to keep entries alive beyond the default ledger window.

/// Minimum ledger TTL threshold before we bump (≈ 30 days at 5s/ledger).
pub const BUMP_THRESHOLD: u32 = 518_400;

/// Target TTL after a bump (≈ 5.2 years at 5s/ledger).
/// Must exceed MAX_LOCK_DURATION_SECS in ledger units (157_788_000s / 5s = 31_557_600 ledgers)
/// so a max-duration deposit cannot expire before its unlock time.
pub const BUMP_TARGET: u32 = 33_000_000;

// ----------------------------------------------------------------
//  Deposit helpers
// ----------------------------------------------------------------

/// Persist a new vault entry for `depositor`.
pub fn set_deposit(env: &Env, depositor: &Address, entry: &VaultEntry) {
    let key = VaultKey::Deposit(depositor.clone());
    env.storage().persistent().set(&key, entry);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Retrieve the vault entry for `depositor` — bumps TTL (use for writes/mutations).
pub fn get_deposit(env: &Env, depositor: &Address) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone());
    let entry: Option<VaultEntry> = env.storage().persistent().get(&key);
    if entry.is_some() {
        // Refresh TTL so active vaults never expire during state-changing calls.
        env.storage()
            .persistent()
            .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    }
    entry
}

/// Retrieve the vault entry for `depositor` — does NOT bump TTL.
/// Use this in read-only / view functions to avoid charging callers
/// for unnecessary storage write operations.
pub fn get_deposit_readonly(env: &Env, depositor: &Address) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone());
    env.storage().persistent().get(&key)
}

/// Remove the vault entry for `depositor` after a successful withdrawal.
pub fn remove_deposit(env: &Env, depositor: &Address) {
    let key = VaultKey::Deposit(depositor.clone());
    env.storage().persistent().remove(&key);
}

// ----------------------------------------------------------------
//  Admin helpers
// ----------------------------------------------------------------

/// Store the admin address (called once during initialization).
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&VaultKey::Admin, admin);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::Admin, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Retrieve the admin address.
pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::Admin)
}

/// Store a pending admin address for two-step transfer.
pub fn set_pending_admin(env: &Env, pending: &Address) {
    env.storage()
        .persistent()
        .set(&VaultKey::PendingAdmin, pending);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::PendingAdmin, BUMP_THRESHOLD, BUMP_TARGET);
}

/// Retrieve the pending admin address.
pub fn get_pending_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::PendingAdmin)
}

/// Remove the pending admin entry (after acceptance or cancellation).
pub fn remove_pending_admin(env: &Env) {
    env.storage().persistent().remove(&VaultKey::PendingAdmin);
}

// ----------------------------------------------------------------
//  Runtime limits helpers
// ----------------------------------------------------------------

pub fn set_max_deposit(env: &Env, v: i128) {
    env.storage().persistent().set(&VaultKey::MaxDeposit, &v);
    env.storage().persistent().extend_ttl(&VaultKey::MaxDeposit, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_max_deposit(env: &Env) -> Option<i128> {
    env.storage().persistent().get(&VaultKey::MaxDeposit)
}

pub fn set_max_lock_secs(env: &Env, v: u64) {
    env.storage().persistent().set(&VaultKey::MaxLockSecs, &v);
    env.storage().persistent().extend_ttl(&VaultKey::MaxLockSecs, BUMP_THRESHOLD, BUMP_TARGET);
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

/// Append `depositor` to the global depositor list.
pub fn add_depositor(env: &Env, depositor: &Address) {
    let mut list = get_depositor_list(env);
    list.push_back(depositor.clone());
    save_depositor_list(env, &list);
}

/// Remove `depositor` from the global depositor list.
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

/// Returns the total number of active depositors.
pub fn get_depositor_count(env: &Env) -> u32 {
    get_depositor_list(env).len()
}

/// Returns a page of depositors starting at `offset`, up to `limit` entries.
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
