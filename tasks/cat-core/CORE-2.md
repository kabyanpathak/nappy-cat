# CORE-2: Complete optional Google sign-in

**Type:** Task

**Stage:** Alongside relevant features

**Priority:** High

**Status:** In progress — scaffold exists; working flow unverified

**Dependencies:** [CORE-1](../cat-core/CORE-1.md), [CORE-6](../cat-core/CORE-6.md)

## Goal

Finish the existing Google authentication work as an optional desktop capability. It must remain independent of Pomodoro, local tasks, and Linear.

## Acceptance criteria

- [ ] A user can authenticate through the system browser and return to the native app using the current supported installed-app flow.
- [ ] PKCE and state validation protect the login. Denial, malformed callbacks, cancellation, timeout, and overlapping attempts have clear outcomes.
- [ ] Any temporary callback listener closes on every terminal path; no persistent auth server or embedded browser is required.
- [ ] Minimal scopes match a documented feature, credentials follow CORE-6, and sign-out preserves local work. Drive access and music playback are not implied.

## Documentation and learning

- [Google OAuth for installed apps](https://developers.google.com/identity/protocols/oauth2/native-app)
- [oauth2 5.0.0 documentation](https://docs.rs/oauth2/5.0.0/oauth2/)

## Design question

What does successful Google authentication actually authorize in this product?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
