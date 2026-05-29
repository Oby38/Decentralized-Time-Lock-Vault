#![cfg(test)]

extern crate std;

use soroban_sdk::{
    testutils::{Address as _, Ledger, LedgerInfo},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, IntoVal, Symbol, Vec, symbol_short,
};

use crate::{
    constants::{MAX_BATCH_SIZE, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS},
    contract::{TimeLockVault, TimeLockVaultClient},
    errors::VaultError,
    types::WithdrawResult,
};

// ================================================================
//  Test helpers
// ================================================================

/// Returns (env, vault_client, token_address, admin, alice).
fn setup() -> (Env, TimeLockVaultClient<'static>, Address, Address, Address) {
    let env = Env::default();
    env.mock_all_auths();

    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);

    let admin: Address = Address::generate(&env);
    let alice: Address = Address::generate(&env);

    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = token_id.address();

    StellarAssetClient::new(&env, &token_address).mint(&alice, &10_000);

    vault.initialize(&admin, &None, &None);

    (env, vault, token_address, admin, alice)
}

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

    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = token_id.address();

    StellarAssetClient::new(&env, &token_address).mint(&alice, &1_000_000);
    vault.initialize(&admin, &max_deposit, &max_lock_secs);

    (env, vault, token_address, admin, alice)
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
    let (_env, vault, _token, admin, _alice) = setup();
    assert_eq!(vault.get_admin(), Some(admin));
}

#[test]
fn test_double_initialize_fails() {
    let (_env, vault, _token, admin, _alice) = setup();
    assert_eq!(vault.try_initialize(&admin, &None, &None), Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_is_initialized() {
    let env = Env::default();
    env.mock_all_auths();
    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);
    let admin: Address = Address::generate(&env);
    assert!(!vault.is_initialized());
    vault.initialize(&admin, &None, &None);
    assert!(vault.is_initialized());
    vault.renounce_admin(&admin);
    assert!(vault.is_initialized());
}

use arbitrary::{Arbitrary, Unstructured};
use rand::RngCore;

#[derive(Arbitrary, Debug)]
struct DepositParams {
    amount: i128,
    unlock_time: u64,
}

#[test]
fn test_deposit_property_validation() {
    let (env, vault, token, _admin, alice) = setup();
    let mut rng = rand::thread_rng();

    for _ in 0..100 {
        let mut bytes = [0u8; 64];
        rng.fill_bytes(&mut bytes);
        let mut unstructured = Unstructured::new(&bytes);
        let params: DepositParams = match Arbitrary::arbitrary(&mut unstructured) {
            Ok(p) => p,
            Err(_) => continue,
        };

        let now = env.ledger().timestamp();
        let max_deposit = MAX_DEPOSIT_AMOUNT;
        let max_lock = MAX_LOCK_DURATION_SECS;
        let min_lock = crate::types::MIN_LOCK_DURATION_SECS;

        let result = vault.try_deposit(&alice, &token, &params.amount, &params.unlock_time, &0);

        if params.amount <= 0 {
            assert_eq!(result, Err(Ok(VaultError::InvalidAmount)));
        } else if params.amount > max_deposit {
            assert_eq!(result, Err(Ok(VaultError::AmountTooLarge)));
        } else if params.unlock_time <= now {
            assert_eq!(result, Err(Ok(VaultError::UnlockTimeNotInFuture)));
        } else if params.unlock_time.saturating_sub(now) > max_lock {
            assert_eq!(result, Err(Ok(VaultError::LockDurationTooLong)));
        } else if params.unlock_time.saturating_sub(now) < min_lock {
            assert_eq!(result, Err(Ok(VaultError::LockDurationTooShort)));
        } else {
            // Should succeed
            assert!(result.is_ok());
            // Need to cleanup
            vault.withdraw(&alice);
        }
    }
}


#[test]
fn test_deposit_success() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    let entry = vault.get_vault(&alice).expect("entry should exist");
    assert_eq!(entry.amount, 1_000);
    assert_eq!(entry.unlock_time, unlock_time);
    assert_eq!(entry.token, token);
    assert_eq!(entry.penalty_bps, 0);

    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (symbol_short!("deposit"), alice.clone(), token.clone()).into_val(&env),
            (1_000_i128, unlock_time).into_val(&env),
        )
    );
}

