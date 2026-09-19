# APP-1: Establish a minimal command-line application

**Type:** Task

**Stage:** Foundation

**Priority:** Normal

**Status:** Planned

**Dependencies:** [CORE-1](../cat-core/CORE-1.md)

## Goal

Provide enough developer-facing commands to exercise working features before building a full-screen TUI or GUI.

## Acceptance criteria

- [ ] The app starts signed out and offers discoverable help and clear results for implemented commands.
- [ ] Storage and network delays have understandable failure/cancellation behavior and do not block independent timer work.
- [ ] Command handling uses shared feature behavior; domain rules do not depend on a terminal renderer or GUI.
- [ ] Output and exit behavior are documented. No full-screen navigation, desktop window, or future feature placeholder is required.

## Documentation and learning

- [clap documentation](https://docs.rs/clap/latest/clap/)
- [Rust Book](https://doc.rust-lang.org/book/)

## Design question

What is the smallest interface that lets you use and verify a feature end to end?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
