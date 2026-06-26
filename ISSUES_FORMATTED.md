---
`Ledger deposit path` bypasses the paused contract guard
- Priority: Critical
- Difficulty: Advanced
- Labels: "bug", "security", "pause"

Description

The current implementation of ``deposit_by_ledger` bypasses the paused contract guard` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
`Ledger deposit path` does not enforce minimum lock duration
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "validation", "contract"

Description

The current implementation of ``deposit_by_ledger` does not enforce minimum lock duration` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
`Ledger deposit path` does not enforce maximum lock duration
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "validation", "contract"

Description

The current implementation of ``deposit_by_ledger` does not enforce maximum lock duration` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
`Withdraw-to path` only works for time-based deposits, ignoring ledger deposits
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "contract", "storage"

Description

The current implementation of ``withdraw_to` only works for time-based deposits, ignoring ledger deposits` introduces a contract behavior gap that must be corrected. The withdrawal implementation does not correctly handle deposits created by ledger-based locks. This inconsistency leaves valid ledger deposits unreachable through the public withdraw API. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

Withdraw actions should support ledger-based deposits and return the correct outcome for both timestamp and ledger locks.

Tasks

- [ ] Review `withdraw_to` logic and verify ledger deposit compatibility.
- [ ] Add ledger deposit handling if missing.
- [ ] Add tests that exercise withdrawal of ledger-based deposits.

---
`Emergency withdrawal path` only works for time-based deposits, ignoring ledger deposits
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "admin", "recovery"

Description

The current implementation of ``emergency_withdraw` only works for time-based deposits, ignoring ledger deposits` introduces a contract behavior gap that must be corrected. The emergency recovery path only supports timestamp-based deposits and ignores ledger-based vault entries. That exposes a recovery gap where some deposits cannot be recovered by admin functions. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Emergency recovery should cover both timestamp and ledger-based deposits so admin recovery is complete.

Tasks

- [ ] Review emergency withdrawal paths for ledger and timestamp deposits.
- [ ] Extend `emergency_withdraw` to support ledger-based deposit entries.
- [ ] Add tests that exercise emergency recovery for ledger deposits.

---
`Vault query` does not expose ledger-based deposits
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "api", "storage"

Description

The current implementation of ``get_vault` does not expose ledger-based deposits` introduces a contract behavior gap that must be corrected. The vault query API currently omits ledger-based deposits from its results. External clients cannot reliably inspect all active vaults, undermining transparency and off-chain indexing. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

The vault query should return all active deposits regardless of whether they were created by timestamp or ledger lock semantics.

Tasks

- [ ] Audit vault query implementation for ledger deposit inclusion.
- [ ] Correct query behavior to return both time-based and ledger-based deposits.
- [ ] Add tests for query results with mixed deposit types.

---
`Time remaining query` ignores ledger-based deposits and returns 0
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "api", "ux"

Description

The current implementation of ``time_remaining` ignores ledger-based deposits and returns 0` introduces a contract behavior gap that must be corrected. The time remaining calculation ignores ledger-based deposits and returns misleading values. This can cause callers to believe a deposit is unlocked when it is still locked by ledger sequence. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

The time remaining query should compute the correct remaining lock interval for ledger-based deposits and not return misleading zero values.

Tasks

- [ ] Audit time remaining calculation for ledger deposit entries.
- [ ] Fix the logic so ledger-based deposits produce correct remaining lock values.
- [ ] Add regression tests for ledger-derived remaining times.

---
`Deposit ID enumeration` skips ledger-based deposit IDs
- Priority: Medium
- Difficulty: Intermediate
- Labels: "bug", "api", "storage"

Description

The current implementation of ``get_deposit_ids` skips ledger-based deposit IDs` introduces a contract behavior gap that must be corrected. The deposit identifier query does not include ledger-based entries, so clients cannot enumerate every deposit. This breaks deposit discovery and any off-chain feature that relies on a complete deposit list. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Deposit ID enumeration should list every deposit, including ledger-based entries, so external indexers can discover all active vaults.

Tasks

- [ ] Audit deposit ID enumeration for ledger deposit entries.
- [ ] Update `get_deposit_ids` to include all active deposits.
- [ ] Add tests for ledger deposit ID visibility.

---
`Vault batch query` reads only time-based deposits, not ledger-based deposits
- Priority: Medium
- Difficulty: Intermediate
- Labels: "bug", "api", "storage"

Description

The current implementation of ``get_vault_batch` reads only time-based deposits, not ledger-based deposits` introduces a contract behavior gap that must be corrected. The batch vault query currently omits ledger-based deposits, preventing complete client-side vault enumeration. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Batch vault queries should include ledger deposits and return complete vault state for clients.

Tasks

- [ ] Audit vault query implementation for ledger deposit inclusion.
- [ ] Correct query behavior to return both time-based and ledger-based deposits.
- [ ] Add tests for query results with mixed deposit types.

---
`Deposit cancellation` cannot cancel ledger-based deposits
- Priority: Medium
- Difficulty: Intermediate
- Labels: "bug", "contract", "storage"

Description

The current implementation of ``cancel_deposit` cannot cancel ledger-based deposits` introduces a contract behavior gap that must be corrected. The cancel flow does not support ledger-based deposits, creating an inconsistent user experience. Depositors may not be able to cancel deposits they expect to manage, exposing functional gaps in the contract logic. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Cancel deposit should support the same deposit types and allow users to cancel valid ledger-based deposits where appropriate.

Tasks

- [ ] Review cancel flow for ledger deposit support.
- [ ] Extend cancel logic to handle ledger-based deposits consistently.
- [ ] Add tests covering cancellation of ledger deposits.

---
`Depositor removal` can clear an address while ledger deposits remain active
- Priority: Medium
- Difficulty: Intermediate
- Labels: "bug", "storage", "consistency"

Description

