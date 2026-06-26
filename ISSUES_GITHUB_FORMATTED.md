---
Ledger deposit path bypasses the paused contract guard
- Priority: Critical
- Difficulty: Advanced
- Labels: "bug", "security", "pause"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
Ledger deposit path does not enforce minimum lock duration
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "validation", "contract"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
Ledger deposit path does not enforce maximum lock duration
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "validation", "contract"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
Withdraw-to path only works for time-based deposits, ignoring ledger deposits
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "contract", "storage"

Description

The `withdraw_to` implementation in `contracts/time-lock-vault/src/contract.rs` currently only supports time-based deposits and ignores ledger-based vault entries. That means valid ledger deposits cannot be withdrawn through this public API, creating a functional gap. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`. This inconsistency risks broken withdrawal behavior and poor client interoperability.

Expected Behavior

Withdraw-to should support both time-based and ledger-based deposits and return correct results for all active vault entries.

Tasks

- [ ] Review `withdraw_to` implementation in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend withdrawal handling to support ledger-based deposits.
- [ ] Add tests covering withdrawal of ledger deposit entries.

---
Emergency withdrawal path only works for time-based deposits, ignoring ledger deposits
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "admin", "recovery"

Description

The emergency withdrawal path in `contracts/time-lock-vault/src/contract.rs` does not support ledger-based deposits. As a result, some deposits cannot be recovered by the admin emergency flow, leaving funds stuck. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`. This undermines recovery guarantees and increases operational risk.

Expected Behavior

Emergency withdrawal should recover both timestamp-based and ledger-based deposits so admin recovery flows are complete.

Tasks

- [ ] Review emergency withdrawal logic in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend support to ledger-based deposits.
- [ ] Add recovery tests for ledger deposits.

---
Vault query does not expose ledger-based deposits
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "api", "storage"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Time remaining query ignores ledger-based deposits and returns 0
- Priority: High
- Difficulty: Advanced
- Labels: "bug", "api", "ux"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Deposit ID enumeration skips ledger-based deposit IDs
- Priority: Medium
- Difficulty: Intermediate
- Labels: "bug", "api", "storage"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Vault batch query reads only time-based deposits, not ledger-based deposits
- Priority: Medium
- Difficulty: Intermediate
- Labels: "bug", "api", "storage"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Cancel deposit cannot cancel ledger-based deposits
- Priority: Medium
- Difficulty: Intermediate
- Labels: "bug", "contract", "storage"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Depositor removal can clear an address while ledger deposits remain active
- Priority: Medium
- Difficulty: Intermediate
- Labels: "bug", "storage", "consistency"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
README documents non-existent batch_Emergency withdrawal path API
- Priority: Medium
- Difficulty: Beginner
- Labels: "bug", "documentation", "contract"

Description

The emergency withdrawal path in `contracts/time-lock-vault/src/contract.rs` does not support ledger-based deposits. As a result, some deposits cannot be recovered by the admin emergency flow, leaving funds stuck. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`. This undermines recovery guarantees and increases operational risk.

Expected Behavior

Emergency withdrawal should recover both timestamp-based and ledger-based deposits so admin recovery flows are complete.

Tasks

- [ ] Review emergency withdrawal logic in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend support to ledger-based deposits.
- [ ] Add recovery tests for ledger deposits.

---
README omits Ledger deposit path, Withdraw-to path, and ledger deposit semantics
- Priority: Medium
- Difficulty: Beginner
- Labels: "bug", "documentation", "api"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
Ledger deposit path uses a different validation path than deposit/deposit_for
- Priority: Low
- Difficulty: Intermediate
- Labels: "bug", "refactor", "contract"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
Initialize treats zero max_lock_secs as LockDurationTooLong instead of explicit invalid config
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "validation", "contract"

Description

The initialization flow in `contracts/time-lock-vault/src/contract.rs` currently handles invalid configuration values ambiguously. This makes it harder to distinguish invalid inputs from actual runtime lock errors. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`. This issue can cause misconfiguration and reduce deployment safety.

Expected Behavior

Initialization should validate configuration inputs explicitly and fail with appropriate errors for invalid `max_lock_secs` values.

Tasks

