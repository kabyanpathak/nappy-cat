# QUALITY-3: Meet the runtime footprint target

**Type:** Task

**Stage:** Throughout, final pass before release

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-11](../nappy-cat/APP-11.md), [MUSIC-5](../music/MUSIC-5.md)

## Goal

Measure and improve the app’s actual resource use against the less-than-100-MB finalized runtime-memory goal.

## Acceptance criteria

- [ ] Define a repeatable measurement method, supported platforms, representative dataset, and treatment of helper processes and transient peaks.
- [ ] Record release-build idle, focus, sync, playback, and TUI-only, GUI-only, permitted combined use, and each window-mode memory usage, including steady state and peaks.
- [ ] Measure idle CPU, startup behavior, package size, and background/hidden operation separately; build-size flags alone do not demonstrate runtime memory.
- [ ] Resolve over-budget cases or obtain an explicit product-target revision before claiming compliance. Repeat relevant measurements after expensive feature changes.

## Scope and sequencing

The inherited <100 MB interpretation is runtime memory. Baseline measurement starts with APP-1; dependencies apply to final sign-off.

## Documentation and learning

- [Cargo build profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [egui documentation](https://docs.rs/egui/latest/egui/)

## Design question

Where is the memory actually going when the app is idle versus playing music?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
