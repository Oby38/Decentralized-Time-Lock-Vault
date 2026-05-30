use soroban_sdk::{Address, Env, Vec};

use crate::errors::VaultError;
use crate::types::{VaultEntry, VaultKey};

// ----------------------------------------------------------------
//  Persistent storage TTL constants
// ----------------------------------------------------------------

/// Minimum remaining TTL (in ledgers) before a bump is applied (≈ 30 days at 5s/ledger).
pub const BUMP_THRESHOLD: u32 = 518_400;

/// Target TTL after a bump (≈ 5.2 years at 5s/ledger).
pub const BUMP_TARGET: u32 = 33_000_000;

// ----------------------------------------------------------------
//  Deposit counter helpers
// ----------------------------------------------------------------

/// Returns the next sequential deposit ID for `depositor` and increments the counter.
pub fn next_deposit_id(env: &Env, depositor: &Address) -> u32 {
    let key = VaultKey::DepositCounter(depositor.clone());
    let id: u32 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(id + 1));
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
    id
}

// ----------------------------------------------------------------
//  Deposit helpers
// ----------------------------------------------------------------

/// Persists `entry` under `VaultKey::Deposit(depositor, deposit_id)` and bumps TTL.
pub fn set_deposit(env: &Env, depositor: &Address, deposit_id: u32, entry: &VaultEntry) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

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

/// Loads a deposit entry without bumping TTL. Use for read-only queries or before deletion.
pub fn get_deposit_readonly(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().get(&key)
}

/// Removes the deposit entry from persistent storage.
pub fn remove_deposit(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().remove(&key);
}

// ----------------------------------------------------------------
//  Admin helpers
// ----------------------------------------------------------------

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&VaultKey::Admin, admin);
}

pub fn get_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&VaultKey::Admin)
}

pub fn set_pending_admin(env: &Env, pending: &Address) {
    env.storage().instance().set(&VaultKey::PendingAdmin, pending);
}

pub fn get_pending_admin(env: &Env) -> Option<Address> {
    env.storage().instance().get(&VaultKey::PendingAdmin)
}

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
//  Beneficiary helpers
// ----------------------------------------------------------------

pub fn set_beneficiary(env: &Env, depositor: &Address, beneficiary: &Address) {
    let key = VaultKey::Beneficiary(depositor.clone());
    env.storage().persistent().set(&key, beneficiary);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn get_beneficiary(env: &Env, depositor: &Address) -> Option<Address> {
    env.storage()
        .persistent()
        .get(&VaultKey::Beneficiary(depositor.clone()))
}

pub fn remove_beneficiary(env: &Env, depositor: &Address) {
    env.storage()
        .persistent()
        .remove(&VaultKey::Beneficiary(depositor.clone()));
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
