# Security Policy

## Supported Versions

| Version | Supported |
|---------|-----------|
| 0.1.x   | ✅        |

## Reporting a Vulnerability

**Please do NOT open a public GitHub issue for security vulnerabilities.**

- **Email:** security@example.com *(replace with actual contact before going to production)*
- **Response time:** within 72 hours

Please include:
- A clear description of the vulnerability
- Steps to reproduce or a proof-of-concept
- The potential impact (fund loss, unauthorized access, etc.)

We will acknowledge receipt within 72 hours and aim to release a fix within 14 days for critical issues.

## Scope

The following are considered in-scope vulnerabilities:

- Smart contract logic bugs that could result in fund loss or theft
- Unauthorized access to locked funds (auth bypass)
- Storage manipulation vulnerabilities
- Re-entrancy or checks-effects-interactions violations
- Admin privilege escalation

## Out of Scope

- Stellar network-level issues (report to [Stellar](https://www.stellar.org/bug-bounty-program))
- Bugs in `soroban-sdk` itself (report to [Stellar/rs-soroban-sdk](https://github.com/stellar/rs-soroban-sdk))
- Issues requiring physical access to a user's private key
- Social engineering attacks

## Disclosure Policy

We follow coordinated disclosure. Please allow us reasonable time to patch before any public disclosure.

### Disclosure Timeline

| Step | Timeframe |
|------|-----------|
| Acknowledgement of your report | Within 72 hours of receipt |
| Initial triage and severity assessment | Within 5 business days |
| Status update to reporter | Every 7 days until resolved |
| Patch release for Critical/High issues | Within 14 days of confirmation |
| Patch release for Medium/Low issues | Within 60 days of confirmation |
| Public disclosure (coordinated with reporter) | After patch is available, or after 90 days maximum |

If a fix requires redeployment of the contract (because Soroban contracts are immutable), the disclosure timeline begins from when a migration path is communicated to users.

## Severity Guidelines

We use a four-level severity scale aligned with CVSS v3.

### Critical

Direct, exploitable fund loss or theft with no prerequisites.

Examples:
- Logic bug allowing an attacker to withdraw another depositor's funds
- Auth bypass on `emergency_withdraw` sending funds to a non-depositor address
- Re-entrancy enabling double-withdrawal of the same deposit

### High

Significant fund loss or lock-up that requires specific conditions or limited attacker control.

Examples:
- Depositor funds permanently locked due to a storage corruption bug
- Admin privilege escalation enabling unauthorized `emergency_withdraw` calls
- Incorrect penalty calculation causing material fund loss on `cancel_deposit`

### Medium

Contract misbehaviour that does not directly cause fund loss but degrades correctness or availability.

Examples:
- Incorrect event data emitted (wrong amount or deposit_id)
- TTL bump logic failing to extend storage, risking entry expiry before unlock time
- Edge-case overflow in `time_remaining` returning an incorrect value

### Low / Informational

Minor issues or improvements with negligible security impact.

Examples:
- Missing input validation that is harmless in practice
- Documentation inaccuracies about contract behaviour
- Gas/instruction inefficiencies with no exploitable consequence
