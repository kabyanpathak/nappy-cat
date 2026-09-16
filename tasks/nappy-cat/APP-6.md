# APP-6: Make preferences and connection state understandable

**Type:** Task

**Stage:** Incrementally with each feature

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-1](../nappy-cat/APP-1.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Give people a clear place to control the behavior of features they actually have. Grow this surface with the roadmap.

## Acceptance criteria

- [ ] Available preferences survive restart and have understandable defaults; unavailable future features are not presented as working.
- [ ] Connected accounts, sign-in progress, failures, and disconnect actions are visible independently for each provider as it ships.
- [ ] Tracking, notifications/sounds, motion, and window preferences appear when their features are added and actually affect behavior.
- [ ] Account changes preserve local work and clearly explain any pending or blocked remote actions.

## Documentation and learning

- [egui documentation](https://docs.rs/egui/latest/egui/)
- [keyring documentation — one credential-storage option](https://docs.rs/keyring/latest/keyring/)

## Design question

Which choices need a persistent preference, and which are better as one-time actions?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