- [ ] Review `initialize` validation paths in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Improve error handling for invalid `max_lock_secs` values.
- [ ] Add tests for invalid initialization inputs.

---
Ledger deposit path does not validate unlock_ledger against network sequence drift
- Priority: Low
- Difficulty: Intermediate
- Labels: "bug", "validation", "future-proofing"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
Get_depositors pagination accepts an unbounded limit, leading to high memory use
- Priority: Low
- Difficulty: Intermediate
- Labels: "bug", "api", "scalability"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
VaultEntry.depositor duplicates the address available in the storage key
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "storage", "types"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
LedgerVaultEntry.depositor duplicates the address available in the storage key
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "storage", "types"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
README has no explicit example for pause/unpause behavior
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "documentation", "admin"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Ledger-based deposits are not documented as part of Vault query and Time remaining query
- Priority: Low
- Difficulty: Beginner
- Labels: "bug", "documentation", "api"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Advance_time test helper reconstructs ledger state instead of incrementing sequence consistently
- Priority: Low
- Difficulty: Intermediate
- Labels: "bug", "testing", "helpers"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No explicit token contract validation allows malicious token contracts
- Priority: Critical
- Difficulty: Advanced
- Labels: "security", "token", "validation"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Ledger deposit path bypasses pause, weakening emergency shutdown controls
- Priority: High
- Difficulty: Advanced
- Labels: "security", "admin", "pause"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
Emergency withdrawal path does not support ledger deposits, leaving some funds unrecoverable in recovery flow
- Priority: High
- Difficulty: Advanced
- Labels: "security", "admin", "recovery"

Description

The emergency withdrawal path in `contracts/time-lock-vault/src/contract.rs` does not support ledger-based deposits. As a result, some deposits cannot be recovered by the admin emergency flow, leaving funds stuck. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`. This undermines recovery guarantees and increases operational risk.

Expected Behavior

Emergency withdrawal should recover both timestamp-based and ledger-based deposits so admin recovery flows are complete.

Tasks

- [ ] Review emergency withdrawal logic in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend support to ledger-based deposits.
- [ ] Add recovery tests for ledger deposits.

---
Time remaining query returns 0 for ledger deposits, creating a misleading unlocked signal
- Priority: High
- Difficulty: Intermediate
- Labels: "security", "api", "ux"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Vault query and Vault batch query hide ledger deposit state from external indexers
- Priority: Medium
- Difficulty: Intermediate
- Labels: "security", "transparency", "api"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Cancel deposit inability to cancel ledger deposits weakens depositor control
- Priority: Medium
- Difficulty: Intermediate
- Labels: "security", "contract", "ux"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Admin storage reads do not bump TTL; admin privilege can expire unintentionally
- Priority: Medium
- Difficulty: Intermediate
- Labels: "security", "storage", "admin"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Ledger deposit sequence semantics are not documented, raising future validation risk
- Priority: Medium
- Difficulty: Intermediate
- Labels: "security", "documentation", "contract"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
No freeze mechanism for an address in case of compromised depositor or token abuse
- Priority: Medium
- Difficulty: Advanced
- Labels: "security", "admin", "contract"

Description

The contract lacks an administrative freeze mechanism for compromised or abusive depositor addresses. That reduces the ability to mitigate fraud or abuse. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`. This issue weakens incident response and operational security.

Expected Behavior

The contract should provide an administrative freeze mechanism for compromised depositor accounts.

Tasks

- [ ] Design administratively controlled depositor freeze behavior.
- [ ] Implement freeze state in storage helpers.
- [ ] Add tests for freeze/unfreeze operations.

---
No wallet recovery or migration path for ledger and timestamp deposits simultaneously
- Priority: Medium
- Difficulty: Advanced
- Labels: "security", "upgrades", "admin"

Description

The repository does not provide a clear recovery or migration path for mixed ledger and timestamp deposit models. This may complicate upgrades and preservation of depositor funds. Affected files include `contracts/time-lock-vault/src/contract.rs` and related documentation. This issue increases upgrade risk and user uncertainty.

Expected Behavior

The repository should add a recovery or migration path that covers both ledger-based and timestamp-based deposits.

Tasks

