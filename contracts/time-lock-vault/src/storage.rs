use soroban_sdk::{Address, Env, Vec};

use crate::types::{VaultEntry, VaultKey};

// ----------------------------------------------------------------
//  Persistent storage TTL constants
// ----------------------------------------------------------------

pub const BUMP_THRESHOLD: u32 = 518_400;
pub const BUMP_TARGET: u32 = 33_000_000;
pub const MAX_DEPOSITORS_PAGE_SIZE: u32 = 100;

fn extend_ttl(env: &Env, key: &VaultKey) {
    env.storage()
        .persistent()
        .extend_ttl(key, BUMP_THRESHOLD, BUMP_TARGET);
}

// ----------------------------------------------------------------
//  Deposit counter helpers
// ----------------------------------------------------------------

pub fn next_deposit_id(env: &Env, depositor: &Address) -> u32 {
    let key = VaultKey::DepositCounter(depositor.clone());
    let id: u32 = env.storage().persistent().get(&key).unwrap_or(0);
    let next = id + 1;
    env.storage().persistent().set(&key, &next);
    extend_ttl(env, &key);
    id
}

pub fn get_deposit_ids(env: &Env, depositor: &Address) -> Vec<u32> {
    let key = VaultKey::DepositIds(depositor.clone());
    env.storage().persistent().get(&key).unwrap_or_else(|| Vec::new(env))
}

fn save_deposit_ids(env: &Env, depositor: &Address, ids: &Vec<u32>) {
    let key = VaultKey::DepositIds(depositor.clone());
    env.storage().persistent().set(&key, ids);
    extend_ttl(env, &key);
}

// ----------------------------------------------------------------
//  Deposit helpers
// ----------------------------------------------------------------

pub fn set_deposit(env: &Env, depositor: &Address, deposit_id: u32, entry: &VaultEntry) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    extend_ttl(env, &key);

    let mut ids = get_deposit_ids(env, depositor);
    let mut contains = false;
    for existing_id in ids.iter() {
        if existing_id == deposit_id {
            contains = true;
            break;
        }
    }
    if !contains {
        ids.push_back(deposit_id);
        save_deposit_ids(env, depositor, &ids);
    }
}

pub fn get_deposit(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    let entry: Option<VaultEntry> = env.storage().persistent().get(&key);
    if entry.is_some() {
        extend_ttl(env, &key);
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

    let mut ids = get_deposit_ids(env, depositor);
    let mut filtered = Vec::new(env);
    for existing_id in ids.iter() {
        if existing_id != deposit_id {
            filtered.push_back(existing_id);
        }
    }
    save_deposit_ids(env, depositor, &filtered);

    if filtered.is_empty() {
        remove_depositor(env, depositor);
    }
}

// ----------------------------------------------------------------
//  Admin helpers
// ----------------------------------------------------------------

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().persistent().set(&VaultKey::Admin, admin);
    extend_ttl(env, &VaultKey::Admin);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().persistent().get(&VaultKey::Admin)
}

pub fn set_pending_admin(env: &Env, pending: &Address) {
    env.storage().persistent().set(&VaultKey::PendingAdmin, pending);
    extend_ttl(env, &VaultKey::PendingAdmin);
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
    env.storage().persistent().set(&VaultKey::Initialized, &true);
    extend_ttl(env, &VaultKey::Initialized);
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
    extend_ttl(env, &VaultKey::MaxDeposit);
}

pub fn get_max_deposit(env: &Env) -> Option<i128> {
    env.storage().persistent().get(&VaultKey::MaxDeposit)
}

pub fn set_max_lock_secs(env: &Env, v: u64) {
    env.storage().persistent().set(&VaultKey::MaxLockSecs, &v);
    extend_ttl(env, &VaultKey::MaxLockSecs);
}

pub fn get_max_lock_secs(env: &Env) -> Option<u64> {
    env.storage().persistent().get(&VaultKey::MaxLockSecs)
}

// ----------------------------------------------------------------
//  Fee recipient helpers
// ----------------------------------------------------------------

pub fn set_fee_recipient(env: &Env, recipient: &Address) {
    env.storage().persistent().set(&VaultKey::FeeRecipient, recipient);
    extend_ttl(env, &VaultKey::FeeRecipient);
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
    env.storage().persistent().set(&VaultKey::DepositorList, list);
    extend_ttl(env, &VaultKey::DepositorList);
}

pub fn add_depositor(env: &Env, depositor: &Address) {
    let mut list = get_depositor_list(env);
    let mut already_present = false;
    for existing in list.iter() {
        if existing == *depositor {
            already_present = true;
            break;
        }
    }
    if !already_present {
        list.push_back(depositor.clone());
        save_depositor_list(env, &list);
    }
}

pub fn remove_depositor(env: &Env, depositor: &Address) {
    let list = get_depositor_list(env);
    let mut new_list = Vec::new(env);
    for addr in list.iter() {
        if addr != *depositor {
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
    let bounded_limit = limit.min(MAX_DEPOSITORS_PAGE_SIZE);
    let mut page = Vec::new(env);
    let end = (offset + bounded_limit).min(len);
    for i in offset..end {
        page.push_back(list.get(i).unwrap());
    }
    page
}
