# CORE-6: Manage credentials and account lifecycle

**Type:** Task

**Stage:** Before provider connections

**Priority:** High

**Status:** Planned

**Dependencies:** [CORE-1](../cat-core/CORE-1.md)

## Goal

Establish secure, understandable credential handling shared in principle across providers while keeping their accounts independent.

## Acceptance criteria

- [ ] Credentials are kept out of source control, ordinary local data, diagnostics, and exported troubleshooting material; OS credential storage is evaluated on supported platforms.
- [ ] Expiry, refresh failure, revocation, and unavailable credential storage result in actionable account state without blocking local tools.
- [ ] Disconnect clears the relevant local credentials and stops provider requests; reconnect/account changes cannot send pending work to a different account.
- [ ] The distributed app does not rely on an embedded client secret remaining confidential. Provider-specific requirements are recorded.

## Documentation and learning

- [keyring documentation — one credential-storage option](https://docs.rs/keyring/latest/keyring/)
- [oauth2 5.0.0 documentation](https://docs.rs/oauth2/5.0.0/oauth2/)

## Design question

Which account owns a queued action, and what should happen if that account is disconnected?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
