# CORE-4: Build reliable Pomodoro sessions

**Type:** Task

**Stage:** 1. Pomodoro

**Priority:** High

**Status:** Planned

**Dependencies:** [CORE-1](../cat-core/CORE-1.md)

## Goal

Create a focus/break timer whose behavior remains correct independently of rendering speed. Keep this first useful feature small.

## Acceptance criteria

- [ ] Focus, short-break, and long-break sessions support start, pause, resume, reset, and completion with an unambiguous next state.
- [ ] Named presets define durations and the number of focus intervals before a long break. Users can customize and retain them; invalid values have clear outcomes.
- [ ] Define whether preset edits or selection affect an active interval or only future intervals, and verify that behavior.
- [ ] Completed, interrupted, and abandoned sessions are distinguishable; completion is not recorded twice.
- [ ] Pauses, sleep/wake, clock changes, and restart have documented behavior that does not silently grant focus time.
- [ ] Behavior can be verified without waiting through real-length focus sessions; settings and records can be retained through CORE-5 when integrated.

## Documentation and learning

- [Focusd: timer and configuration reference](https://github.com/BibekBhusal0/focusd)
- [Rust time concepts](https://doc.rust-lang.org/std/time/index.html)
- [Testing with Tokio](https://tokio.rs/tokio/topics/testing)

## Design question

Which time should count as focused work when a device sleeps or the application closes?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
