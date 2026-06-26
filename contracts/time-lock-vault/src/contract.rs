use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

use crate::{
    constants::{MAX_BATCH_SIZE, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS, MIN_LOCK_DURATION_SECS},
    errors::VaultError,
    events, storage,
    types::{LedgerVaultEntry, VaultEntry, WithdrawResult},
};

#[contract]
pub struct TimeLockVault;

#[contractimpl]
impl TimeLockVault {
    // ----------------------------------------------------------------
    //  Initialization
    // ----------------------------------------------------------------

    pub fn initialize(
        env: Env,
        admin: Address,
        fee_recipient: Address,
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
            return Err(VaultError::ContractPaused);
        }

        if storage::is_frozen(&env, &depositor) {
            return Err(VaultError::DepositorFrozen);
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
        let lock_duration: u64 = unlock_time.saturating_sub(now);
        if lock_duration > max_lock {
            return Err(VaultError::LockDurationTooLong);
        }
        if lock_duration < MIN_LOCK_DURATION_SECS {
            return Err(VaultError::LockDurationTooShort);
        }

        let deposit_id = storage::next_deposit_id(&env, &depositor);

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
        storage::add_depositor(&env, &depositor);
        events::deposit(&env, &depositor, &token, deposit_id, amount, unlock_time);

        Ok(deposit_id)
    }

    pub fn deposit_for(
        env: Env,
        payer: Address,
        depositor: Address,
        token: Address,
        amount: i128,
        unlock_time: u64,
        penalty_bps: u32,
    ) -> Result<u32, VaultError> {
        payer.require_auth();

        if storage::is_paused(&env) {
            return Err(VaultError::ContractPaused);
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
        let lock_duration: u64 = unlock_time.saturating_sub(now);
        if lock_duration > max_lock {
            return Err(VaultError::LockDurationTooLong);
        }
        if lock_duration < MIN_LOCK_DURATION_SECS {
            return Err(VaultError::LockDurationTooShort);
        }

        let deposit_id = storage::next_deposit_id(&env, &depositor);

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&payer, &env.current_contract_address(), &amount);

        let entry = VaultEntry {
            token: token.clone(),
            amount,
            unlock_time,
            depositor: depositor.clone(),
            penalty_bps,
        };

        storage::set_deposit(&env, &depositor, deposit_id, &entry);
        storage::add_depositor(&env, &depositor);
        events::deposit(&env, &depositor, &token, deposit_id, amount, unlock_time);

        Ok(deposit_id)
    }

    // ----------------------------------------------------------------
    //  Core: Deposit by Ledger Sequence
    // ----------------------------------------------------------------

