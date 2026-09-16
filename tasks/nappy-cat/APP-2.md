# APP-2: Define startup, background use, and shutdown

**Type:** Task

**Stage:** Foundation

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-1](../nappy-cat/APP-1.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Make Nappy Cat predictable when it is opened, hidden, reopened, or closed. Establish ownership before multiple windows and tracking arrive.

## Acceptance criteria

- [ ] Closing a window, hiding the app, and quitting have clear behavior; users can recover a hidden app and explicitly exit.
- [ ] Reopening or launching a second instance does not duplicate timers, conflicting saves, tracking, or provider work.
- [ ] Quitting finishes or safely cancels pending work and preserves recoverable local state without leaving workers or listeners running.
- [ ] Sleep/wake and restart restore a coherent state without treating time spent closed as active use.

## Documentation and learning

- [eframe documentation](https://docs.rs/eframe/latest/eframe/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)

## Design question

Who owns an active timer when the visible window disappears?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
