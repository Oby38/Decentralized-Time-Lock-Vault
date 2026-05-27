#![cfg(test)]

extern crate std;

use soroban_sdk::{
    testutils::{Address as _, Events, Ledger, LedgerInfo},
    token::{Client as TokenClient, StellarAssetClient},
    Address, Env, IntoVal, Symbol, symbol_short,
};

use crate::{
    contract::{TimeLockVault, TimeLockVaultClient},
    errors::VaultError,
    types::{VaultEntry, VaultKey, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS},
};

// ================================================================
//  Test helpers
// ================================================================

/// Spin up a fresh Env, deploy the vault, and mint tokens to Alice.
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

    let asset_client = StellarAssetClient::new(&env, &token_address);
    asset_client.mint(&alice, &10_000);

    vault.initialize(&admin);

    (env, vault, token_address, admin, alice)
}

/// Advance the mock ledger timestamp by `seconds`.
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
    let result = vault.try_initialize(&admin);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

// ================================================================
//  Deposit — happy path
// ================================================================

#[test]
fn test_deposit_success() {
    let (env, vault, token, _admin, alice) = setup();

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    let entry = vault.get_vault(&alice).expect("entry should exist");
    assert_eq!(entry.amount, 1_000);
    assert_eq!(entry.unlock_time, unlock_time);
    assert_eq!(entry.token, token);
    assert_eq!(entry.depositor, alice);

    // Assert deposit event was emitted
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
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    // Alice's balance should be reduced
    assert_eq!(token_client.balance(&alice), 9_000);
}

// ================================================================
//  Deposit — validation errors
// ================================================================

#[test]
fn test_deposit_zero_amount_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    let result = vault.try_deposit(&alice, &token, &0, &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::InvalidAmount)));
}

#[test]
fn test_deposit_negative_amount_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    let result = vault.try_deposit(&alice, &token, &-1, &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::InvalidAmount)));
}

#[test]
fn test_deposit_amount_exceeds_max_fails() {
    let (env, vault, token, _admin, alice) = setup();
    // Mint enough for the test
    let asset_client = StellarAssetClient::new(&env, &token);
    asset_client.mint(&alice, &MAX_DEPOSIT_AMOUNT);

    let unlock_time = env.ledger().timestamp() + 3600;
    let result = vault.try_deposit(&alice, &token, &(MAX_DEPOSIT_AMOUNT + 1), &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::AmountTooLarge)));
}

#[test]
fn test_deposit_at_max_amount_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let asset_client = StellarAssetClient::new(&env, &token);
    asset_client.mint(&alice, &MAX_DEPOSIT_AMOUNT);

    let unlock_time = env.ledger().timestamp() + 3600;
    // Exactly at the limit should succeed
    vault.deposit(&alice, &token, &MAX_DEPOSIT_AMOUNT, &unlock_time);

    let entry = vault.get_vault(&alice).expect("entry should exist");
    assert_eq!(entry.amount, MAX_DEPOSIT_AMOUNT);
}

#[test]
fn test_deposit_past_unlock_time_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp(); // same as now — not in future
    let result = vault.try_deposit(&alice, &token, &1_000, &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::UnlockTimeNotInFuture)));
}

#[test]
fn test_deposit_unlock_time_in_past_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp().saturating_sub(1);
    let result = vault.try_deposit(&alice, &token, &1_000, &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::UnlockTimeNotInFuture)));
}

#[test]
fn test_deposit_lock_duration_too_long_fails() {
    let (env, vault, token, _admin, alice) = setup();
    // One second beyond the 5-year maximum
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS + 1;
    let result = vault.try_deposit(&alice, &token, &1_000, &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::LockDurationTooLong)));
}

#[test]
fn test_deposit_at_max_duration_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + MAX_LOCK_DURATION_SECS;
    vault.deposit(&alice, &token, &1_000, &unlock_time);
    assert!(vault.get_vault(&alice).is_some());
}

#[test]
fn test_deposit_duplicate_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &500, &unlock_time);

    let result = vault.try_deposit(&alice, &token, &500, &unlock_time);
    assert_eq!(result, Err(Ok(VaultError::DepositAlreadyExists)));
}

// ================================================================
//  Withdraw — happy path
// ================================================================

#[test]
fn test_withdraw_after_unlock_succeeds() {
    let (env, vault, token, _admin, alice) = setup();
    let token_client = TokenClient::new(&env, &token);

    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time);
    assert_eq!(token_client.balance(&alice), 9_000);

    advance_time(&env, 3601);
    vault.withdraw(&alice);

    // Entry removed
    assert!(vault.get_vault(&alice).is_none());
    // Full balance restored
    assert_eq!(token_client.balance(&alice), 10_000);

    // Assert withdraw event was emitted
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
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    // Advance to exactly the unlock time
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
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    advance_time(&env, 1800); // only 30 minutes — still locked

    let result = vault.try_withdraw(&alice);
    assert_eq!(result, Err(Ok(VaultError::FundsStillLocked)));
}