    pub fn deposit_by_ledger(
        env: Env,
        depositor: Address,
        token: Address,
        amount: i128,
        unlock_ledger: u32,
        penalty_bps: u32,
    ) -> Result<u32, VaultError> {
        depositor.require_auth();

        if storage::is_paused(&env) {
            return Err(VaultError::ContractPaused);
        }

        if storage::is_frozen(&env, &depositor) {
            return Err(VaultError::DepositorFrozen);
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

        let current_ledger = env.ledger().sequence();
        if unlock_ledger <= current_ledger {
            return Err(VaultError::UnlockTimeNotInFuture);
        }

        let lock_ledgers = unlock_ledger.saturating_sub(current_ledger);
        let max_lock = storage::get_max_lock_secs(&env).unwrap_or(MAX_LOCK_DURATION_SECS);
        let max_lock_ledgers = (max_lock / storage::LEDGER_SECONDS) as u32;
        if lock_ledgers > max_lock_ledgers {
            return Err(VaultError::LockDurationTooLong);
        }
        let min_lock_ledgers = (MIN_LOCK_DURATION_SECS / storage::LEDGER_SECONDS) as u32;
        if lock_ledgers < min_lock_ledgers {
            return Err(VaultError::LockDurationTooShort);
        }

        let deposit_id = storage::next_deposit_id(&env, &depositor);

        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        let entry = LedgerVaultEntry {
            token: token.clone(),
            amount,
            unlock_ledger,
            depositor: depositor.clone(),
            penalty_bps,
        };

        storage::set_deposit_by_ledger(&env, &depositor, deposit_id, &entry);
        storage::add_depositor(&env, &depositor);
        events::deposit(&env, &depositor, &token, deposit_id, amount, unlock_ledger as u64);

        Ok(deposit_id)
    }

    // ----------------------------------------------------------------
    //  Core: Cancel Deposit (early exit with penalty)
    // ----------------------------------------------------------------

    pub fn cancel_deposit(env: Env, depositor: Address, deposit_id: u32) -> Result<(), VaultError> {
        depositor.require_auth();

        // Try timestamp-based deposit first
        if let Some(entry) = storage::get_deposit(&env, &depositor, deposit_id) {
            let now = env.ledger().timestamp();
            if now >= entry.unlock_time {
                return Err(VaultError::FundsAlreadyUnlocked);
            }

            storage::remove_deposit(&env, &depositor, deposit_id);
            if !storage::has_any_deposit(&env, &depositor) {
                storage::remove_depositor(&env, &depositor);
            }

            let token_client = token::Client::new(&env, &entry.token);
            let contract = env.current_contract_address();

            let penalty: i128 = (entry.amount * entry.penalty_bps as i128) / 10_000;
            let refund = entry.amount - penalty;

            if penalty > 0 {
                let fee_recipient =
                    storage::get_fee_recipient(&env).unwrap_or_else(|| depositor.clone());
                token_client.transfer(&contract, &fee_recipient, &penalty);
            }
            if refund > 0 {
                token_client.transfer(&contract, &depositor, &refund);
            }

            events::deposit_cancelled(&env, &depositor, &entry.token, entry.amount, penalty);
            return Ok(());
        }

        // Try ledger-based deposit
        if let Some(entry) = storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id) {
            let current_ledger = env.ledger().sequence();
            if current_ledger >= entry.unlock_ledger {
                return Err(VaultError::FundsAlreadyUnlocked);
            }

            storage::remove_deposit_by_ledger(&env, &depositor, deposit_id);
            if !storage::has_any_deposit(&env, &depositor) {
                storage::remove_depositor(&env, &depositor);
            }

            let token_client = token::Client::new(&env, &entry.token);
            let contract = env.current_contract_address();

            let penalty: i128 = (entry.amount * entry.penalty_bps as i128) / 10_000;
            let refund = entry.amount - penalty;

            if penalty > 0 {
                let fee_recipient =
                    storage::get_fee_recipient(&env).unwrap_or_else(|| depositor.clone());
                token_client.transfer(&contract, &fee_recipient, &penalty);
            }
            if refund > 0 {
                token_client.transfer(&contract, &depositor, &refund);
            }

            events::deposit_cancelled(&env, &depositor, &entry.token, entry.amount, penalty);
            return Ok(());
        }

        Err(VaultError::NoDepositFound)
    }

    pub fn withdraw(env: Env, depositor: Address, deposit_id: u32) -> Result<(), VaultError> {
        depositor.require_auth();

        if storage::is_frozen(&env, &depositor) {
            return Err(VaultError::DepositorFrozen);
        }

        if let Some(entry) = storage::get_deposit_readonly(&env, &depositor, deposit_id) {
            let now = env.ledger().timestamp();
            if now < entry.unlock_time {
                return Err(VaultError::FundsStillLocked);
            }

            storage::remove_deposit(&env, &depositor, deposit_id);
            if !storage::has_any_deposit(&env, &depositor) {
                storage::remove_depositor(&env, &depositor);
            }

            let token_client = token::Client::new(&env, &entry.token);
            token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);

            events::withdraw(&env, &depositor, &entry.token, deposit_id, entry.amount);
            return Ok(());
        }