#[test]
fn test_deposit_transfers_tokens_to_contract() {
    let (env, vault, token, _admin, alice) = setup();
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
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    assert_eq!(vault.try_deposit(&alice, &token, &0, &unlock_time, &0), Err(Ok(VaultError::InvalidAmount)));
}

#[test]
fn test_deposit_negative_amount_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    assert_eq!(vault.try_deposit(&alice, &token, &-1, &unlock_time, &0), Err(Ok(VaultError::InvalidAmount)));
}

#[test]
fn test_deposit_amount_exceeds_max_fails() {
    let (env, vault, token, _admin, alice) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &MAX_DEPOSIT_AMOUNT);
    let unlock_time = env.ledger().timestamp() + 3600;
    assert_eq!(vault.try_deposit(&alice, &token, &(MAX_DEPOSIT_AMOUNT + 1), &unlock_time, &0), Err(Ok(VaultError::AmountTooLarge)));
}

#[test]
fn test_deposit_at_max_amount_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    StellarAssetClient::new(&env, &token).mint(&alice, &MAX_DEPOSIT_AMOUNT);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &MAX_DEPOSIT_AMOUNT, &unlock_time, &0);
    assert_eq!(vault.get_vault(&alice).unwrap().amount, MAX_DEPOSIT_AMOUNT);
}

#[test]
fn test_deposit_minimum_amount_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;

    vault.deposit(&alice, &token, &1, &unlock_time, &0);

    let entry = vault.get_vault(&alice, &0).expect("entry should exist");
    assert_eq!(entry.amount, 1);
    assert_eq!(entry.unlock_time, unlock_time);
    assert_eq!(entry.token, token);

    // Alice started with 10_000; 1 unit transferred to contract
    assert_eq!(token_client.balance(&alice), 9_999);
    assert_eq!(token_client.balance(&vault.address), 1);
}

#[test]
fn test_deposit_past_unlock_time_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp();
    assert_eq!(vault.try_deposit(&alice, &token, &1_000, &unlock_time, &0), Err(Ok(VaultError::UnlockTimeNotInFuture)));
}

#[test]
fn test_deposit_lock_duration_too_long_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS + 1;
    assert_eq!(vault.try_deposit(&alice, &token, &1_000, &unlock_time, &0), Err(Ok(VaultError::LockDurationTooLong)));
}

#[test]
fn test_deposit_at_max_duration_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert!(vault.get_vault(&alice).is_some());
}

#[test]
fn test_deposit_duplicate_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &500, &unlock_time, &0);
    assert_eq!(vault.try_deposit(&alice, &token, &500, &unlock_time, &0), Err(Ok(VaultError::DepositAlreadyExists)));
}

#[test]
fn test_deposit_invalid_penalty_bps_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    assert_eq!(vault.try_deposit(&alice, &token, &1_000, &unlock_time, &10_001), Err(Ok(VaultError::InvalidPenaltyBps)));
}

// ================================================================
//  Withdraw — happy path
// ================================================================

#[test]
fn test_withdraw_after_unlock_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice);

    assert!(vault.get_vault(&alice).is_none());
    assert_eq!(token_client.balance(&alice), 10_000);

    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (symbol_short!("withdraw"), alice.clone(), token.clone()).into_val(&env),
            1_000_i128.into_val(&env),
        )
    );
}

#[test]
fn test_withdraw_exactly_at_unlock_time_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3600);
    vault.withdraw(&alice);
    assert!(vault.get_vault(&alice).is_none());
}

// ================================================================
//  Withdraw — error paths
// ================================================================

#[test]
fn test_withdraw_before_unlock_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 1800);
    assert_eq!(vault.try_withdraw(&alice), Err(Ok(VaultError::FundsStillLocked)));
}

#[test]
fn test_withdraw_no_deposit_fails() {
    let (_env, vault, _token, _admin, alice) = setup();
    assert_eq!(vault.try_withdraw(&alice), Err(Ok(VaultError::NoDepositFound)));
}

#[test]
fn test_redeposit_after_withdraw_succeeds() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice);
    let new_unlock = env.ledger().timestamp() + 7200;
    vault.deposit(&alice, &token, &500, &new_unlock, &0);
    assert_eq!(vault.get_vault(&alice).unwrap().amount, 500);
}