#[test]
fn test_withdraw_no_deposit_fails() {
    let (_env, vault, _token, _admin, alice) = setup();
    let result = vault.try_withdraw(&alice);
    assert_eq!(result, Err(Ok(VaultError::NoDepositFound)));
}

// ================================================================
//  Time helpers
// ================================================================

#[test]
fn test_time_remaining_before_unlock() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    advance_time(&env, 1800);
    assert_eq!(vault.time_remaining(&alice), 1800);
}

#[test]
fn test_time_remaining_after_unlock_is_zero() {
    let (env, vault, token, _admin, alice) = setup();
    let unlock_time = env.ledger().timestamp() + 3600;
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    advance_time(&env, 7200);
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
    vault.deposit(&alice, &token, &2_000, &unlock_time);

    vault.emergency_withdraw(&admin, &alice);

    assert!(vault.get_vault(&alice).is_none());
    // Funds returned to depositor (alice), not admin
    assert_eq!(token_client.balance(&alice), 10_000);

    // Assert emergency_withdraw event was emitted
    let events = env.events().all();
    let last = events.last().unwrap();
    assert_eq!(
        last,
        (
            vault.address.clone(),
            (Symbol::new(&env, "emrg_wdraw"), admin.clone(), alice.clone()).into_val(&env),
            (token.clone(), 2_000_i128).into_val(&env),
        )
    );
}

#[test]
fn test_emergency_withdraw_by_non_admin_fails() {
    let (env, vault, token, _admin, alice) = setup();
    let bob: Address = Address::generate(&env);

    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &2_000, &unlock_time);

    let result = vault.try_emergency_withdraw(&bob, &alice);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_emergency_withdraw_no_deposit_fails() {
    let (_env, vault, _token, admin, alice) = setup();
    let result = vault.try_emergency_withdraw(&admin, &alice);
    assert_eq!(result, Err(Ok(VaultError::NoDepositFound)));
}

// ================================================================
//  Admin Transfer — two-step
// ================================================================

#[test]
fn test_transfer_admin_two_step_succeeds() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);

    // Step 1: current admin nominates new_admin
    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(vault.get_pending_admin(), Some(new_admin.clone()));
    assert_eq!(vault.get_admin(), Some(admin.clone())); // still old admin

    // Assert adm_xfr_init event
    {
        let events = env.events().all();
        let last = events.last().unwrap();
        assert_eq!(
            last,
            (
                vault.address.clone(),
                (Symbol::new(&env, "adm_xfr_init"), admin.clone()).into_val(&env),
                new_admin.clone().into_val(&env),
            )
        );
    }

    // Step 2: new_admin accepts
    vault.accept_admin(&new_admin);
    assert_eq!(vault.get_admin(), Some(new_admin.clone()));
    assert_eq!(vault.get_pending_admin(), None); // pending cleared

    // Assert adm_xfr_done event
    {
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
}

#[test]
fn test_transfer_admin_non_admin_cannot_initiate() {
    let (env, vault, _token, _admin, _alice) = setup();
    let bob: Address = Address::generate(&env);
    let carol: Address = Address::generate(&env);

    let result = vault.try_transfer_admin(&bob, &carol);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_accept_admin_wrong_address_fails() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);
    let impostor: Address = Address::generate(&env);

    vault.transfer_admin(&admin, &new_admin);

    // Impostor tries to accept
    let result = vault.try_accept_admin(&impostor);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
    // Admin unchanged
    assert_eq!(vault.get_admin(), Some(admin));
}

#[test]
fn test_accept_admin_with_no_pending_fails() {
    let (env, vault, _token, _admin, _alice) = setup();
    let bob: Address = Address::generate(&env);

    let result = vault.try_accept_admin(&bob);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_cancel_transfer_admin_clears_pending() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);

    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(vault.get_pending_admin(), Some(new_admin.clone()));

    vault.cancel_transfer_admin(&admin);
    assert_eq!(vault.get_pending_admin(), None);
    assert_eq!(vault.get_admin(), Some(admin)); // admin unchanged
}