- [ ] Define a recovery/migration path for mixed deposit models.
- [ ] Document the recovery behavior and interface.
- [ ] Add integration tests for migration scenarios.

---
Fee fallback to depositor in Cancel deposit is not clearly documented
- Priority: Low
- Difficulty: Intermediate
- Labels: "security", "contract", "ux"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Withdraw-to path allows any recipient address without additional validation
- Priority: Low
- Difficulty: Intermediate
- Labels: "security", "ux", "contract"

Description

The `withdraw_to` implementation in `contracts/time-lock-vault/src/contract.rs` currently only supports time-based deposits and ignores ledger-based vault entries. That means valid ledger deposits cannot be withdrawn through this public API, creating a functional gap. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`. This inconsistency risks broken withdrawal behavior and poor client interoperability.

Expected Behavior

Withdraw-to should support both time-based and ledger-based deposits and return correct results for all active vault entries.

Tasks

- [ ] Review `withdraw_to` implementation in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend withdrawal handling to support ledger-based deposits.
- [ ] Add tests covering withdrawal of ledger deposit entries.

---
Ledger deposit path provides a sequence-based lock without cross-checking timestamp conversions
- Priority: Low
- Difficulty: Intermediate
- Labels: "security", "contract", "future-proofing"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
No on-chain key versioning in persistent storage for future contract upgrades
- Priority: Medium
- Difficulty: Advanced
- Labels: "security", "storage", "upgrades"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
No event emitted when cancel_transfer_admin is invoked with no pending admin present
- Priority: Low
- Difficulty: Beginner
- Labels: "security", "events", "admin"

Description

The admin transfer cancellation path does not emit an event when no pending admin exists. This reduces off-chain auditability and makes monitoring admin state changes harder. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/events.rs`. This issue weakens operational transparency.

Expected Behavior

The admin transfer cancellation path should emit an event in the no-pending-admin case, ensuring off-chain observability.

Tasks

- [ ] Add event emission for admin transfer cancellation when no pending admin exists.
- [ ] Update monitoring documentation to include the new event.
- [ ] Add tests verifying the event is emitted.

---
Lock_duration validation is duplicated in multiple deposit paths, increasing audit surface
- Priority: Low
- Difficulty: Beginner
- Labels: "security", "audit", "refactor"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Storage::add_depositor scans the entire depositor list on every deposit
- Priority: Medium
- Difficulty: Intermediate
- Labels: "performance", "storage", "cost"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Storage::Depositor removal rebuilds the depositor list each removal
- Priority: Medium
- Difficulty: Intermediate
- Labels: "performance", "storage", "cost"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Deposit ID enumeration iterates all deposit IDs up to the counter for every call
- Priority: Medium
- Difficulty: Intermediate
- Labels: "performance", "storage", "scalability"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Get_depositors_page has no defensive cap on limit
- Priority: Medium
- Difficulty: Intermediate
- Labels: "performance", "api", "memory"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Event topics include full Address values, increasing payload size
- Priority: Low
- Difficulty: Intermediate
- Labels: "performance", "events", "cost"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
VaultEntry stores depositor twice, increasing persistent storage footprint
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "storage", "types"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
LedgerVaultEntry stores depositor twice, increasing persistent storage footprint
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "storage", "types"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Token::Client::new() is recreated in each function instead of using a helper
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "contract", "refactor"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Shared deposit validation code is duplicated across paths
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "contract", "refactor"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Time remaining query loads full entry data when only timestamp comparison is required
- Priority: Low
- Difficulty: Intermediate
- Labels: "performance", "storage", "contract"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Setup() test helper re-registers the contract for every test
- Priority: Low
- Difficulty: Intermediate
- Labels: "performance", "testing", "dx"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Advance_time test helper reconstructs a full ledger snapshot on every call
- Priority: Low
- Difficulty: Beginner
- Labels: "performance", "testing", "dx"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
README lacks concrete Soroban CLI invocation examples for deposit and withdraw
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "dx", "readme"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
CHANGELOG does not clearly document the addition of ledger-based deposits
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "audit"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository should maintain appropriate documentation and governance artifacts for contributors, security, and release history.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
CONTRIBUTING lacks Soroban-specific contribution and testing guidance
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "contributing"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository should maintain appropriate documentation and governance artifacts for contributors, security, and release history.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
SECURITY.md has no responsible disclosure process or severity guidelines
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "security"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository should maintain appropriate documentation and governance artifacts for contributors, security, and release history.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
BUMP_THRESHOLD and BUMP_TARGET constants are undocumented in storage.rs
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "constants"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
MAX_DEPOSIT_AMOUNT comment should clarify units and short/long-scale terminology
- Priority: Low
- Difficulty: Beginner
- Labels: "documentation", "types"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
VaultEntry and LedgerVaultEntry fields lack unit documentation
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "types", "api"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Events.rs lacks a module-level explanation of event topic conventions
- Priority: Low
- Difficulty: Beginner
- Labels: "documentation", "events"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Storage.rs does not document the complete persistent key layout
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "storage"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Contract.rs does not explain the security model for Emergency withdrawal path
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "admin", "contract"

