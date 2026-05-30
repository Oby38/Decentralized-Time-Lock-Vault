use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

use crate::{
    errors::VaultError,
    events,
    storage,
    constants::{MAX_BATCH_SIZE, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS, MIN_LOCK_DURATION_SECS},
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
    ///
    /// # Arguments
    /// * `admin`         — Address that gains emergency-withdrawal and admin privileges.
    /// * `max_deposit`   — Optional runtime override for the maximum deposit amount.
    /// * `max_lock_secs` — Optional runtime override for the maximum lock duration.
    ///
    /// # Errors
    /// * `AlreadyInitialized` — Contract has already been initialized.
    pub fn initialize(
        env: Env,
        admin: Address,
        max_deposit: Option<i128>,
        max_lock_secs: Option<u64>,
    ) -> Result<(), VaultError> {
        admin.require_auth();

        if storage::is_initialized(&env) {
            return Err(VaultError::Unauthorized);
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

        if storage::get_deposit_readonly(&env, &depositor).is_some() {
            return Err(VaultError::DepositAlreadyExists);
        }

        let token_client = token::Client::new(&env, &token);
        debug_assert!(amount > 0, "transfer amount must be positive");
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        let entry = VaultEntry {
            token: token.clone(),
            amount,
            unlock_time,
            penalty_bps,
        };
        storage::set_deposit(&env, &depositor, &entry);
        storage::add_depositor(&env, &depositor);

        events::deposit(&env, &depositor, &token, amount, unlock_time);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Cancel Deposit (early exit with penalty)
    // ----------------------------------------------------------------

    /// Cancel an active deposit before the unlock time, paying a penalty.
    /// Penalty goes to `fee_recipient`; remainder returned to depositor.
    /// Fails with `FundsStillLocked` if already past unlock time — use `withdraw`.
    pub fn cancel_deposit(env: Env, depositor: Address) -> Result<(), VaultError> {
        depositor.require_auth();

        let entry = storage::get_deposit(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        let now = env.ledger().timestamp();
        if now >= entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        // Checks-Effects-Interactions
        storage::remove_deposit(&env, &depositor);
        storage::remove_depositor(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        let contract = env.current_contract_address();

        let penalty: i128 = (entry.amount * entry.penalty_bps as i128) / 10_000;
        let refund = entry.amount - penalty;

        if penalty > 0 {
            let fee_recipient = storage::get_fee_recipient(&env)
                .unwrap_or_else(|| depositor.clone());
            debug_assert!(penalty > 0, "transfer amount must be positive");
            token_client.transfer(&contract, &fee_recipient, &penalty);
        }
        if refund > 0 {
            debug_assert!(refund > 0, "transfer amount must be positive");
            token_client.transfer(&contract, &depositor, &refund);
        }

        events::deposit_cancelled(&env, &depositor, &entry.token, entry.amount, penalty);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Extend Lock
    // ----------------------------------------------------------------

    /// Extend the unlock time of an active deposit.
    /// `new_unlock_time` must be strictly greater than the current unlock time
    /// and must not exceed `now + max_lock_secs`.
    pub fn extend_lock(
        env: Env,
        depositor: Address,
        new_unlock_time: u64,
    ) -> Result<(), VaultError> {
        depositor.require_auth();

        let mut entry = storage::get_deposit(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        if new_unlock_time <= entry.unlock_time {
            return Err(VaultError::LockWouldNotIncrease);
        }

        let now = env.ledger().timestamp();
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        if new_unlock_time.saturating_sub(now) > max_lock {
            return Err(VaultError::LockDurationTooLong);
        }

        entry.unlock_time = new_unlock_time;
        // set_deposit increments the counter — but this is an update, not a new deposit.
        // Write directly to avoid double-counting.
        let key = crate::types::VaultKey::Deposit(depositor.clone());
        env.storage().persistent().set(&key, &entry);
        env.storage()
            .persistent()
            .extend_ttl(&key, storage::BUMP_THRESHOLD, storage::BUMP_TARGET);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Withdraw
    // ----------------------------------------------------------------

    pub fn withdraw(env: Env, depositor: Address) -> Result<(), VaultError> {
        depositor.require_auth();

        let entry = storage::get_deposit_readonly(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        let now = env.ledger().timestamp();
        if now < entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        // Checks-Effects-Interactions: clear storage BEFORE external call
        storage::remove_deposit(&env, &depositor);
        storage::remove_depositor(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        debug_assert!(entry.amount > 0, "transfer amount must be positive");
        token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);

        events::withdraw(&env, &depositor, &entry.token, entry.amount);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Admin: Emergency Withdrawal
    // ----------------------------------------------------------------

    pub fn emergency_withdraw(
        env: Env,
        admin: Address,
        depositor: Address,
    ) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        let entry = storage::get_deposit_readonly(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        // Checks-Effects-Interactions
        storage::remove_deposit(&env, &depositor);
        storage::remove_depositor(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        debug_assert!(entry.amount > 0, "transfer amount must be positive");
        token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);

        events::emergency_withdraw(&env, &admin, &depositor, &entry.token, entry.amount, entry.unlock_time);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Admin: Batch Emergency Withdrawal
    // ----------------------------------------------------------------

    /// Process emergency withdrawals for multiple depositors in a single transaction.
    ///
    /// **Best-effort**: depositors with no active deposit are skipped and recorded
    /// as `success: false` in the returned results — the call never panics due to a
    /// missing deposit, so valid entries are always processed.
    ///
    /// **Single auth**: the admin signs once for the entire batch.
    ///
    /// # Arguments
    /// * `admin`      — Must be the current admin (signs once for the whole batch).
    /// * `depositors` — List of depositor addresses to process (max `MAX_BATCH_SIZE`).
    ///
    /// # Returns
    /// A `Vec<WithdrawResult>` with one entry per input address indicating success/skip.
    ///
    /// # Errors
    /// * `Unauthorized`  — Caller is not the admin.
    /// * `BatchTooLarge` — `depositors.len() > MAX_BATCH_SIZE`.
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
            let entry = match storage::get_deposit_readonly(&env, &depositor) {
                Some(e) => e,
                None => {
                    results.push_back(WithdrawResult { depositor, success: false });
                    continue;
                }
            };

            // Checks-Effects-Interactions: clear state before external token call.
            storage::remove_deposit(&env, &depositor);
            storage::remove_depositor(&env, &depositor);

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

    pub fn get_vault(env: Env, depositor: Address) -> Option<VaultEntry> {
        storage::get_deposit_readonly(&env, &depositor)
    }

    pub fn get_vault_with_time_remaining(env: Env, depositor: Address) -> Option<(VaultEntry, u64)> {
        let entry = storage::get_deposit_readonly(&env, &depositor)?;
        let now = env.ledger().timestamp();
        let remaining = entry.unlock_time.saturating_sub(now);
        Some((entry, remaining))
    }

    pub fn time_remaining(env: Env, depositor: Address) -> u64 {
        match storage::get_deposit_readonly(&env, &depositor) {
            None => 0,
            Some(entry) => {
                let now = env.ledger().timestamp();
                entry.unlock_time.saturating_sub(now)
            }
        }
    }

    pub fn has_deposit(env: Env, depositor: Address) -> bool {
        storage::get_deposit_readonly(&env, &depositor).is_some()
    }

    pub fn get_time(env: Env) -> u64 {
        env.ledger().timestamp()
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

    /// Returns the effective limits for this deployment.
    pub fn get_constants(env: Env) -> (i128, u64) {
        let max_deposit = storage::get_max_deposit(&env).unwrap_or(MAX_DEPOSIT_AMOUNT);
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        (max_deposit, max_lock)
    }

    pub fn get_fee_recipient(env: Env) -> Option<Address> {
        storage::get_fee_recipient(&env)
    }

    /// Returns the total number of addresses with an active deposit (backed by DepositorList).
    pub fn get_depositor_count(env: Env) -> u32 {
        storage::get_depositor_count(&env)
    }

    pub fn get_depositors(env: Env, offset: u32, limit: u32) -> Vec<Address> {
        storage::get_depositors_page(&env, offset, limit)
    }

    /// Returns the total number of active deposits tracked by the Instance-storage counter.
    pub fn get_deposit_count(env: Env) -> u64 {
        storage::get_deposit_count(&env)
    }

    pub fn is_initialized(env: Env) -> bool {
        storage::is_initialized(&env)
    }
}
