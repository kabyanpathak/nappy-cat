# PROGRESS-1: Track app-use time separately from focus

**Type:** Task

**Stage:** 8. Tracking and GUI

**Priority:** Normal

**Status:** Planned

**Dependencies:** [TUI-4](../tui/TUI-4.md), [APP-2](../nappy-cat/APP-2.md), [CORE-4](../cat-core/CORE-4.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Measure time Nappy Cat is running while the device is awake, including background use, without monitoring other applications.

## Acceptance criteria

- [ ] App-use time and active focus time have distinct definitions and totals; breaks and paused focus do not count as focus.
- [ ] Sleep, explicit tracking pauses, and closed-app intervals are excluded from app-use time.
- [ ] Restarts, multiple windows/processes, and repeated events do not duplicate counted intervals.
- [ ] Tracking works offline with bounded persistence and respects the user’s preference to pause it.

## Scope and sequencing

Start after TUI-4 delivers the terminal release, following the accepted Spotify stage outcome.

## Documentation and learning

- [Rust time concepts](https://doc.rust-lang.org/std/time/index.html)
- [Testing in Rust](https://doc.rust-lang.org/book/ch11-00-testing.html)

## Design question

What observable evidence separates a running process from an awake, countable usage interval?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