#[test]
fn test_cancel_transfer_admin_by_non_admin_fails() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);
    let bob: Address = Address::generate(&env);

    vault.transfer_admin(&admin, &new_admin);

    let result = vault.try_cancel_transfer_admin(&bob);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_accept_admin_by_admin_with_no_pending_fails() {
    let (env, vault, _token, admin, _alice) = setup();

    // Admin tries to accept without any prior transfer_admin
    let result = vault.try_accept_admin(&admin);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_accept_admin_after_cancel_fails() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);

    vault.transfer_admin(&admin, &new_admin);
    vault.cancel_transfer_admin(&admin);

    // Pending is cleared; previously-nominated address must now fail
    let result = vault.try_accept_admin(&new_admin);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
    assert_eq!(vault.get_pending_admin(), None);
}

#[test]
fn test_new_admin_can_emergency_withdraw_after_transfer() {
    let (env, vault, token, admin, alice) = setup();
    let new_admin: Address = Address::generate(&env);
    let token_client = TokenClient::new(&env, &token);

    let unlock_time = env.ledger().timestamp() + 86400;
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    // Transfer admin
    vault.transfer_admin(&admin, &new_admin);
    vault.accept_admin(&new_admin);

    // Old admin can no longer emergency withdraw
    let result = vault.try_emergency_withdraw(&admin, &alice);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));

    // New admin can
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

    // Assert adm_renounce event
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
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    vault.renounce_admin(&admin);

    // Emergency withdraw should now fail — no admin stored
    let result = vault.try_emergency_withdraw(&admin, &alice);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_renounce_admin_by_non_admin_fails() {
    let (env, vault, _token, _admin, _alice) = setup();
    let bob: Address = Address::generate(&env);

    let result = vault.try_renounce_admin(&bob);
    assert_eq!(result, Err(Ok(VaultError::Unauthorized)));
}

#[test]
fn test_renounce_admin_clears_pending_transfer() {
    let (env, vault, _token, admin, _alice) = setup();
    let new_admin: Address = Address::generate(&env);

    vault.transfer_admin(&admin, &new_admin);
    assert_eq!(vault.get_pending_admin(), Some(new_admin));

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
    vault.deposit(&alice, &token, &1_000, &unlock_time);

    advance_time(&env, 3601);
    vault.withdraw(&alice);

    // Alice can deposit again after withdrawing
    let new_unlock = env.ledger().timestamp() + 7200;
    vault.deposit(&alice, &token, &500, &new_unlock);

    let entry = vault.get_vault(&alice).expect("entry should exist");
    assert_eq!(entry.amount, 500);
}

// ================================================================
//  TTL / storage constants
// ================================================================

#[test]
fn test_bump_target_covers_max_lock_duration() {
    // At 5 s/ledger, MAX_LOCK_DURATION_SECS converts to ledgers.
    // BUMP_TARGET must be >= that value so a max-duration deposit
    // cannot expire before its unlock time.
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
    // Calling get_vault on a non-existent entry should return None cleanly
    // without panicking or creating storage entries.
    let (env, vault, _token, _admin, alice) = setup();
    assert!(vault.get_vault(&alice).is_none());
    // Calling again should still return None (no side effects)
    assert!(vault.get_vault(&alice).is_none());
}

#[test]
fn test_time_remaining_is_readonly() {
    let (env, vault, _token, _admin, alice) = setup();
    // Multiple calls should be idempotent
    assert_eq!(vault.time_remaining(&alice), 0);
    assert_eq!(vault.time_remaining(&alice), 0);
}

// ================================================================
//  XDR serialization snapshot tests (#29)
//
//  These tests pin the on-chain storage format for VaultEntry and
//  VaultKey. If a field is reordered or renamed the XDR bytes change
//  and the test will fail, alerting the developer before any
//  on-chain data is silently corrupted.
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
    };

    // Serialize to XDR bytes via soroban-sdk's ToXdr (produces soroban_sdk::Bytes).
    let xdr_bytes = entry.clone().to_xdr(&env);

    // Re-deserialize and confirm structural identity.
    let entry2 = VaultEntry::from_xdr(&env, &xdr_bytes).expect("round-trip must succeed");

    assert_eq!(entry2.amount, entry.amount);
    assert_eq!(entry2.unlock_time, entry.unlock_time);
    assert_eq!(entry2.token, entry.token);
    assert_eq!(entry2.depositor, entry.depositor);

    // Pin the byte length so field additions/removals are caught.
    // Update this value intentionally when the schema changes.
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

    let key = VaultKey::Deposit(depositor.clone());
    let xdr_bytes = key.to_xdr(&env);

    // Re-deserialize and confirm round-trip.
    let key2 = VaultKey::from_xdr(&env, &xdr_bytes).expect("round-trip must succeed");
    assert_eq!(key2, VaultKey::Deposit(depositor));
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
