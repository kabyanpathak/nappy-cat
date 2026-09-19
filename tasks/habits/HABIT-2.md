# HABIT-2: Record habit completion in the calendar

**Type:** Task

**Stage:** 5b. Habits

**Priority:** Normal

**Status:** Planned

**Dependencies:** [HABIT-1](../habits/HABIT-1.md)

## Goal

Make the calendar the place to inspect and record habit occurrences, without requiring the later analytics system.

## Acceptance criteria

- [ ] Mark a specific occurrence complete, undo completion, or skip it; pending and missed outcomes have defined meanings.
- [ ] Repeated actions cannot double-count completion, and restart preserves both schedule and recorded outcomes.
- [ ] Calendar inspection distinguishes planned habits from completed ones; task completion and focus sessions do not silently complete habits.
- [ ] Basic occurrence records are available for later tracking; no daily dashboard, streak calculation, or GUI is required.

## Documentation and learning

- [Rust Book](https://doc.rust-lang.org/book/)
- [Chrono](https://docs.rs/chrono/latest/chrono/)

## Design question

How do you identify the same occurrence after a restart or schedule change?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
