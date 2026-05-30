use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

use crate::{
    constants::{MAX_BATCH_SIZE, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS, MIN_LOCK_DURATION_SECS},
    errors::VaultError,
    events,
    storage,
    types::{VaultEntry, WithdrawResult},
};

#[contract]
pub struct TimeLockVault;

#[contractimpl]
impl TimeLockVault {
    // ----------------------------------------------------------------
    //  Initialization
    // ----------------------------------------------------------------

    /// Initialize the contract with an admin address.
    /// Must be called once immediately after deployment.
    pub fn initialize(
        env: Env,
        admin: Address,
        max_deposit: Option<i128>,
        max_lock_secs: Option<u64>,
    ) -> Result<(), VaultError> {
        admin.require_auth();

        if storage::is_initialized(&env) {
            return Err(VaultError::AlreadyInitialized);
        }
        storage::set_admin(&env, &admin);
        storage::set_initialized(&env);

        if let Some(v) = max_deposit {
            if v <= 0 {
                return Err(VaultError::InvalidAmount);
            }
            storage::set_max_deposit(&env, v);
        }
        if let Some(v) = max_lock_secs {
            if v == 0 {
                return Err(VaultError::LockDurationTooLong);
            }
            storage::set_max_lock_secs(&env, v);
        }

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Deposit
    // ----------------------------------------------------------------

    pub fn deposit(
        env: Env,
        depositor: Address,
        token: Address,
        amount: i128,
        unlock_time: u64,
        penalty_bps: u32,
    ) -> Result<(), VaultError> {
        depositor.require_auth();

        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }
        let max_deposit = storage::get_max_deposit(&env).unwrap_or(MAX_DEPOSIT_AMOUNT);
        if amount > max_deposit {
            return Err(VaultError::AmountTooLarge);
        }
        if penalty_bps > 10_000 {
            return Err(VaultError::InvalidPenaltyBps);
        }

        let now = env.ledger().timestamp();
        if unlock_time <= now {
            return Err(VaultError::UnlockTimeNotInFuture);
        }
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        let lock_duration = unlock_time.saturating_sub(now);
        if lock_duration > max_lock {
            return Err(VaultError::LockDurationTooLong);
        }
        if lock_duration < MIN_LOCK_DURATION_SECS {
            return Err(VaultError::LockDurationTooShort);
        }

        let deposit_id = storage::next_deposit_id(&env, &depositor);

        // Check no active deposit with this id (should always be fresh, but guard anyway)
        if storage::get_deposit_readonly(&env, &depositor, deposit_id).is_some() {
            return Err(VaultError::DepositAlreadyExists);
        }

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        let entry = VaultEntry {
            token: token.clone(),
            amount,
            unlock_time,
            penalty_bps,
            beneficiary: None,
        };
        storage::set_deposit(&env, &depositor, deposit_id, &entry);
        storage::add_depositor(&env, &depositor);

        events::deposit(&env, &depositor, &token, amount, unlock_time);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Renew Deposit (issue #89)
    // ----------------------------------------------------------------

    /// Atomically extend the lock time and/or top up the amount of an active deposit.
    ///
    /// - `new_unlock_time` must be ≥ the existing unlock_time (cannot shorten lock).
    /// - `additional_amount` can be 0 (pure time extension).
    /// - Total amount after top-up must not exceed `MAX_DEPOSIT_AMOUNT`.
    pub fn renew_deposit(
        env: Env,
        depositor: Address,
        deposit_id: u32,
        additional_amount: i128,
        new_unlock_time: u64,
    ) -> Result<(), VaultError> {
        depositor.require_auth();

        if additional_amount < 0 {
            return Err(VaultError::InvalidAmount);
        }

        let mut entry = storage::get_deposit(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        if new_unlock_time < entry.unlock_time {
            return Err(VaultError::LockWouldNotIncrease);
        }

        let now = env.ledger().timestamp();
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        if new_unlock_time.saturating_sub(now) > max_lock {
            return Err(VaultError::LockDurationTooLong);
        }

        let new_total = entry.amount.checked_add(additional_amount)
            .ok_or(VaultError::AmountTooLarge)?;
        let max_deposit = storage::get_max_deposit(&env).unwrap_or(MAX_DEPOSIT_AMOUNT);
        if new_total > max_deposit {
            return Err(VaultError::AmountTooLarge);
        }

        // Transfer top-up tokens before updating state (CEI: validate first, then effects)
        if additional_amount > 0 {
            let token_client = token::Client::new(&env, &entry.token);
            token_client.transfer(&depositor, &env.current_contract_address(), &additional_amount);
        }

        entry.amount = new_total;
        entry.unlock_time = new_unlock_time;
        storage::set_deposit(&env, &depositor, deposit_id, &entry);

        events::renew_deposit(&env, &depositor, &entry.token, new_total, new_unlock_time);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Cancel Deposit (early exit with penalty)
    // ----------------------------------------------------------------

    /// Cancel an active deposit before the unlock time, paying a penalty.
    /// Penalty goes to `fee_recipient`; remainder returned to depositor.
    /// Fails with `FundsStillLocked` if already past unlock time — use `withdraw`.
    pub fn cancel_deposit(env: Env, depositor: Address, deposit_id: u32) -> Result<(), VaultError> {
        depositor.require_auth();

        let entry = storage::get_deposit_readonly(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        let now = env.ledger().timestamp();
        if now >= entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        // Checks-Effects-Interactions
        storage::remove_deposit(&env, &depositor, deposit_id);
        storage::remove_depositor(&env, &depositor);
        storage::remove_beneficiary(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        let contract = env.current_contract_address();

        let penalty: i128 = (entry.amount * entry.penalty_bps as i128) / 10_000;
        let refund = entry.amount - penalty;

        if penalty > 0 {
            let fee_recipient = storage::get_fee_recipient(&env)
                .unwrap_or_else(|| depositor.clone());
            token_client.transfer(&contract, &fee_recipient, &penalty);
        }
        if refund > 0 {
            token_client.transfer(&contract, &depositor, &refund);
        }

        events::deposit_cancelled(&env, &depositor, &entry.token, entry.amount, penalty);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Extend Lock
    // ----------------------------------------------------------------

    /// Extend the unlock time of an active deposit.
    /// `new_unlock_time` must be strictly greater than the current unlock time.
    pub fn extend_lock(
        env: Env,
        depositor: Address,
        deposit_id: u32,
        new_unlock_time: u64,
    ) -> Result<(), VaultError> {
        depositor.require_auth();

        let mut entry = storage::get_deposit(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        if new_unlock_time <= entry.unlock_time {
            return Err(VaultError::LockWouldNotIncrease);
        }

        let now = env.ledger().timestamp();
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        if new_unlock_time.saturating_sub(now) > max_lock {
            return Err(VaultError::LockDurationTooLong);
        }

        let old_unlock_time = entry.unlock_time;
        entry.unlock_time = new_unlock_time;
        storage::set_deposit(&env, &depositor, deposit_id, &entry);

        events::lock_extended(&env, &depositor, deposit_id, old_unlock_time, new_unlock_time);
        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Withdraw
    // ----------------------------------------------------------------

    pub fn withdraw(env: Env, depositor: Address, deposit_id: u32) -> Result<(), VaultError> {
        depositor.require_auth();

        let entry = storage::get_deposit_readonly(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        let now = env.ledger().timestamp();
        if now < entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        // Determine recipient: beneficiary if set, otherwise depositor
        let recipient = storage::get_beneficiary(&env, &depositor)
            .unwrap_or_else(|| depositor.clone());

        // Checks-Effects-Interactions: clear storage BEFORE external call
        storage::remove_deposit(&env, &depositor, deposit_id);
        storage::remove_depositor(&env, &depositor);
        storage::remove_beneficiary(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        token_client.transfer(&env.current_contract_address(), &recipient, &entry.amount);

        events::withdraw(&env, &depositor, &entry.token, entry.amount);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Beneficiary (issue #90)
    // ----------------------------------------------------------------

    /// Set a beneficiary for the depositor's vault.
    /// On withdrawal, funds will be sent to `beneficiary` instead of `depositor`.
    pub fn set_beneficiary(
        env: Env,
        depositor: Address,
        beneficiary: Address,
    ) -> Result<(), VaultError> {
        depositor.require_auth();
        storage::set_beneficiary(&env, &depositor, &beneficiary);
        Ok(())
    }

    /// Returns the beneficiary for `depositor`, or `None` if not set.
    pub fn get_beneficiary(env: Env, depositor: Address) -> Option<Address> {
        storage::get_beneficiary(&env, &depositor)
    }

    // ----------------------------------------------------------------
    //  Admin: Emergency Withdrawal
    // ----------------------------------------------------------------

    pub fn emergency_withdraw(
        env: Env,
        admin: Address,
        depositor: Address,
        deposit_id: u32,
    ) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        let entry = storage::get_deposit_readonly(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        // Checks-Effects-Interactions
        storage::remove_deposit(&env, &depositor, deposit_id);
        storage::remove_depositor(&env, &depositor);
        storage::remove_beneficiary(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);

        events::emergency_withdraw(&env, &admin, &depositor, &entry.token, entry.amount, entry.unlock_time);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Admin: Batch Emergency Withdrawal
    // ----------------------------------------------------------------

    pub fn batch_emergency_withdraw(
        env: Env,
        admin: Address,
        depositors: Vec<Address>,
    ) -> Result<Vec<WithdrawResult>, VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        if depositors.len() > MAX_BATCH_SIZE {
            return Err(VaultError::BatchTooLarge);
        }

        let contract = env.current_contract_address();
        let mut results: Vec<WithdrawResult> = Vec::new(&env);

        for depositor in depositors.iter() {
            // Use deposit_id 0 for batch (single-deposit-per-address model)
            let entry = match storage::get_deposit_readonly(&env, &depositor, 0) {
                Some(e) => e,
                None => {
                    results.push_back(WithdrawResult { depositor, success: false });
                    continue;
                }
            };

            storage::remove_deposit(&env, &depositor, 0);
            storage::remove_depositor(&env, &depositor);
            storage::remove_beneficiary(&env, &depositor);

            let token_client = token::Client::new(&env, &entry.token);
            token_client.transfer(&contract, &depositor, &entry.amount);

            events::batch_emergency_withdraw_item(
                &env,
                &admin,
                &depositor,
                &entry.token,
                entry.amount,
                entry.unlock_time,
            );

            results.push_back(WithdrawResult { depositor, success: true });
        }

        Ok(results)
    }

    // ----------------------------------------------------------------
    //  Admin: Two-Step Admin Transfer
    // ----------------------------------------------------------------

    pub fn transfer_admin(env: Env, admin: Address, new_admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }
        if new_admin == stored_admin {
            return Err(VaultError::InvalidAdmin);
        }
        storage::set_pending_admin(&env, &new_admin);
        events::admin_transfer_initiated(&env, &admin, &new_admin);
        Ok(())
    }

    pub fn accept_admin(env: Env, new_admin: Address) -> Result<(), VaultError> {
        new_admin.require_auth();
        let pending = storage::get_pending_admin(&env).ok_or(VaultError::Unauthorized)?;
        if new_admin != pending {
            return Err(VaultError::Unauthorized);
        }
        storage::set_admin(&env, &new_admin);
        storage::remove_pending_admin(&env);
        events::admin_transfer_accepted(&env, &new_admin);
        Ok(())
    }

    pub fn cancel_transfer_admin(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;
        storage::remove_pending_admin(&env);
        events::admin_transfer_cancelled(&env, &admin);
        Ok(())
    }

    pub fn renounce_admin(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;
        env.storage()
            .instance()
            .remove(&crate::types::VaultKey::Admin);
        storage::remove_pending_admin(&env);
        events::admin_renounced(&env, &admin);
        Ok(())
    }

    // ----------------------------------------------------------------
    //  Read-only Queries
    // ----------------------------------------------------------------

    pub fn get_vault(env: Env, depositor: Address, deposit_id: u32) -> Option<VaultEntry> {
        storage::get_deposit_readonly(&env, &depositor, deposit_id)
    }

    pub fn get_time(env: Env) -> u64 {
        env.ledger().timestamp()
    }

    pub fn time_remaining(env: Env, depositor: Address, deposit_id: u32) -> u64 {
        match storage::get_deposit_readonly(&env, &depositor, deposit_id) {
            None => 0,
            Some(entry) => {
                let now = env.ledger().timestamp();
                entry.unlock_time.saturating_sub(now)
            }
        }
    }

    pub fn has_deposit(env: Env, depositor: Address, deposit_id: u32) -> bool {
        storage::get_deposit_readonly(&env, &depositor, deposit_id).is_some()
    }

    pub fn get_admin(env: Env) -> Option<Address> {
        storage::get_admin(&env)
    }

    pub fn get_pending_admin(env: Env) -> Option<Address> {
        storage::get_pending_admin(&env)
    }

    pub fn is_admin(env: Env, address: Address) -> bool {
        storage::get_admin(&env).map_or(false, |a| a == address)
    }

    pub fn get_constants(env: Env) -> (i128, u64) {
        let max_deposit = storage::get_max_deposit(&env).unwrap_or(MAX_DEPOSIT_AMOUNT);
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        (max_deposit, max_lock)
    }

    pub fn get_fee_recipient(env: Env) -> Option<Address> {
        storage::get_fee_recipient(&env)
    }

    pub fn get_depositor_count(env: Env) -> u32 {
        storage::get_depositor_count(&env)
    }

    pub fn get_depositors(env: Env, offset: u32, limit: u32) -> Vec<Address> {
        storage::get_depositors_page(&env, offset, limit)
    }

    pub fn is_initialized(env: Env) -> bool {
        storage::is_initialized(&env)
    }
}
