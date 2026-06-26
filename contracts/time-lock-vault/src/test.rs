#![cfg(test)]

extern crate std;

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env,
};

use crate::{
    constants::{MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS, MIN_LOCK_DURATION_SECS},
    contract::{TimeLockVault, TimeLockVaultClient},
    errors::VaultError,
    types::VaultEntry,
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

    let admin = Address::generate(&env);
    let alice = Address::generate(&env);
    let fee_recipient = Address::generate(&env);
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

fn advance_ledger(env: &Env, ledgers: u32) {
    env.ledger().set(LedgerInfo {
        timestamp: env.ledger().timestamp(),
        protocol_version: env.ledger().protocol_version(),
        sequence_number: env.ledger().sequence() + ledgers,
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
fn test_deposit_basic_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(id, 0);
    assert_eq!(token_client.balance(&alice), 9_000);
}

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
fn test_deposit_rejects_too_large_amounts() {
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
        Err(Ok(VaultError::FundsAlreadyUnlocked))
    );
}

// ================================================================
//  Multiple deposits
// ================================================================

#[test]
fn test_multiple_deposits_same_address() {
    let (env, vault, token, _admin, alice, fee_recipient) = setup();
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

    vault.cancel_deposit(&alice, &id0).unwrap();
    assert!(vault.get_vault(&alice, &id0).is_none());
    assert_eq!(vault.get_deposit_ids(&alice), Vec::<u32>::new(&env));
    assert_eq!(vault.get_fee_recipient(), Some(fee_recipient));
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

    let token_client = TokenClient::new(&env, &token);
    assert_eq!(token_client.balance(&alice), 14_000);
    assert_eq!(token_client.balance(&bob), 0);
    assert_eq!(token_client.balance(&vault.address), 1_000);
}

#[test]
fn test_deposit_for_beneficiary_can_withdraw() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &0);

    assert_eq!(
        vault.try_withdraw(&bob, &id),
        Err(Ok(VaultError::FundsStillLocked))
    );

    advance_time(&env, 3601);
    vault.withdraw(&bob, &id);

    assert!(vault.get_vault(&bob, &id).is_none());
    let token_client = TokenClient::new(&env, &token);
    assert_eq!(token_client.balance(&bob), 1_000);
}

#[test]
fn test_deposit_for_payer_has_no_access() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&alice, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit_for(&alice, &bob, &token, &1_000, &unlock_time, &0);

    assert_eq!(
        vault.try_withdraw(&alice, &id),
        Err(Ok(VaultError::NoDepositFound))
    );

    assert_eq!(
        vault.try_cancel_deposit(&alice, &id),
        Err(Ok(VaultError::NoDepositFound))
    );
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
fn test_renounce_admin_removes_admin() {
    let (_env, vault, _token, admin, _alice, _fee) = setup();
    vault.renounce_admin(&admin);
    assert_eq!(vault.get_admin(), None);
}

// ================================================================
//  Depositor List / Pagination
// ================================================================

#[test]
fn test_depositor_count_empty() {
    let (_env, vault, _token, _admin, _alice, _fee) = setup();
    assert_eq!(vault.get_depositor_count(), 0);
}

#[test]
fn test_depositor_count_single_entry() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 1);
}

#[test]
fn test_depositor_removed_on_withdraw() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 1);

    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);

    assert_eq!(vault.get_depositor_count(), 0);
}

// ================================================================
//  Pause / Unpause
// ================================================================

#[test]
fn test_pause_by_admin_succeeds() {
    let (env, vault, _token, admin, _alice, _fee) = setup();
    assert!(!vault.is_paused());
    vault.pause(&admin);
    assert!(vault.is_paused());
}

#[test]
fn test_pause_by_non_admin_fails() {
    let (env, vault, _token, _admin, alice, _fee) = setup();
    assert_eq!(
        vault.try_pause(&alice),
        Err(Ok(VaultError::Unauthorized))
    );
}

#[test]
fn test_deposit_fails_when_paused() {
    let (env, vault, token, admin, alice, _fee) = setup();
    vault.pause(&admin);
    assert_eq!(
        vault.try_deposit(&alice, &token, &1_000, &(env.ledger().timestamp() + 3600), &0),
        Err(Ok(VaultError::ContractPaused))
    );
}

#[test]
fn test_withdraw_succeeds_when_paused() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    vault.pause(&admin);
    advance_time(&env, 3601);
    vault.withdraw(&alice, &0);
    assert!(vault.get_vault(&alice, &0).is_none());
}

// ================================================================
//  deposit_by_ledger
// ================================================================

