use soroban_sdk::{Address, Env, Symbol};

pub fn deposit(env: &Env, depositor: &Address, token: &Address, amount: i128, unlock_time: u64) {
    let topics = (Symbol::new(env, "deposit"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, unlock_time));
}

pub fn withdraw(env: &Env, depositor: &Address, token: &Address, amount: i128) {
    let topics = (Symbol::new(env, "withdraw"), depositor.clone(), token.clone());
    env.events().publish(topics, amount);
}

pub fn emergency_withdraw(
    env: &Env,
    admin: &Address,
    depositor: &Address,
    token: &Address,
    amount: i128,
    unlock_time: u64,
) {
    let topics = (
        Symbol::new(env, "emrg_wdraw"),
        admin.clone(),
        depositor.clone(),
    );
    env.events().publish(topics, (token.clone(), amount, unlock_time));
}

/// Emitted once per successfully processed depositor inside `batch_emergency_withdraw`.
pub fn batch_emergency_withdraw_item(
    env: &Env,
    admin: &Address,
    depositor: &Address,
    token: &Address,
    amount: i128,
    unlock_time: u64,
) {
    emergency_withdraw(env, admin, depositor, token, amount, unlock_time);
}

pub fn admin_transfer_initiated(env: &Env, current_admin: &Address, pending_admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_init"), current_admin.clone());
    env.events().publish(topics, pending_admin.clone());
}

pub fn admin_transfer_accepted(env: &Env, new_admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_done"), new_admin.clone());
    env.events().publish(topics, ());
}

pub fn admin_transfer_cancelled(env: &Env, admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_cancel"), admin.clone());
    env.events().publish(topics, ());
}

pub fn admin_renounced(env: &Env, former_admin: &Address) {
    let topics = (Symbol::new(env, "adm_renounce"), former_admin.clone());
    env.events().publish(topics, ());
}

pub fn deposit_cancelled(
    env: &Env,
    depositor: &Address,
    token: &Address,
    amount: i128,
    penalty: i128,
) {
    let topics = (Symbol::new(env, "dep_cancel"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, penalty));
}

pub fn lock_extended(
    env: &Env,
    depositor: &Address,
    deposit_id: u32,
    old_unlock_time: u64,
    new_unlock_time: u64,
) {
    let topics = (Symbol::new(env, "lock_ext"), depositor.clone());
    env.events()
        .publish(topics, (deposit_id, old_unlock_time, new_unlock_time));
}

/// Emitted when `renew_deposit` atomically extends the lock and/or tops up the amount.
pub fn renew_deposit(
    env: &Env,
    depositor: &Address,
    token: &Address,
    new_amount: i128,
    new_unlock_time: u64,
) {
    let topics = (Symbol::new(env, "dep_renew"), depositor.clone(), token.clone());
    env.events().publish(topics, (new_amount, new_unlock_time));
}
