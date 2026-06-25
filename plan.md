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

## Labels Used

`bug` `feature` `testing` `documentation` `devops` `security` `good-first-issue` `intermediate` `advanced`
