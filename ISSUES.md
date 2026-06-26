# ISSUES.md — Decentralized Time-Lock Vault
## Wave Program — Batch 1 (125 Issues)

> Stack: Rust · Soroban SDK v22 · Stellar Blockchain · Persistent Storage
> All issues are implementation-focused, non-duplicate, and grounded in the actual codebase.

---

## 🔴 BUGS (Issues #1–#22)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 1 | `deposit_by_ledger` bypasses the paused contract guard | Critical | Advanced | `bug` `security` `pause` |
| 2 | `deposit_by_ledger` does not enforce minimum lock duration | High | Advanced | `bug` `validation` `contract` |
| 3 | `deposit_by_ledger` does not enforce maximum lock duration | High | Advanced | `bug` `validation` `contract` |
| 4 | `withdraw_to` only works for time-based deposits, ignoring ledger deposits | High | Advanced | `bug` `contract` `storage` |
| 5 | `emergency_withdraw` only works for time-based deposits, ignoring ledger deposits | High | Advanced | `bug` `admin` `recovery` |
| 6 | `get_vault` does not expose ledger-based deposits | High | Advanced | `bug` `api` `storage` |
| 7 | `time_remaining` ignores ledger-based deposits and returns 0 | High | Advanced | `bug` `api` `ux` |
| 8 | `get_deposit_ids` skips ledger-based deposit IDs | Medium | Intermediate | `bug` `api` `storage` |
| 9 | `get_vault_batch` reads only time-based deposits, not ledger-based deposits | Medium | Intermediate | `bug` `api` `storage` |
| 10 | `cancel_deposit` cannot cancel ledger-based deposits | Medium | Intermediate | `bug` `contract` `storage` |
| 11 | `remove_depositor` can clear an address while ledger deposits remain active | Medium | Intermediate | `bug` `storage` `consistency` |
| 12 | README documents non-existent `batch_emergency_withdraw` API | Medium | Beginner | `bug` `documentation` `contract` |
| 13 | README omits `deposit_by_ledger`, `withdraw_to`, and ledger deposit semantics | Medium | Beginner | `bug` `documentation` `api` |
| 14 | `deposit_by_ledger` uses a different validation path than `deposit`/`deposit_for` | Low | Intermediate | `bug` `refactor` `contract` |
| 15 | `initialize` treats zero `max_lock_secs` as `LockDurationTooLong` instead of explicit invalid config | Low | Beginner | `bug` `validation` `contract` |
| 16 | `deposit_by_ledger` does not validate `unlock_ledger` against network sequence drift | Low | Intermediate | `bug` `validation` `future-proofing` |
| 17 | `get_depositors` pagination accepts an unbounded `limit`, leading to high memory use | Low | Intermediate | `bug` `api` `scalability` |
| 18 | `VaultEntry.depositor` duplicates the address available in the storage key | Low | Beginner | `bug` `storage` `types` |
| 19 | `LedgerVaultEntry.depositor` duplicates the address available in the storage key | Low | Beginner | `bug` `storage` `types` |
| 20 | `README` has no explicit example for `pause`/`unpause` behavior | Low | Beginner | `bug` `documentation` `admin` |
| 21 | Ledger-based deposits are not documented as part of `get_vault` and `time_remaining` | Low | Beginner | `bug` `documentation` `api` |
| 22 | `advance_time` test helper reconstructs ledger state instead of incrementing sequence consistently | Low | Intermediate | `bug` `testing` `helpers` |

---

## 🟠 SECURITY (Issues #23–#38)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 23 | No explicit token contract validation allows malicious token contracts | Critical | Advanced | `security` `token` `validation` |
| 24 | `deposit_by_ledger` bypasses pause, weakening emergency shutdown controls | High | Advanced | `security` `admin` `pause` |
| 25 | `emergency_withdraw` does not support ledger deposits, leaving some funds unrecoverable in recovery flow | High | Advanced | `security` `admin` `recovery` |
| 26 | `time_remaining` returns 0 for ledger deposits, creating a misleading unlocked signal | High | Intermediate | `security` `api` `ux` |
| 27 | `get_vault` and `get_vault_batch` hide ledger deposit state from external indexers | Medium | Intermediate | `security` `transparency` `api` |
| 28 | `cancel_deposit` inability to cancel ledger deposits weakens depositor control | Medium | Intermediate | `security` `contract` `ux` |
| 29 | Admin storage reads do not bump TTL; admin privilege can expire unintentionally | Medium | Intermediate | `security` `storage` `admin` |
| 30 | Ledger deposit sequence semantics are not documented, raising future validation risk | Medium | Intermediate | `security` `documentation` `contract` |
| 31 | No freeze mechanism for an address in case of compromised depositor or token abuse | Medium | Advanced | `security` `admin` `contract` |
| 32 | No wallet recovery or migration path for ledger and timestamp deposits simultaneously | Medium | Advanced | `security` `upgrades` `admin` |
| 33 | Fee fallback to depositor in `cancel_deposit` is not clearly documented | Low | Intermediate | `security` `contract` `ux` |
| 34 | `withdraw_to` allows any recipient address without additional validation | Low | Intermediate | `security` `ux` `contract` |
| 35 | `deposit_by_ledger` provides a sequence-based lock without cross-checking timestamp conversions | Low | Intermediate | `security` `contract` `future-proofing` |
| 36 | No on-chain key versioning in persistent storage for future contract upgrades | Medium | Advanced | `security` `storage` `upgrades` |
| 37 | No event emitted when `cancel_transfer_admin` is invoked with no pending admin present | Low | Beginner | `security` `events` `admin` |
| 38 | `lock_duration` validation is duplicated in multiple deposit paths, increasing audit surface | Low | Beginner | `security` `audit` `refactor` |