// ================================================================
//  cancel_deposit
// ================================================================

#[test]
fn test_cancel_deposit_zero_penalty_returns_full_amount() {
    let (env, vault, token, _admin, alice) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.cancel_deposit(&alice);
    assert!(vault.get_vault(&alice).is_none());
    assert_eq!(token_client.balance(&alice), 10_000);
}

#[test]
fn test_cancel_deposit_no_deposit_fails() {
    let (_env, vault, _token, _admin, alice) = setup();
    assert_eq!(vault.try_cancel_deposit(&alice), Err(Ok(VaultError::NoDepositFound)));
}

#[test]
fn test_cancel_deposit_after_unlock_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &500);
    advance_time(&env, 3601);
    assert_eq!(vault.try_cancel_deposit(&alice), Err(Ok(VaultError::FundsStillLocked)));
}

#[test]
fn test_cancel_deposit_penalty_stored_in_vault_entry() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &500);
    assert_eq!(vault.get_vault(&alice).unwrap().penalty_bps, 500);
}

// ================================================================
//  extend_lock
// ================================================================

#[test]
fn test_extend_lock_success() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let original_unlock = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &original_unlock, &0);

    let new_unlock = original_unlock + 7200;
    vault.extend_lock(&alice, &id, &new_unlock);

    let entry = vault.get_vault(&alice, &id).unwrap();
    assert_eq!(entry.unlock_time, new_unlock);
}

#[test]
fn test_extend_lock_shorten_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let original_unlock = env.ledger().timestamp() + 7200;
    let id = vault.deposit(&alice, &token, &1_000, &original_unlock, &0);

    let shorter = original_unlock - 3600;
    let result = vault.try_extend_lock(&alice, &id, &shorter);
    assert_eq!(result, Err(Ok(VaultError::LockWouldNotIncrease)));
}

#[test]
fn test_extend_lock_same_time_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock, &0);

    let result = vault.try_extend_lock(&alice, &id, &unlock);
    assert_eq!(result, Err(Ok(VaultError::LockWouldNotIncrease)));
}

#[test]
fn test_extend_lock_exceeds_max_fails() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let original_unlock = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &original_unlock, &0);

    let too_far = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS + 1;
    let result = vault.try_extend_lock(&alice, &id, &too_far);
    assert_eq!(result, Err(Ok(VaultError::LockDurationTooLong)));
}

#[test]
fn test_extend_lock_no_deposit_fails() {
    let (_env, vault, _token, _admin, alice, _fee) = setup();
    let result = vault.try_extend_lock(&alice, &0, &100_000);
    assert_eq!(result, Err(Ok(VaultError::NoDepositFound)));
}

#[test]
fn test_extend_lock_requires_depositor_auth() {
    let env = Env::default();
    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);
    let admin: Address = Address::generate(&env);
    let alice: Address = Address::generate(&env);
    let bob: Address = Address::generate(&env);
    let fee: Address = Address::generate(&env);
    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let token = token_id.address();
    StellarAssetClient::new(&env, &token).mint(&alice, &10_000);
    vault.initialize(&admin, &None, &None);

    let unlock = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock, &0);

    // Bob (not depositor) tries to extend — should fail auth
    let result = vault.try_extend_lock(&bob, &id, &(unlock + 3600));
    assert!(result.is_err());
}

#[test]
fn test_extend_lock_bumps_ttl() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock = env.ledger().timestamp() + 3600;
    let id = vault.deposit(&alice, &token, &1_000, &unlock, &0);

    let key = VaultKey::Deposit(alice.clone(), id);
    let live_before = env.storage().persistent().get_ttl(&key);

    env.ledger().set(LedgerInfo {
        timestamp: env.ledger().timestamp() + 100,
        protocol_version: env.ledger().protocol_version(),
        sequence_number: env.ledger().sequence(),
        network_id: Default::default(),
        base_reserve: 10,
        min_temp_entry_ttl: 16,
        min_persistent_entry_ttl: 4096,
        max_entry_ttl: 33_000_000,
    });

    vault.extend_lock(&alice, &id, &(unlock + 7200));

    let live_after = env.storage().persistent().get_ttl(&key);
    assert!(live_after >= live_before);
}

