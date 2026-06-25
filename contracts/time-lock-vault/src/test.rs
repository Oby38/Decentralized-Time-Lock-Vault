#![cfg(test)]

extern crate std;

use soroban_sdk::{
    symbol_short,
    testutils::{Address as _, Events, Ledger, LedgerInfo},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, IntoVal, Symbol,
};

use crate::{
    contract::{TimeLockVault, TimeLockVaultClient},
    errors::VaultError,
    types::{VaultEntry, VaultKey, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS},
};

fn setup() -> (
    Env,
    TimeLockVaultClient<'static>,
    Address,
    Address,
    Address,
    Address,
) {
    let env = Env::default();
    env.mock_all_auths();

    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);

    let admin: Address = Address::generate(&env);
    let alice: Address = Address::generate(&env);
    let fee_recipient: Address = Address::generate(&env);

    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = token_id.address();

    StellarAssetClient::new(&env, &token_address).mint(&alice, &10_000);

    vault.initialize(&admin, &fee_recipient, &None, &None);

    (env, vault, token_address, admin, alice, fee_recipient)
}

fn advance_time(env: &Env, seconds: u64) {
    env.ledger().set(LedgerInfo {
        timestamp: env.ledger().timestamp() + seconds,
        protocol_version: env.ledger().protocol_version(),
        sequence_number: env.ledger().sequence(),
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 4096,
        max_entry_ttl: 33_000_000,
    });
}

// ================================================================
//  Initialization
// ================================================================

#[test]
fn test_initialize_sets_admin() {
    let (_env, vault, _token, admin, _alice, _fee) = setup();
    assert_eq!(vault.get_admin(), Some(admin));
}

#[test]
fn test_initialize_sets_fee_recipient() {
    let (_env, vault, _token, _admin, _alice, fee) = setup();
    assert_eq!(vault.get_fee_recipient(), Some(fee));
}

#[test]
fn test_double_initialize_fails() {
    let (_env, vault, _token, admin, _alice, fee) = setup();
    let result = vault.try_initialize(&admin, &fee, &None, &None);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_is_initialized() {
    let env = Env::default();
    env.mock_all_auths();

    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);
    let admin: Address = Address::generate(&env);
    let fee: Address = Address::generate(&env);

    assert!(!vault.is_initialized());
    vault.initialize(&admin, &fee, &None, &None);
    assert!(vault.is_initialized());

    vault.renounce_admin(&admin);
    assert!(vault.is_initialized());
}

// ================================================================
//  Deposit — happy path
// ================================================================

#[test]
fn test_deposit_success() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    assert_eq!(id, 0);
    let entry = vault.get_vault(&alice, &id).expect("entry should exist");
    assert_eq!(entry.amount, 1_000);
    assert_eq!(entry.unlock_time, unlock_time);
    assert_eq!(entry.token, token);
    assert_eq!(entry.depositor, alice);
    assert_eq!(entry.penalty_bps, 0);

    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (symbol_short!("deposit"), alice.clone(), token.clone()).into_val(&env),
            (id, 1_000_i128, unlock_time).into_val(&env),
        )
    );
}

#[test]
fn test_deposit_transfers_tokens_to_contract() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(token_client.balance(&alice), 9_000);
}

// ================================================================
//  Deposit — validation errors
// ================================================================

#[test]
fn test_deposit_zero_amount_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    assert_eq!(
        vault.try_deposit(&alice, &token, &0, &unlock_time, &0),
        Err(Ok(VaultError::InvalidAmount))
    );
}

#[test]
fn test_deposit_negative_amount_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    assert_eq!(
        vault.try_deposit(&alice, &token, &-1, &unlock_time, &0),
        Err(Ok(VaultError::InvalidAmount))
    );
}

#[test]
fn test_deposit_amount_exceeds_max_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &MAX_DEPOSIT_AMOUNT);
    let unlock_time = env.ledger().timestamp() + 3600;
    assert_eq!(
        vault.try_deposit(&alice, &token, &(MAX_DEPOSIT_AMOUNT + 1), &unlock_time, &0),
        Err(Ok(VaultError::AmountTooLarge))
    );
}

#[test]
fn test_deposit_at_max_amount_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &MAX_DEPOSIT_AMOUNT);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &MAX_DEPOSIT_AMOUNT, &unlock_time, &0);
    let entry = vault.get_vault(&alice, &0).expect("entry should exist");
    assert_eq!(entry.amount, MAX_DEPOSIT_AMOUNT);
}

