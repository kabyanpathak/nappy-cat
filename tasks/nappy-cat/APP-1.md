# APP-1: Create a responsive native shell

**Type:** Task

**Stage:** Foundation

**Priority:** High

**Status:** Planned

**Dependencies:** [CORE-1](../cat-core/CORE-1.md)

## Goal

Launch a small native Rust application that can host the first Pomodoro workflow and remain responsive as services are added.

## Acceptance criteria

- [ ] The app starts signed out without opening a browser and shows a usable native window.
- [ ] Slow storage or network work does not freeze input or rendering; failures can be surfaced without replacing the whole app.
- [ ] Background work and UI updates stay bounded, and an idle window does not require continuous high-frequency repainting.
- [ ] Application state can outlive a particular view so later modes can share the same work.

## Documentation and learning

- [eframe documentation](https://docs.rs/eframe/latest/eframe/)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [Optional reading: eframe_template](https://github.com/emilk/eframe_template)

## Design question

How will the interface learn that work has finished without continually doing unnecessary work?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
