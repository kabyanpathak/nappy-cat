# TUI-2: Bring Pomodoro and Linear into the TUI

**Type:** Task

**Stage:** 7. Integrated TUI

**Priority:** Normal

**Status:** Planned

**Dependencies:** [TUI-1](../tui/TUI-1.md)

## Goal

Expose the proven focus/task functionality through a cohesive terminal workflow.

## Acceptance criteria

- [ ] Provide timer state, countdown, cycle progress, preset selection/editing, and session controls.
- [ ] Users can browse/link/create/update Linear work and resolve supported retry/conflict situations through terminal controls.
- [ ] A persistent compact timer indicator remains visible during task work; transitions preserve edits and active sessions.
- [ ] Reuse the command-stage behavior and records; frontend actions do not create a separate task or timer system.

## Documentation and learning

- [Ratatui documentation](https://ratatui.rs/)
- [Focusd timer screen](https://github.com/BibekBhusal0/focusd/blob/master/src/tui/pages/timer.rs)

## Design question

How can a user manage tasks without losing awareness of an active focus session?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
