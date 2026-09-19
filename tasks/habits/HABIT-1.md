# HABIT-1: Define recurring habits on the calendar

**Type:** Task

**Stage:** 5b. Habits

**Priority:** Normal

**Status:** Planned

**Dependencies:** [CAL-2](../calendar/CAL-2.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Build habits directly on the calendar already delivered, with a deliberately small recurrence model.

## Acceptance criteria

- [ ] Create, edit, pause, and archive habits with a stated daily/weekly schedule and start date.
- [ ] Scheduled occurrences appear in the same calendar agenda with a clear habit identity; repeated refresh/restart does not duplicate them.
- [ ] Define schedule-edit, timezone, daylight-saving, and missed-occurrence behavior without rewriting past records unexpectedly.
- [ ] Habit schedules work through commands and persist offline; analytics, streak dashboards, and rewards are deferred to tracking.

## Documentation and learning

- [Chrono](https://docs.rs/chrono/latest/chrono/)
- [iCalendar concepts, RFC 5545](https://www.rfc-editor.org/info/rfc5545/)

## Design question

When a habit schedule changes, which past and future occurrences should change?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
