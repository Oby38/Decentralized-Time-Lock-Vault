use soroban_sdk::{contract, contractimpl, token, Address, Env, Vec};

use crate::{
    errors::VaultError,
    events,
    storage,
    types::{VaultEntry, MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS},
};

// ============================================================
//  TimeLockVault Contract
// ============================================================

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
    /// * `fee_recipient` — Address that receives penalty fees on early cancellation.
    ///
    /// # Errors
    /// * `Unauthorized` — Contract has already been initialized.
    pub fn initialize(env: Env, admin: Address, fee_recipient: Address) -> Result<(), VaultError> {
        // FIX: require_auth FIRST before any state reads (correct Soroban pattern).
        admin.require_auth();

        // Prevent re-initialization.
        if storage::get_admin(&env).is_some() {
            return Err(VaultError::Unauthorized);
        }

        storage::set_admin(&env, &admin);
        storage::set_fee_recipient(&env, &fee_recipient);
        Ok(())
    }

    // ----------------------------------------------------------------
    //  Core: Deposit
    // ----------------------------------------------------------------

    /// Lock `amount` of `token` until `unlock_time` (Unix seconds).
    ///
    /// One active deposit is allowed per address at a time.
    /// Call `withdraw` first before creating a new deposit.
    ///
    /// # Arguments
    /// * `depositor`   — The address locking the funds (must sign).
    /// * `token`       — SEP-41 token contract address.
    /// * `amount`      — Positive amount in the token's smallest unit.
    ///                   Must be > 0 and ≤ MAX_DEPOSIT_AMOUNT (10^15).
    /// * `unlock_time` — Future Unix timestamp (seconds) for the lock expiry.
    ///                   Must be > now and ≤ now + MAX_LOCK_DURATION_SECS (5 years).
    /// * `penalty_bps` — Early-exit penalty in basis points (0–10000).
    ///
    /// # Errors
    /// * `InvalidAmount`         — `amount` ≤ 0.
    /// * `AmountTooLarge`        — `amount` > MAX_DEPOSIT_AMOUNT.
    /// * `UnlockTimeNotInFuture` — `unlock_time` ≤ current ledger timestamp.
    /// * `LockDurationTooLong`   — Lock period exceeds 5 years.
    /// * `DepositAlreadyExists`  — A live deposit already exists for this address.
    /// * `InvalidPenaltyBps`     — `penalty_bps` > 10000.
    pub fn deposit(
        env: Env,
        depositor: Address,
        token: Address,
        amount: i128,
        unlock_time: u64,
        penalty_bps: u32,
    ) -> Result<(), VaultError> {
        // --- Auth (always first) ---
        depositor.require_auth();

        // --- Amount validation ---
        if amount <= 0 {
            return Err(VaultError::InvalidAmount);
        }
        if amount > MAX_DEPOSIT_AMOUNT {
            return Err(VaultError::AmountTooLarge);
        }

        // --- Penalty validation ---
        if penalty_bps > 10_000 {
            return Err(VaultError::InvalidPenaltyBps);
        }

        // --- Time validation ---
        let now = env.ledger().timestamp();
        if unlock_time <= now {
            return Err(VaultError::UnlockTimeNotInFuture);
        }
        // FIX: enforce maximum lock duration to prevent unbounded TTL requirements.
        let lock_duration = unlock_time.saturating_sub(now);
        if lock_duration > MAX_LOCK_DURATION_SECS {
            return Err(VaultError::LockDurationTooLong);
        }

        // --- Duplicate deposit guard ---
        if storage::has_deposit(&env, &depositor) {
            return Err(VaultError::DepositAlreadyExists);
        }

        // --- Transfer tokens from depositor → this contract ---
        let token_client = token::Client::new(&env, &token);
        token_client.transfer(&depositor, &env.current_contract_address(), &amount);

        // --- Persist the vault entry ---
        let entry = VaultEntry {
            token: token.clone(),
            amount,
            unlock_time,
            depositor: depositor.clone(),
            penalty_bps,
        };
        storage::set_deposit(&env, &depositor, &entry);

        // --- Maintain global depositor list ---
        storage::add_depositor(&env, &depositor);

        // --- Emit event ---
        events::deposit(&env, &depositor, &token, amount, unlock_time);

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

    /// Withdraw locked funds after the unlock time has passed.
    ///
    /// # Arguments
    /// * `depositor` — The address that originally deposited (must sign).
    ///
    /// # Errors
    /// * `NoDepositFound`   — No active deposit for this address.
    /// * `FundsStillLocked` — Current time is before `unlock_time`.
    pub fn withdraw(env: Env, depositor: Address) -> Result<(), VaultError> {
        // --- Auth ---
        depositor.require_auth();

        // --- Load deposit (bumps TTL — this is a state-changing call) ---
        let entry = storage::get_deposit(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        // --- Time check ---
        let now = env.ledger().timestamp();
        if now < entry.unlock_time {
            return Err(VaultError::FundsStillLocked);
        }

        // --- Checks-Effects-Interactions: clear storage BEFORE external call ---
        storage::remove_deposit(&env, &depositor);
        storage::remove_depositor(&env, &depositor);

        // --- Transfer tokens from contract → depositor ---
        let token_client = token::Client::new(&env, &entry.token);
        token_client.transfer(
            &env.current_contract_address(),
            &depositor,
            &entry.amount,
        );

        // --- Emit event ---
        events::withdraw(&env, &depositor, &entry.token, entry.amount);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Admin: Emergency Withdrawal
    // ----------------------------------------------------------------

    /// Admin-only: forcibly return funds to the original depositor.
    /// Intended for emergency recovery only. Funds always go back to
    /// the depositor — never to the admin.
    ///
    /// # Arguments
    /// * `admin`     — Must match the stored admin address (must sign).
    /// * `depositor` — The depositor whose funds will be returned.
    ///
    /// # Errors
    /// * `Unauthorized`   — Caller is not the stored admin.
    /// * `NoDepositFound` — No active deposit for `depositor`.
    pub fn emergency_withdraw(
        env: Env,
        admin: Address,
        depositor: Address,
    ) -> Result<(), VaultError> {
        // --- Auth ---
        admin.require_auth();

        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }

        // --- Load deposit ---
        let entry = storage::get_deposit(&env, &depositor)
            .ok_or(VaultError::NoDepositFound)?;

        // --- Checks-Effects-Interactions ---
        storage::remove_deposit(&env, &depositor);
        storage::remove_depositor(&env, &depositor);

        // --- Return funds to depositor (NOT to admin) ---
        let token_client = token::Client::new(&env, &entry.token);
        token_client.transfer(
            &env.current_contract_address(),
            &depositor,
            &entry.amount,
        );

        // --- Emit event ---
        events::emergency_withdraw(&env, &admin, &depositor, &entry.token, entry.amount);

        Ok(())
    }

    // ----------------------------------------------------------------
    //  Admin: Two-Step Admin Transfer
    // ----------------------------------------------------------------

    /// Step 1 — Current admin nominates a new admin address.
    /// The new admin must call `accept_admin` to complete the transfer.
    /// This prevents accidentally transferring admin rights to a wrong address.
    ///
    /// # Arguments
    /// * `admin`       — Current admin (must sign).
    /// * `new_admin`   — Address being nominated.
    ///
    /// # Errors
    /// * `Unauthorized` — Caller is not the current admin.
    pub fn transfer_admin(
        env: Env,
        admin: Address,
        new_admin: Address,
    ) -> Result<(), VaultError> {
        admin.require_auth();

        let stored_admin = storage::get_admin(&env).ok_or(VaultError::Unauthorized)?;
        if admin != stored_admin {
            return Err(VaultError::Unauthorized);
        }

        storage::set_pending_admin(&env, &new_admin);
        events::admin_transfer_initiated(&env, &admin, &new_admin);

        Ok(())
    }

    /// Step 2 — Pending admin accepts the nomination and becomes the new admin.
    ///
    /// # Arguments
    /// * `new_admin` — Must match the stored pending admin (must sign).
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

    /// Cancel a pending admin transfer. Only the current admin can cancel.
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

    /// Permanently renounce admin privileges.
    /// After this call, emergency_withdraw and admin functions are disabled forever.
    /// This makes the vault fully trustless — use with caution.
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

        // Remove admin and any pending transfer.
        env.storage().persistent().remove(&crate::types::VaultKey::Admin);
        storage::remove_pending_admin(&env);

        events::admin_renounced(&env, &admin);
        Ok(())
    }

    // ----------------------------------------------------------------
    //  Read-only Queries
    // ----------------------------------------------------------------

    /// Returns the vault entry for `depositor`, or `None` if no deposit exists.
    /// FIX: uses readonly helper — does NOT bump TTL, so callers pay no extra fees.
    pub fn get_vault(env: Env, depositor: Address) -> Option<VaultEntry> {
        storage::get_deposit_readonly(&env, &depositor)
    }

    /// Returns the current ledger timestamp (useful for client-side UX).
    pub fn get_time(env: Env) -> u64 {
        env.ledger().timestamp()
    }

    /// Returns the number of seconds remaining until the vault unlocks.
    /// Returns 0 if already unlocked or no deposit exists.
    /// FIX: uses readonly helper — does NOT bump TTL.
    pub fn time_remaining(env: Env, depositor: Address) -> u64 {
        match storage::get_deposit_readonly(&env, &depositor) {
            None => 0,
            Some(entry) => {
                let now = env.ledger().timestamp();
                if now >= entry.unlock_time {
                    0
                } else {
                    entry.unlock_time - now
                }
            }
        }
    }

    /// Returns the current admin address, or `None` if admin has been renounced.
    pub fn get_admin(env: Env) -> Option<Address> {
        storage::get_admin(&env)
    }

    /// Returns the pending admin address (during a two-step transfer), or `None`.
    pub fn get_pending_admin(env: Env) -> Option<Address> {
        storage::get_pending_admin(&env)
    }

    /// Returns the protocol constants for client-side validation.
    pub fn get_constants(_env: Env) -> (i128, u64) {
        (MAX_DEPOSIT_AMOUNT, MAX_LOCK_DURATION_SECS)
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