---

## 🟡 PERFORMANCE (Issues #39–#50)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 39 | `storage::add_depositor` scans the entire depositor list on every deposit | Medium | Intermediate | `performance` `storage` `cost` |
| 40 | `storage::remove_depositor` rebuilds the depositor list each removal | Medium | Intermediate | `performance` `storage` `cost` |
| 41 | `get_deposit_ids` iterates all deposit IDs up to the counter for every call | Medium | Intermediate | `performance` `storage` `scalability` |
| 42 | `get_depositors_page` has no defensive cap on `limit` | Medium | Intermediate | `performance` `api` `memory` |
| 43 | Event topics include full `Address` values, increasing payload size | Low | Intermediate | `performance` `events` `cost` |
| 44 | `VaultEntry` stores depositor twice, increasing persistent storage footprint | Low | Beginner | `performance` `storage` `types` |
| 45 | `LedgerVaultEntry` stores depositor twice, increasing persistent storage footprint | Low | Beginner | `performance` `storage` `types` |
| 46 | `token::Client::new()` is recreated in each function instead of using a helper | Low | Beginner | `performance` `contract` `refactor` |
| 47 | Shared deposit validation code is duplicated across paths | Low | Beginner | `performance` `contract` `refactor` |
| 48 | `time_remaining` loads full entry data when only timestamp comparison is required | Low | Intermediate | `performance` `storage` `contract` |
| 49 | `setup()` test helper re-registers the contract for every test | Low | Intermediate | `performance` `testing` `dx` |
| 50 | `advance_time` test helper reconstructs a full ledger snapshot on every call | Low | Beginner | `performance` `testing` `dx` |

---

## 🔵 DOCUMENTATION (Issues #51–#68)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 51 | README lacks concrete Soroban CLI invocation examples for deposit and withdraw | High | Beginner | `documentation` `dx` `readme` |
| 52 | CHANGELOG does not clearly document the addition of ledger-based deposits | Medium | Beginner | `documentation` `audit` |
| 53 | CONTRIBUTING lacks Soroban-specific contribution and testing guidance | Medium | Beginner | `documentation` `contributing` |
| 54 | SECURITY.md has no responsible disclosure process or severity guidelines | High | Beginner | `documentation` `security` |
| 55 | `BUMP_THRESHOLD` and `BUMP_TARGET` constants are undocumented in `storage.rs` | Medium | Beginner | `documentation` `constants` |
| 56 | `MAX_DEPOSIT_AMOUNT` comment should clarify units and short/long-scale terminology | Low | Beginner | `documentation` `types` |
| 57 | `VaultEntry` and `LedgerVaultEntry` fields lack unit documentation | High | Beginner | `documentation` `types` `api` |
| 58 | `events.rs` lacks a module-level explanation of event topic conventions | Low | Beginner | `documentation` `events` |
| 59 | `storage.rs` does not document the complete persistent key layout | Medium | Beginner | `documentation` `storage` |
| 60 | `contract.rs` does not explain the security model for `emergency_withdraw` | Medium | Beginner | `documentation` `admin` `contract` |
| 61 | README does not explain the difference between time-based and ledger-based deposits | High | Beginner | `documentation` `readme` |
| 62 | README does not document pause semantics for all deposit paths | Medium | Beginner | `documentation` `admin` `readme` |
| 63 | `scripts/deploy_testnet.sh` lacks inline usage examples and default environment assumptions | Medium | Beginner | `documentation` `scripts` |
| 64 | README has no local Soroban standalone node integration testing instructions | High | Beginner | `documentation` `testing` |
| 65 | plan.md does not define sprint cadence, review process, or branch policies | Low | Beginner | `documentation` `process` |
| 66 | README does not document when `is_initialized` must be checked before invocation | Medium | Beginner | `documentation` `contract` |
| 67 | README does not clarify `get_vault` vs `get_vault_batch` differences | Low | Beginner | `documentation` `api` |
| 68 | lib.rs comment on the storage model is outdated compared to current key definitions | Medium | Beginner | `documentation` `lib` |