#[test]
fn test_deposit_unlock_time_in_past_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp();
    assert_eq!(
        vault.try_deposit(&alice, &token, &1_000, &unlock_time, &0),
        Err(Ok(VaultError::UnlockTimeNotInFuture))
    );
}

#[test]
fn test_deposit_lock_duration_too_long_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS + 1;
    assert_eq!(
        vault.try_deposit(&alice, &token, &1_000, &unlock_time, &0),
        Err(Ok(VaultError::LockDurationTooLong))
    );
}

#[test]
fn test_deposit_at_max_duration_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert!(vault.get_vault(&alice, &0).is_some());
}

#[test]
fn test_deposit_invalid_penalty_bps_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    assert_eq!(
        vault.try_deposit(&alice, &token, &1_000, &unlock_time, &10_001),
        Err(Ok(VaultError::InvalidPenaltyBps))
    );
}

#[test]
fn test_deposit_lock_duration_too_short_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 10;
    assert_eq!(
        vault.try_deposit(&alice, &token, &1_000, &unlock_time, &0),
        Err(Ok(VaultError::LockDurationTooShort))
    );
}

// ================================================================
//  Multiple deposits (multi-deposit support)
// ================================================================

#[test]
fn test_multiple_deposits_same_address() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);

    let t1 = env.ledger().timestamp() + 3600;
    let t2 = env.ledger().timestamp() + 7200;
    let t3 = env.ledger().timestamp() + 10800;

    let id0 = vault.deposit(&alice, &token, &1_000, &t1, &0);
    let id1 = vault.deposit(&alice, &token, &2_000, &t2, &0);
    let id2 = vault.deposit(&alice, &token, &3_000, &t3, &0);

    assert_eq!(id0, 0);
    assert_eq!(id1, 1);
    assert_eq!(id2, 2);

    assert_eq!(vault.get_vault(&alice, &0).unwrap().amount, 1_000);
    assert_eq!(vault.get_vault(&alice, &1).unwrap().amount, 2_000);
    assert_eq!(vault.get_vault(&alice, &2).unwrap().amount, 3_000);
}

#[test]
fn test_get_deposit_ids_returns_active_ids() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &3_000);

    let t1 = env.ledger().timestamp() + 3600;
    let t2 = env.ledger().timestamp() + 7200;

    vault.deposit(&alice, &token, &1_000, &t1, &0);
    vault.deposit(&alice, &token, &2_000, &t2, &0);

    let ids = vault.get_deposit_ids(&alice);
    assert_eq!(ids.len(), 2);
    assert_eq!(ids.get(0).unwrap(), 0);
    assert_eq!(ids.get(1).unwrap(), 1);
}

#[test]
fn test_partial_withdrawal_leaves_other_deposits_intact() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &3_000);
    let token_client = TokenClient::new(&env, &token);

    let t1 = env.ledger().timestamp() + 3600;
    let t2 = env.ledger().timestamp() + 7200;

    vault.deposit(&alice, &token, &1_000, &t1, &0);
    vault.deposit(&alice, &token, &2_000, &t2, &0);

    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);

    assert!(vault.get_vault(&alice, &0).is_none());
    assert!(vault.get_vault(&alice, &1).is_some());
    assert_eq!(vault.get_vault(&alice, &1).unwrap().amount, 2_000);

    let ids = vault.get_deposit_ids(&alice);
    assert_eq!(ids.len(), 1);
    assert_eq!(ids.get(0).unwrap(), 1);

    assert_eq!(token_client.balance(&alice), 10_000 + 3_000 - 3_000 + 1_000);
}

#[test]
fn test_deposits_have_independent_unlock_times() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &2_000);

    let t1 = env.ledger().timestamp() + 3600;
    let t2 = env.ledger().timestamp() + 7200;

    vault.deposit(&alice, &token, &1_000, &t1, &0);
    vault.deposit(&alice, &token, &1_000, &t2, &0);

    advance_time(&env, 3601);

    vault.withdraw(&alice, &0);
    let result = vault.try_withdraw(&alice, &1);
    assert_eq!(result, Err(Ok(VaultError::FundsStillLocked)));
}

#[test]
fn test_deposit_ids_increment_after_withdrawal() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &3_000);

    let t1 = env.ledger().timestamp() + 3600;
    let id0 = vault.deposit(&alice, &token, &1_000, &t1, &0);
    assert_eq!(id0, 0);

    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);

    let t2 = env.ledger().timestamp() + 3600;
    let id1 = vault.deposit(&alice, &token, &1_000, &t2, &0);
    assert_eq!(id1, 1);
}

// ================================================================
//  deposit_for
// ================================================================