// ================================================================
//  Time helpers
// ================================================================

#[test]
fn test_time_remaining_before_unlock() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 1800);
    assert_eq!(vault.time_remaining(&alice), 1800);
}

#[test]
fn test_time_remaining_after_unlock_is_zero() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 7200);
    assert_eq!(vault.time_remaining(&alice), 0);
}

#[test]
fn test_time_remaining_one_second_before_unlock() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3599);
    assert_eq!(vault.time_remaining(&alice), 1);
}

#[test]
fn test_time_remaining_exactly_at_unlock_is_zero() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3600);
    assert_eq!(vault.time_remaining(&alice), 0);
}

#[test]
fn test_time_remaining_one_second_past_unlock_no_underflow() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    assert_eq!(vault.time_remaining(&alice), 0);
}

#[test]
fn test_time_remaining_no_deposit_is_zero() {
    let (_env, vault, _token, _admin, alice) = setup();
    assert_eq!(vault.time_remaining(&alice), 0);
}

#[test]
fn test_get_time_returns_ledger_timestamp() {
    let (env, vault, _token, _admin, _alice) = setup();
    assert_eq!(vault.get_time(), env.ledger().timestamp());
}

#[test]
fn test_get_constants_returns_correct_values() {
    let (_env, vault, _token, _admin, _alice) = setup();
    let (max_amount, max_duration) = vault.get_constants();
    assert_eq!(max_amount, MAX_DEPOSIT_AMOUNT);
    assert_eq!(max_duration, MAX_LOCK_DURATION_SECS);
}

// ================================================================
//  Emergency Withdrawal
// ================================================================

#[test]
fn test_emergency_withdraw_by_admin_before_unlock_succeeds() {
    let (env, vault, token, admin, alice) = setup();
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &2_000, &unlock_time, &0);
    vault.emergency_withdraw(&admin, &alice);

    assert!(vault.get_vault(&alice).is_none());
    assert_eq!(token_client.balance(&alice), 10_000);

    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (Symbol::new(&env, "emrg_wdraw"), admin.clone(), alice.clone()).into_val(&env),
            (token.clone(), 2_000_i128, unlock_time).into_val(&env),
        )
    );
}

#[test]
fn test_emergency_withdraw_by_non_admin_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let bob: Address = Address::generate(&env);
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &2_000, &unlock_time, &0);
    assert_eq!(vault.try_emergency_withdraw(&bob, &alice), Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_emergency_withdraw_no_deposit_fails() {
    let (_env, vault, _token, admin, alice) = setup();
    assert_eq!(vault.try_emergency_withdraw(&admin, &alice), Err(Ok(VaultError::NoDepositFound)));
}

// ================================================================
//  batch_emergency_withdraw
// ================================================================

#[test]
fn test_batch_emergency_withdraw_all_valid() {
    let (env, vault, token, admin, alice) = setup();
    let token_client = TokenClient::new(&env, &token);

    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&bob, &5_000);
    StellarAssetClient::new(&env, &token).mint(&carol, &5_000);

    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.deposit(&bob,   &token, &2_000, &unlock_time, &0);
    vault.deposit(&carol, &token, &3_000, &unlock_time, &0);

    let mut depositors: Vec<Address> = Vec::new(&env);
    depositors.push_back(alice.clone());
    depositors.push_back(bob.clone());
    depositors.push_back(carol.clone());

    let results = vault.batch_emergency_withdraw(&admin, &depositors);

    // All three succeed
    assert_eq!(results.len(), 3);
    assert_eq!(results.get(0).unwrap(), WithdrawResult { depositor: alice.clone(), success: true });
    assert_eq!(results.get(1).unwrap(), WithdrawResult { depositor: bob.clone(),   success: true });
    assert_eq!(results.get(2).unwrap(), WithdrawResult { depositor: carol.clone(), success: true });

    // Funds returned to each depositor
    assert_eq!(token_client.balance(&alice), 10_000);
    assert_eq!(token_client.balance(&bob),    5_000);
    assert_eq!(token_client.balance(&carol),  5_000);

    // Storage cleared for all three
    assert!(vault.get_vault(&alice).is_none());
    assert!(vault.get_vault(&bob).is_none());
    assert!(vault.get_vault(&carol).is_none());

    // Depositor list emptied
    assert_eq!(vault.get_depositor_count(), 0);

    // One emrg_wdraw event emitted per depositor — verify the last 3 events.
    let all_events = env.events().all();
    let n = all_events.len();
    assert!(n >= 3);
    for i in (n - 3)..n {
        let e = all_events.get(i).unwrap();
        let depositor = [alice.clone(), bob.clone(), carol.clone()][(i - (n - 3)) as usize].clone();
        let amount = [1_000_i128, 2_000, 3_000][(i - (n - 3)) as usize];
        // topics tuple: (Symbol("emrg_wdraw"), admin, depositor)
        assert_eq!(
            e.1,
            (Symbol::new(&env, "emrg_wdraw"),
             admin.clone(),
             depositor.clone())
                .into_val(&env),
        );
        // data tuple: (token, amount, unlock_time)
        assert_eq!(
            e.2,
            (token.clone(), amount, unlock_time).into_val(&env),
        );
    }
}

