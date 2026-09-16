# CORE-3: Provide durable local task behavior

**Type:** Task

**Stage:** 2. Tasks with Linear

**Priority:** High

**Status:** Planned

**Dependencies:** [CORE-5](../cat-core/CORE-5.md), [APP-3](../nappy-cat/APP-3.md)

## Goal

Give the Linear workflow a small local task foundation that continues to work without a provider account. This is the shared behavior, not a separate polished to-do application.

## Acceptance criteria

- [ ] Tasks can be created, read, edited, completed, reopened, and deleted with stable identity.
- [ ] Tasks survive restart and failed saves do not appear as successful changes.
- [ ] Local tasks and provider links remain distinguishable; deleting a local task does not silently delete a remote issue.
- [ ] The behavior is usable independently of GUI rendering and remains available offline.

## Documentation and learning

- [The Rust Programming Language](https://doc.rust-lang.org/book/)
- [Serde documentation](https://serde.rs/)
- [Testing in Rust](https://doc.rust-lang.org/book/ch11-00-testing.html)

## Design question

How will you distinguish a local task from its optional relationship to a provider?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