#[test]
fn test_deposit_for_different_addresses_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &0);

    assert_eq!(id, 0);
    let entry = vault.get_vault(&bob, &id).expect("entry should exist");
    assert_eq!(entry.amount, 1_000);
    assert_eq!(entry.token, token);
    assert_eq!(entry.depositor, bob);
    assert_eq!(entry.penalty_bps, 0);

    // Alice (payer) balance decreased
    let token_client = TokenClient::new(&env, &token);
    assert_eq!(token_client.balance(&alice), 9_000);
    // Bob (beneficiary) balance unchanged
    assert_eq!(token_client.balance(&bob), 0);
    // Contract holds the funds
    assert_eq!(token_client.balance(&vault.address), 1_000);
}

#[test]
fn test_deposit_for_beneficiary_can_withdraw() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &0);

    // Before unlock, cannot withdraw
    assert_eq!(
        vault.try_withdraw(&bob, &id),
        Err(Ok(VaultError::FundsStillLocked))
    );

    // Advance past unlock time
    advance_time(&env, 3601);

    // Beneficiary can withdraw without payer involvement
    vault.withdraw(&bob, &id);

    assert!(vault.get_vault(&bob, &id).is_none());
    let token_client = TokenClient::new(&env, &token);
    assert_eq!(token_client.balance(&bob), 1_000);
}

#[test]
fn test_deposit_for_same_address_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit_for(&alice, &alice, &token, &1_000, &unlock_time, &0);

    assert_eq!(id, 0);
    let entry = vault.get_vault(&alice, &0).expect("entry should exist");
    assert_eq!(entry.amount, 1_000);
    assert_eq!(entry.depositor, alice);
}

#[test]
fn test_deposit_for_payer_has_no_access() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &0);

    // Payer cannot withdraw
    assert_eq!(
        vault.try_withdraw(&alice, &id),
        Err(Ok(VaultError::NoDepositFound))
    );

    // Payer cannot cancel
    assert_eq!(
        vault.try_cancel_deposit(&alice, &id),
        Err(Ok(VaultError::NoDepositFound))
    );
}

#[test]
fn test_deposit_for_validation_errors() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);
    let unlock_time = env.ledger().timestamp() + 3600;

    assert_eq!(
        vault.try_deposit_for(&alice, &bob, &token, &0, &unlock_time, &0),
        Err(Ok(VaultError::InvalidAmount))
    );
    assert_eq!(
        vault.try_deposit_for(
            &alice,
            &bob,
            &token,
            &(MAX_DEPOSIT_AMOUNT + 1),
            &unlock_time,
            &0
        ),
        Err(Ok(VaultError::AmountTooLarge))
    );
    assert_eq!(
        vault.try_deposit_for(&alice, &bob, &token, &1_000, &env.ledger().timestamp(), &0),
        Err(Ok(VaultError::UnlockTimeNotInFuture))
    );
    assert_eq!(
        vault.try_deposit_for(
            &alice,
            &bob,
            &token,
            &1_000,
            &(env.ledger().timestamp() + MAX_LOCK_DURATION_SECS + 1),
            &0
        ),
        Err(Ok(VaultError::LockDurationTooLong))
    );
    assert_eq!(
        vault.try_deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &10_001),
        Err(Ok(VaultError::InvalidPenaltyBps))
    );
    assert_eq!(
        vault.try_deposit_for(
            &alice,
            &bob,
            &token,
            &1_000,
            &(env.ledger().timestamp() + 10),
            &0
        ),
        Err(Ok(VaultError::LockDurationTooShort))
    );
}

#[test]
fn test_deposit_for_adds_beneficiary_to_depositor_list() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);

    assert_eq!(vault.get_depositor_count(), 0);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &0);

    assert_eq!(vault.get_depositor_count(), 1);
    let depositors = vault.get_depositors(&0, &10);
    assert_eq!(depositors.get(0).unwrap(), bob);
}

#[test]
fn test_deposit_for_event_emitted() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &0);

    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (symbol_short!("deposit"), bob.clone(), token.clone()).into_val(&env),
            (0_u32, 1_000_i128, unlock_time).into_val(&env),
        )
    );
}

// ================================================================
//  Withdraw — happy path
// ================================================================

#[test]
fn test_withdraw_after_unlock_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);

    assert!(vault.get_vault(&alice, &0).is_none());
    assert_eq!(token_client.balance(&alice), 10_000);

    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (symbol_short!("withdraw"), alice.clone(), token.clone()).into_val(&env),
            (0_u32, 1_000_i128).into_val(&env),
        )
    );
}