The current implementation of ``remove_depositor` can clear an address while ledger deposits remain active` introduces a contract behavior gap that must be corrected. The depositor removal path can remove an address while ledger deposits remain active. This risks leaving orphaned deposit state and breaking retrieval APIs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`.

Expected Behavior

Removing a depositor should not break active ledger deposits or leave orphaned state in storage.

Tasks

- [ ] Review depositor removal logic and ledger deposit interactions.
- [ ] Prevent removal of a depositor with active ledger deposits or clear related state safely.
- [ ] Add tests for depositor removal under mixed deposit conditions.

---
README documents non-existent `batch_Emergency withdrawal path` API
- Priority: Medium
- Difficulty: Beginner
- Labels: "bug", "documentation", "contract"

Description

The current implementation of `README documents non-existent `batch_emergency_withdraw` API` introduces a contract behavior gap that must be corrected. The emergency recovery path only supports timestamp-based deposits and ignores ledger-based vault entries. That exposes a recovery gap where some deposits cannot be recovered by admin functions. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Emergency recovery should cover both timestamp and ledger-based deposits so admin recovery is complete.

Tasks

- [ ] Review emergency withdrawal paths for ledger and timestamp deposits.
- [ ] Extend `emergency_withdraw` to support ledger-based deposit entries.
- [ ] Add tests that exercise emergency recovery for ledger deposits.

---
README omits `Ledger deposit path`, `withdraw_to`, and ledger deposit semantics
- Priority: Medium
- Difficulty: Beginner
- Labels: "bug", "documentation", "api"

Description

The current implementation of `README omits `deposit_by_ledger`, `withdraw_to`, and ledger deposit semantics` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
`Ledger deposit path` uses a different validation path than `deposit`/`deposit_for`
- Priority: Low
- Difficulty: Intermediate
- Labels: "bug", "refactor", "contract"

Description

The current implementation of ``deposit_by_ledger` uses a different validation path than `deposit`/`deposit_for`` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
`initialize` treats zero `max_lock_secs` as `LockDurationTooLong` instead of explicit invalid config
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "validation", "contract"

Description

The current implementation of ``initialize` treats zero `max_lock_secs` as `LockDurationTooLong` instead of explicit invalid config` introduces a contract behavior gap that must be corrected. The initialization validation path does not distinguish invalid config from lock duration failures clearly. This makes configuration errors harder to debug and may allow invalid runtime settings. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

Initialization should validate config inputs explicitly and fail with appropriate errors for invalid `max_lock_secs` values.

Tasks

- [ ] Review `initialize` validation paths in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Improve error handling for invalid configuration inputs.
- [ ] Add tests for invalid `max_lock_secs` values during initialization.

---
`Ledger deposit path` does not validate `unlock_ledger` against network sequence drift
- Priority: Low
- Difficulty: Intermediate
- Labels: "bug", "validation", "future-proofing"

Description

The current implementation of ``deposit_by_ledger` does not validate `unlock_ledger` against network sequence drift` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
`get_depositors` pagination accepts an unbounded `limit`, leading to high memory use
- Priority: Low
- Difficulty: Intermediate
- Labels: "bug", "api", "scalability"

Description

The current implementation of ``get_depositors` pagination accepts an unbounded `limit`, leading to high memory use` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
`VaultEntry.depositor` duplicates the address available in the storage key
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "storage", "types"

Description

The current implementation of ``VaultEntry.depositor` duplicates the address available in the storage key` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
`LedgerVaultEntry.depositor` duplicates the address available in the storage key
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "storage", "types"

Description

The current implementation of ``LedgerVaultEntry.depositor` duplicates the address available in the storage key` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
`README` has no explicit example for `pause`/`unpause` behavior
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "documentation", "admin"

Description

The current implementation of ``README` has no explicit example for `pause`/`unpause` behavior` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

Repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
Ledger-based deposits are not documented as part of `Vault query` and `time_remaining`
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "documentation", "api"

Description

The current implementation of `Ledger-based deposits are not documented as part of `get_vault` and `time_remaining`` introduces a contract behavior gap that must be corrected. The vault query API currently omits ledger-based deposits from its results. External clients cannot reliably inspect all active vaults, undermining transparency and off-chain indexing. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

The vault query should return all active deposits regardless of whether they were created by timestamp or ledger lock semantics.

Tasks

- [ ] Audit vault query implementation for ledger deposit inclusion.
- [ ] Correct query behavior to return both time-based and ledger-based deposits.
- [ ] Add tests for query results with mixed deposit types.

---
`advance_time` test helper reconstructs ledger state instead of incrementing sequence consistently
- Priority: Low
- Difficulty: Intermediate
- Labels: "bug", "testing", "helpers"

Description

The current implementation of ``advance_time` test helper reconstructs ledger state instead of incrementing sequence consistently` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
No explicit token contract validation allows malicious token contracts
- Priority: Critical
- Difficulty: Advanced
- Labels: "security", "token", "validation"

Description

The current implementation of `No explicit token contract validation allows malicious token contracts` introduces a contract behavior gap that must be corrected. The contract accepts token addresses without explicit validation before transfers. This can allow malicious or malformed token contracts to be used, compromising safety and accounting. Affected files: `contracts/time-lock-vault/src/contract.rs` and `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Token contract addresses should be validated before use to prevent malicious or malformed token contracts.

Tasks

- [ ] Add token contract validation before performing token transfers.
- [ ] Document the validation expectations.
- [ ] Add tests for invalid token contract addresses.

---
`Ledger deposit path` bypasses pause, weakening emergency shutdown controls
- Priority: High
- Difficulty: Advanced
- Labels: "security", "admin", "pause"

Description

The current implementation of ``deposit_by_ledger` bypasses pause, weakening emergency shutdown controls` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
`Emergency withdrawal path` does not support ledger deposits, leaving some funds unrecoverable in recovery flow
- Priority: High
- Difficulty: Advanced
- Labels: "security", "admin", "recovery"

Description

The current implementation of ``emergency_withdraw` does not support ledger deposits, leaving some funds unrecoverable in recovery flow` introduces a contract behavior gap that must be corrected. The emergency recovery path only supports timestamp-based deposits and ignores ledger-based vault entries. That exposes a recovery gap where some deposits cannot be recovered by admin functions. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Emergency recovery should cover both timestamp and ledger-based deposits so admin recovery is complete.

Tasks

- [ ] Review emergency withdrawal paths for ledger and timestamp deposits.
- [ ] Extend `emergency_withdraw` to support ledger-based deposit entries.
- [ ] Add tests that exercise emergency recovery for ledger deposits.

