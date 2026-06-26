use soroban_sdk::{Address, Env, Vec};

use crate::types::{VaultEntry, VaultKey};

// ----------------------------------------------------------------
//  Persistent storage TTL constants
// ----------------------------------------------------------------

pub const BUMP_THRESHOLD: u32 = 518_400;

/// Target TTL after a bump (≈ 5.2 years at 5s/ledger).
/// Must exceed MAX_LOCK_DURATION_SECS in ledger units (157_788_000s / 5s = 31_557_600 ledgers)
/// so a max-duration deposit cannot expire before its unlock time.
pub const BUMP_TARGET: u32 = 33_000_000;

// ----------------------------------------------------------------
//  Deposit counter helpers
// ----------------------------------------------------------------

/// Returns the next deposit ID for `depositor` and increments the counter.
pub fn next_deposit_id(env: &Env, depositor: &Address) -> u32 {
    let key = VaultKey::DepositCounter(depositor.clone());
    let id: u32 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(id + 1));
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    id
}

/// Returns all active deposit IDs for `depositor`.
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

pub fn set_deposit(env: &Env, depositor: &Address, deposit_id: u32, entry: &VaultEntry) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

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

pub fn get_deposit_readonly(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().get(&key)
}

pub fn remove_deposit(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().remove(&key);
}

// ----------------------------------------------------------------
//  Admin helpers
// ----------------------------------------------------------------

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&VaultKey::Admin, admin);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::Admin, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::Admin)
}

pub fn set_pending_admin(env: &Env, pending: &Address) {
    env.storage()
        .persistent()
        .set(&VaultKey::PendingAdmin, pending);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::PendingAdmin, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_pending_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::PendingAdmin)
}

pub fn remove_pending_admin(env: &Env) {
    env.storage().persistent().remove(&VaultKey::PendingAdmin);
}

// ----------------------------------------------------------------
//  Initialized flag
// ----------------------------------------------------------------

pub fn set_initialized(env: &Env) {
    env.storage()
        .persistent()
        .set(&VaultKey::Initialized, &true);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::Initialized, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn is_initialized(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get::<VaultKey, bool>(&VaultKey::Initialized)
        .unwrap_or(false)
}

// ----------------------------------------------------------------
//  Runtime limits helpers
// ----------------------------------------------------------------

pub fn set_max_deposit(env: &Env, v: i128) {
    env.storage().persistent().set(&VaultKey::MaxDeposit, &v);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::MaxDeposit, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_max_deposit(env: &Env) -> Option<i128> {
    env.storage().persistent().get(&VaultKey::MaxDeposit)
}

pub fn set_max_lock_secs(env: &Env, v: u64) {
    env.storage().persistent().set(&VaultKey::MaxLockSecs, &v);
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

// ----------------------------------------------------------------
//  Pause state helpers
// ----------------------------------------------------------------

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().persistent().set(&VaultKey::Paused, &paused);
    env.storage()
        .persistent()
        .extend_ttl(&VaultKey::Paused, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get::<VaultKey, bool>(&VaultKey::Paused)
        .unwrap_or(false)
}