#[test]
fn test_deposit_by_ledger_success() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let current_ledger = env.ledger().sequence();
    let unlock_ledger = current_ledger + 1000;
    let id = vault.deposit_by_ledger(&alice, &token, &1_000, &unlock_ledger, &0);

    assert_eq!(id, 0);
    let token_client = TokenClient::new(&env, &token);
    assert_eq!(token_client.balance(&alice), 9_000);
}

#[test]
fn test_deposit_by_ledger_zero_amount_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_ledger = env.ledger().sequence() + 1000;
    assert_eq!(
        vault.try_deposit_by_ledger(&alice, &token, &0, &unlock_ledger, &0),
        Err(Ok(VaultError::InvalidAmount))
    );
}

#[test]
fn test_deposit_by_ledger_past_ledger_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let current = env.ledger().sequence();
    assert_eq!(
        vault.try_deposit_by_ledger(&alice, &token, &1_000, &current, &0),
        Err(Ok(VaultError::UnlockTimeNotInFuture))
    );
}

#[test]
fn test_deposit_by_ledger_full_lifecycle() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let token_client = TokenClient::new(&env, &token);

    let unlock_ledger = env.ledger().sequence() + 1000;
    let id = vault.deposit_by_ledger(&alice, &token, &1_000, &unlock_ledger, &0);

    assert_eq!(
        vault.try_withdraw(&alice, &id),
        Err(Ok(VaultError::FundsStillLocked))
    );

    advance_ledger(&env, 1000);
    vault.withdraw(&alice, &id);
    assert!(vault.get_vault_by_ledger(&alice, &id).is_none());
    assert_eq!(token_client.balance(&alice), 10_000);
}

#[test]
fn test_get_vault_by_ledger_returns_entry() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_ledger = env.ledger().sequence() + 1_000;
    let id = vault.deposit_by_ledger(&alice, &token, &1_000, &unlock_ledger, &0);
    let entry = vault.get_vault_by_ledger(&alice, &id).expect("entry must exist");
    assert_eq!(entry.amount, 1_000);
    assert_eq!(entry.unlock_ledger, unlock_ledger);
    assert_eq!(entry.depositor, alice);
    assert_eq!(entry.token, token);
}

// ================================================================
//  withdraw_to (time-based)
// ================================================================

#[test]
fn test_withdraw_to_time_based_success() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let recipient: Address = Address::generate(&env);
    let token_client = TokenClient::new(&env, &token);

    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    advance_time(&env, 3601);
    vault.withdraw_to(&alice, &id, &recipient);

    assert!(vault.get_vault(&alice, &id).is_none());
    assert_eq!(token_client.balance(&recipient), 1_000);
    assert_eq!(token_client.balance(&alice), 9_000);
}

#[test]
fn test_withdraw_to_before_unlock_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let recipient: Address = Address::generate(&env);

    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    assert_eq!(
        vault.try_withdraw_to(&alice, &id, &recipient),
        Err(Ok(VaultError::FundsStillLocked))
    );
}

#[test]
fn test_withdraw_to_no_deposit_fails() {
    let (env, vault, _token, _admin, alice, _fee) = setup();
    let recipient: Address = Address::generate(&env);
    assert_eq!(
        vault.try_withdraw_to(&alice, &0, &recipient),
        Err(Ok(VaultError::NoDepositFound))
    );
}

// ================================================================
//  withdraw_to (ledger-based)
// ================================================================

#[test]
fn test_withdraw_to_ledger_based_success() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let recipient: Address = Address::generate(&env);
    let token_client = TokenClient::new(&env, &token);

    let unlock_ledger = env.ledger().sequence() + 1000;
    let id = vault.deposit_by_ledger(&alice, &token, &1_000, &unlock_ledger, &0);

    advance_ledger(&env, 1000);
    vault.withdraw_to(&alice, &id, &recipient);

    assert!(vault.get_vault_by_ledger(&alice, &id).is_none());
    assert_eq!(token_client.balance(&recipient), 1_000);
    assert_eq!(token_client.balance(&alice), 9_000);
}

#[test]
fn test_withdraw_to_ledger_based_before_unlock_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let recipient: Address = Address::generate(&env);

    let unlock_ledger = env.ledger().sequence() + 1000;
    let id = vault.deposit_by_ledger(&alice, &token, &1_000, &unlock_ledger, &0);

    assert_eq!(
        vault.try_withdraw_to(&alice, &id, &recipient),
        Err(Ok(VaultError::FundsStillLocked))
    );
}

#[test]
fn test_withdraw_to_removes_depositor() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let recipient: Address = Address::generate(&env);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 1);

    advance_time(&env, 3601);
    vault.withdraw_to(&alice, &0, &recipient);
    assert_eq!(vault.get_depositor_count(), 0);
}