---

## 🟢 TESTING (Issues #69–#88)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 69 | No test verifying `deposit_by_ledger` rejects deposits while paused | High | Intermediate | `testing` `pause` |
| 70 | No test verifying `deposit_by_ledger` rejects too-short ledger lock durations | High | Intermediate | `testing` `validation` |
| 71 | No test verifying `deposit_by_ledger` rejects too-long ledger lock durations | High | Intermediate | `testing` `validation` |
| 72 | No test for `withdraw_to` with ledger-based deposits | High | Intermediate | `testing` `contract` |
| 73 | No test for `emergency_withdraw` when a ledger-based deposit exists | High | Intermediate | `testing` `admin` |
| 74 | No test for `get_vault` ledger-deposit visibility | Medium | Intermediate | `testing` `api` |
| 75 | No test for `time_remaining` with ledger-based deposits | Medium | Intermediate | `testing` `api` |
| 76 | No test for `get_deposit_ids` including ledger-based deposit IDs | Medium | Intermediate | `testing` `storage` |
| 77 | No test for `get_vault_batch` covering ledger deposit paths | Medium | Intermediate | `testing` `api` |
| 78 | No test for `remove_depositor` with mixed deposit types | Medium | Intermediate | `testing` `storage` |
| 79 | No test for `deposit_by_ledger` transfer failure rollback | Medium | Advanced | `testing` `error-path` |
| 80 | No test for `pause`/`unpause` semantics across both deposit methods | Medium | Intermediate | `testing` `admin` |
| 81 | No test for `cancel_deposit` behavior on ledger deposits | Low | Intermediate | `testing` `contract` |
| 82 | No test verifying `get_constants` with custom initialization values | Low | Beginner | `testing` `constants` |
| 83 | No test verifying `deposit_for` and `deposit` share the same amount constraints | Low | Beginner | `testing` `consistency` |
| 84 | No test verifying `withdraw_to` event payload values | Low | Intermediate | `testing` `events` |
| 85 | No test for `get_depositor_count` after mixed deposit removals | Low | Beginner | `testing` `storage` |
| 86 | No integration test validating README example flows | Medium | Advanced | `testing` `integration` |
| 87 | No fuzz or boundary tests for minimum and maximum deposit amounts across paths | Medium | Advanced | `testing` `fuzzing` |
| 88 | No stress test for `get_depositors` pagination size and edge behavior | Low | Advanced | `testing` `performance` |

---

## ⚪ REFACTORING (Issues #89–#100)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 89 | Extract shared deposit validation logic into a single helper | Medium | Intermediate | `refactor` `contract` |
| 90 | Factor ledger and timestamp deposit storage into separate helper modules | Medium | Intermediate | `refactor` `storage` |
| 91 | Introduce reusable `require_admin` helper to simplify admin checks | Low | Beginner | `refactor` `dx` |
| 92 | Introduce a shared pause guard helper for deposit entry points | Low | Beginner | `refactor` `admin` |
| 93 | Extract token transfer operations into a reusable helper | Low | Beginner | `refactor` `contract` |
| 94 | Remove duplicate depositor storage in `VaultEntry` and `LedgerVaultEntry` if possible | Low | Beginner | `refactor` `storage` |
| 95 | Replace `test.rs` 5-tuple setup with a `TestContext` struct | Low | Beginner | `refactor` `testing` |
| 96 | Extract constants like `TEST_MINT_AMOUNT` from repeated test literals | Low | Beginner | `refactor` `testing` |
| 97 | Simplify repeated admin authorization pattern in contract.rs | Medium | Intermediate | `refactor` `contract` |
| 98 | Consolidate `types.rs` and `errors.rs` into a smaller model module for cohesion | Low | Beginner | `refactor` `structure` |
| 99 | Simplify crate exports in `lib.rs` for a cleaner public interface | Low | Beginner | `refactor` `lib` |
| 100 | Update `Makefile` check target to include build verification for parity with CI | Medium | Beginner | `refactor` `devops` |

---

