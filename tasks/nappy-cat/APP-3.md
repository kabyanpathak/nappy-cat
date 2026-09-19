# APP-3: Expose Pomodoro commands and a minimal status display

**Type:** Task

**Stage:** 1. Pomodoro

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-2](../nappy-cat/APP-2.md), [CORE-4](../cat-core/CORE-4.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Make Pomodoro usable before the complete TUI, with simple commands and at most a small terminal status line or bar.

## Acceptance criteria

- [ ] Users can select/customize presets and start, pause, resume, and reset sessions using the minimal interface.
- [ ] A compact status display communicates current phase, remaining time, running/paused state, and cycle progress without a full-screen dashboard.
- [ ] Completion and interruption are clear; optional notifications respect preferences and denied permissions.
- [ ] The workflow works offline and signed out, retains relevant settings/session outcomes, and has consistent sleep/restart behavior. Full history views and daily tracking come later.

## Documentation and learning

- [clap documentation](https://docs.rs/clap/latest/clap/)
- [Rust time concepts](https://doc.rust-lang.org/std/time/index.html)
- [Focusd source reference](https://github.com/BibekBhusal0/focusd)

## Design question

Which few pieces of status are enough to use the timer while doing other work?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
