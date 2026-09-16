# APP-3: Deliver the first usable Pomodoro interface

**Type:** Task

**Stage:** 1. Pomodoro

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-2](../nappy-cat/APP-2.md), [CORE-4](../cat-core/CORE-4.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Make the timer useful in everyday work before adding connected tasks or game features.

## Acceptance criteria

- [ ] The native interface shows current session state, remaining time, duration preferences, and start/pause/resume/reset controls.
- [ ] Completion and interruption are clear; optional notifications or sounds respect user preferences and denied permissions.
- [ ] The timer works offline and signed out, persists the relevant settings/session outcome, and handles restart and sleep consistently.
- [ ] The main workflow is usable with a keyboard. Tasks, rewards, music, and animated cats are not required to finish this ticket.

## Documentation and learning

- [egui documentation](https://docs.rs/egui/latest/egui/)
- [Rust time concepts](https://doc.rust-lang.org/std/time/index.html)

## Design question

What should the user see immediately after returning from a break or a suspended device?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
