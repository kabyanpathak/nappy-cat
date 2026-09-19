# TUI-1: Build the full terminal shell after core workflows

**Type:** Task

**Stage:** 7. Integrated TUI

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-4](../nappy-cat/APP-4.md), [MUSIC-5](../music/MUSIC-5.md)

## Goal

Begin full-screen terminal implementation after Pomodoro and Linear work end to end and the Spotify stage outcome is accepted.

## Acceptance criteria

- [ ] Provide keyboard navigation, discoverable help, clear focus, and responsive resizing with usable small-terminal behavior.
- [ ] Terminal initialization, normal exit, cancellation, and failures restore a usable terminal.
- [ ] Shared application state remains independent of screens; switching pages cannot restart a session or discard work.
- [ ] Input stays responsive during storage/provider operations, with bounded refresh and meaningful loading/offline/error states.

## Documentation and learning

- [Ratatui documentation](https://ratatui.rs/)
- [Focusd TUI source](https://github.com/BibekBhusal0/focusd/tree/master/src/tui)

## Design question

Which state belongs to a screen, and which must survive every screen change?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
