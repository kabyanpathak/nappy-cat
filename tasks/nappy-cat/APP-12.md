# APP-12: Complete accessibility and interaction polish

**Type:** Task

**Stage:** 9. GUI polish and release

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-10](../nappy-cat/APP-10.md), [APP-6](../nappy-cat/APP-6.md)

## Goal

Make the complete app comfortable to navigate across its supported platforms and display settings. Basic accessibility still belongs in each earlier feature.

## Acceptance criteria

- [ ] Keyboard navigation, visible focus, readable scaling/contrast, labels, and non-color status cues work across all main flows.
- [ ] Reduced motion and notification preferences are consistent; focus transitions and popup/full-screen exits are predictable.
- [ ] Platform accessibility behavior is evaluated, limitations are recorded, and essential controls have usable alternatives.
- [ ] Empty, loading, offline, and failure states explain the next action without discarding work.

## Documentation and learning

- [egui documentation](https://docs.rs/egui/latest/egui/)
- [eframe documentation](https://docs.rs/eframe/latest/eframe/)

## Design question

Can a user complete the main flows without relying on motion, color, or precise mouse placement?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