Description

The emergency withdrawal path in `contracts/time-lock-vault/src/contract.rs` does not support ledger-based deposits. As a result, some deposits cannot be recovered by the admin emergency flow, leaving funds stuck. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`. This undermines recovery guarantees and increases operational risk.

Expected Behavior

Emergency withdrawal should recover both timestamp-based and ledger-based deposits so admin recovery flows are complete.

Tasks

- [ ] Review emergency withdrawal logic in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend support to ledger-based deposits.
- [ ] Add recovery tests for ledger deposits.

---
README does not explain the difference between time-based and ledger-based deposits
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "readme"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
README does not document pause semantics for all deposit paths
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "admin", "readme"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Scripts/deploy_testnet.sh lacks inline usage examples and default environment assumptions
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "scripts"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
README has no local Soroban standalone node integration testing instructions
- Priority: High
- Difficulty: Beginner
- Labels: "documentation", "testing"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Plan.md does not define sprint cadence, review process, or branch policies
- Priority: Low
- Difficulty: Beginner
- Labels: "documentation", "process"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
README does not document when is_initialized must be checked before invocation
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "contract"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
README does not clarify Vault query vs Vault batch query differences
- Priority: Low
- Difficulty: Beginner
- Labels: "documentation", "api"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The repository documentation should clearly describe the current contract APIs, ledger vs timestamp deposit behavior, and pause semantics.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Lib.rs comment on the storage model is outdated compared to current key definitions
- Priority: Medium
- Difficulty: Beginner
- Labels: "documentation", "lib"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
No test verifying Ledger deposit path rejects deposits while paused
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "pause"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
No test verifying Ledger deposit path rejects too-short ledger lock durations
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "validation"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
No test verifying Ledger deposit path rejects too-long ledger lock durations
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "validation"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
No test for Withdraw-to path with ledger-based deposits
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "contract"

Description

The `withdraw_to` implementation in `contracts/time-lock-vault/src/contract.rs` currently only supports time-based deposits and ignores ledger-based vault entries. That means valid ledger deposits cannot be withdrawn through this public API, creating a functional gap. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`. This inconsistency risks broken withdrawal behavior and poor client interoperability.

Expected Behavior

Withdraw-to should support both time-based and ledger-based deposits and return correct results for all active vault entries.

Tasks

- [ ] Review `withdraw_to` implementation in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend withdrawal handling to support ledger-based deposits.
- [ ] Add tests covering withdrawal of ledger deposit entries.

---
No test for Emergency withdrawal path when a ledger-based deposit exists
- Priority: High
- Difficulty: Intermediate
- Labels: "testing", "admin"

Description