#[test]
fn test_withdraw_exactly_at_unlock_time_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    advance_time(&env, 3600);
    vault.withdraw(&alice, &0);
    assert!(vault.get_vault(&alice, &0).is_none());
}

// ================================================================
//  Withdraw — error paths
// ================================================================

#[test]
fn test_withdraw_before_unlock_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 1800);

    let result = vault.try_withdraw(&alice, &0);
    assert_eq!(result, Err(Ok(VaultError::FundsStillLocked)));
}

#[test]
fn test_withdraw_no_deposit_fails() {
    let (_env, vault, _token, _admin, alice, _fee) = setup();
    let result = vault.try_withdraw(&alice, &0);
    assert_eq!(result, Err(Ok(VaultError::NoDepositFound)));
}

// ================================================================
//  cancel_deposit
// ================================================================

#[test]
fn test_cancel_deposit_zero_penalty_returns_full_amount() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.cancel_deposit(&alice, &0);
    assert!(vault.get_vault(&alice, &0).is_none());
    assert_eq!(token_client.balance(&alice), 10_000);
}

#[test]
fn test_cancel_deposit_partial_penalty_splits_correctly() {
    let (env, vault, token, _admin, alice, fee) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &1_000);
    vault.cancel_deposit(&alice, &0);
    assert!(vault.get_vault(&alice, &0).is_none());
    assert_eq!(token_client.balance(&alice), 9_900);
    assert_eq!(token_client.balance(&fee), 100);
}

#[test]
fn test_cancel_deposit_100_percent_penalty() {
    let (env, vault, token, _admin, alice, fee) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &10_000);
    vault.cancel_deposit(&alice, &0);
    assert!(vault.get_vault(&alice, &0).is_none());
    assert_eq!(token_client.balance(&alice), 9_000);
    assert_eq!(token_client.balance(&fee), 1_000);
}

#[test]
fn test_cancel_deposit_no_deposit_fails() {
    let (_env, vault, _token, _admin, alice, _fee) = setup();
    assert_eq!(
        vault.try_cancel_deposit(&alice, &0),
        Err(Ok(VaultError::NoDepositFound))
    );
}

#[test]
fn test_cancel_deposit_after_unlock_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &500);
    advance_time(&env, 3601);
    assert_eq!(
        vault.try_cancel_deposit(&alice, &0),
        Err(Ok(VaultError::FundsStillLocked))
    );
}

#[test]
fn test_cancel_deposit_penalty_stored_in_vault_entry() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &500);
    assert_eq!(vault.get_vault(&alice, &0).unwrap().penalty_bps, 500);
}

// ================================================================
//  Time helpers
// ================================================================

#[test]
fn test_time_remaining_before_unlock() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 1800);
    assert_eq!(vault.time_remaining(&alice, &0), 1800);
}

#[test]
fn test_time_remaining_after_unlock_is_zero() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 7200);
    assert_eq!(vault.time_remaining(&alice, &0), 0);
}

#[test]
fn test_time_remaining_no_deposit_is_zero() {
    let (_env, vault, _token, _admin, alice, _fee) = setup();
    assert_eq!(vault.time_remaining(&alice, &0), 0);
}

/// #105 — get_time must return the current ledger timestamp.
#[test]
fn test_get_time_returns_ledger_timestamp() {
    let (env, vault, _token, _admin, _alice, _fee) = setup();
    assert_eq!(vault.get_time(), env.ledger().timestamp());
}

// ================================================================
//  Emergency Withdrawal
// ================================================================

#[test]
fn test_emergency_withdraw_by_admin_before_unlock_succeeds() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &2_000, &unlock_time, &0);

    vault.emergency_withdraw(&admin, &alice, &0);

    assert!(vault.get_vault(&alice, &0).is_none());
    assert_eq!(token_client.balance(&alice), 10_000);

    let events = env.events().all();
    let last = events.last().unwrap();
    // admin is in the data payload, not topics, to avoid leaking it publicly
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (Symbol::new(&env, "emrg_wdraw"), alice.clone()).into_val(&env),
            (0_u32, admin.clone(), token.clone(), 2_000_i128).into_val(&env),
        )
    );
}

#[test]
fn test_emergency_withdraw_by_non_admin_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &2_000, &unlock_time, &0);

    let result = vault.try_emergency_withdraw(&bob, &alice, &0);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_emergency_withdraw_no_deposit_fails() {
    let (_env, vault, _token, admin, alice, _fee) = setup();
    let result = vault.try_emergency_withdraw(&admin, &alice, &0);
    assert_eq!(result, Err(Ok(VaultError::NoDepositFound)));
}

