# CAL-1: Build local calendar behavior

**Type:** Task

**Stage:** 5a. Calendar

**Priority:** Normal

**Status:** Planned

**Dependencies:** [MUSIC-2](../music/MUSIC-2.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Add a useful local calendar after the accepted YouTube Music stage, accessible through the minimal interface before full TUI integration.

## Acceptance criteria

- [ ] Create, inspect, edit, and delete local events with stable identity, date/time, duration, and optional notes.
- [ ] Support timed and all-day events with explicit timezone, daylight-saving, and cross-midnight behavior; invalid ranges are explained.
- [ ] A date/range agenda is usable through commands, survives restart, and works offline and signed out.
- [ ] Define a bounded first recurrence scope and clear exclusions. External Google/Apple calendar sync, invitations, and shared calendars are not required.

## Documentation and learning

- [Chrono](https://docs.rs/chrono/latest/chrono/)
- [iCalendar concepts, RFC 5545](https://www.rfc-editor.org/info/rfc5545/)
- [clap documentation](https://docs.rs/clap/latest/clap/)

## Design question

What makes an all-day event different from a timed event when a user changes timezone?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
