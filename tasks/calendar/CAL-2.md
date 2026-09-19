# CAL-2: Connect planned work to the calendar

**Type:** Task

**Stage:** 5a. Calendar

**Priority:** Normal

**Status:** Planned

**Dependencies:** [CAL-1](../calendar/CAL-1.md), [APP-4](../nappy-cat/APP-4.md)

## Goal

Let people place work on their calendar while keeping planning separate from actual completion.

## Acceptance criteria

- [ ] Schedule a local or linked Linear task as a calendar block with a visible relationship to the task.
- [ ] Rescheduling/deleting a block does not silently delete the task, mutate a remote issue, or record focus completion.
- [ ] Overlapping blocks are visible and planned time remains distinct from completed focus time.
- [ ] The agenda can be inspected and changed through the minimal interface without a GUI; saved relationships survive restart.

## Documentation and learning

- [Chrono](https://docs.rs/chrono/latest/chrono/)
- [iCalendar concepts, RFC 5545](https://www.rfc-editor.org/info/rfc5545/)

## Design question

What should remain when the task behind a planned calendar block is removed?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