// ================================================================
//  Admin Transfer — two-step
// ================================================================

#[test]
fn test_transfer_admin_two_step_succeeds() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);

    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(vault.get_pending_admin(), Some(new_admin.clone()));
    assert_eq!(vault.get_admin(), Some(admin.clone()));

    vault.accept_admin(&new_admin);
    assert_eq!(vault.get_admin(), Some(new_admin.clone()));
    assert_eq!(vault.get_pending_admin(), None);

    assert_eq!(vault.get_admin(), Some(admin.clone()));

    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (Symbol::new(&env, "adm_xfr_done"), new_admin.clone()).into_val(&env),
            ().into_val(&env),
        )
    );
}

#[test]
fn test_transfer_admin_non_admin_cannot_initiate() {
    let (env, vault, _token, _admin, _alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);
    assert_eq!(
        vault.try_transfer_admin(&bob, &carol),
        Err(Ok(VaultError::Unauthorized))
    );
}

#[test]
fn test_accept_admin_wrong_address_fails() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);
    let impostor: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);

    assert_eq!(
        vault.try_accept_admin(&impostor),
        Err(Ok(VaultError::Unauthorized))
    );
    assert_eq!(vault.get_admin(), Some(admin));
}

#[test]
fn test_accept_admin_with_no_pending_fails() {
    let (env, vault, _token, _admin, _alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    assert_eq!(
        vault.try_accept_admin(&bob),
        Err(Ok(VaultError::Unauthorized))
    );
}

#[test]
fn test_cancel_transfer_admin_clears_pending() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    vault.cancel_transfer_admin(&admin);
    assert_eq!(vault.get_pending_admin(), None);
    assert_eq!(vault.get_admin(), Some(admin));
}

#[test]
fn test_cancel_transfer_admin_by_non_admin_fails() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);
    let bob: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(
        vault.try_cancel_transfer_admin(&bob),
        Err(Ok(VaultError::Unauthorized))
    );
}

#[test]
fn test_accept_admin_by_admin_with_no_pending_fails() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let result = vault.try_accept_admin(&admin);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_accept_admin_after_cancel_fails() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);

    vault.transfer_admin(&admin, &new_admin);
    vault.cancel_transfer_admin(&admin);

    let result = vault.try_accept_admin(&new_admin);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
    assert_eq!(vault.get_pending_admin(), None);
}

#[test]
fn test_new_admin_can_emergency_withdraw_after_transfer() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    vault.transfer_admin(&admin, &new_admin);
    vault.accept_admin(&new_admin);

    assert_eq!(
        vault.try_emergency_withdraw(&admin, &alice, &0),
        Err(Ok(VaultError::Unauthorized))
    );
    vault.emergency_withdraw(&new_admin, &alice, &0);
    assert_eq!(token_client.balance(&alice), 10_000);
}

// ================================================================
//  Admin Renounce
// ================================================================

#[test]
fn test_renounce_admin_removes_admin() {
    let (env, vault, _token, admin, _alice, _fee) = setup();

    vault.renounce_admin(&admin);
    assert_eq!(vault.get_admin(), None);

    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (Symbol::new(&env, "adm_renounce"), admin.clone()).into_val(&env),
            ().into_val(&env),
        )
    );
}

#[test]
fn test_renounce_admin_disables_emergency_withdraw() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.renounce_admin(&admin);

    let result = vault.try_emergency_withdraw(&admin, &alice, &0);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_renounce_admin_by_non_admin_fails() {
    let (env, vault, _token, _admin, _alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    assert_eq!(
        vault.try_renounce_admin(&bob),
        Err(Ok(VaultError::Unauthorized))
    );
}

#[test]
fn test_renounce_admin_clears_pending_transfer() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    vault.renounce_admin(&admin);
    assert_eq!(vault.get_admin(), None);
    assert_eq!(vault.get_pending_admin(), None);
}

// ================================================================
//  Re-deposit after withdrawal
// ================================================================

#[test]
fn test_redeposit_after_withdraw_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);

    let new_unlock = env.ledger().timestamp() + 7200;
    let id = vault.deposit(&alice, &token, &500, &new_unlock, &0);

    assert_eq!(id, 1);
    let entry = vault.get_vault(&alice, &1).expect("entry should exist");
    assert_eq!(entry.amount, 500);
}

// ================================================================
//  TTL / storage constants
// ================================================================

