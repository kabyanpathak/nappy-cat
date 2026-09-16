# CORE-7: Evolve the crate boundaries as features grow

**Type:** Task

**Stage:** As needed after working features

**Priority:** As needed

**Status:** Planned

**Dependencies:** [CORE-1](../cat-core/CORE-1.md)

## Goal

Use the README’s future structure goal to improve maintainability when real features justify separation. Keep ownership of the design decisions.

## Acceptance criteria

- [ ] Identify a concrete coupling, testing, or dependency problem before extracting a crate.
- [ ] Preserve cat-core, keep the app focused on composition, and keep domain behavior testable without GUI or live provider accounts.
- [ ] Initially group window modes together, music providers together, and tracking with rewards; document any later reason to separate them.
- [ ] After a chosen extraction, workspace builds, behavior, task links, and architecture documentation remain consistent. No empty speculative crates are required.

## Documentation and learning

- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Design question

What problem does an additional crate solve that a module would not?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
