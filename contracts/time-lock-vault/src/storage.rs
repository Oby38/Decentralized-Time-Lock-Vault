use soroban_sdk::{Address, Env, Vec};

use crate::constants::MAX_LOCK_DURATION_SECS;
use crate::types::{LedgerVaultEntry, VaultEntry, VaultKey};

pub const LEDGER_SECONDS: u64 = 5;

pub const BUMP_THRESHOLD: u32 = 518_400;
pub const BUMP_TARGET: u32 = ((MAX_LOCK_DURATION_SECS + LEDGER_SECONDS - 1) / LEDGER_SECONDS) as u32;

// ----------------------------------------------------------------
//  TTL helper
// ----------------------------------------------------------------

pub fn extend_ttl(env: &Env, key: &VaultKey) {
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
    let key = VaultKey::ActiveDepositIds(depositor.clone());
    env.storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env))
}

fn add_active_deposit_id(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::ActiveDepositIds(depositor.clone());
    let mut ids: Vec<u32> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env));
    ids.push_back(deposit_id);
    env.storage().persistent().set(&key, &ids);
    extend_ttl(env, &key);
}

fn remove_active_deposit_id(env: &Env, depositor: &Address, deposit_id: u32) {
    let key = VaultKey::ActiveDepositIds(depositor.clone());
    let ids: Vec<u32> = env
        .storage()
        .persistent()
        .get(&key)
        .unwrap_or_else(|| Vec::new(env));
    let mut new_ids: Vec<u32> = Vec::new(env);
    for id in ids.iter() {
        if id != deposit_id {
            new_ids.push_back(id);
        }
    }
    if new_ids.is_empty() {
        env.storage().persistent().remove(&key);
    } else {
        env.storage().persistent().set(&key, &new_ids);
        extend_ttl(env, &key);
    }
}

/// Returns true if `depositor` has at least one active deposit.
/// Uses the `ActiveDepositCount` counter maintained alongside deposit ids.
pub fn has_any_deposit(env: &Env, depositor: &Address) -> bool {
    let active_key = VaultKey::ActiveDepositCount(depositor.clone());
    let active: u32 = env.storage().persistent().get(&active_key).unwrap_or(0);
    active > 0
}

