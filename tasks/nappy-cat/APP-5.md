# APP-5: Decide whether a separate simple to-do view is useful

**Type:** Research / decision

**Stage:** 3. Optional simple to-do

**Priority:** Optional

**Status:** Planned

**Dependencies:** [APP-4](../nappy-cat/APP-4.md)

## Goal

Add a very small local-only task experience only if the existing controls do not already meet that need. Keep music next in the roadmap.

## Acceptance criteria

- [ ] Record whether the existing local task controls are sufficient; a documented decision to skip completes this optional ticket.
- [ ] If needed, provide local create/edit/complete/reopen/delete with offline persistence, using the existing task behavior.
- [ ] If adapting another Rust project, review its license, dependencies, maintenance, and footprint before importing anything.
- [ ] The result stays within the native app and does not become a second task system or a large prerequisite for music.

## Documentation and learning

- [egui documentation](https://docs.rs/egui/latest/egui/)
- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)

## Design question

What useful interaction is missing from the existing task view?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
