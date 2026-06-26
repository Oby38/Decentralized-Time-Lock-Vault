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

    /// Initialize the contract with an admin address and fee recipient.
    /// Must be called once immediately after deployment.
    ///
    /// # Arguments
    /// * `admin`         — Address that gains emergency-withdrawal and admin privileges.
    /// * `fee_recipient` — Address that receives penalty fees on early cancellation.
    ///
    /// # Errors
    /// * `Unauthorized` — Contract has already been initialized.
    pub fn initialize(env: Env, admin: Address, fee_recipient: Address) -> Result<(), VaultError> {
        admin.require_auth();

        if storage::get_admin(&env).is_some() {
            return Err(VaultError::Unauthorized);
        }
        storage::set_admin(&env, &admin);
        storage::set_initialized(&env);
        storage::set_fee_recipient(&env, &fee_recipient);

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
        penalty_bps: u32,
    ) -> Result<u32, VaultError> {
        depositor.require_auth();

        if storage::is_paused(&env) {
            return Err(VaultError::Unauthorized);
        }

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
        // Enforce a minimum lock duration to avoid trivial deposits that
        // immediately expire and waste persistent storage.
        if lock_duration < MIN_LOCK_DURATION_SECS {
            return Err(VaultError::LockDurationTooShort);
        }

        // Get the next deposit ID for this depositor
        let deposit_id = storage::next_deposit_id(&env, &depositor);

        // Check for existing deposits
        if storage::get_deposit_readonly(&env, &depositor, deposit_id).is_some() {
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

        // Maintain global depositor list
        storage::add_depositor(&env, &depositor);

        // Emit event
        events::deposit(&env, &depositor, deposit_id, &token, amount, unlock_time);

        Ok(deposit_id)
    }



    /// Lock `amount` of `token` until `unlock_ledger_seq` (ledger sequence instead of timestamp).
    /// Returns a deposit ID. Validates lock duration is within min/max bounds.
    pub fn deposit_by_ledger(
        env: Env,
        depositor: Address,
        token: Address,
        amount: i128,
        unlock_ledger_seq: u32,
    ) -> Result<u32, VaultError> {
        depositor.require_auth();

        if storage::is_paused(&env) {
            return Err(VaultError::Unauthorized);
        }

        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }
        let max_deposit = storage::get_max_deposit(&env).unwrap_or(MAX_DEPOSIT_AMOUNT);
        if amount > max_deposit {
            return Err(VaultError::AmountTooLarge);
        }

        let current_ledger = env.ledger().sequence();
        if unlock_ledger_seq <= current_ledger {
            return Err(VaultError::UnlockTimeNotInFuture);
        }

        let lock_duration_ledgers = unlock_ledger_seq.saturating_sub(current_ledger);
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        // Convert max lock duration (seconds) to ledger sequence (at ~5s per ledger)
        let max_lock_ledgers = (max_lock / 5) as u32;
        if lock_duration_ledgers > max_lock_ledgers {
            return Err(VaultError::LockDurationTooLong);
        }

        // Enforce minimum lock duration (~60s = ~12 ledgers at 5s per ledger)
        let min_lock_ledgers = (MIN_LOCK_DURATION_SECS / 5) as u32;
        if lock_duration_ledgers < min_lock_ledgers {
            return Err(VaultError::LockDurationTooShort);
        }

        let deposit_id = storage::next_deposit_id(&env, &depositor);

        if storage::get_deposit_readonly(&env, &depositor, deposit_id).is_some() {
            return Err(VaultError::DepositAlreadyExists);
        }

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        // Convert ledger sequence back to timestamp for storage
        // Approximate: current_time + (ledger_diff * 5)
        let approximate_unlock_time = env.ledger().timestamp() + ((lock_duration_ledgers as u64) * 5);

        let entry = VaultEntry {
            token: token.clone(),
            amount,
            unlock_time: approximate_unlock_time,
            depositor: depositor.clone(),
            penalty_bps: 0,
        };
        storage::set_deposit(&env, &depositor, deposit_id, &entry);

        storage::add_depositor(&env, &depositor);
        events::deposit(&env, &depositor, deposit_id, &token, amount, approximate_unlock_time);

        Ok(deposit_id)
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
    /// * `deposit_id` — The ID of the deposit to cancel.
    ///
    /// # Errors
    /// * `NoDepositFound`   — No active deposit for this address and ID.
    /// * `FundsStillLocked` — Vault is already past unlock time; use `withdraw`.
    pub fn cancel_deposit(env: Env, depositor: Address, deposit_id: u32) -> Result<(), VaultError> {
        depositor.require_auth();

        let entry = storage::get_deposit(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        // Cancellation is only valid while still locked.
        let now = env.ledger().timestamp();
        if now >= entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        // Checks-Effects-Interactions
        storage::remove_deposit(&env, &depositor, deposit_id);
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

    /// Withdraw funds if `now >= unlock_time`. Returns the amount withdrawn.
    ///
    /// # Arguments
    /// * `depositor` — The address that originally deposited (must sign).
    /// * `deposit_id` — The ID of the deposit to withdraw.
    ///
    /// # Errors
    /// * `NoDepositFound`   — No active deposit for this address and ID.
    /// * `FundsStillLocked` — Lock period not yet expired.
    pub fn withdraw(env: Env, depositor: Address, deposit_id: u32) -> Result<(), VaultError> {
        depositor.require_auth();

        // Load deposit without bumping TTL; the entry will be deleted
        let entry = storage::get_deposit_readonly(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        let now = env.ledger().timestamp();
        if now < entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        // Checks-Effects-Interactions: clear storage BEFORE external call
        storage::remove_deposit(&env, &depositor, deposit_id);
        storage::remove_depositor(&env, &depositor);

        let token_client = token::Client::new(&env, &entry.token);
        token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);

        events::withdraw(&env, &depositor, deposit_id, &entry.token, entry.amount);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Admin: Emergency Withdrawal
    // ----------------------------------------------------------------

    /// Admin-only. Force-withdraw funds for a depositor regardless of lock time.
    /// Funds always return to the depositor, never to the admin.
    ///
    /// # Arguments
    /// * `admin`     — The admin address (must sign).
    /// * `depositor` — The address that originally deposited.
    /// * `deposit_id` — The ID of the deposit to withdraw.
    ///
    /// # Errors
    /// * `Unauthorized`   — Caller is not the admin.
    /// * `NoDepositFound` — No active deposit for the depositor and ID.
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

        // Load deposit without bumping TTL; the entry will be deleted
        let entry = storage::get_deposit_readonly(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        // Checks-Effects-Interactions
        storage::remove_deposit(&env, &depositor, deposit_id);
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

    /// Step 1 of admin transfer: nominate a new admin.
    ///
    /// # Arguments
    /// * `admin` — Current admin (must sign).
    /// * `new_admin` — Address to nominate as pending admin.
    ///
    /// # Errors
    /// * `Unauthorized` — Caller is not the current admin.
    /// * `InvalidAdmin` — New admin is the same as the current admin.
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

    /// Step 2 of admin transfer: accept and become the new admin.
    ///
    /// # Arguments
    /// * `new_admin` — The pending admin address (must sign).
    ///
    /// # Errors
    /// * `Unauthorized` — Caller is not the pending admin.
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

    /// Cancel a pending admin transfer.
    ///
    /// # Arguments
    /// * `admin` — Current admin (must sign).
    ///
    /// # Errors
    /// * `Unauthorized` — Caller is not the current admin.
    pub fn cancel_transfer_admin(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }
        storage::remove_pending_admin(&env);
        Ok(())
    }

    /// Permanently renounce admin privileges. Makes the vault fully trustless.
    ///
    /// # Arguments
    /// * `admin` — Current admin (must sign).
    ///
    /// # Errors
    /// * `Unauthorized` — Caller is not the current admin.
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

    /// Admin-only. Pause all deposit operations.
    ///
    /// # Arguments
    /// * `admin` — The admin address (must sign).
    ///
    /// # Errors
    /// * `Unauthorized` — Caller is not the admin.
    pub fn pause_deposits(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }
        storage::set_paused(&env, true);
        Ok(())
    }

    /// Admin-only. Unpause deposit operations.
    ///
    /// # Arguments
    /// * `admin` — The admin address (must sign).
    ///
    /// # Errors
    /// * `Unauthorized` — Caller is not the admin.
    pub fn unpause_deposits(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }
        storage::set_paused(&env, false);
        Ok(())
    }

    /// Returns whether deposits are currently paused.
    pub fn is_paused(env: Env) -> bool {
        storage::is_paused(&env)
    }

    // ----------------------------------------------------------------
    //  Read-only Queries
    // ----------------------------------------------------------------

    /// Returns the vault entry for a specific deposit, or `None` if not found.
    /// Does bump TTL on read (used for active deposits).
    pub fn get_vault(env: Env, depositor: Address, deposit_id: u32) -> Option<VaultEntry> {
        storage::get_deposit(&env, &depositor, deposit_id)
    }

    /// Returns all deposit IDs for a depositor.
    pub fn get_deposit_ids(env: Env, depositor: Address) -> Vec<u32> {
        storage::get_deposit_ids(&env, &depositor)
    }

    /// Returns the current ledger timestamp.
    pub fn get_time(env: Env) -> u64 {
        env.ledger().timestamp()
    }

    /// Returns seconds until unlock for a specific deposit. Returns `0` if unlocked or no deposit exists.
    pub fn time_remaining(env: Env, depositor: Address, deposit_id: u32) -> u64 {
        match storage::get_deposit_readonly(&env, &depositor, deposit_id) {
            None => 0,
            Some(entry) => {
                let now = env.ledger().timestamp();
                entry.unlock_time.saturating_sub(now)
            }
        }
    }

    /// Returns the current admin, or `None` if renounced.
    pub fn get_admin(env: Env) -> Option<Address> {
        storage::get_admin(&env)
    }

    /// Returns the pending admin during a transfer, or `None`.
    pub fn get_pending_admin(env: Env) -> Option<Address> {
        storage::get_pending_admin(&env)
    }

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

    /// Returns whether the contract has been initialized.
    pub fn is_initialized(env: Env) -> bool {
        storage::is_initialized(&env)
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
