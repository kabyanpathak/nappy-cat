# QUALITY-2: Verify recovery across the complete app

**Type:** Task

**Stage:** Throughout, final pass after remaining extras

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-12](../nappy-cat/APP-12.md), [MUSIC-5](../music/MUSIC-5.md), [LINEAR-4](../linear/LINEAR-4.md), [PROGRESS-3](../progress/PROGRESS-3.md)

## Goal

Exercise the failures that cross feature boundaries and are easy to miss in isolated tests.

## Acceptance criteria

- [ ] Test interruption during saves, pending sync, authentication, mode changes, and active sessions without losing committed local work.
- [ ] Cover sleep/wake, restart, duplicate instances, midnight/timezone changes, expired credentials, conflicts, and provider outages.
- [ ] Tasks, focus, progression, and accepted music capabilities remain consistent when used together.
- [ ] Record reproducible findings, fix release-blocking failures, and retain regression coverage where useful.

## Scope and sequencing

Run relevant scenarios as features land; the listed dependencies are for closing the final integrated verification ticket.

## Documentation and learning

- [Testing in Rust](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [Testing with Tokio](https://tokio.rs/tokio/topics/testing)

## Design question

Which individually correct features could interfere with each other when they share a process?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
