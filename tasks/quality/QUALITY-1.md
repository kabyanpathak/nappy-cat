# QUALITY-1: Establish repeatable development checks

**Type:** Task

**Stage:** Throughout, starting with foundation

**Priority:** High

**Status:** Planned

**Dependencies:** [CORE-1](../cat-core/CORE-1.md)

## Goal

Make it easy to verify your own changes and learn from failures as each feature is implemented.

## Acceptance criteria

- [ ] Document reproducible formatting, lint, build, and relevant test commands for the declared toolchain.
- [ ] Automated checks cover meaningful domain behavior and regressions as those behaviors arrive, without requiring personal tokens or live accounts.
- [ ] Remote verification, if used, matches supported build environments and reports actionable failures.
- [ ] Known scaffold failures are visible until resolved rather than hidden by disabling checks.

## Documentation and learning

- [Testing in Rust](https://doc.rust-lang.org/book/ch11-00-testing.html)
- [GitHub Actions for Rust](https://docs.github.com/en/actions/tutorials/build-and-test-code/rust)
- [Testing with Tokio](https://tokio.rs/tokio/topics/testing)

## Design question

Which failures should be caught before opening the app, and which need a real desktop session?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