#[test]
fn test_batch_emergency_withdraw_mixed_valid_invalid() {
    let (env, vault, token, admin, alice) = setup();
    let token_client = TokenClient::new(&env, &token);

    let bob: Address = Address::generate(&env);   // no deposit
    let carol: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&carol, &5_000);

    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    // bob has no deposit
    vault.deposit(&carol, &token, &3_000, &unlock_time, &0);

    let mut depositors: Vec<Address> = Vec::new(&env);
    depositors.push_back(alice.clone());
    depositors.push_back(bob.clone());
    depositors.push_back(carol.clone());

    let results = vault.batch_emergency_withdraw(&admin, &depositors);

    assert_eq!(results.len(), 3);
    assert_eq!(results.get(0).unwrap(), WithdrawResult { depositor: alice.clone(), success: true  });
    assert_eq!(results.get(1).unwrap(), WithdrawResult { depositor: bob.clone(),   success: false });
    assert_eq!(results.get(2).unwrap(), WithdrawResult { depositor: carol.clone(), success: true  });

    // Valid depositors got their funds back
    assert_eq!(token_client.balance(&alice), 10_000);
    assert_eq!(token_client.balance(&carol),  5_000);

    // bob's balance unchanged (he had no deposit)
    assert_eq!(token_client.balance(&bob), 0);

    assert!(vault.get_vault(&alice).is_none());
    assert!(vault.get_vault(&carol).is_none());
    assert_eq!(vault.get_depositor_count(), 0);
}

#[test]
fn test_batch_emergency_withdraw_non_admin_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let bob: Address = Address::generate(&env);
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    let mut depositors: Vec<Address> = Vec::new(&env);
    depositors.push_back(alice.clone());

    assert_eq!(
        vault.try_batch_emergency_withdraw(&bob, &depositors),
        Err(Ok(VaultError::Unauthorized))
    );

    // Deposit untouched
    assert!(vault.get_vault(&alice).is_some());
}

#[test]
fn test_batch_emergency_withdraw_empty_list() {
    let (env, vault, _token, admin, _alice) = setup();
    let depositors: Vec<Address> = Vec::new(&env);
    let results = vault.batch_emergency_withdraw(&admin, &depositors);
    assert_eq!(results.len(), 0);
}

#[test]
fn test_batch_emergency_withdraw_exceeds_max_batch_size_fails() {
    let (env, vault, _token, admin, _alice) = setup();
    let mut depositors: Vec<Address> = Vec::new(&env);
    for _ in 0..=(MAX_BATCH_SIZE) {
        depositors.push_back(Address::generate(&env));
    }
    assert_eq!(
        vault.try_batch_emergency_withdraw(&admin, &depositors),
        Err(Ok(VaultError::BatchTooLarge))
    );
}

#[test]
fn test_batch_emergency_withdraw_single_auth_covers_batch() {
    // env.auths() should show exactly one auth entry for the admin,
    // not one per depositor.
    let (env, vault, token, admin, alice) = setup();
    let bob: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&bob, &5_000);

    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.deposit(&bob,   &token, &2_000, &unlock_time, &0);

    let mut depositors: Vec<Address> = Vec::new(&env);
    depositors.push_back(alice.clone());
    depositors.push_back(bob.clone());

    vault.batch_emergency_withdraw(&admin, &depositors);

    // The last recorded auth should be the admin's, not alice's or bob's.
    let auths = env.auths();
    assert_eq!(auths.last().unwrap().0, admin);
}

