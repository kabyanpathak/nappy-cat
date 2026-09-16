# LINEAR-1: Connect a Linear account

**Type:** Task

**Stage:** 2. Tasks with Linear

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-3](../nappy-cat/APP-3.md), [CORE-6](../cat-core/CORE-6.md)

## Goal

Provide an independent Linear login suitable for a distributed native desktop app.

## Acceptance criteria

- [ ] Confirm current official PKCE, redirects, scopes, account eligibility, and refresh requirements, recording the chosen supported flow.
- [ ] A user can connect and disconnect through browser authentication with clear success, denial, cancellation, and timeout outcomes.
- [ ] Credentials follow CORE-6 and Google login is not required. Callback resources are released when authentication ends.
- [ ] Expired/revoked credentials and account changes produce clear state while local tasks and Pomodoro keep working.

## Documentation and learning

- [Linear OAuth and PKCE](https://linear.app/developers/oauth-2-0-authentication)
- [oauth2 5.0.0 documentation](https://docs.rs/oauth2/5.0.0/oauth2/)

## Design question

Which pieces of provider authorization are shared concepts, and which belong only to Linear?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
