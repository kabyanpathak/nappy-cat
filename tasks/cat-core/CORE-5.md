# CORE-5: Persist local work and recover safely

**Type:** Task

**Stage:** Foundation

**Priority:** High

**Status:** Planned

**Dependencies:** [CORE-1](../cat-core/CORE-1.md)

## Goal

Keep user work and preferences durable without a cloud account. Begin with the timer and settings, extending storage only as later features arrive.

## Acceptance criteria

- [ ] Choose and justify a compact local storage approach and an appropriate per-user application-data location.
- [ ] Saved settings and session records survive restart; interrupted or failed writes do not silently erase the last valid state.
- [ ] Data-format changes and unreadable data have a documented migration/recovery path that preserves recoverable user data.
- [ ] Tasks, provider links, queued changes, progress, and unlocks can be added in their own stages. Credentials are handled separately by CORE-6.

## Documentation and learning

- [Serde documentation](https://serde.rs/)
- [rusqlite documentation — one storage option](https://docs.rs/rusqlite/latest/rusqlite/)

## Design question

What evidence would convince you that an interrupted save cannot destroy someone’s work?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
