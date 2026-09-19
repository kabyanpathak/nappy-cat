# PROGRESS-2: Produce trustworthy daily history

**Type:** Task

**Stage:** 8. Tracking and GUI

**Priority:** Normal

**Status:** Planned

**Dependencies:** [PROGRESS-1](../progress/PROGRESS-1.md), [HABIT-2](../habits/HABIT-2.md)

## Goal

Turn recorded sessions, habit outcomes, and app use into local daily summaries that remain credible across date and clock changes.

## Acceptance criteria

- [ ] Totals are assigned to the user’s local date, including sessions crossing midnight.
- [ ] Timezone changes, daylight-saving changes, clock adjustment, and restart have explicit behavior without negative or doubled totals.
- [ ] Daily records survive restart and remain distinguishable from unfinished or interrupted sessions.
- [ ] Habit summaries distinguish scheduled, completed, skipped, and missed occurrences using HABIT-2 records; corrections do not duplicate totals.
- [ ] History has a considered retention/resource policy and can be inspected without depending on an online account.

## Documentation and learning

- [Rust time concepts](https://doc.rust-lang.org/std/time/index.html)
- [Chrono date and time concepts](https://docs.rs/chrono/latest/chrono/)
- [Serde documentation](https://serde.rs/)

## Design question

How should a day’s history behave when the user travels to a different timezone?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
