use soroban_sdk::{symbol_short, Address, Env, Symbol};

pub fn deposit(env: &Env, depositor: &Address, token: &Address, amount: i128, unlock_time: u64) {
    let topics = (symbol_short!("deposit"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, unlock_time));
}

pub fn withdraw(env: &Env, depositor: &Address, token: &Address, amount: i128) {
    let topics = (symbol_short!("withdraw"), depositor.clone(), token.clone());
    env.events().publish(topics, amount);
}

pub fn emergency_withdraw(
    env: &Env,
    admin: &Address,
    depositor: &Address,
    token: &Address,
    amount: i128,
) {
    let topics = (
        Symbol::new(env, "emrg_wdraw"),
        admin.clone(),
        depositor.clone(),
    );
    env.events().publish(topics, (token.clone(), amount));
}

/// Emitted once per successfully processed depositor inside `batch_emergency_withdraw`.
/// Same shape as `emergency_withdraw` so event consumers need no special handling.
pub fn batch_emergency_withdraw_item(
    env: &Env,
    admin: &Address,
    depositor: &Address,
    token: &Address,
    amount: i128,
) {
    emergency_withdraw(env, admin, depositor, token, amount);
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