The emergency withdrawal path in `contracts/time-lock-vault/src/contract.rs` does not support ledger-based deposits. As a result, some deposits cannot be recovered by the admin emergency flow, leaving funds stuck. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`. This undermines recovery guarantees and increases operational risk.

Expected Behavior

Emergency withdrawal should recover both timestamp-based and ledger-based deposits so admin recovery flows are complete.

Tasks

- [ ] Review emergency withdrawal logic in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend support to ledger-based deposits.
- [ ] Add recovery tests for ledger deposits.

---
No test for Vault query ledger-deposit visibility
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "api"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No test for Time remaining query with ledger-based deposits
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "api"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No test for Deposit ID enumeration including ledger-based deposit IDs
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "storage"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
No test for Vault batch query covering ledger deposit paths
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "api"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No test for Depositor removal with mixed deposit types
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "storage"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
No test for Ledger deposit path transfer failure rollback
- Priority: Medium
- Difficulty: Advanced
- Labels: "testing", "error-path"

Description

The ledger deposit path in `contracts/time-lock-vault/src/contract.rs` is currently missing validation rules that other deposit methods enforce. This allows paused contracts to accept deposits and permits ledger-based lock durations that violate the configured min/max bounds. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`, and `README.md`. This gap increases the risk of inconsistent deposit state and incorrect behavior for clients that rely on ledger and timestamp lock types.

Expected Behavior

The ledger deposit path should enforce pause state and correct lock-duration validation, matching the timestamp deposit flow and exposing ledger deposits consistently through public queries.

Tasks

- [ ] Inspect `deposit_by_ledger` in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Add pause-state validation for ledger deposit entry points.
- [ ] Add minimum and maximum lock duration checks for ledger sequence deposits.
- [ ] Update any API or storage helpers that expose ledger deposits.
- [ ] Add regression tests for paused state and invalid ledger locks.

---
No test for pause/unpause semantics across both deposit methods
- Priority: Medium
- Difficulty: Intermediate
- Labels: "testing", "admin"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No test for Cancel deposit behavior on ledger deposits
- Priority: Low
- Difficulty: Intermediate
- Labels: "testing", "contract"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No test verifying get_constants with custom initialization values
- Priority: Low
- Difficulty: Beginner
- Labels: "testing", "constants"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No test verifying deposit_for and deposit share the same amount constraints
- Priority: Low
- Difficulty: Beginner
- Labels: "testing", "consistency"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No test verifying Withdraw-to path event payload values
- Priority: Low
- Difficulty: Intermediate
- Labels: "testing", "events"

Description

The `withdraw_to` implementation in `contracts/time-lock-vault/src/contract.rs` currently only supports time-based deposits and ignores ledger-based vault entries. That means valid ledger deposits cannot be withdrawn through this public API, creating a functional gap. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`. This inconsistency risks broken withdrawal behavior and poor client interoperability.

Expected Behavior

Withdraw-to should support both time-based and ledger-based deposits and return correct results for all active vault entries.

Tasks

- [ ] Review `withdraw_to` implementation in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend withdrawal handling to support ledger-based deposits.
- [ ] Add tests covering withdrawal of ledger deposit entries.

---
No test for get_depositor_count after mixed deposit removals
- Priority: Low
- Difficulty: Beginner
- Labels: "testing", "storage"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
No integration test validating README example flows
- Priority: Medium
- Difficulty: Advanced
- Labels: "testing", "integration"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No fuzz or boundary tests for minimum and maximum deposit amounts across paths
- Priority: Medium
- Difficulty: Advanced
- Labels: "testing", "fuzzing"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
No stress test for get_depositors pagination size and edge behavior
- Priority: Low
- Difficulty: Advanced
- Labels: "testing", "performance"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Extract shared deposit validation logic into a single helper
- Priority: Medium
- Difficulty: Intermediate
- Labels: "refactor", "contract"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

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

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Introduce reusable require_admin helper to simplify admin checks
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "dx"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

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

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

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

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Remove duplicate depositor storage in VaultEntry and LedgerVaultEntry if possible
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "storage"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Replace test.rs 5-tuple setup with a TestContext struct
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "testing"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
Extract constants like TEST_MINT_AMOUNT from repeated test literals
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "testing"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
Simplify repeated admin authorization pattern in contract.rs
- Priority: Medium
- Difficulty: Intermediate
- Labels: "refactor", "contract"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Consolidate types.rs and errors.rs into a smaller model module for cohesion
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "structure"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Simplify crate exports in lib.rs for a cleaner public interface
- Priority: Low
- Difficulty: Beginner
- Labels: "refactor", "lib"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Investigate the reported contract behavior.
- [ ] Implement the fix in the relevant source files.
- [ ] Add tests or documentation to lock in the behavior.

---
Update Makefile check target to include build verification for parity with CI
- Priority: Medium
- Difficulty: Beginner
- Labels: "refactor", "devops"

Description

The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files. This issue impacts release reliability and developer workflows.

Expected Behavior

CI and tooling should enforce the missing workflow, documentation, or validation checks before merges.

Tasks

- [ ] Update CI or tooling configuration with the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow locally if possible.

---
Add top_up(depositor, amount) to increase a lock without changing unlock time
- Priority: High
- Difficulty: Intermediate
- Labels: "feature", "contract"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add extend_lock(depositor, new_unlock_time) to lengthen existing locks
- Priority: High
- Difficulty: Intermediate
- Labels: "feature", "contract"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add batch_Emergency withdrawal path to match README and support recovery migration
- Priority: High
- Difficulty: Advanced
- Labels: "feature", "admin", "security"

Description

The emergency withdrawal path in `contracts/time-lock-vault/src/contract.rs` does not support ledger-based deposits. As a result, some deposits cannot be recovered by the admin emergency flow, leaving funds stuck. Affected modules: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/storage.rs`. This undermines recovery guarantees and increases operational risk.

