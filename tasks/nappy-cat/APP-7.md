# APP-7: Show daily progress and earned rewards

**Type:** Task

**Stage:** 8. Tracking and GUI

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-13](../nappy-cat/APP-13.md), [PROGRESS-2](../progress/PROGRESS-2.md), [PROGRESS-3](../progress/PROGRESS-3.md)

## Goal

Make today’s use and progress understandable without confusing app-running time with focused work.

## Acceptance criteria

- [ ] App-use time and focus time are labelled and displayed separately with meaningful empty and partial-day states.
- [ ] Users can inspect the planned daily summary, see earned cosmetics, and select available rewards.
- [ ] Progress updates survive view changes and restart without duplicate awards or misleading totals.
- [ ] Equivalent history/progress inspection is available in TUI and GUI as tracking ships; both remain useful offline and respect tracking preferences.

## Scope and sequencing

Start after TUI-4; see the stage gates in the task index.

## Documentation and learning

- [egui documentation](https://docs.rs/egui/latest/egui/)
- [Serde documentation](https://serde.rs/)

## Design question

What can a daily number honestly tell the user, and what would overstate their focus?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
