# TUI-3: Integrate music, calendar, habits, and settings

**Type:** Task

**Stage:** 7. Integrated TUI

**Priority:** Normal

**Status:** Planned

**Dependencies:** [TUI-2](../tui/TUI-2.md), [CAL-2](../calendar/CAL-2.md), [HABIT-2](../habits/HABIT-2.md), [MUSIC-5](../music/MUSIC-5.md), [APP-6](../nappy-cat/APP-6.md)

## Goal

Bring all functionality delivered so far into the terminal application.

## Acceptance criteria

- [ ] Accepted music capabilities and provider selection are usable alongside focus/tasks, with accurate account and playback/control state.
- [ ] Users can browse/edit the calendar, schedule task blocks, and manage/complete habit occurrences in that calendar.
- [ ] Preferences and account actions share the existing storage and feature behavior; unsupported provider actions are clearly labelled.
- [ ] Keyboard-only end-to-end workflows remain usable with empty data, offline state, provider failures, and terminal resizing. Later tracking views are not required yet.

## Documentation and learning

- [Ratatui documentation](https://ratatui.rs/)
- [Chrono](https://docs.rs/chrono/latest/chrono/)

## Design question

Which information should stay visible when moving between calendar, tasks, and music?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
