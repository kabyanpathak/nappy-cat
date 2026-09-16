# LINEAR-4: Recover sync after outages and competing edits

**Type:** Task

**Stage:** 2. Tasks with Linear

**Priority:** High

**Status:** Planned

**Dependencies:** [LINEAR-3](../linear/LINEAR-3.md), [CORE-5](../cat-core/CORE-5.md)

## Goal

Make connected tasks reliable under real network and account conditions rather than only on the happy path.

## Acceptance criteria

- [ ] Pending work survives restart and exposes pending, synced, failed, and conflict states.
- [ ] Manual refresh and modest automatic refresh while connected keep work current without requiring a public webhook server. Retries are bounded, honor provider limits, and avoid duplicate creates after ambiguous results.
- [ ] Concurrent local/remote changes have an explicit resolution path without silent overwrites or data loss.
- [ ] Disconnect, revocation, account/team changes, and local unlinking cannot misdirect queued updates. Local tasks remain usable throughout.

## Documentation and learning

- [Linear rate limits](https://linear.app/developers/rate-limiting)
- [Testing with Tokio](https://tokio.rs/tokio/topics/testing)

## Design question

When both copies change, what information does the user need to choose the correct result?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
