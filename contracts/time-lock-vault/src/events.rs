use soroban_sdk::{Address, Env, Symbol, symbol_short};

// ----------------------------------------------------------------
//  Event helpers
// ----------------------------------------------------------------
// symbol_short! enforces the 32-byte topic limit at compile time.
// Symbol::new is used for strings > 9 chars that still fit in 32 bytes.

/// Emitted when a user successfully locks funds.
pub fn deposit(env: &Env, depositor: &Address, token: &Address, amount: i128, unlock_time: u64) {
    let topics = (symbol_short!("deposit"), depositor.clone(), token.clone());
    env.events().publish(topics, (amount, unlock_time));
}

/// Emitted when a user successfully withdraws unlocked funds.
pub fn withdraw(env: &Env, depositor: &Address, token: &Address, amount: i128) {
    let topics = (symbol_short!("withdraw"), depositor.clone(), token.clone());
    env.events().publish(topics, amount);
}

/// Emitted when the admin performs an emergency withdrawal.
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

/// Emitted when the current admin initiates an admin transfer.
pub fn admin_transfer_initiated(env: &Env, current_admin: &Address, pending_admin: &Address) {
    let topics = (
        Symbol::new(env, "adm_xfr_init"),
        current_admin.clone(),
    );
    env.events().publish(topics, pending_admin.clone());
}

/// Emitted when the pending admin accepts and becomes the new admin.
pub fn admin_transfer_accepted(env: &Env, new_admin: &Address) {
    let topics = (Symbol::new(env, "adm_xfr_done"), new_admin.clone());
    env.events().publish(topics, ());
}

/// Emitted when the admin renounces their role (sets admin to a dead address).
pub fn admin_renounced(env: &Env, former_admin: &Address) {
    let topics = (Symbol::new(env, "adm_renounce"), former_admin.clone());
    env.events().publish(topics, ());
}

/// Emitted when a depositor cancels early and pays a penalty.
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