Expected Behavior

Emergency withdrawal should recover both timestamp-based and ledger-based deposits so admin recovery flows are complete.

Tasks

- [ ] Review emergency withdrawal logic in `contracts/time-lock-vault/src/contract.rs`.
- [ ] Extend support to ledger-based deposits.
- [ ] Add recovery tests for ledger deposits.

---
Add batch_withdraw to withdraw multiple deposits in one call
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "contract", "scalability"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add deposit_on_behalf for third-party deposit flow
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "contract", "ux"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add admin-configurable token whitelist for accepted token contracts
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "admin", "security"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add get_all_vaults or paginated aggregate query for off-chain indexing
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "api", "scalability"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add get_total_locked(token) aggregate query for TVL and analytics
- Priority: Medium
- Difficulty: Intermediate
- Labels: "feature", "api", "analytics"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add runtime update support for fee_recipient without redeploying
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "admin", "economics"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add admin-managed emergency freeze for specific depositors or tokens
- Priority: Medium
- Difficulty: Advanced
- Labels: "feature", "admin", "security"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add configurable deposit penalty caps or fee rules for Cancel deposit
- Priority: Low
- Difficulty: Advanced
- Labels: "feature", "contract", "economics"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add a vault_status query summarizing contract pause/admin state
- Priority: Low
- Difficulty: Intermediate
- Labels: "feature", "api", "ux"

Description

The contract currently lacks this feature, which would improve usability, scalability, or security for users and administrators. Adding the feature will close a functional gap and align the contract with common vault management expectations. Affected files: `contracts/time-lock-vault/src/contract.rs`, `contracts/time-lock-vault/src/types.rs`, and `README.md` documentation. This issue impacts product capability and UX.

Expected Behavior

The repository should implement the requested feature with a consistent API and storage model, including documentation.

Tasks

- [ ] Design the feature API and storage support.
- [ ] Implement the contract and type changes.
- [ ] Document usage examples in `README.md`.

---
Add cargo audit to CI to catch dependency vulnerabilities
- Priority: High
- Difficulty: Intermediate
- Labels: "devops", "ci", "security"

Description

The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files. This issue impacts release reliability and developer workflows.

Expected Behavior

CI and tooling should enforce the missing workflow, documentation, or validation checks before merges.

Tasks

- [ ] Update CI or tooling configuration with the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow locally if possible.

---
Add a GitHub Release workflow that builds optimized WASM assets
- Priority: High
- Difficulty: Intermediate
- Labels: "devops", "ci", "release"

Description

The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files. This issue impacts release reliability and developer workflows.

Expected Behavior

CI and tooling should enforce the missing workflow, documentation, or validation checks before merges.

Tasks

- [ ] Update CI or tooling configuration with the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow locally if possible.

---
Add cargo test --release --features testutils to CI for optimized build coverage
- Priority: Medium
- Difficulty: Intermediate
- Labels: "devops", "ci", "testing"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
Add shell syntax and usage validation for scripts/deploy_testnet.sh
- Priority: Medium
- Difficulty: Intermediate
- Labels: "devops", "ci", "scripts"

Description