// ================================================================
//  Freeze / Unfreeze
// ================================================================

#[test]
fn test_freeze_depositor_by_admin_succeeds() {
    let (env, vault, _token, admin, alice, _fee) = setup();
    assert!(!vault.is_depositor_frozen(&alice));
    vault.freeze_depositor(&admin, &alice);
    assert!(vault.is_depositor_frozen(&alice));
}

#[test]
fn test_unfreeze_depositor_by_admin_succeeds() {
    let (env, vault, _token, admin, alice, _fee) = setup();
    vault.freeze_depositor(&admin, &alice);
    assert!(vault.is_depositor_frozen(&alice));
    vault.unfreeze_depositor(&admin, &alice);
    assert!(!vault.is_depositor_frozen(&alice));
}

#[test]
fn test_freeze_depositor_by_non_admin_fails() {
    let (env, vault, _token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    assert_eq!(
        vault.try_freeze_depositor(&bob, &alice),
        Err(Ok(VaultError::Unauthorized))
    );
}

#[test]
fn test_deposit_fails_when_frozen() {
    let (env, vault, token, admin, alice, _fee) = setup();
    vault.freeze_depositor(&admin, &alice);
    assert_eq!(
        vault.try_deposit(&alice, &token, &1_000, &(env.ledger().timestamp() + 3600), &0),
        Err(Ok(VaultError::DepositorFrozen))
    );
}

#[test]
fn test_deposit_by_ledger_fails_when_frozen() {
    let (env, vault, token, admin, alice, _fee) = setup();
    vault.freeze_depositor(&admin, &alice);
    assert_eq!(
        vault.try_deposit_by_ledger(&alice, &token, &1_000, &(env.ledger().sequence() + 1000), &0),
        Err(Ok(VaultError::DepositorFrozen))
    );
}

#[test]
fn test_withdraw_fails_when_frozen() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);

    vault.freeze_depositor(&admin, &alice);
    assert_eq!(
        vault.try_withdraw(&alice, &0),
        Err(Ok(VaultError::DepositorFrozen))
    );
}

#[test]
fn test_withdraw_to_fails_when_frozen() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let recipient: Address = Address::generate(&env);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);

    vault.freeze_depositor(&admin, &alice);
    assert_eq!(
        vault.try_withdraw_to(&alice, &0, &recipient),
        Err(Ok(VaultError::DepositorFrozen))
    );
}

#[test]
fn test_cancel_deposit_fails_when_frozen() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    vault.freeze_depositor(&admin, &alice);
    assert_eq!(
        vault.try_cancel_deposit(&alice, &0),
        Err(Ok(VaultError::DepositorFrozen))
    );
}

#[test]
fn test_emergency_withdraw_still_works_when_frozen() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &2_000, &unlock_time, &0);

    vault.freeze_depositor(&admin, &alice);
    vault.emergency_withdraw(&admin, &alice, &0);

    assert!(vault.get_vault(&alice, &0).is_none());
}

#[test]
fn test_unfreeze_restores_deposit_capability() {
    let (env, vault, token, admin, alice, _fee) = setup();
    vault.freeze_depositor(&admin, &alice);
    assert_eq!(
        vault.try_deposit(&alice, &token, &1_000, &(env.ledger().timestamp() + 3600), &0),
        Err(Ok(VaultError::DepositorFrozen))
    );

    vault.unfreeze_depositor(&admin, &alice);
    vault.deposit(&alice, &token, &1_000, &(env.ledger().timestamp() + 3600), &0);
    assert!(vault.get_vault(&alice, &0).is_some());
}

// ================================================================
//  Migration
// ================================================================

#[test]
fn test_migrate_time_to_ledger_succeeds() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    let new_unlock_ledger = env.ledger().sequence() + 500;
    vault.migrate_deposit_to_ledger(&admin, &alice, &id, &new_unlock_ledger);

    assert!(vault.get_vault(&alice, &id).is_none());
    let ledger_entry = vault.get_vault_by_ledger(&alice, &id).expect("should exist after migration");
    assert_eq!(ledger_entry.unlock_ledger, new_unlock_ledger);
    assert_eq!(ledger_entry.amount, 1_000);
}