---
`Time remaining query` returns 0 for ledger deposits, creating a misleading unlocked signal
- Priority: High
- Difficulty: Intermediate
- Labels: "security", "api", "ux"

Description

The current implementation of ``time_remaining` returns 0 for ledger deposits, creating a misleading unlocked signal` introduces a contract behavior gap that must be corrected. The time remaining calculation ignores ledger-based deposits and returns misleading values. This can cause callers to believe a deposit is unlocked when it is still locked by ledger sequence. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

The time remaining query should compute the correct remaining lock interval for ledger-based deposits and not return misleading zero values.

Tasks

- [ ] Audit time remaining calculation for ledger deposit entries.
- [ ] Fix the logic so ledger-based deposits produce correct remaining lock values.
- [ ] Add regression tests for ledger-derived remaining times.

---
`get_vault` and `Vault batch query` hide ledger deposit state from external indexers
- Priority: Medium
- Difficulty: Intermediate
- Labels: "security", "transparency", "api"

Description

The current implementation of ``get_vault` and `get_vault_batch` hide ledger deposit state from external indexers` introduces a contract behavior gap that must be corrected. The batch vault query currently omits ledger-based deposits, preventing complete client-side vault enumeration. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Batch vault queries should include ledger deposits and return complete vault state for clients.

Tasks

- [ ] Audit vault query implementation for ledger deposit inclusion.
- [ ] Correct query behavior to return both time-based and ledger-based deposits.
- [ ] Add tests for query results with mixed deposit types.

---
`Deposit cancellation` inability to cancel ledger deposits weakens depositor control
- Priority: Medium
- Difficulty: Intermediate
- Labels: "security", "contract", "ux"

Description

The current implementation of ``cancel_deposit` inability to cancel ledger deposits weakens depositor control` introduces a contract behavior gap that must be corrected. The cancel flow does not support ledger-based deposits, creating an inconsistent user experience. Depositors may not be able to cancel deposits they expect to manage, exposing functional gaps in the contract logic. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Cancel deposit should support the same deposit types and allow users to cancel valid ledger-based deposits where appropriate.

Tasks

- [ ] Review cancel flow for ledger deposit support.
- [ ] Extend cancel logic to handle ledger-based deposits consistently.
- [ ] Add tests covering cancellation of ledger deposits.

---
Admin storage reads do not bump TTL; admin privilege can expire unintentionally
- Priority: Medium
- Difficulty: Intermediate
- Labels: "security", "storage", "admin"

Description

The current implementation of `Admin storage reads do not bump TTL; admin privilege can expire unintentionally` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
Ledger deposit sequence semantics are not documented, raising future validation risk
- Priority: Medium
- Difficulty: Intermediate
- Labels: "security", "documentation", "contract"

Description

The current implementation of `Ledger deposit sequence semantics are not documented, raising future validation risk` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
No freeze mechanism for an address in case of compromised depositor or token abuse
- Priority: Medium
- Difficulty: Advanced
- Labels: "security", "admin", "contract"

Description

The current implementation of `No freeze mechanism for an address in case of compromised depositor or token abuse` introduces a contract behavior gap that must be corrected. The contract lacks a depositor freeze capability for compromised or abusive accounts. This reduces admin control and increases risk during fraud or abuse incidents. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

The contract should provide an administrative freeze mechanism for compromised depositor accounts.

Tasks

- [ ] Design depositor freeze behavior and admin controls.
- [ ] Implement freeze state tracking in storage helpers.
- [ ] Add tests for freeze and unfreeze scenarios.

---
No wallet recovery or migration path for ledger and timestamp deposits simultaneously
- Priority: Medium
- Difficulty: Advanced
- Labels: "security", "upgrades", "admin"

Description

The current implementation of `No wallet recovery or migration path for ledger and timestamp deposits simultaneously` introduces a contract behavior gap that must be corrected. The contract lacks a migration or recovery path for mixed ledger and timestamp deposit models. This may complicate future upgrades or user migration flows. Affected files: `contracts/time-lock-vault/src/contract.rs`, repo docs, and migration tooling.

Expected Behavior

The repository should add a recovery or migration path that covers both ledger and timestamp-based deposits.

Tasks

- [ ] Design upgrade and migration behavior for mixed deposit types.
- [ ] Document the recovery path.
- [ ] Add integration tests for migration scenarios.

---
Fee fallback to depositor in `Deposit cancellation` is not clearly documented
- Priority: Low
- Difficulty: Intermediate
- Labels: "security", "contract", "ux"

Description

The current implementation of `Fee fallback to depositor in `cancel_deposit` is not clearly documented` introduces a contract behavior gap that must be corrected. The cancel flow does not support ledger-based deposits, creating an inconsistent user experience. Depositors may not be able to cancel deposits they expect to manage, exposing functional gaps in the contract logic. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Cancel deposit should support the same deposit types and allow users to cancel valid ledger-based deposits where appropriate.

Tasks

- [ ] Review cancel flow for ledger deposit support.
- [ ] Extend cancel logic to handle ledger-based deposits consistently.
- [ ] Add tests covering cancellation of ledger deposits.

---
`Withdraw-to path` allows any recipient address without additional validation
- Priority: Low
- Difficulty: Intermediate
- Labels: "security", "ux", "contract"

Description

The current implementation of ``withdraw_to` allows any recipient address without additional validation` introduces a contract behavior gap that must be corrected. The withdrawal implementation does not correctly handle deposits created by ledger-based locks. This inconsistency leaves valid ledger deposits unreachable through the public withdraw API. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

Withdraw actions should support ledger-based deposits and return the correct outcome for both timestamp and ledger locks.

Tasks

- [ ] Review `withdraw_to` logic and verify ledger deposit compatibility.
- [ ] Add ledger deposit handling if missing.
- [ ] Add tests that exercise withdrawal of ledger-based deposits.

---
`Ledger deposit path` provides a sequence-based lock without cross-checking timestamp conversions
- Priority: Low
- Difficulty: Intermediate
- Labels: "security", "contract", "future-proofing"

Description

The current implementation of ``deposit_by_ledger` provides a sequence-based lock without cross-checking timestamp conversions` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
No on-chain key versioning in persistent storage for future contract upgrades
- Priority: Medium
- Difficulty: Advanced
- Labels: "security", "storage", "upgrades"