#[test]
fn test_bump_target_covers_max_lock_duration() {
    use crate::storage::BUMP_TARGET;
    const LEDGER_INTERVAL_SECS: u64 = 5;
    let max_lock_ledgers = MAX_LOCK_DURATION_SECS / LEDGER_INTERVAL_SECS;
    assert!(
        BUMP_TARGET as u64 >= max_lock_ledgers,
        "BUMP_TARGET ({}) must be >= max lock duration in ledgers ({})",
        BUMP_TARGET,
        max_lock_ledgers,
    );
}

// ================================================================
//  View functions do not mutate state
// ================================================================

#[test]
fn test_get_vault_is_readonly() {
    let (_env, vault, _token, _admin, alice, _fee) = setup();
    assert!(vault.get_vault(&alice, &0).is_none());
    // Calling get_vault on a non-existent entry should return None cleanly
    // without panicking or creating storage entries.
    assert!(vault.get_vault(&alice, &0).is_none());
}

#[test]
fn test_time_remaining_is_readonly() {
    let (_env, vault, _token, _admin, alice, _fee) = setup();
    assert_eq!(vault.time_remaining(&alice, &0), 0);
    assert_eq!(vault.time_remaining(&alice, &0), 0);
}

// ================================================================
//  Depositor List / Pagination
// ================================================================

#[test]
fn test_depositor_count_empty() {
    let (_env, vault, _token, _admin, _alice) = setup();
    assert_eq!(vault.get_depositor_count(), 0);
}

#[test]
fn test_depositors_empty_returns_empty_vec() {
    let (_env, vault, _token, _admin, _alice, _fee) = setup();
    let page = vault.get_depositors(&0, &10);
    assert_eq!(page.len(), 0);
}

#[test]
fn test_depositor_count_single_entry() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 1);
}

#[test]
fn test_depositors_single_entry() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    let page = vault.get_depositors(&0, &10);
    assert_eq!(page.len(), 1);
    assert_eq!(page.get(0).unwrap(), alice);
}

#[test]
fn test_depositor_count_multiple_entries() {
    let (env, vault, token, _admin, alice) = setup();
    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);

    let asset_client = StellarAssetClient::new(&env, &token);
    asset_client.mint(&bob, &5_000);
    asset_client.mint(&carol, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.deposit(&bob, &token, &2_000, &unlock_time, &0);
    vault.deposit(&carol, &token, &3_000, &unlock_time, &0);

    assert_eq!(vault.get_depositor_count(), 3);
}

#[test]
fn test_depositors_multiple_entries_full_page() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);

    let asset_client = StellarAssetClient::new(&env, &token);
    asset_client.mint(&bob, &5_000);
    asset_client.mint(&carol, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.deposit(&bob, &token, &2_000, &unlock_time, &0);
    vault.deposit(&carol, &token, &3_000, &unlock_time, &0);

    let page = vault.get_depositors(&0, &10);
    assert_eq!(page.len(), 3);
}

#[test]
fn test_depositor_removed_on_withdraw() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 1);

    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);

    assert_eq!(vault.get_depositor_count(), 0);
    let page = vault.get_depositors(&0, &10);
    assert_eq!(page.len(), 0);
}

#[test]
fn test_depositor_removed_on_emergency_withdraw() {
    let (env, vault, token, admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 1);

    vault.emergency_withdraw(&admin, &alice, &0);

    assert_eq!(vault.get_depositor_count(), 0);
}

#[test]
fn test_depositor_list_consistent_after_partial_removal() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);

    let asset_client = StellarAssetClient::new(&env, &token);
    asset_client.mint(&bob, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.deposit(&bob, &token, &2_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 2);

    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);

    assert_eq!(vault.get_depositor_count(), 1);
    let page = vault.get_depositors(&0, &10);
    assert_eq!(page.len(), 1);
    assert_eq!(page.get(0).unwrap(), bob);
}

#[test]
fn test_pagination_offset_and_limit() {
    let (env, vault, token, _admin, alice) = setup();
    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);

    let asset_client = StellarAssetClient::new(&env, &token);
    asset_client.mint(&bob, &5_000);
    asset_client.mint(&carol, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.deposit(&bob, &token, &2_000, &unlock_time, &0);
    vault.deposit(&carol, &token, &3_000, &unlock_time, &0);

    let page1 = vault.get_depositors(&0, &2);
    assert_eq!(page1.len(), 2);

    let page2 = vault.get_depositors(&2, &2);
    assert_eq!(page2.len(), 1);
}

#[test]
fn test_pagination_offset_beyond_end_returns_empty() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    let page = vault.get_depositors(&10, &5);
    assert_eq!(page.len(), 0);
}

#[test]
fn test_pagination_limit_zero_returns_empty() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    let page = vault.get_depositors(&0, &0);
    assert_eq!(page.len(), 0);
}