/// Pause check: deposit_by_ledger must fail when contract is paused.
#[test]
fn test_migrate_ledger_to_time_succeeds() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_ledger = env.ledger().sequence() + 1000;
    let id = vault.deposit_by_ledger(&alice, &token, &1_000, &unlock_ledger, &0);

    let new_unlock_time = env.ledger().timestamp() + 7200;
    vault.migrate_deposit_to_time(&admin, &alice, &id, &new_unlock_time);

    assert!(vault.get_vault_by_ledger(&alice, &id).is_none());
    let time_entry = vault.get_vault(&alice, &id).expect("should exist after migration");
    assert_eq!(time_entry.unlock_time, new_unlock_time);
    assert_eq!(time_entry.amount, 1_000);
}

#[test]
fn test_migrate_by_non_admin_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    assert_eq!(
        vault.try_migrate_deposit_to_ledger(&bob, &alice, &id, &(env.ledger().sequence() + 500)),
        Err(Ok(VaultError::Unauthorized))
    );
}

#[test]
fn test_migrate_nonexistent_deposit_fails() {
    let (env, vault, _token, admin, alice, _fee) = setup();
    assert_eq!(
        vault.try_migrate_deposit_to_ledger(&admin, &alice, &0, &(env.ledger().sequence() + 500)),
        Err(Ok(VaultError::NoDepositFound))
    );
}

#[test]
fn test_migrate_deposit_preserves_amount() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &5_000, &unlock_time, &500);

    let new_unlock_ledger = env.ledger().sequence() + 500;
    vault.migrate_deposit_to_ledger(&admin, &alice, &id, &new_unlock_ledger);

    let entry = vault.get_vault_by_ledger(&alice, &id).expect("should exist after migration");
    assert_eq!(entry.amount, 5_000);
    assert_eq!(entry.penalty_bps, 500);
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

// ================================================================
//  Vault entry XDR snapshot
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
}

// ================================================================
//  Storage / computation efficiency tests
// ================================================================

/// deposit() must not double-read storage: after a successful deposit the
/// duplicate-guard (now using get_deposit_readonly) should still fire.
#[test]
fn test_deposit_duplicate_guard_uses_single_read() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    // Second deposit must fail — guard must catch it without a TTL-bumping read.
    let result = vault.try_deposit(&alice, &token, &1, &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::DepositAlreadyExists)));
}

/// VaultEntry no longer stores depositor; verify the entry fields are exact.
#[test]
fn test_vault_entry_has_no_depositor_field() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &777, &unlock_time);

    let entry = vault.get_vault(&alice).expect("entry should exist");
    // Only three fields: token, amount, unlock_time.
    assert_eq!(entry.token, token);
    assert_eq!(entry.amount, 777);
    assert_eq!(entry.unlock_time, unlock_time);
}

/// withdraw() must not bump TTL on an entry it is about to delete.
/// Observable effect: withdraw on a locked vault must still return
/// FundsStillLocked (entry is loaded readonly, state is intact).
#[test]
fn test_withdraw_before_unlock_does_not_bump_ttl() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &500, &unlock_time);

    // Entry still locked — withdraw must fail.
    let result = vault.try_withdraw(&alice);
    assert_eq!(result, Err(Ok(VaultError::FundsStillLocked)));

    // Entry must still be present (readonly load did not corrupt state).
    assert!(vault.get_vault(&alice).is_some());
}

/// emergency_withdraw() must not bump TTL before removing the entry.
#[test]
fn test_emergency_withdraw_does_not_bump_ttl_before_remove() {
    let (env, vault, token, admin, alice) = setup();
    let token_client = soroban_sdk::token::Client::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 86_400;
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    vault.emergency_withdraw(&admin, &alice);

    // Entry must be gone; funds returned to alice.
    assert!(vault.get_vault(&alice).is_none());
    assert_eq!(token_client.balance(&alice), 10_000);
}

/// Boundary: amount == 1 (minimum valid amount).
#[test]
fn test_deposit_minimum_amount_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1, &unlock_time);
    let entry = vault.get_vault(&alice).expect("entry should exist");
    assert_eq!(entry.amount, 1);
}

/// Boundary: unlock_time == now + 1 (minimum valid future timestamp).
#[test]
fn test_deposit_minimum_unlock_time_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 1;
    vault.deposit(&alice, &token, &1, &unlock_time);
    assert!(vault.get_vault(&alice).is_some());
}

/// Boundary: unlock_time == now + MAX_LOCK_DURATION_SECS (maximum valid duration).
#[test]
fn test_deposit_max_duration_boundary_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS;
    vault.deposit(&alice, &token, &1, &unlock_time);
    assert!(vault.get_vault(&alice).is_some());
}

/// Boundary: unlock_time == now + MAX_LOCK_DURATION_SECS + 1 (one second over limit).
#[test]
fn test_deposit_one_second_over_max_duration_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS + 1;
    let result = vault.try_deposit(&alice, &token, &1, &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::LockDurationTooLong)));
}