Description

The current implementation of `No on-chain key versioning in persistent storage for future contract upgrades` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
No event emitted when `cancel_transfer_admin` is invoked with no pending admin present
- Priority: Low
- Difficulty: Beginner
- Labels: "security", "events", "admin"

Description

The current implementation of `No event emitted when `cancel_transfer_admin` is invoked with no pending admin present` introduces a contract behavior gap that must be corrected. The admin transfer cancellation path does not emit an event when no pending admin exists. This makes off-chain monitoring and auditing less reliable for cancellation actions. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/events.rs`.

Expected Behavior

The admin transfer cancellation path should emit an event even when no pending admin exists, ensuring off-chain observability.

Tasks

- [ ] Add event emission for admin transfer cancellation when no pending admin exists.
- [ ] Update monitoring documentation to include the new event.
- [ ] Add tests ensuring the event is emitted in the expected condition.

---
`lock_duration` validation is duplicated in multiple deposit paths, increasing audit surface
- Priority: Low
- Difficulty: Beginner
- Labels: "security", "audit", "refactor"

Description

The current implementation of ``lock_duration` validation is duplicated in multiple deposit paths, increasing audit surface` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
`storage::add_depositor` scans the entire depositor list on every deposit
- Priority: Medium
- Difficulty: Intermediate
- Labels: "performance", "storage", "cost"

Description

The current implementation of ``storage::add_depositor` scans the entire depositor list on every deposit` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
`storage::Depositor removal` rebuilds the depositor list each removal
- Priority: Medium
- Difficulty: Intermediate
- Labels: "performance", "storage", "cost"

Description

The current implementation of ``storage::remove_depositor` rebuilds the depositor list each removal` introduces a contract behavior gap that must be corrected. The depositor removal path can remove an address while ledger deposits remain active. This risks leaving orphaned deposit state and breaking retrieval APIs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`.

Expected Behavior

Removing a depositor should not break active ledger deposits or leave orphaned state in storage.

Tasks

- [ ] Review depositor removal logic and ledger deposit interactions.
- [ ] Prevent removal of a depositor with active ledger deposits or clear related state safely.
- [ ] Add tests for depositor removal under mixed deposit conditions.

---
`Deposit ID enumeration` iterates all deposit IDs up to the counter for every call
- Priority: Medium
- Difficulty: Intermediate
- Labels: "performance", "storage", "scalability"

Description

The current implementation of ``get_deposit_ids` iterates all deposit IDs up to the counter for every call` introduces a contract behavior gap that must be corrected. The deposit identifier query does not include ledger-based entries, so clients cannot enumerate every deposit. This breaks deposit discovery and any off-chain feature that relies on a complete deposit list. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Deposit ID enumeration should list every deposit, including ledger-based entries, so external indexers can discover all active vaults.

Tasks

- [ ] Audit deposit ID enumeration for ledger deposit entries.
- [ ] Update `get_deposit_ids` to include all active deposits.
- [ ] Add tests for ledger deposit ID visibility.

---
`get_depositors_page` has no defensive cap on `limit`
- Priority: Medium
- Difficulty: Intermediate
- Labels: "performance", "api", "memory"

Description

The current implementation of ``get_depositors_page` has no defensive cap on `limit`` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
Event topics include full `Address` values, increasing payload size
- Priority: Low
- Difficulty: Intermediate
- Labels: "performance", "events", "cost"

Description

The current implementation of `Event topics include full `Address` values, increasing payload size` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
`VaultEntry` stores depositor twice, increasing persistent storage footprint
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "storage", "types"

Description

The current implementation of ``VaultEntry` stores depositor twice, increasing persistent storage footprint` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
`LedgerVaultEntry` stores depositor twice, increasing persistent storage footprint
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "storage", "types"

Description

The current implementation of ``LedgerVaultEntry` stores depositor twice, increasing persistent storage footprint` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
`token::Client::new()` is recreated in each function instead of using a helper
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "contract", "refactor"

Description

The current implementation of ``token::Client::new()` is recreated in each function instead of using a helper` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
Shared deposit validation code is duplicated across paths
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "contract", "refactor"

Description

The current implementation of `Shared deposit validation code is duplicated across paths` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
`Time remaining query` loads full entry data when only timestamp comparison is required
- Priority: Low
- Difficulty: Intermediate
- Labels: "performance", "storage", "contract"

Description

The current implementation of ``time_remaining` loads full entry data when only timestamp comparison is required` introduces a contract behavior gap that must be corrected. The time remaining calculation ignores ledger-based deposits and returns misleading values. This can cause callers to believe a deposit is unlocked when it is still locked by ledger sequence. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

The time remaining query should compute the correct remaining lock interval for ledger-based deposits and not return misleading zero values.

Tasks

- [ ] Audit time remaining calculation for ledger deposit entries.
- [ ] Fix the logic so ledger-based deposits produce correct remaining lock values.
- [ ] Add regression tests for ledger-derived remaining times.

---
`setup()` test helper re-registers the contract for every test
- Priority: Low
- Difficulty: Intermediate
- Labels: "performance", "testing", "dx"

Description

The current implementation of ``setup()` test helper re-registers the contract for every test` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
`advance_time` test helper reconstructs a full ledger snapshot on every call
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "testing", "dx"

Description

The current implementation of ``advance_time` test helper reconstructs a full ledger snapshot on every call` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
README lacks concrete Soroban CLI invocation examples for deposit and withdraw
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "dx", "readme"

Description

The current implementation of `README lacks concrete Soroban CLI invocation examples for deposit and withdraw` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

Repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
CHANGELOG does not clearly document the addition of ledger-based deposits
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "audit"

Description

The current implementation of `CHANGELOG does not clearly document the addition of ledger-based deposits` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The repository should maintain appropriate project documentation and governance artifacts for contributors, security, and release history.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
CONTRIBUTING lacks Soroban-specific contribution and testing guidance
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "contributing"

Description

The current implementation of `CONTRIBUTING lacks Soroban-specific contribution and testing guidance` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The repository should maintain appropriate project documentation and governance artifacts for contributors, security, and release history.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
SECURITY.md has no responsible disclosure process or severity guidelines
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "security"

Description

