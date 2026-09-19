# APP-13: Add a GUI over the shared productivity features

**Type:** Task

**Stage:** 8. Tracking and GUI

**Priority:** Normal

**Status:** Planned

**Dependencies:** [TUI-4](../tui/TUI-4.md)

## Goal

Introduce the broader-audience native GUI alongside tracking, keeping the terminal interface supported.

## Acceptance criteria

- [ ] A responsive native shell exposes existing Pomodoro, Linear, music, calendar, habits, and preferences through discoverable controls.
- [ ] GUI and TUI use the same records and domain behavior; neither frontend is required to run the other.
- [ ] Opening/switching frontends preserves work. Define simultaneous-use behavior and prevent duplicate timer, sync, and tracking ownership.
- [ ] Use a restrained panel-based visual direction with optional terminal-inspired typography/borders. Basic keyboard access and clear offline/error states are present from the start; later companion modes remain separate.

## Documentation and learning

- [egui documentation](https://docs.rs/egui/latest/egui/)
- [eframe documentation](https://docs.rs/eframe/latest/eframe/)

## Design question

How can both interfaces feel suited to their users while producing the same outcomes?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