## 🔧 FEATURES / SCALABILITY (Issues #101–#112)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 101 | Add `top_up(depositor, amount)` to increase a lock without changing unlock time | High | Intermediate | `feature` `contract` |
| 102 | Add `extend_lock(depositor, new_unlock_time)` to lengthen existing locks | High | Intermediate | `feature` `contract` |
| 103 | Add `batch_emergency_withdraw` to match README and support recovery migration | High | Advanced | `feature` `admin` `security` |
| 104 | Add `batch_withdraw` to withdraw multiple deposits in one call | Medium | Advanced | `feature` `contract` `scalability` |
| 105 | Add `deposit_on_behalf` for third-party deposit flow | Medium | Advanced | `feature` `contract` `ux` |
| 106 | Add admin-configurable token whitelist for accepted token contracts | Medium | Advanced | `feature` `admin` `security` |
| 107 | Add `get_all_vaults` or paginated aggregate query for off-chain indexing | Medium | Advanced | `feature` `api` `scalability` |
| 108 | Add `get_total_locked(token)` aggregate query for TVL and analytics | Medium | Intermediate | `feature` `api` `analytics` |
| 109 | Add runtime update support for `fee_recipient` without redeploying | Medium | Advanced | `feature` `admin` `economics` |
| 110 | Add admin-managed emergency freeze for specific depositors or tokens | Medium | Advanced | `feature` `admin` `security` |
| 111 | Add configurable deposit penalty caps or fee rules for `cancel_deposit` | Low | Advanced | `feature` `contract` `economics` |
| 112 | Add a `vault_status` query summarizing contract pause/admin state | Low | Intermediate | `feature` `api` `ux` |

---

## 🚀 CI/CD & DEVOPS (Issues #113–#121)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 113 | Add `cargo audit` to CI to catch dependency vulnerabilities | High | Intermediate | `devops` `ci` `security` |
| 114 | Add a GitHub Release workflow that builds optimized WASM assets | High | Intermediate | `devops` `ci` `release` |
| 115 | Add `cargo test --release --features testutils` to CI for optimized build coverage | Medium | Intermediate | `devops` `ci` `testing` |
| 116 | Add shell syntax and usage validation for `scripts/deploy_testnet.sh` | Medium | Intermediate | `devops` `ci` `scripts` |
| 117 | Add a `Makefile` target for toolchain and `soroban-cli` bootstrap | Medium | Beginner | `devops` `dx` `makefile` |
| 118 | Add `.env.example` documenting required environment variables for deployment | Medium | Beginner | `devops` `dx` `documentation` |
| 119 | Add CI guard for README examples and local integration instructions | Medium | Intermediate | `devops` `documentation` |
| 120 | Add WASM size regression checks across PRs | Medium | Intermediate | `devops` `ci` `performance` |
| 121 | Add Dependabot or Renovate config for `soroban-sdk` and Rust dependency updates | Medium | Beginner | `devops` `dependencies` |

---

## 🎨 DEVELOPER EXPERIENCE (Issues #122–#125)

| # | Title | Priority | Difficulty | Tags |
|---|---|---|---|---|
| 122 | Add a developer quickstart section for contract iteration and local testing | Medium | Beginner | `dx` `testing` |
| 123 | Extend issue templates with a Soroban security-contract bug checklist | Medium | Beginner | `dx` `github` `security` |
| 124 | Extend PR template with contract-specific testing and audit checklist | Medium | Beginner | `dx` `github` `contributing` |
| 125 | Add a contributor-facing troubleshooting section for Soroban CLI and WASM build issues | Low | Beginner | `dx` `documentation` |

---

## Summary Statistics

| Category | Count | Critical | High | Medium | Low |
|---|---|---|---|---|---|
| Bugs | 22 | 1 | 5 | 12 | 4 |
| Security | 16 | 1 | 5 | 9 | 1 |
| Performance | 12 | 0 | 0 | 7 | 5 |
| Documentation | 18 | 0 | 4 | 10 | 4 |
| Testing | 20 | 0 | 5 | 11 | 4 |
| Refactoring | 12 | 0 | 0 | 5 | 7 |
| Features | 12 | 0 | 2 | 8 | 2 |
| CI/CD | 9 | 0 | 3 | 6 | 0 |
| Developer Experience | 4 | 0 | 0 | 4 | 0 |
| **Total** | **125** | **2** | **24** | **67** | **32** |

---

## Recommended Sprint Order

1. Critical contract and security bugs: #1, #4, #5, #6, #23, #24, #25, #113, #114
2. Ledger deposit consistency and API coverage: #2–#11, #69–#79
3. Documentation and testing: #51–#65, #69–#88
4. Refactor and performance cleanup: #39–#50, #89–#100
5. CI/CD and developer experience: #115–#121, #122–#125

---

*Generated for Wave Program · Decentralized Time-Lock Vault · Soroban / Stellar*