The current implementation of `SECURITY.md has no responsible disclosure process or severity guidelines` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The repository should maintain appropriate project documentation and governance artifacts for contributors, security, and release history.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
`BUMP_THRESHOLD` and `BUMP_TARGET` constants are undocumented in `storage.rs`
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "constants"

Description

The current implementation of ``BUMP_THRESHOLD` and `BUMP_TARGET` constants are undocumented in `storage.rs`` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
`MAX_DEPOSIT_AMOUNT` comment should clarify units and short/long-scale terminology
- Priority: Low
- Difficulty: Beginner
- Labels: "documentation", "types"

Description

The current implementation of ``MAX_DEPOSIT_AMOUNT` comment should clarify units and short/long-scale terminology` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
`VaultEntry` and `LedgerVaultEntry` fields lack unit documentation
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "types", "api"

Description

The current implementation of ``VaultEntry` and `LedgerVaultEntry` fields lack unit documentation` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
`events.rs` lacks a module-level explanation of event topic conventions
- Priority: Low
- Difficulty: Beginner
- Labels: "documentation", "events"

Description

The current implementation of ``events.rs` lacks a module-level explanation of event topic conventions` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
`storage.rs` does not document the complete persistent key layout
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "storage"

Description

The current implementation of ``storage.rs` does not document the complete persistent key layout` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
`contract.rs` does not explain the security model for `Emergency withdrawal path`
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "admin", "contract"

Description

The current implementation of ``contract.rs` does not explain the security model for `emergency_withdraw`` introduces a contract behavior gap that must be corrected. The emergency recovery path only supports timestamp-based deposits and ignores ledger-based vault entries. That exposes a recovery gap where some deposits cannot be recovered by admin functions. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Emergency recovery should cover both timestamp and ledger-based deposits so admin recovery is complete.

Tasks

- [ ] Review emergency withdrawal paths for ledger and timestamp deposits.
- [ ] Extend `emergency_withdraw` to support ledger-based deposit entries.
- [ ] Add tests that exercise emergency recovery for ledger deposits.

---
README does not explain the difference between time-based and ledger-based deposits
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "readme"

Description

The current implementation of `README does not explain the difference between time-based and ledger-based deposits` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

Repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
README does not document pause semantics for all deposit paths
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "admin", "readme"

Description

The current implementation of `README does not document pause semantics for all deposit paths` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

Repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
`scripts/deploy_testnet.sh` lacks inline usage examples and default environment assumptions
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "scripts"

Description

The current implementation of ``scripts/deploy_testnet.sh` lacks inline usage examples and default environment assumptions` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
README has no local Soroban standalone node integration testing instructions
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "testing"

Description

The current implementation of `README has no local Soroban standalone node integration testing instructions` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

Repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
plan.md does not define sprint cadence, review process, or branch policies
- Priority: Low
- Difficulty: Beginner
- Labels: "documentation", "process"

Description

The current implementation of `plan.md does not define sprint cadence, review process, or branch policies` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
README does not document when `is_initialized` must be checked before invocation
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "contract"

Description

The current implementation of `README does not document when `is_initialized` must be checked before invocation` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

Repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
README does not clarify `get_vault` vs `Vault batch query` differences
- Priority: Low
- Difficulty: Beginner
- Labels: "documentation", "api"

Description

The current implementation of `README does not clarify `get_vault` vs `get_vault_batch` differences` introduces a contract behavior gap that must be corrected. The batch vault query currently omits ledger-based deposits, preventing complete client-side vault enumeration. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Batch vault queries should include ledger deposits and return complete vault state for clients.

Tasks

- [ ] Audit vault query implementation for ledger deposit inclusion.
- [ ] Correct query behavior to return both time-based and ledger-based deposits.
- [ ] Add tests for query results with mixed deposit types.

---
lib.rs comment on the storage model is outdated compared to current key definitions
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "lib"

Description

The current implementation of `lib.rs comment on the storage model is outdated compared to current key definitions` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
No test verifying `Ledger deposit path` rejects deposits while paused
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "pause"

Description

The current implementation of `No test verifying `deposit_by_ledger` rejects deposits while paused` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
No test verifying `Ledger deposit path` rejects too-short ledger lock durations
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "validation"

Description

The current implementation of `No test verifying `deposit_by_ledger` rejects too-short ledger lock durations` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
No test verifying `Ledger deposit path` rejects too-long ledger lock durations
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "validation"

Description

The current implementation of `No test verifying `deposit_by_ledger` rejects too-long ledger lock durations` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
No test for `Withdraw-to path` with ledger-based deposits
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "contract"

Description

The current implementation of `No test for `withdraw_to` with ledger-based deposits` introduces a contract behavior gap that must be corrected. The withdrawal implementation does not correctly handle deposits created by ledger-based locks. This inconsistency leaves valid ledger deposits unreachable through the public withdraw API. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

Withdraw actions should support ledger-based deposits and return the correct outcome for both timestamp and ledger locks.

Tasks

- [ ] Review `withdraw_to` logic and verify ledger deposit compatibility.
- [ ] Add ledger deposit handling if missing.
- [ ] Add tests that exercise withdrawal of ledger-based deposits.

---
No test for `Emergency withdrawal path` when a ledger-based deposit exists
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "admin"

Description

The current implementation of `No test for `emergency_withdraw` when a ledger-based deposit exists` introduces a contract behavior gap that must be corrected. The emergency recovery path only supports timestamp-based deposits and ignores ledger-based vault entries. That exposes a recovery gap where some deposits cannot be recovered by admin functions. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Emergency recovery should cover both timestamp and ledger-based deposits so admin recovery is complete.

Tasks

- [ ] Review emergency withdrawal paths for ledger and timestamp deposits.
- [ ] Extend `emergency_withdraw` to support ledger-based deposit entries.
- [ ] Add tests that exercise emergency recovery for ledger deposits.

---
No test for `Vault query` ledger-deposit visibility
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "api"

Description

The current implementation of `No test for `get_vault` ledger-deposit visibility` introduces a contract behavior gap that must be corrected. The vault query API currently omits ledger-based deposits from its results. External clients cannot reliably inspect all active vaults, undermining transparency and off-chain indexing. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

The vault query should return all active deposits regardless of whether they were created by timestamp or ledger lock semantics.