// ================================================================
//  Admin Transfer — two-step
// ================================================================

#[test]
fn test_transfer_admin_two_step_succeeds() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);

    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(vault.get_pending_admin(), Some(new_admin.clone()));
    assert_eq!(vault.get_admin(), Some(admin.clone())); // still old admin

    vault.accept_admin(&new_admin);
    assert_eq!(vault.get_admin(), Some(new_admin.clone()));
    assert_eq!(vault.get_pending_admin(), None);
}

#[test]
fn test_transfer_admin_non_admin_cannot_initiate() {
    let (env, vault, _token, _admin, _alice) = setup();
    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);
    assert_eq!(vault.try_transfer_admin(&bob, &carol), Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_accept_admin_wrong_address_fails() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);
    let impostor: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(vault.try_accept_admin(&impostor), Err(Ok(VaultError::Unauthorized)));
    assert_eq!(vault.get_admin(), Some(admin));
}

#[test]
fn test_accept_admin_with_no_pending_fails() {
    let (env, vault, _token, _admin, _alice) = setup();
    let bob: Address = Address::generate(&env);
    assert_eq!(vault.try_accept_admin(&bob), Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_cancel_transfer_admin_clears_pending() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    vault.cancel_transfer_admin(&admin);
    assert_eq!(vault.get_pending_admin(), None);
    assert_eq!(vault.get_admin(), Some(admin));
}

#[test]
fn test_cancel_transfer_admin_emits_event() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    vault.cancel_transfer_admin(&admin);

    let last = env.events().all().last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (Symbol::new(&env, "adm_xfr_cancel"), admin.clone()).into_val(&env),
            ().into_val(&env),
        )
    );
}

#[test]
fn test_cancel_transfer_admin_by_non_admin_fails() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);
    let bob: Address = Address::generate(&env);
    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(vault.try_cancel_transfer_admin(&bob), Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_new_admin_can_emergency_withdraw_after_transfer() {
    let (env, vault, token, admin, alice) = setup();
    let new_admin: Address = Address::generate(&env);
    let token_client = TokenClient::new(&env, &token);
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    vault.transfer_admin(&admin, &new_admin);
    vault.accept_admin(&new_admin);

    assert_eq!(vault.try_emergency_withdraw(&admin, &alice), Err(Ok(VaultError::Unauthorized)));
    vault.emergency_withdraw(&new_admin, &alice);
    assert_eq!(token_client.balance(&alice), 10_000);
}

// ================================================================
//  Admin Renounce
// ================================================================

#[test]
fn test_renounce_admin_removes_admin() {
    let (env, vault, _token, admin, _alice) = setup();
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
    let (env, vault, token, admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.renounce_admin(&admin);
    assert_eq!(vault.try_emergency_withdraw(&admin, &alice), Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_renounce_admin_by_non_admin_fails() {
    let (env, vault, _token, _admin, _alice) = setup();
    let bob: Address = Address::generate(&env);
    assert_eq!(vault.try_renounce_admin(&bob), Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_renounce_admin_clears_pending_transfer() {
    let (env, vault, _token, admin, _alice) = setup();
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
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice);

    let new_unlock = env.ledger().timestamp() + 7200;
    vault.deposit(&alice, &token, &500, &new_unlock, &0);
    assert_eq!(vault.get_vault(&alice).unwrap().amount, 500);
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
fn test_depositor_count_multiple_entries() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&bob, &5_000);
    StellarAssetClient::new(&env, &token).mint(&carol, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.deposit(&bob,   &token, &2_000, &unlock_time, &0);
    vault.deposit(&carol, &token, &3_000, &unlock_time, &0);
    assert_eq!(vault.get_depositor_count(), 3);
}

#[test]
fn test_depositor_removed_on_withdraw() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice);
    assert_eq!(vault.get_depositor_count(), 0);
}

#[test]
fn test_depositor_removed_on_emergency_withdraw() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.emergency_withdraw(&admin, &alice);
    assert_eq!(vault.get_depositor_count(), 0);
}

#[test]
fn test_pagination_offset_and_limit() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);
    StellarAssetClient::new(&env, &token).mint(&bob, &5_000);
    StellarAssetClient::new(&env, &token).mint(&carol, &5_000);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.deposit(&bob,   &token, &2_000, &unlock_time, &0);
    vault.deposit(&carol, &token, &3_000, &unlock_time, &0);

    let page1 = vault.get_depositors(&0, &2);
    assert_eq!(page1.len(), 2);

    let page2 = vault.get_depositors(&2, &2);
    assert_eq!(page2.len(), 1);
}