The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files. This issue impacts release reliability and developer workflows.

Expected Behavior

CI and tooling should enforce the missing workflow, documentation, or validation checks before merges.

Tasks

- [ ] Update CI or tooling configuration with the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow locally if possible.

---
Add a Makefile target for toolchain and soroban-cli bootstrap
- Priority: Medium
- Difficulty: Beginner
- Labels: "devops", "dx", "makefile"

Description

The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files. This issue impacts release reliability and developer workflows.

Expected Behavior

CI and tooling should enforce the missing workflow, documentation, or validation checks before merges.

Tasks

- [ ] Update CI or tooling configuration with the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow locally if possible.

---
Add .env.example documenting required environment variables for deployment
- Priority: Medium
- Difficulty: Beginner
- Labels: "devops", "dx", "documentation"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

CI and tooling should enforce the missing workflow, documentation, or validation checks before merges.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Add CI guard for README examples and local integration instructions
- Priority: Medium
- Difficulty: Intermediate
- Labels: "devops", "documentation"

Description

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

CI and tooling should enforce the missing workflow, documentation, or validation checks before merges.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

---
Add WASM size regression checks across PRs
- Priority: Medium
- Difficulty: Intermediate
- Labels: "devops", "ci", "performance"

Description

The current implementation creates unnecessary storage or computation overhead in the contract path. This increases ledger resource usage and may inflate execution costs. Affected files: `contracts/time-lock-vault/src/storage.rs`, `contracts/time-lock-vault/src/contract.rs`, and related test helpers. This issue impacts cost and scalability.

Expected Behavior

The contract should minimize storage scans, avoid unbounded parameters, and keep public APIs performant and scalable.

Tasks

- [ ] Review the referenced storage and query implementation.
- [ ] Reduce unnecessary scans and unbounded parameters.
- [ ] Add performance or boundary tests.

---
Add Dependabot or Renovate config for soroban-sdk and Rust dependency updates
- Priority: Medium
- Difficulty: Beginner
- Labels: "devops", "dependencies"

Description

The repository tooling or CI configuration currently omits an important validation or workflow step. This increases the risk of regressions, deployment failures, or unnoticed dependency issues. Affected files: `.github/workflows/ci.yml`, `Makefile`, `scripts/deploy_testnet.sh`, or repo config files. This issue impacts release reliability and developer workflows.

Expected Behavior

CI and tooling should enforce the missing workflow, documentation, or validation checks before merges.

Tasks

- [ ] Update CI or tooling configuration with the missing validation step.
- [ ] Add documentation or examples for the workflow change.
- [ ] Validate the new CI workflow locally if possible.

---
Add a developer quickstart section for contract iteration and local testing
- Priority: Medium
- Difficulty: Beginner
- Labels: "dx", "testing"

Description

The test suite does not cover this behavior and leaves a regression gap in contract validation. Without a dedicated test, future changes can break this behavior silently. Affected files: `contracts/time-lock-vault/src/test.rs` and related helpers. This issue impacts release confidence and reliability.

Expected Behavior

Add focused tests for the missing behavior so regressions are caught before release.

Tasks

- [ ] Add or extend tests to cover the missing behavior.
- [ ] Validate the new tests with the existing suite.
- [ ] Document the new coverage scenario.

---
Extend issue templates with a Soroban security-contract bug checklist
- Priority: Medium
- Difficulty: Beginner
- Labels: "dx", "github", "security"

Description

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

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

The current implementation contains a gap or inconsistency that should be corrected. Affected files include the contract source, storage helpers, and documentation. This issue impacts contract correctness, observability, or developer experience.

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

Repository documentation is missing required details, examples, or security guidance for the current contract implementation. That reduces developer understanding and increases the risk of incorrect integration, audits, and contributor onboarding. Affected files may include `README.md`, `CHANGELOG.md`, `CONTRIBUTING.md`, `SECURITY.md`, or script documentation.

Expected Behavior

The implementation should be corrected so the contract behaves consistently, safely, and transparently for all affected flows.

Tasks

- [ ] Update the relevant documentation file with the missing content.
- [ ] Add examples or guidance that clarify current contract behavior.
- [ ] Validate documentation changes against the current codebase.