Tasks

- [ ] Audit vault query implementation for ledger deposit inclusion.
- [ ] Correct query behavior to return both time-based and ledger-based deposits.
- [ ] Add tests for query results with mixed deposit types.

---
No test for `Time remaining query` with ledger-based deposits
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "api"

Description

The current implementation of `No test for `time_remaining` with ledger-based deposits` introduces a contract behavior gap that must be corrected. The time remaining calculation ignores ledger-based deposits and returns misleading values. This can cause callers to believe a deposit is unlocked when it is still locked by ledger sequence. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

The time remaining query should compute the correct remaining lock interval for ledger-based deposits and not return misleading zero values.

Tasks

- [ ] Audit time remaining calculation for ledger deposit entries.
- [ ] Fix the logic so ledger-based deposits produce correct remaining lock values.
- [ ] Add regression tests for ledger-derived remaining times.

---
No test for `Deposit ID enumeration` including ledger-based deposit IDs
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "storage"

Description

The current implementation of `No test for `get_deposit_ids` including ledger-based deposit IDs` introduces a contract behavior gap that must be corrected. The deposit identifier query does not include ledger-based entries, so clients cannot enumerate every deposit. This breaks deposit discovery and any off-chain feature that relies on a complete deposit list. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Deposit ID enumeration should list every deposit, including ledger-based entries, so external indexers can discover all active vaults.

Tasks

- [ ] Audit deposit ID enumeration for ledger deposit entries.
- [ ] Update `get_deposit_ids` to include all active deposits.
- [ ] Add tests for ledger deposit ID visibility.

---
No test for `Vault batch query` covering ledger deposit paths
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "api"

Description

The current implementation of `No test for `get_vault_batch` covering ledger deposit paths` introduces a contract behavior gap that must be corrected. The batch vault query currently omits ledger-based deposits, preventing complete client-side vault enumeration. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Batch vault queries should include ledger deposits and return complete vault state for clients.

Tasks

- [ ] Audit vault query implementation for ledger deposit inclusion.
- [ ] Correct query behavior to return both time-based and ledger-based deposits.
- [ ] Add tests for query results with mixed deposit types.

---
No test for `Depositor removal` with mixed deposit types
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "storage"

Description

The current implementation of `No test for `remove_depositor` with mixed deposit types` introduces a contract behavior gap that must be corrected. The depositor removal path can remove an address while ledger deposits remain active. This risks leaving orphaned deposit state and breaking retrieval APIs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`.

Expected Behavior

Removing a depositor should not break active ledger deposits or leave orphaned state in storage.

Tasks

- [ ] Review depositor removal logic and ledger deposit interactions.
- [ ] Prevent removal of a depositor with active ledger deposits or clear related state safely.
- [ ] Add tests for depositor removal under mixed deposit conditions.

---
No test for `Ledger deposit path` transfer failure rollback
- Priority: Medium
- Difficulty: Advanced
- Labels: "testing", "error-path"

Description

The current implementation of `No test for `deposit_by_ledger` transfer failure rollback` introduces a contract behavior gap that must be corrected. The ledger-based deposit path is currently missing validation checks and consistency with the timestamp-based flow. This can lead to paused contracts accepting deposits, invalid lock durations, and a mismatch between ledger state and public API queries. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and README documentation.

Expected Behavior

The ledger deposit path should use the same pause and duration validation as the timestamp deposit path, exposing all ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.

---
No test for `pause`/`unpause` semantics across both deposit methods
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "admin"

Description

The current implementation of `No test for `pause`/`unpause` semantics across both deposit methods` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
No test for `Deposit cancellation` behavior on ledger deposits
- Priority: Low
- Difficulty: Intermediate
- Labels: "testing", "contract"

Description

The current implementation of `No test for `cancel_deposit` behavior on ledger deposits` introduces a contract behavior gap that must be corrected. The cancel flow does not support ledger-based deposits, creating an inconsistent user experience. Depositors may not be able to cancel deposits they expect to manage, exposing functional gaps in the contract logic. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Cancel deposit should support the same deposit types and allow users to cancel valid ledger-based deposits where appropriate.

Tasks

- [ ] Review cancel flow for ledger deposit support.
- [ ] Extend cancel logic to handle ledger-based deposits consistently.
- [ ] Add tests covering cancellation of ledger deposits.

---
No test verifying `get_constants` with custom initialization values
- Priority: Low
- Difficulty: Beginner
- Labels: "testing", "constants"

Description

The current implementation of `No test verifying `get_constants` with custom initialization values` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
No test verifying `deposit_for` and `deposit` share the same amount constraints
- Priority: Low
- Difficulty: Beginner
- Labels: "testing", "consistency"

Description

The current implementation of `No test verifying `deposit_for` and `deposit` share the same amount constraints` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
No test verifying `Withdraw-to path` event payload values
- Priority: Low
- Difficulty: Intermediate
- Labels: "testing", "events"

Description

The current implementation of `No test verifying `withdraw_to` event payload values` introduces a contract behavior gap that must be corrected. The withdrawal implementation does not correctly handle deposits created by ledger-based locks. This inconsistency leaves valid ledger deposits unreachable through the public withdraw API. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`.

Expected Behavior

Withdraw actions should support ledger-based deposits and return the correct outcome for both timestamp and ledger locks.

Tasks

- [ ] Review `withdraw_to` logic and verify ledger deposit compatibility.
- [ ] Add ledger deposit handling if missing.
- [ ] Add tests that exercise withdrawal of ledger-based deposits.

---
No test for `get_depositor_count` after mixed deposit removals
- Priority: Low
- Difficulty: Beginner
- Labels: "testing", "storage"

Description

The current implementation of `No test for `get_depositor_count` after mixed deposit removals` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
No integration test validating README example flows
- Priority: Medium
- Difficulty: Advanced
- Labels: "testing", "integration"

Description

The current implementation of `No integration test validating README example flows` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

Repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
No fuzz or boundary tests for minimum and maximum deposit amounts across paths
- Priority: Medium
- Difficulty: Advanced
- Labels: "testing", "fuzzing"

Description

The current implementation of `No fuzz or boundary tests for minimum and maximum deposit amounts across paths` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
No stress test for `get_depositors` pagination size and edge behavior
- Priority: Low
- Difficulty: Advanced
- Labels: "testing", "performance"

