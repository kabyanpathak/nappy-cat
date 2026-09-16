# APP-9: Add companion and compact popup modes

**Type:** Task

**Stage:** 5. First extras

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-2](../nappy-cat/APP-2.md), [APP-4](../nappy-cat/APP-4.md), [APP-7](../nappy-cat/APP-7.md), [APP-8](../nappy-cat/APP-8.md)

## Goal

Let people keep a small cat beside their work and reach useful controls through a compact popup.

## Acceptance criteria

- [ ] The companion is movable, always-on-top is optional, and the popup gives access to timer, tasks, and the accepted music capability.
- [ ] Switching or hiding views preserves sessions, task edits, provider state, and time accounting.
- [ ] Window placement can be recovered after display changes; unsupported special window behavior has a usable normal-window fallback.
- [ ] Timer, Linear tasks, the first music outcome, and initial progression form a coherent presentable app. Full screen and all cosmetic extras are not required.

## Scope and sequencing

This is the first-extras completion point before Spotify, after the accepted YouTube Music outcome.

## Documentation and learning

- [eframe documentation](https://docs.rs/eframe/latest/eframe/)
- [egui documentation](https://docs.rs/egui/latest/egui/)

## Design question

How can the cat stay easy to reach without obstructing the user’s actual work?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