#[test]
fn test_pagination_offset_beyond_end_returns_empty() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(vault.get_depositors(&10, &5).len(), 0);
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

    let token_id = env.register_stellar_asset_contract_v2(admin.clone());
    let token_address = token_id.address();

    StellarAssetClient::new(&env, &token_address).mint(&alice, &1_000_000);
    vault.initialize(&admin, &max_deposit, &max_lock_secs);

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
    vault.withdraw(&alice);
    let result = vault.try_deposit(&alice, &token, &501, &(env.ledger().timestamp() + 3600), &0);
    assert_eq!(result, Err(Ok(VaultError::AmountTooLarge)));
}

#[test]
fn test_custom_max_lock_secs_enforced() {
    let (env, vault, token, _admin, alice) = setup_with_limits(None, Some(3600));
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &100, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice);
    let result = vault.try_deposit(&alice, &token, &100, &(env.ledger().timestamp() + 3601), &0);
    assert_eq!(result, Err(Ok(VaultError::LockDurationTooLong)));
}

#[test]
fn test_initialize_invalid_max_deposit_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);
    let admin: Address = Address::generate(&env);
    assert_eq!(vault.try_initialize(&admin, &Some(0_i128), &None), Err(Ok(VaultError::InvalidAmount)));
}

#[test]
fn test_initialize_invalid_max_lock_secs_fails() {
    let env = Env::default();
    env.mock_all_auths();
    let vault_id = env.register(TimeLockVault, ());
    let vault = TimeLockVaultClient::new(&env, &vault_id);
    let admin: Address = Address::generate(&env);
    assert_eq!(vault.try_initialize(&admin, &None, &Some(0_u64)), Err(Ok(VaultError::LockDurationTooLong)));
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
//  Auth assertion tests
// ================================================================

#[test]
fn test_auth_deposit_requires_depositor() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    assert_eq!(env.auths()[0].0, alice);
}

#[test]
fn test_auth_withdraw_requires_depositor() {
    let (env, vault, token, _admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    advance_time(&env, 3601);
    vault.withdraw(&alice);
    assert_eq!(env.auths()[0].0, alice);
}

#[test]
fn test_auth_emergency_withdraw_requires_admin() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);
    vault.emergency_withdraw(&admin, &alice);
    assert_eq!(env.auths()[0].0, admin);
}

#[test]
fn test_auth_renounce_admin_requires_admin() {
    let (env, vault, _token, admin, _alice) = setup();
    vault.renounce_admin(&admin);
    assert_eq!(env.auths()[0].0, admin);
}

// ================================================================
//  Emergency withdraw — post-state assertions (#27)
// ================================================================

#[test]
fn test_emergency_withdraw_clears_vault_entry() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    vault.emergency_withdraw(&admin, &alice);

    assert!(vault.get_vault(&alice).is_none());
}

// ================================================================
//  New view functions
// ================================================================

#[test]
fn test_emergency_withdraw_twice_returns_no_deposit_found() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    vault.emergency_withdraw(&admin, &alice);

    assert_eq!(
        vault.try_emergency_withdraw(&admin, &alice),
        Err(Ok(VaultError::NoDepositFound))
    );
}

#[test]
fn test_redeposit_after_emergency_withdraw_succeeds() {
    let (env, vault, token, admin, alice, _fee) = setup();
    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time, &0);

    vault.emergency_withdraw(&admin, &alice);

    let new_unlock = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &500, &new_unlock, &0);
    assert_eq!(vault.get_vault(&alice).unwrap().amount, 500);
}