Description

The current implementation of `No stress test for `get_depositors` pagination size and edge behavior` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
Extract shared deposit validation logic into a single helper
- Priority: Medium
- Difficulty: Intermediate
- Labels: "refactor", "contract"

Description

The current implementation of `Extract shared deposit validation logic into a single helper` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Factor ledger and timestamp deposit storage into separate helper modules
- Priority: Medium
- Difficulty: Intermediate
- Labels: "refactor", "storage"

Description

The current implementation of `Factor ledger and timestamp deposit storage into separate helper modules` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
Introduce reusable `require_admin` helper to simplify admin checks
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "dx"

Description

The current implementation of `Introduce reusable `require_admin` helper to simplify admin checks` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Introduce a shared pause guard helper for deposit entry points
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "admin"

Description

The current implementation of `Introduce a shared pause guard helper for deposit entry points` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Extract token transfer operations into a reusable helper
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "contract"

Description

The current implementation of `Extract token transfer operations into a reusable helper` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Remove duplicate depositor storage in `VaultEntry` and `LedgerVaultEntry` if possible
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "storage"

Description

The current implementation of `Remove duplicate depositor storage in `VaultEntry` and `LedgerVaultEntry` if possible` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
Replace `test.rs` 5-tuple setup with a `TestContext` struct
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "testing"

Description

The current implementation of `Replace `test.rs` 5-tuple setup with a `TestContext` struct` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
Extract constants like `TEST_MINT_AMOUNT` from repeated test literals
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "testing"

Description

The current implementation of `Extract constants like `TEST_MINT_AMOUNT` from repeated test literals` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
Simplify repeated admin authorization pattern in contract.rs
- Priority: Medium
- Difficulty: Intermediate
- Labels: "refactor", "contract"

Description

The current implementation of `Simplify repeated admin authorization pattern in contract.rs` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Consolidate `types.rs` and `errors.rs` into a smaller model module for cohesion
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "structure"

Description

The current implementation of `Consolidate `types.rs` and `errors.rs` into a smaller model module for cohesion` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Simplify crate exports in `lib.rs` for a cleaner public interface
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "lib"

Description

The current implementation of `Simplify crate exports in `lib.rs` for a cleaner public interface` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Update `Makefile` check target to include build verification for parity with CI
- Priority: Medium
- Difficulty: Beginner
- Labels: "refactor", "devops"

Description

The current implementation of `Update `Makefile` check target to include build verification for parity with CI` introduces a contract behavior gap that must be corrected. The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files.

Expected Behavior

The CI and repository tooling should enforce the missing validation or documentation checks before merges.

Tasks

- [ ] Update CI or tooling configuration to include the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow by running the relevant job locally if possible.

---
Add `top_up(depositor, amount)` to increase a lock without changing unlock time
- Priority: High
- Difficulty: Intermediate
- Labels: "feature", "contract"

Description

The current implementation of `Add `top_up(depositor, amount)` to increase a lock without changing unlock time` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add `extend_lock(depositor, new_unlock_time)` to lengthen existing locks
- Priority: High
- Difficulty: Intermediate
- Labels: "feature", "contract"

Description

The current implementation of `Add `extend_lock(depositor, new_unlock_time)` to lengthen existing locks` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add `batch_Emergency withdrawal path` to match README and support recovery migration
- Priority: High
- Difficulty: Advanced
- Labels: "feature", "admin", "security"

Description

The current implementation of `Add `batch_emergency_withdraw` to match README and support recovery migration` introduces a contract behavior gap that must be corrected. The emergency recovery path only supports timestamp-based deposits and ignores ledger-based vault entries. That exposes a recovery gap where some deposits cannot be recovered by admin functions. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Emergency recovery should cover both timestamp and ledger-based deposits so admin recovery is complete.

Tasks

- [ ] Review emergency withdrawal paths for ledger and timestamp deposits.
- [ ] Extend `emergency_withdraw` to support ledger-based deposit entries.
- [ ] Add tests that exercise emergency recovery for ledger deposits.

---
Add `batch_withdraw` to withdraw multiple deposits in one call
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "contract", "scalability"

Description

The current implementation of `Add `batch_withdraw` to withdraw multiple deposits in one call` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add `deposit_on_behalf` for third-party deposit flow
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "contract", "ux"

Description

The current implementation of `Add `deposit_on_behalf` for third-party deposit flow` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add admin-configurable token whitelist for accepted token contracts
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "admin", "security"

Description

The current implementation of `Add admin-configurable token whitelist for accepted token contracts` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add `get_all_vaults` or paginated aggregate query for off-chain indexing
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "api", "scalability"

Description

The current implementation of `Add `get_all_vaults` or paginated aggregate query for off-chain indexing` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add `get_total_locked(token)` aggregate query for TVL and analytics
- Priority: Medium
- Difficulty: Intermediate
- Labels: "feature", "api", "analytics"

Description

The current implementation of `Add `get_total_locked(token)` aggregate query for TVL and analytics` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add runtime update support for `fee_recipient` without redeploying
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "admin", "economics"

Description

The current implementation of `Add runtime update support for `fee_recipient` without redeploying` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add admin-managed emergency freeze for specific depositors or tokens
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "admin", "security"

Description

The current implementation of `Add admin-managed emergency freeze for specific depositors or tokens` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add configurable deposit penalty caps or fee rules for `Deposit cancellation`
- Priority: Low
- Difficulty: Advanced
- Labels: "feature", "contract", "economics"

Description

