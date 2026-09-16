# CORE-1: Establish the development baseline

**Type:** Task

**Stage:** Foundation

**Priority:** High

**Status:** Partially established — workspace exists; baseline fixes and decisions pending

**Dependencies:** None

## Goal

Understand the existing workspace and make it a dependable starting point for your own feature work. Preserve cat-core and the Nappy Cat app identity.

## Acceptance criteria

- [ ] Record the current build/test results, resolve the unfinished scaffold issues needed for a usable baseline, and distinguish existing work from completed features.
- [ ] Choose the first supported desktop platform and describe how additional platforms will be evaluated.
- [ ] Describe the responsibilities of the app and shared library, plus the local-data and background-operation expectations, without designing every future feature.
- [ ] Document the Rust toolchain and repeatable local verification commands. The existing two-package workspace is retained and cat-core remains independent of GUI rendering.

## Observed starting point

On September 16, 2026, workspace metadata passed after the app rename. An offline workspace check reached the existing `cat-core` OAuth scaffold and reported four error-type mismatches. The Rust source was left unchanged; resolving the baseline remains part of your implementation work.

## Documentation and learning

- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)

## Design question

What belongs in a reusable library, and what should remain an application decision?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
