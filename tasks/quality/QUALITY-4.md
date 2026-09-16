# QUALITY-4: Make diagnostics useful without exposing private data

**Type:** Task

**Stage:** Throughout provider and storage work

**Priority:** Normal

**Status:** Planned

**Dependencies:** [CORE-5](../cat-core/CORE-5.md), [CORE-6](../cat-core/CORE-6.md)

## Goal

Help diagnose real failures while keeping account secrets and personal task content out of routine troubleshooting output.

## Acceptance criteria

- [ ] Errors identify the failed operation and useful next action while preserving local work.
- [ ] Diagnostics avoid credentials and unnecessary task/music content, remain bounded, and do not transmit data automatically.
- [ ] Any troubleshooting report is inspectable by the user and has clear collection/removal behavior.
- [ ] Logging/storage failure cannot bring down normal local functionality; representative auth, sync, and save failures can be investigated.

## Documentation and learning

- [tracing documentation](https://docs.rs/tracing/latest/tracing/)
- [keyring documentation — one credential-storage option](https://docs.rs/keyring/latest/keyring/)

## Design question

What is the least information needed to distinguish two failures without recording someone’s private work?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