The current implementation of `Add configurable deposit penalty caps or fee rules for `cancel_deposit`` introduces a contract behavior gap that must be corrected. The cancel flow does not support ledger-based deposits, creating an inconsistent user experience. Depositors may not be able to cancel deposits they expect to manage, exposing functional gaps in the contract logic. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`.

Expected Behavior

Cancel deposit should support the same deposit types and allow users to cancel valid ledger-based deposits where appropriate.

Tasks

- [ ] Review cancel flow for ledger deposit support.
- [ ] Extend cancel logic to handle ledger-based deposits consistently.
- [ ] Add tests covering cancellation of ledger deposits.

---
Add a `vault_status` query summarizing contract pause/admin state
- Priority: Low
- Difficulty: Intermediate
- Labels: "feature", "api", "ux"

Description

The current implementation of `Add a `vault_status` query summarizing contract pause/admin state` introduces a contract behavior gap that must be corrected. The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding it will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation.

Expected Behavior

The repository should add the requested feature with a clear API surface and consistent storage behavior.

Tasks

- [ ] Design the new API surface in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add storage helpers and type support for the feature.
- [ ] Document usage examples in `README.md`.

---
Add `cargo audit` to CI to catch dependency vulnerabilities
- Priority: High
- Difficulty: Intermediate
- Labels: "devops", "ci", "security"

Description

The current implementation of `Add `cargo audit` to CI to catch dependency vulnerabilities` introduces a contract behavior gap that must be corrected. The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files.

Expected Behavior

The CI and repository tooling should enforce the missing validation or documentation checks before merges.

Tasks

- [ ] Update CI or tooling configuration to include the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow by running the relevant job locally if possible.

---
Add a GitHub Release workflow that builds optimized WASM assets
- Priority: High
- Difficulty: Intermediate
- Labels: "devops", "ci", "release"

Description

The current implementation of `Add a GitHub Release workflow that builds optimized WASM assets` introduces a contract behavior gap that must be corrected. The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files.

Expected Behavior

The CI and repository tooling should enforce the missing validation or documentation checks before merges.

Tasks

- [ ] Update CI or tooling configuration to include the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow by running the relevant job locally if possible.

---
Add `cargo test --release --features testutils` to CI for optimized build coverage
- Priority: Medium
- Difficulty: Intermediate
- Labels: "devops", "ci", "testing"

Description

The current implementation of `Add `cargo test --release --features testutils` to CI for optimized build coverage` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
Add shell syntax and usage validation for `scripts/deploy_testnet.sh`
- Priority: Medium
- Difficulty: Intermediate
- Labels: "devops", "ci", "scripts"

Description

The current implementation of `Add shell syntax and usage validation for `scripts/deploy_testnet.sh`` introduces a contract behavior gap that must be corrected. The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files.

Expected Behavior

The CI and repository tooling should enforce the missing validation or documentation checks before merges.

Tasks

- [ ] Update CI or tooling configuration to include the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow by running the relevant job locally if possible.

---
Add a `Makefile` target for toolchain and `soroban-cli` bootstrap
- Priority: Medium
- Difficulty: Beginner
- Labels: "devops", "dx", "makefile"

Description

The current implementation of `Add a `Makefile` target for toolchain and `soroban-cli` bootstrap` introduces a contract behavior gap that must be corrected. The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files.

Expected Behavior

The CI and repository tooling should enforce the missing validation or documentation checks before merges.

Tasks

- [ ] Update CI or tooling configuration to include the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow by running the relevant job locally if possible.

---
Add `.env.example` documenting required environment variables for deployment
- Priority: Medium
- Difficulty: Beginner
- Labels: "devops", "dx", "documentation"

Description

The current implementation of `Add `.env.example` documenting required environment variables for deployment` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The CI and repository tooling should enforce the missing validation or documentation checks before merges.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
Add CI guard for README examples and local integration instructions
- Priority: Medium
- Difficulty: Intermediate
- Labels: "devops", "documentation"

Description

The current implementation of `Add CI guard for README examples and local integration instructions` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

Repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

---
Add WASM size regression checks across PRs
- Priority: Medium
- Difficulty: Intermediate
- Labels: "devops", "ci", "performance"

Description

The current implementation of `Add WASM size regression checks across PRs` introduces a contract behavior gap that must be corrected. The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs for common operations. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and test helpers.

Expected Behavior

The contract should minimize storage scans, avoid unbounded query parameters, and keep public APIs performant.

Tasks

- [ ] Review the referenced storage helpers and query functions.
- [ ] Reduce unnecessary scans and limit unbounded parameters.
- [ ] Add targeted performance or boundary tests.

---
Add Dependabot or Renovate config for `soroban-sdk` and Rust dependency updates
- Priority: Medium
- Difficulty: Beginner
- Labels: "devops", "dependencies"

Description

The current implementation of `Add Dependabot or Renovate config for `soroban-sdk` and Rust dependency updates` introduces a contract behavior gap that must be corrected. The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files.

Expected Behavior

The CI and repository tooling should enforce the missing validation or documentation checks before merges.

Tasks

- [ ] Update CI or tooling configuration to include the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow by running the relevant job locally if possible.

---
Add a developer quickstart section for contract iteration and local testing
- Priority: Medium
- Difficulty: Beginner
- Labels: "dx", "testing"

Description

The current implementation of `Add a developer quickstart section for contract iteration and local testing` introduces a contract behavior gap that must be corrected. The test suite does not cover this behavior, leaving a gap in contract validation and regression protection. Without a dedicated test, future changes can break the contract silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught in continuous integration.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario in test comments.

---
Extend issue templates with a Soroban security-contract bug checklist
- Priority: Medium
- Difficulty: Beginner
- Labels: "dx", "github", "security"

Description

The current implementation of `Extend issue templates with a Soroban security-contract bug checklist` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Extend PR template with contract-specific testing and audit checklist
- Priority: Medium
- Difficulty: Beginner
- Labels: "dx", "github", "contributing"

Description

The current implementation of `Extend PR template with contract-specific testing and audit checklist` introduces a contract behavior gap that must be corrected. The current implementation is missing a required behavior or contains an inconsistency that should be corrected. This issue impacts contract correctness, observability, or developer experience. Affected files include the contract source, storage helpers, and documentation for this feature.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Add a contributor-facing troubleshooting section for Soroban CLI and WASM build issues
- Priority: Low
- Difficulty: Beginner
- Labels: "dx", "documentation"

Description

The current implementation of `Add a contributor-facing troubleshooting section for Soroban CLI and WASM build issues` introduces a contract behavior gap that must be corrected. The repository documentation currently lacks the required details, examples, or references for this contract behavior. This gap reduces developer understanding and increases the chance of incorrect integration or audit assumptions. Affected files: `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or relevant script docs.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or references that clarify the contract behavior.
- [ ] Validate the documentation changes against current contract APIs.

