# CORE-1: Existing Workspace and Local Foundation

**Component:** Workspace and `crates/cat-core`

**Priority:** Foundation

**Status:** Workspace exists; new foundation work planned

**Dependencies:** None

## Goal

Keep the existing two-crate layout and establish the local foundation for Nappy Cat. The product name changes in documentation; package names and paths stay unchanged. This task replaces the old workspace-only scope.

## Milestones

- [x] Existing virtual workspace contains `crates/cat-core` and `apps/open-cat`, with a path dependency and shared lockfile.
- [ ] Define module boundaries for tasks, Pomodoro, progress, Google auth, and Linear auth/sync using the master specification.
- [ ] Choose a compact local store and OS application-data location; document atomic writes, schema versions, migrations, and recovery.
- [ ] Define stable identifiers and persisted records for tasks, sessions, daily totals, unlocks, settings, provider mappings, and pending sync.
- [ ] Define single-instance ownership or equivalent coordination so multiple processes cannot duplicate timers, tracking, or writes.
- [ ] Establish baseline build/test status and record existing scaffold failures separately from new work.

## Acceptance

Local productivity data has a documented lifecycle independent of Google or Linear accounts. Domain logic has no GUI dependency. Do not create speculative crates or claim workspace dependency centralization already exists; current dependencies are declared per package.

Future extraction candidates are documented in the [master specification](../../CAT_ECOSYSTEM_MASTER_SPEC.md). Implementation of storage and domain behavior belongs to the dependent tasks.
