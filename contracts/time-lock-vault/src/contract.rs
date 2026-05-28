use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

use crate::{
    errors::VaultError,
    events,
    storage,
    types::{VaultEntry, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS, MIN_LOCK_DURATION_SECS},
};

#[contract]
pub struct TimeLockVault;

#[contractimpl]
impl TimeLockVault {
    // ----------------------------------------------------------------
    //  Initialization
    // ----------------------------------------------------------------

    pub fn initialize(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
    /// Initialize the contract with an admin address.
    /// Must be called once immediately after deployment.
    ///
    /// # Arguments
    /// * `admin`         — Address that gains emergency-withdrawal and admin privileges.
    /// * `fee_recipient` — Address that receives penalty fees on early cancellation.
    ///
    /// # Errors
    /// * `Unauthorized` — Contract has already been initialized.
    pub fn initialize(env: Env, admin: Address, fee_recipient: Address) -> Result<(), VaultError> {
        // FIX: require_auth FIRST before any state reads (correct Soroban pattern).
    pub fn initialize(
        env: Env,
        admin: Address,
        max_deposit: Option<i128>,
        max_lock_secs: Option<u64>,
    ) -> Result<(), VaultError> {
        admin.require_auth();

        if storage::get_admin(&env).is_some() {
            return Err(VaultError::Unauthorized);
        }
        storage::set_admin(&env, &admin);
        storage::set_initialized(&env);
        storage::set_fee_recipient(&env, &fee_recipient);

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

    /// Lock `amount` of `token` until `unlock_time`. Returns a deposit ID.
    pub fn deposit(
        env: Env,
        depositor: Address,
        token: Address,
        amount: i128,
        unlock_time: u64,
    ) -> Result<u32, VaultError> {
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
        if unlock_time.saturating_sub(now) > MAX_LOCK_DURATION_SECS {
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        let lock_duration = unlock_time.saturating_sub(now);
        if lock_duration > max_lock {
            return Err(VaultError::LockDurationTooLong);
        }
        // Enforce a minimum lock duration to avoid trivial deposits that
        // immediately expire and waste persistent storage.
        if lock_duration < MIN_LOCK_DURATION_SECS {
            return Err(VaultError::LockDurationTooShort);
        }

        if storage::has_deposit(&env, &depositor) {
        // --- Duplicate deposit guard ---
        if storage::get_deposit_readonly(&env, &depositor).is_some() {
            return Err(VaultError::DepositAlreadyExists);
        }

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        let entry = VaultEntry {
            token: token.clone(),
            amount,
            unlock_time,
            depositor: depositor.clone(),
            penalty_bps,
        };
        storage::set_deposit(&env, &depositor, deposit_id, &entry);

        events::deposit(&env, &depositor, deposit_id, &token, amount, unlock_time);
        // --- Maintain global depositor list ---
        storage::add_depositor(&env, &depositor);

        // --- Maintain global depositor list ---
        storage::add_depositor(&env, &depositor);

        // --- Emit event ---
        events::deposit(&env, &depositor, &token, amount, unlock_time);

        Ok(deposit_id)
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
    //  Core: Cancel Deposit (early exit with penalty)
    // ----------------------------------------------------------------

    /// Cancel an active deposit before the unlock time, paying a penalty.
    ///
    /// The penalty (stored as `penalty_bps` at deposit time) is sent to the
    /// `fee_recipient`. The remainder is returned to the depositor.
    /// If the vault is already unlocked, use `withdraw` instead.
    ///
    /// # Arguments
    /// * `depositor` — The address that originally deposited (must sign).
    ///
    /// # Errors
    /// * `NoDepositFound`   — No active deposit for this address.
    /// * `FundsStillLocked` — Vault is already past unlock time; use `withdraw`.
    pub fn cancel_deposit(env: Env, depositor: Address) -> Result<(), VaultError> {
        depositor.require_auth();

        let entry = storage::get_deposit(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        // Cancellation is only valid while still locked.
        let now = env.ledger().timestamp();
        if now >= entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        // --- Checks-Effects-Interactions ---
        storage::remove_deposit(&env, &depositor);
        storage::remove_depositor(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        let contract = env.current_contract_address();

        // penalty = amount * penalty_bps / 10_000  (integer, rounds down)
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
    //  Core: Withdraw
    // ----------------------------------------------------------------

    pub fn withdraw(env: Env, depositor: Address, deposit_id: u32) -> Result<(), VaultError> {
        depositor.require_auth();

        let entry =
            storage::get_deposit(&env, &depositor, deposit_id).ok_or(VaultError::NoDepositFound)?;
        // --- Load deposit without bumping TTL; the entry will be deleted ---
        let entry = storage::get_deposit_readonly(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        let now = env.ledger().timestamp();
        if now < entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        storage::remove_deposit(&env, &depositor, deposit_id);
        // --- Checks-Effects-Interactions: clear storage BEFORE external call ---
        storage::remove_deposit(&env, &depositor);
        storage::remove_depositor(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);

        events::withdraw(&env, &depositor, deposit_id, &entry.token, entry.amount);

        Ok(())
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

        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }

        let entry =
            storage::get_deposit(&env, &depositor, deposit_id).ok_or(VaultError::NoDepositFound)?;

        storage::remove_deposit(&env, &depositor, deposit_id);
        // --- Load deposit without bumping TTL; the entry will be deleted ---
        let entry = storage::get_deposit_readonly(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        // --- Checks-Effects-Interactions ---
        storage::remove_deposit(&env, &depositor);
        storage::remove_depositor(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);

        events::emergency_withdraw(
            &env,
            &admin,
            &depositor,
            deposit_id,
            &entry.token,
            entry.amount,
        );

        Ok(())
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

        // Prevent nominating the current admin as the pending admin — this
        // would be a no-op that wastes storage and emits a misleading event.
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
        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }
        storage::remove_pending_admin(&env);
        events::admin_transfer_cancelled(&env, &admin);
        Ok(())
    }

    pub fn renounce_admin(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }
        env.storage()
            .persistent()
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

    pub fn get_deposit_ids(env: Env, depositor: Address) -> Vec<u32> {
        storage::get_deposit_ids(&env, &depositor)
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

    pub fn get_admin(env: Env) -> Option<Address> {
        storage::get_admin(&env)
    }

    pub fn get_pending_admin(env: Env) -> Option<Address> {
        storage::get_pending_admin(&env)
    }

    pub fn get_constants(_env: Env) -> (i128, u64) {
        (MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS)
    /// Returns the effective limits for this deployment.
    /// Returns runtime-configured values if set, otherwise compile-time defaults.
    pub fn get_constants(env: Env) -> (i128, u64) {
        let max_deposit = storage::get_max_deposit(&env).unwrap_or(MAX_DEPOSIT_AMOUNT);
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        (max_deposit, max_lock)
    }

    /// Returns the fee recipient address set at initialization.
    pub fn get_fee_recipient(env: Env) -> Option<Address> {
        storage::get_fee_recipient(&env)
    }

    // ----------------------------------------------------------------
    //  Admin Tooling: Depositor Enumeration
    // ----------------------------------------------------------------

    /// Returns the total number of active depositors.
    pub fn get_depositor_count(env: Env) -> u32 {
        storage::get_depositor_count(&env)
    }

    /// Returns a paginated slice of active depositor addresses.
    ///
    /// # Arguments
    /// * `offset` — Zero-based start index.
    /// * `limit`  — Maximum number of addresses to return.
    pub fn get_depositors(env: Env, offset: u32, limit: u32) -> Vec<Address> {
        storage::get_depositors_page(&env, offset, limit)
    }

    pub fn is_initialized(env: Env) -> bool {
        storage::is_initialized(&env)
    }

    /// Returns the fee recipient address, or `None` if not set.
    pub fn get_fee_recipient(env: Env) -> Option<Address> {
        storage::get_fee_recipient(&env)
    }

    // ----------------------------------------------------------------
    //  Admin Tooling: Depositor Enumeration
    // ----------------------------------------------------------------

    /// Returns the total number of active depositors.
    pub fn get_depositor_count(env: Env) -> u32 {
        storage::get_depositor_count(&env)
    }

    /// Returns a paginated slice of active depositor addresses.
    ///
    /// # Arguments
    /// * `offset` — Zero-based start index.
    /// * `limit`  — Maximum number of addresses to return.
    pub fn get_depositors(env: Env, offset: u32, limit: u32) -> Vec<Address> {
        storage::get_depositors_page(&env, offset, limit)
    }
}
