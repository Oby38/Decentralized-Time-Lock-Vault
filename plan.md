# Wave Program Contribution Plan — Decentralized Time-Lock Vault

## Overview

This document describes how the Decentralized Time-Lock Vault participates in the Wave Program. Maintainers post scoped, well-defined issues each sprint cycle that contributors can pick up independently.

---

## Work Categories

### 1. Bug Fixes
Issues where contract behavior deviates from spec. Examples: edge cases in timestamp arithmetic, TTL bump not firing under certain conditions, error codes returning the wrong variant, or token transfer failures not rolling back state correctly. Each bug issue includes reproduction steps, expected vs actual behavior, and affected function names.

### 2. New Features
Scoped additions to the contract surface. Examples: multi-deposit support per address, partial withdrawals, deposit top-up, time extension on existing locks, whitelisted token lists, or a fee mechanism. Each feature issue includes a design note, the proposed function signature, and acceptance criteria.

### 3. Testing
Expanding the test suite beyond current coverage. Examples: fuzz tests for boundary amounts, property-based tests for time arithmetic, integration tests against a local Soroban network, auth-bypass attempt tests, and gas/fee benchmarks. Each test issue specifies which function or scenario to target and what assertions to add.

### 4. Documentation
Improving developer and user-facing docs. Examples: inline NatSpec-style comments, a tutorial walkthrough, architecture decision records (ADRs), a changelog, CLI usage examples, or translation of docs to other languages. Each doc issue includes the target audience and the specific gap to fill.

### 5. DevOps & Tooling
CI/CD improvements, deployment automation, and developer experience. Examples: adding a Testnet smoke-test job to CI, a contract upgrade migration script, a Makefile target for local Soroban network setup, or a release tagging workflow.

### 6. Security & Auditing
Code review tasks scoped to specific modules. Examples: reviewing auth ordering in all entry points, verifying CEI pattern compliance, checking for integer edge cases, or writing a threat model document.

---

## Sprint Structure

| Phase | Activity |
|---|---|
| Issue posting | Maintainer opens scoped issues with labels, descriptions, and difficulty tags |
| Pickup window | Contributors comment to claim an issue (first-come) |
| Sprint cycle | 1–2 weeks per issue depending on complexity |
| Review | PR reviewed by maintainer, feedback given within 48 hours |
| Merge | Merged to `main` after CI passes and approval |

---

## Sprint Cadence

Sprints run on a **two-week cycle** aligned to UTC Mondays.

| Milestone | When |
|---|---|
| Sprint kick-off | Monday 09:00 UTC — maintainer publishes new issues and labels them `sprint-current` |
| Claim deadline | Wednesday 23:59 UTC — unclaimed issues are re-labeled `sprint-backlog` and deferred |
| Mid-sprint check | Thursday 09:00 UTC — contributors post a one-line status comment on their issue |
| PR deadline | Following Sunday 23:59 UTC — PRs opened after this time roll into the next sprint |
| Sprint close | Monday 08:59 UTC — maintainer merges approved PRs, closes resolved issues, and opens the next sprint |

**Frequency:** A new sprint starts every second Monday. The current sprint number and active issue list are pinned in the repository's Discussions tab at the start of each cycle.

**Complexity sizing:**

| Label | Expected effort | Sprint slots |
|---|---|---|
| `good-first-issue` | ≤ 4 hours | 1 slot (closes in the same sprint) |
| `intermediate` | 1–2 days | 1 slot |
| `advanced` | 3–5 days | may span 2 sprints; maintainer notes this on the issue |

Contributors working on an `advanced` issue must post a progress update by the mid-sprint check on each Thursday during the issue's lifecycle.

---

## Review Process

All code and documentation contributions go through the following review pipeline before merging.

### 1. Automated checks (required to pass before human review begins)

CI runs automatically on every PR and must be green:

| Check | Tool | Failure blocks merge? |
|---|---|---|
| Formatting | `cargo fmt --all -- --check` | Yes |
| Linting | `cargo clippy -- -D warnings` | Yes |
| Tests | `cargo test --features testutils` | Yes |
| Security audit | `cargo audit` | Yes |
| License/deny | `cargo deny check` | Yes |
| WASM size | `make check-wasm-size` | Yes |
| PR title | Conventional Commits regex | Yes |

### 2. Human review (required after CI passes)

- **Reviewer:** At least one maintainer listed in `.github/CODEOWNERS` must approve.
- **SLA:** First review round within **48 hours** of the PR being marked `Ready for Review` (not Draft).
- **Feedback rounds:** Reviewers leave inline comments. Contributors address feedback and re-request review. Aim to resolve all threads within **24 hours** per round.
- **Approval threshold:** 1 approving review from a code-owner is required; 2 approvals are required for changes to `contract.rs`, `storage.rs`, or any security-sensitive path.

### 3. Review checklist (maintainers verify before approving)

- [ ] PR title follows Conventional Commits format
- [ ] `make check` passes locally (confirmed by CI)
- [ ] New behaviour is covered by tests; coverage does not regress
- [ ] `CHANGELOG.md` updated under `[Unreleased]`
- [ ] README or other docs updated if the public API changed
- [ ] No secrets, test snapshots, or build artefacts committed
- [ ] CEI pattern preserved in any mutating function changes
- [ ] `require_auth()` remains the first call in every mutating entry point

### 4. Merge

- Maintainer performs a **squash-and-merge** to keep `main` history linear.
- The squash commit message uses the PR title (Conventional Commits format) as the subject.
- The PR branch is deleted automatically after merge.

---

## Branch Policies

### Protected branches

| Branch | Protection rules |
|---|---|
| `main` | Direct pushes disabled; requires PR + CI pass + 1 code-owner approval; no force-push; deletion disabled |
| `develop` | Direct pushes disabled; requires PR + CI pass; no force-push |

### Branch naming

All contributor branches must follow the convention defined in `CONTRIBUTING.md`:

| Type | Pattern | Example |
|---|---|---|
| Feature | `feat/<short-description>` | `feat/multi-token-support` |
| Bug fix | `fix/<short-description>` | `fix/unlock-time-overflow` |
| Docs | `docs/<short-description>` | `docs/contributing-guide` |
| Chore / tooling | `chore/<short-description>` | `chore/update-dependencies` |
| Test | `test/<short-description>` | `test/fuzz-deposit-amount` |
| Security | `security/<short-description>` | `security/audit-auth-ordering` |
| Sprint issue | `<type>/issue-<number>-<description>` | `docs/issue-286-plan-sprint-cadence` |

Branches that do not follow this naming convention will have CI labeling skipped and may be asked to rename before review begins.

### Stale branch policy

- Branches with no commits for **30 days** after their PR is merged or closed are deleted by the maintainer.
- Branches with no activity for **14 days** and no open PR receive a `stale` comment; if no response within 7 days the branch is removed.

### Merge strategy

- All merges into `main` and `develop` use **squash merge** to maintain a linear history.
- Merge commits and rebase merges are disabled on both protected branches.
- Force-pushing to `main` or `develop` is prohibited at all times, including for maintainers.

---

## Labels Used

`bug` `feature` `testing` `documentation` `devops` `security` `good-first-issue` `intermediate` `advanced`