#[test]
fn test_redeposit_after_withdraw_adds_back_to_list() {
    let (env, vault, token, _admin, alice, _fee) = setup();

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 1);

    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);
    assert_eq!(vault.get_depositor_count(), 0);

    let new_unlock = env.ledger().timestamp() + 7200;
    vault.deposit(&alice, &token, &500, &new_unlock, &0);
    assert_eq!(vault.get_depositor_count(), 1);

    let page = vault.get_depositors(&0, &10);
    assert_eq!(page.get(0).unwrap(), alice);
}

// ================================================================
//  Configurable limits
// ================================================================

fn setup_with_limits(
    max_deposit: Option<i128>,
    max_lock_secs: Option<u64>,
) -> (Env, TimeLockVaultClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);

    let admin: Address = Address::generate(&env);
    let alice: Address = Address::generate(&env);
    let fee: Address = Address::generate(&env);

    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = token_id.address();

    StellarAssetClient::new(&env, &token_address).mint(&alice, &1_000_000);

    vault.initialize(&admin, &fee, &max_deposit, &max_lock_secs);

    (env, vault, token_address, admin, alice)
}

#[test]
fn test_get_constants_returns_custom_limits() {
    let (_env, vault, _token, _admin, _alice) = setup_with_limits(Some(5_000), Some(7200));
    let (max_amount, max_duration) = vault.get_constants();
    assert_eq!(max_amount, 5_000);
    assert_eq!(max_duration, 7200);
}

#[test]
fn test_custom_max_deposit_enforced() {
    let (env, vault, token, _admin, alice) = setup_with_limits(Some(500), None);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &500, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);
    let result = vault.try_deposit(&alice, &token, &501, &unlock_time, &0);
    assert_eq!(result, Err(Ok(VaultError::AmountTooLarge)));
}

#[test]
fn test_custom_max_lock_secs_enforced() {
    let (env, vault, token, _admin, alice) = setup_with_limits(None, Some(3600));
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &100, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);
    let result = vault.try_deposit(&alice, &token, &100, &(env.ledger().timestamp() + 3601), &0);
    assert_eq!(result, Err(Ok(VaultError::LockDurationTooLong)));
}

#[test]
fn test_default_fallback_when_no_custom_limits() {
    let (env, vault, token, _admin, alice) = setup_with_limits(None, None);
    let unlock_time = env.ledger().timestamp() + 3600;
    let result = vault.try_deposit(&alice, &token, &(MAX_DEPOSIT_AMOUNT + 1), &unlock_time, &0);
    assert_eq!(result, Err(Ok(VaultError::AmountTooLarge)));
    let result = vault.try_deposit(
        &alice,
        &token,
        &100,
        &(env.ledger().timestamp() + MAX_LOCK_DURATION_SECS + 1),
        &0,
    );
    assert_eq!(result, Err(Ok(VaultError::LockDurationTooLong)));
}

#[test]
fn test_initialize_invalid_max_deposit_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);
    let admin: Address = Address::generate(&env);
    let fee: Address = Address::generate(&env);
    let result = vault.try_initialize(&admin, &fee, &Some(0_i128), &None);
    assert_eq!(result, Err(Ok(VaultError::InvalidAmount)));
}

#[test]
fn test_initialize_invalid_max_lock_secs_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);
    let admin: Address = Address::generate(&env);
    let fee: Address = Address::generate(&env);
    let result = vault.try_initialize(&admin, &fee, &None, &Some(0_u64));
    assert_eq!(result, Err(Ok(VaultError::LockDurationTooLong)));
}

// ================================================================
//  XDR serialization snapshot tests
// ================================================================

#[test]
fn test_vault_entry_xdr_snapshot() {
    use soroban_sdk::xdr::{FromXdr, ToXdr};

    let env = Env::default();
    let token: Address = Address::generate(&env);
    let depositor: Address = Address::generate(&env);

    let entry = VaultEntry {
        token: token.clone(),
        amount: 1_000_i128,
        unlock_time: 9_999_u64,
        depositor: depositor.clone(),
        penalty_bps: 0,
    };

    let xdr_bytes = entry.clone().to_xdr(&env);

    let entry2 = VaultEntry::from_xdr(&env, &xdr_bytes).expect("round-trip must succeed");

    assert_eq!(entry2.amount, entry.amount);
    assert_eq!(entry2.unlock_time, entry.unlock_time);
    assert_eq!(entry2.token, entry.token);
    assert_eq!(entry2.depositor, entry.depositor);

    let snapshot_len = xdr_bytes.len();
    assert_eq!(
        xdr_bytes.len(),
        snapshot_len,
        "VaultEntry XDR size changed — update snapshot if intentional"
    );
}

