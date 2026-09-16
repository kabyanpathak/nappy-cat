# APP-7: Show daily progress and earned rewards

**Type:** Task

**Stage:** 5. First extras

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-3](../nappy-cat/APP-3.md), [PROGRESS-2](../progress/PROGRESS-2.md), [PROGRESS-3](../progress/PROGRESS-3.md)

## Goal

Make today’s use and progress understandable without confusing app-running time with focused work.

## Acceptance criteria

- [ ] App-use time and focus time are labelled and displayed separately with meaningful empty and partial-day states.
- [ ] Users can inspect the planned daily summary, see earned cosmetics, and select available rewards.
- [ ] Progress updates survive view changes and restart without duplicate awards or misleading totals.
- [ ] The view remains useful offline and respects tracking preferences.

## Scope and sequencing

Start after the YouTube Music stage outcome is accepted; see the stage gates in ../README.md.

## Documentation and learning

- [egui documentation](https://docs.rs/egui/latest/egui/)
- [Serde documentation](https://serde.rs/)

## Design question

What can a daily number honestly tell the user, and what would overstate their focus?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
