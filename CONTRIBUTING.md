# Contributing

Thank you for your interest in contributing to the Decentralized Time-Lock Vault!

## Branch Naming

| Type | Pattern | Example |
|---|---|---|
| New feature | `feat/<short-description>` | `feat/multi-token-support` |
| Bug fix | `fix/<short-description>` | `fix/unlock-time-overflow` |
| Chore / tooling | `chore/<short-description>` | `chore/update-dependencies` |
| Docs | `docs/<short-description>` | `docs/contributing-guide` |

## Commit Messages

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```
<type>(<scope>): <short summary>

[optional body]
```

Common types: `feat`, `fix`, `docs`, `chore`, `test`, `refactor`, `ci`.

Examples:
```
feat(contract): add multi-depositor support
fix(storage): correct TTL bump on emergency withdraw
docs: add CONTRIBUTING.md
```

## Local Development

```bash
# Full check: fmt + clippy + tests
make check

# Build optimized WASM and report size
make build && make optimize
```

## Performance Conventions

### Cache `env.ledger().timestamp()` in a local variable

Every call to `env.ledger().timestamp()` is a host-function invocation with a non-trivial cost in the Soroban execution environment. Always cache the result in a `let now` binding at the top of any function that reads the ledger timestamp more than once:

```rust
// good
pub fn some_fn(env: Env, depositor: Address) -> Result<(), VaultError> {
    let now = env.ledger().timestamp();
    if now < entry.unlock_time {
        return Err(VaultError::FundsStillLocked);
    }
    // ... use `now` again later without re-invoking the host
}

// bad â€” calls the host twice for the same value
pub fn some_fn(env: Env, depositor: Address) -> Result<(), VaultError> {
    if env.ledger().timestamp() < entry.unlock_time {
        return Err(VaultError::FundsStillLocked);
    }
    let elapsed = env.ledger().timestamp() - start;
}
```

This convention applies to any repeated host accessor (`env.ledger().sequence()`, `env.current_contract_address()`, etc.) â€” read once, store locally, reuse the binding.

## Before Opening a PR

- [ ] `make check` passes locally
- [ ] New tests added for any new behaviour
- [ ] README updated if the public API changed

- [ ] CHANGELOG.md updated under [Unreleased] with a summary of the change

## Submitting a PR

1. Push your branch and open a PR against `main`.
2. Fill in the PR description with a summary of changes and what was tested.
3. Link any related issue with `Closes #<issue-number>`.

## Test Snapshots

Running `cargo test` may generate a `contracts/time-lock-vault/test_snapshots/` directory containing XDR snapshots of contract state produced by the Soroban test environment. These are transient build artefacts, not committed regression fixtures, and are listed in `.gitignore`. Do not commit them.

## Soroban-Specific Guidance

### How the Test Environment Works

Soroban unit tests use the `soroban-sdk` testutils feature to spin up an in-process simulated ledger. There is **no external node or network** required. The simulated environment provides a `MockHost` that emulates ledger storage, events, and auth.

Key points:
- Tests must run natively (without `--target wasm32-unknown-unknown`) so that testutils can compile.
- The `testutils` feature is in `[dev-dependencies]` only and is never compiled into the production WASM.
- Never run `cargo test --target wasm32-unknown-unknown` — it will fail because testutils are not available in the WASM target.

### Running Tests

```bash
# Run the full test suite
cargo test --features testutils

# Run a single test with stdout
cargo test test_deposit_success --features testutils -- --nocapture

# Run tests in release mode (catches optimisation-related edge cases)
cargo test --release --features testutils

# Run all tests with output
cargo test --features testutils -- --nocapture
```

### Writing New Tests

All tests live in `contracts/time-lock-vault/src/test.rs`. Follow these conventions:

1. Use `soroban_sdk::testutils::Ledger` to set the ledger timestamp before calling time-sensitive functions:

```rust
env.ledger().with_mut(|l| {
    l.timestamp = 1_700_000_000;
});
```

2. Advance ledger time to simulate the passage of time:

```rust
env.ledger().with_mut(|l| {
    l.timestamp += 3_600; // advance 1 hour
});
```

3. Assert on typed errors, not on panic messages:

```rust
let err = contract.withdraw(&depositor, &0).unwrap_err();
assert_eq!(err, VaultError::FundsStillLocked.into());
```

4. Every new contract function or behaviour change must be accompanied by at least one positive-path and one negative-path test.

### Soroban SDK Version

The workspace is pinned to `soroban-sdk = "22"` in `Cargo.toml`. Do not change the SDK version in a feature PR — version bumps require a dedicated chore PR with a full audit of breaking-change behaviour.