        if let Some(entry) = storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id) {
            let current_ledger = env.ledger().sequence();
            if current_ledger < entry.unlock_ledger {
                return Err(VaultError::FundsStillLocked);
            }

            storage::remove_deposit_by_ledger(&env, &depositor, deposit_id);
            if !storage::has_any_deposit(&env, &depositor) {
                storage::remove_depositor(&env, &depositor);
            }

            let token_client = token::Client::new(&env, &entry.token);
            token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);

            events::withdraw(&env, &depositor, &entry.token, deposit_id, entry.amount);
            return Ok(());
        }

        Err(VaultError::NoDepositFound)
    }

    pub fn withdraw_to(
        env: Env,
        depositor: Address,
        deposit_id: u32,
        recipient: Address,
    ) -> Result<(), VaultError> {
        depositor.require_auth();

        if storage::is_frozen(&env, &depositor) {
            return Err(VaultError::DepositorFrozen);
        }

        if let Some(entry) = storage::get_deposit_readonly(&env, &depositor, deposit_id) {
            let now = env.ledger().timestamp();
            if now < entry.unlock_time {
                return Err(VaultError::FundsStillLocked);
            }

            storage::remove_deposit(&env, &depositor, deposit_id);
            if !storage::has_any_deposit(&env, &depositor) {
                storage::remove_depositor(&env, &depositor);
            }

            let token_client = token::Client::new(&env, &entry.token);
            token_client.transfer(&env.current_contract_address(), &recipient, &entry.amount);

            events::withdraw_to(&env, &depositor, &recipient, &entry.token, deposit_id, entry.amount);
            return Ok(());
        }

        if let Some(entry) = storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id) {
            let current_ledger = env.ledger().sequence();
            if current_ledger < entry.unlock_ledger {
                return Err(VaultError::FundsStillLocked);
            }

            storage::remove_deposit_by_ledger(&env, &depositor, deposit_id);
            if !storage::has_any_deposit(&env, &depositor) {
                storage::remove_depositor(&env, &depositor);
            }

            let token_client = token::Client::new(&env, &entry.token);
            token_client.transfer(&env.current_contract_address(), &recipient, &entry.amount);

            events::withdraw_to(&env, &depositor, &recipient, &entry.token, deposit_id, entry.amount);
            return Ok(());
        }

        Err(VaultError::NoDepositFound)
    }

    pub fn emergency_withdraw(
        env: Env,
        admin: Address,
        depositor: Address,
        deposit_id: u32,
    ) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        if let Some(entry) = storage::get_deposit_readonly(&env, &depositor, deposit_id) {
            storage::remove_deposit(&env, &depositor, deposit_id);
            if !storage::has_any_deposit(&env, &depositor) {
                storage::remove_depositor(&env, &depositor);
            }
            let token_client = token::Client::new(&env, &entry.token);
            token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);
            events::emergency_withdraw(&env, &admin, &depositor, &entry.token, deposit_id, entry.amount);
            return Ok(());
        }

        if let Some(entry) = storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id) {
            storage::remove_deposit_by_ledger(&env, &depositor, deposit_id);
            if !storage::has_any_deposit(&env, &depositor) {
                storage::remove_depositor(&env, &depositor);
            }
            let token_client = token::Client::new(&env, &entry.token);
            token_client.transfer(&env.current_contract_address(), &depositor, &entry.amount);
            events::emergency_withdraw(&env, &admin, &depositor, &entry.token, deposit_id, entry.amount);
            return Ok(());
        }

        Err(VaultError::NoDepositFound)
    }

    pub fn batch_emergency_withdraw(
        env: Env,
        admin: Address,
        depositors: Vec<(Address, u32)>,
    ) -> Result<Vec<WithdrawResult>, VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        if depositors.len() > MAX_BATCH_SIZE {
            return Err(VaultError::BatchTooLarge);
        }

        let mut results: Vec<WithdrawResult> = Vec::new(&env);

        for item in depositors.iter() {
            let (depositor, deposit_id) = item;

            // Try timestamp-based deposit first
            if let Some(entry) = storage::get_deposit_readonly(&env, &depositor, deposit_id) {
                storage::remove_deposit(&env, &depositor, deposit_id);
                if !storage::has_any_deposit(&env, &depositor) {
                    storage::remove_depositor(&env, &depositor);
                }

                let token_client = token::Client::new(&env, &entry.token);
                token_client.transfer(
                    &env.current_contract_address(),
                    &depositor,
                    &entry.amount,
                );

                events::emergency_withdraw(
                    &env,
                    &admin,
                    &depositor,
                    &entry.token,
                    deposit_id,
                    entry.amount,
                );

                results.push_back(WithdrawResult {
                    depositor,
                    deposit_id,
                    success: true,
                });
                continue;
            }

            // Try ledger-based deposit
            if let Some(entry) = storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id) {
                storage::remove_deposit_by_ledger(&env, &depositor, deposit_id);
                if !storage::has_any_deposit(&env, &depositor) {
                    storage::remove_depositor(&env, &depositor);
                }

                let token_client = token::Client::new(&env, &entry.token);
                token_client.transfer(
                    &env.current_contract_address(),
                    &depositor,
                    &entry.amount,
                );

                events::emergency_withdraw(
                    &env,
                    &admin,
                    &depositor,
                    &entry.token,
                    deposit_id,
                    entry.amount,
                );

                results.push_back(WithdrawResult {
                    depositor,
                    deposit_id,
                    success: true,
                });
                continue;
            }

            results.push_back(WithdrawResult {
                depositor,
                deposit_id,
                success: false,
            });
        }

        Ok(results)
    }

    // ----------------------------------------------------------------
    //  Admin: Pause / Unpause
    // ----------------------------------------------------------------

    pub fn pause(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }
        storage::set_paused(&env, true);
        events::paused(&env, &admin);
        Ok(())
    }

    pub fn unpause(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();
        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }
        storage::set_paused(&env, false);
        events::unpaused(&env, &admin);
        Ok(())
    }

    pub fn is_paused(env: Env) -> bool {
        storage::is_paused(&env)
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

        let pending_admin = storage::get_pending_admin(&env).ok_or(VaultError::Unauthorized)?;
        if new_admin != pending_admin {
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

        if let Some(pending) = storage::get_pending_admin(&env) {
            storage::remove_pending_admin(&env);
            events::admin_transfer_cancelled(&env, &admin, &pending);
        }
        Ok(())
    }

    pub fn renounce_admin(env: Env, admin: Address) -> Result<(), VaultError> {
        admin.require_auth();

        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }

        storage::remove_admin(&env);
        storage::remove_pending_admin(&env);
        events::admin_renounced(&env, &admin);
        Ok(())
    }

    // ----------------------------------------------------------------
    //  Admin: Migration
    // ----------------------------------------------------------------

    pub fn migrate_deposit_to_ledger(
        env: Env,
        admin: Address,
        depositor: Address,
        deposit_id: u32,
        new_unlock_ledger: u32,
    ) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        let entry = storage::get_deposit(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        let current_ledger = env.ledger().sequence();
        if new_unlock_ledger <= current_ledger {
            return Err(VaultError::UnlockTimeNotInFuture);
        }

        let ledger_entry = LedgerVaultEntry {
            token: entry.token.clone(),
            amount: entry.amount,
            unlock_ledger: new_unlock_ledger,
            depositor: entry.depositor.clone(),
            penalty_bps: entry.penalty_bps,
        };

        storage::remove_deposit(&env, &depositor, deposit_id);
        storage::set_deposit_by_ledger(&env, &depositor, deposit_id, &ledger_entry);

        events::migrated(&env, &depositor, deposit_id, true, false);
        Ok(())
    }

    pub fn migrate_deposit_to_time(
        env: Env,
        admin: Address,
        depositor: Address,
        deposit_id: u32,
        new_unlock_time: u64,
    ) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        let entry = storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id)
            .ok_or(VaultError::NoDepositFound)?;

        let now = env.ledger().timestamp();
        if new_unlock_time <= now {
            return Err(VaultError::UnlockTimeNotInFuture);
        }

        let time_entry = VaultEntry {
            token: entry.token.clone(),
            amount: entry.amount,
            unlock_time: new_unlock_time,
            depositor: entry.depositor.clone(),
            penalty_bps: entry.penalty_bps,
        };

        storage::remove_deposit_by_ledger(&env, &depositor, deposit_id);
        storage::set_deposit(&env, &depositor, deposit_id, &time_entry);

        events::migrated(&env, &depositor, deposit_id, false, true);
        Ok(())
    }

    // ----------------------------------------------------------------
    //  Admin: Freeze / Unfreeze
    // ----------------------------------------------------------------

    pub fn freeze_depositor(env: Env, admin: Address, depositor: Address) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        storage::set_frozen(&env, &depositor);
        events::frozen(&env, &admin, &depositor);
        Ok(())
    }

    pub fn unfreeze_depositor(env: Env, admin: Address, depositor: Address) -> Result<(), VaultError> {
        admin.require_auth();
        storage::require_admin(&env, &admin)?;

        storage::remove_frozen(&env, &depositor);
        events::unfrozen(&env, &admin, &depositor);
        Ok(())
    }

    pub fn is_depositor_frozen(env: Env, depositor: Address) -> bool {
        storage::is_frozen(&env, &depositor)
    }

    // ----------------------------------------------------------------
    //  Read-only Queries
    // ----------------------------------------------------------------

    pub fn get_vault(env: Env, depositor: Address, deposit_id: u32) -> Option<VaultEntry> {
        storage::get_deposit_readonly(&env, &depositor, deposit_id)
    }

    pub fn get_vault_by_ledger(env: Env, depositor: Address, deposit_id: u32) -> Option<LedgerVaultEntry> {
        storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id)
    }

    pub fn ledgers_remaining(env: Env, depositor: Address, deposit_id: u32) -> u32 {
        match storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id) {
            None => 0,
            Some(entry) => {
                let current = env.ledger().sequence();
                entry.unlock_ledger.saturating_sub(current)
            }
        }
    }

    pub fn get_vault_batch(env: Env, depositors: Vec<Address>, deposit_id: u32) -> Vec<Option<VaultEntry>> {
        let limit = if depositors.len() > MAX_BATCH_SIZE { MAX_BATCH_SIZE } else { depositors.len() };
        let mut results = Vec::new(&env);
        for i in 0..limit {
            if let Some(depositor) = depositors.get(i) {
                let entry = storage::get_deposit_readonly(&env, &depositor, deposit_id);
                results.push_back(entry);
            }
        }
        results
    }

    pub fn get_deposit_ids(env: Env, depositor: Address) -> Vec<u32> {
        storage::get_deposit_ids(&env, &depositor)
    }

    pub fn get_time(env: Env) -> u64 {
        env.ledger().timestamp()
    }

    pub fn time_remaining(env: Env, depositor: Address, deposit_id: u32) -> u64 {
        // Try timestamp-based deposit first
        if let Some(entry) = storage::get_deposit_readonly(&env, &depositor, deposit_id) {
            return entry.unlock_time.saturating_sub(env.ledger().timestamp());
        }
        // Try ledger-based deposit
        if let Some(entry) = storage::get_deposit_by_ledger_readonly(&env, &depositor, deposit_id) {
            let remaining_ledgers = entry.unlock_ledger.saturating_sub(env.ledger().sequence());
            return (remaining_ledgers as u64).saturating_mul(storage::LEDGER_SECONDS);
        }
        0
    }

    pub fn get_admin(env: Env) -> Option<Address> {
        storage::get_admin(&env)
    }

    pub fn get_pending_admin(env: Env) -> Option<Address> {
        storage::get_pending_admin(&env)
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
        const MAX_PAGE_SIZE: u32 = 100;
        storage::get_depositors_page(&env, offset, limit.min(MAX_PAGE_SIZE))
    }

    pub fn is_initialized(env: Env) -> bool {
        storage::is_initialized(&env)
    }
}