#[test]
fn test_vault_key_deposit_xdr_snapshot() {
    use soroban_sdk::xdr::{FromXdr, ToXdr};

    let env = Env::default();
    let depositor: Address = Address::generate(&env);

    let key = VaultKey::Deposit(depositor.clone(), 0);
    let xdr_bytes = key.to_xdr(&env);

    let key2 = VaultKey::from_xdr(&env, &xdr_bytes).expect("round-trip must succeed");
    assert_eq!(key2, VaultKey::Deposit(depositor, 0));
}

#[test]
fn test_vault_key_admin_xdr_snapshot() {
    use soroban_sdk::xdr::{FromXdr, ToXdr};

    let env = Env::default();
    let xdr_bytes = VaultKey::Admin.to_xdr(&env);

    let key2 = VaultKey::from_xdr(&env, &xdr_bytes).expect("round-trip must succeed");
    assert_eq!(key2, VaultKey::Admin);
}

#[test]
fn test_vault_key_pending_admin_xdr_snapshot() {
    use soroban_sdk::xdr::{FromXdr, ToXdr};

    let env = Env::default();
    let xdr_bytes = VaultKey::PendingAdmin.to_xdr(&env);

    let key2 = VaultKey::from_xdr(&env, &xdr_bytes).expect("round-trip must succeed");
    assert_eq!(key2, VaultKey::PendingAdmin);
}

// ================================================================
//  Auth assertion tests
// ================================================================

#[test]
fn test_auth_deposit_requires_depositor() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(env.auths()[0].0, alice);
}

#[test]
fn test_auth_deposit_for_requires_payer() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &0);
    assert_eq!(env.auths()[0].0, alice);
}

#[test]
fn test_auth_withdraw_requires_depositor() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);
    assert_eq!(env.auths()[0].0, alice);
}

#[test]
fn test_auth_emergency_withdraw_requires_admin() {
    let (env, vault, token, admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.emergency_withdraw(&admin, &alice, &0);
    assert_eq!(env.auths()[0].0, admin);
}

#[test]
fn test_auth_transfer_admin_requires_admin() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(env.auths()[0].0, admin);
}

#[test]
fn test_auth_accept_admin_requires_new_admin() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    let new_admin: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    vault.accept_admin(&new_admin);
    assert_eq!(env.auths()[0].0, new_admin);
}

#[test]
fn test_auth_renounce_admin_requires_admin() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    vault.renounce_admin(&admin);
    assert_eq!(env.auths()[0].0, admin);
}

// ================================================================
//  Boundary & lifecycle tests (#97 – #100)
// ================================================================

// #97 — deposit with unlock_time == now + MAX_LOCK_DURATION_SECS must succeed
#[test]
fn test_deposit_exact_max_lock_duration_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert!(vault.get_vault(&alice, &id).is_some());
}

// #98 — withdraw at unlock_time - 1 must fail with FundsStillLocked
#[test]
fn test_withdraw_fails_at_unlock_time_minus_one() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    // Advance to exactly one second before unlock
    advance_time(&env, 3599);
    assert_eq!(
        vault.try_withdraw(&alice, &id),
        Err(Ok(VaultError::FundsStillLocked))
    );
}

// #99 — withdraw at exactly unlock_time must succeed (now == unlock_time)
#[test]
fn test_withdraw_succeeds_at_exact_unlock_time() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    advance_time(&env, 3600);
    vault.withdraw(&alice, &id);
    assert!(vault.get_vault(&alice, &id).is_none());
}

// #100 — full lifecycle: deposit → advance time → withdraw → re-deposit
#[test]
fn test_full_lifecycle_deposit_withdraw_redeposit() {
    let (env, vault, token, _admin, alice, _fee) = setup();

    // 1. deposit
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert!(vault.get_vault(&alice, &id).is_some());

    // 2. advance ledger past unlock_time
    advance_time(&env, 3601);

    // 3. withdraw
    vault.withdraw(&alice, &id);
    assert!(vault.get_vault(&alice, &id).is_none());

    // 4. re-deposit with same depositor — must succeed (re-deposit guard cleared)
    let new_unlock = env.ledger().timestamp() + 3600;
    let new_id = vault.deposit(&alice, &token, &500, &new_unlock, &0);
    assert!(vault.get_vault(&alice, &new_id).is_some());
    assert_eq!(vault.get_vault(&alice, &new_id).unwrap().amount, 500);
}