pub fn inc_active_count(env: &Env, depositor: &Address) {
    let key = VaultKey::ActiveDepositCount(depositor.clone());
    let count: u32 = env.storage().persistent().get(&key).unwrap_or(0);
    env.storage().persistent().set(&key, &(count + 1));
    env.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn dec_active_count(env: &Env, depositor: &Address) {
    let key = VaultKey::ActiveDepositCount(depositor.clone());
    let count: u32 = env.storage().persistent().get(&key).unwrap_or(0);
    let new_count = count.saturating_sub(1);
    env.storage().persistent().set(&key, &new_count);
    env.storage().persistent().extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

// ----------------------------------------------------------------
//  Deposit helpers
// ----------------------------------------------------------------

pub fn set_deposit(env: &Env, depositor: &Address, deposit_id: u32, entry: &VaultEntry) {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    extend_ttl(env, &key);
    add_active_deposit_id(env, depositor, deposit_id);
    inc_active_count(env, depositor);
}

/// Retrieve the vault entry for `depositor` — does NOT bump TTL.
/// Use this for all reads (writes/mutations as well as view functions)
/// since mutations that follow will either remove the entry or replace it.
pub fn get_deposit(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    let key = VaultKey::Deposit(depositor.clone(), deposit_id);
    let entry: Option<VaultEntry> = env.storage().persistent().get(&key);
    if entry.is_some() {
        extend_ttl(env, &key);
    }
    entry
}

pub fn get_deposit_readonly(env: &Env, depositor: &Address, deposit_id: u32) -> Option<VaultEntry> {
    env.storage()
        .persistent()
        .get(&VaultKey::Deposit(depositor.clone(), deposit_id))
}

pub fn remove_deposit(env: &Env, depositor: &Address, deposit_id: u32) {
    env.storage()
        .persistent()
        .remove(&VaultKey::Deposit(depositor.clone(), deposit_id));
    remove_active_deposit_id(env, depositor, deposit_id);
    dec_active_count(env, depositor);
}

// ----------------------------------------------------------------
//  Ledger-based deposit helpers
// ----------------------------------------------------------------

pub fn set_deposit_by_ledger(env: &Env, depositor: &Address, deposit_id: u32, entry: &LedgerVaultEntry) {
    let key = VaultKey::DepositByLedger(depositor.clone(), deposit_id);
    env.storage().persistent().set(&key, entry);
    extend_ttl(env, &key);
    add_active_deposit_id(env, depositor, deposit_id);
    inc_active_count(env, depositor);
}

pub fn get_deposit_by_ledger_readonly(env: &Env, depositor: &Address, deposit_id: u32) -> Option<LedgerVaultEntry> {
    env.storage()
        .persistent()
        .get(&VaultKey::DepositByLedger(depositor.clone(), deposit_id))
}

pub fn remove_deposit_by_ledger(env: &Env, depositor: &Address, deposit_id: u32) {
    env.storage()
        .persistent()
        .remove(&VaultKey::DepositByLedger(depositor.clone(), deposit_id));
    remove_active_deposit_id(env, depositor, deposit_id);
    dec_active_count(env, depositor);
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

pub fn require_admin(env: &Env, caller: &Address) -> Result<(), crate::errors::VaultError> {
    match get_admin(env) {
        Some(admin) if admin == *caller => Ok(()),
        _ => Err(crate::errors::VaultError::Unauthorized),
    }
}

pub fn remove_admin(env: &Env) {
    env.storage().persistent().remove(&VaultKey::Admin);
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
//  Depositor index helpers
// ----------------------------------------------------------------


fn get_depositor_count_raw(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&VaultKey::DepositorCount)
        .unwrap_or(0)
}

fn set_depositor_count(env: &Env, count: u32) {
    env.storage()
        .persistent()
        .set(&VaultKey::DepositorCount, &count);
    extend_ttl(env, &VaultKey::DepositorCount);
}

fn get_depositor_at(env: &Env, slot: u32) -> Address {
    env.storage()
        .persistent()
        .get(&VaultKey::DepositorAt(slot))
        .unwrap()
}

fn set_depositor_at(env: &Env, slot: u32, addr: &Address) {
    env.storage()
        .persistent()
        .set(&VaultKey::DepositorAt(slot), addr);
    extend_ttl(env, &VaultKey::DepositorAt(slot));
}

fn remove_depositor_at(env: &Env, slot: u32) {
    env.storage()
        .persistent()
        .remove(&VaultKey::DepositorAt(slot));
}

fn get_depositor_slot(env: &Env, addr: &Address) -> Option<u32> {
    env.storage()
        .persistent()
        .get(&VaultKey::DepositorIndex(addr.clone()))
}

fn set_depositor_slot(env: &Env, addr: &Address, slot: u32) {
    env.storage()
        .persistent()
        .set(&VaultKey::DepositorIndex(addr.clone()), &slot);
    extend_ttl(env, &VaultKey::DepositorIndex(addr.clone()));
}

fn remove_depositor_slot(env: &Env, addr: &Address) {
    env.storage()
        .persistent()
        .remove(&VaultKey::DepositorIndex(addr.clone()));
}


pub fn add_depositor(env: &Env, depositor: &Address) {
    let member_key = VaultKey::DepositorMember(depositor.clone());
    if env.storage().persistent().has(&member_key) {
        return;
    }
    env.storage().persistent().set(&member_key, &true);
    env.storage()
        .persistent()
        .extend_ttl(&member_key, BUMP_THRESHOLD, BUMP_TARGET);

    let count = get_depositor_count_raw(env);
    set_depositor_at(env, count, depositor);
    set_depositor_slot(env, depositor, count);
    set_depositor_count(env, count + 1);

}

pub fn remove_depositor(env: &Env, depositor: &Address) {
    let count = get_depositor_count_raw(env);
    if count == 0 {
        return;
    }
    let slot = match get_depositor_slot(env, depositor) {
        Some(s) => s,
        None => return,
    };
    let last = count - 1;
    if slot != last {
        let last_addr = get_depositor_at(env, last);
        set_depositor_at(env, slot, &last_addr);
        set_depositor_slot(env, &last_addr, slot);
    }
    remove_depositor_at(env, last);
    remove_depositor_slot(env, depositor);
    set_depositor_count(env, last);

    let member_key = VaultKey::DepositorMember(depositor.clone());
    if env.storage().persistent().has(&member_key) {
        env.storage().persistent().remove(&member_key);
    }
}

pub fn get_depositor_count(env: &Env) -> u32 {
    get_depositor_count_raw(env)
}

// ----------------------------------------------------------------
//  Paused flag helpers
// ----------------------------------------------------------------

pub fn set_paused(env: &Env, paused: bool) {
    env.storage().persistent().set(&VaultKey::Paused, &paused);
    extend_ttl(env, &VaultKey::Paused);
}

pub fn is_paused(env: &Env) -> bool {
    env.storage()
        .persistent()
        .get::<VaultKey, bool>(&VaultKey::Paused)
        .unwrap_or(false)
}

// ----------------------------------------------------------------
//  Freeze helpers
// ----------------------------------------------------------------

pub fn set_frozen(env: &Env, depositor: &Address) {
    let key = VaultKey::DepositorFrozen(depositor.clone());
    env.storage().persistent().set(&key, &true);
    env.storage()
        .persistent()
        .extend_ttl(&key, BUMP_THRESHOLD, BUMP_TARGET);
}

pub fn is_frozen(env: &Env, depositor: &Address) -> bool {
    let key = VaultKey::DepositorFrozen(depositor.clone());
    env.storage()
        .persistent()
        .get::<VaultKey, bool>(&key)
        .unwrap_or(false)
}

pub fn remove_frozen(env: &Env, depositor: &Address) {
    let key = VaultKey::DepositorFrozen(depositor.clone());
    env.storage().persistent().remove(&key);
}

pub fn get_depositors_page(env: &Env, offset: u32, limit: u32) -> Vec<Address> {
    let count = get_depositor_count_raw(env);
    let mut page: Vec<Address> = Vec::new(env);
    let end = (offset.saturating_add(limit)).min(count);
    for i in offset..end {
        page.push_back(get_depositor_at(env, i));
    }
    page
}
