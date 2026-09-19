# APP-2: Define runtime ownership and shutdown

**Type:** Task

**Stage:** Foundation

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-1](../nappy-cat/APP-1.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Make session and background-work ownership predictable across commands and later frontends.

## Acceptance criteria

- [ ] Document what happens when a command ends, the terminal closes, a frontend detaches, or the user explicitly quits.
- [ ] Repeated launches and later simultaneous frontends cannot create duplicate timers, writes, sync operations, or accounting.
- [ ] Shutdown preserves recoverable work and cancels unnecessary workers/listeners; sleep and restart do not invent active time.
- [ ] Choose the simplest sufficient ownership model. A resident process or local IPC, if justified, has bounded lifetime and an explicit exit; no web server is required.

## Documentation and learning

- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)

## Design question

Who owns an active session when no interface is visible?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
