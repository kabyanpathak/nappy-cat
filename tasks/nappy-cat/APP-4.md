# APP-4: Use Linear tasks through the minimal interface

**Type:** Task

**Stage:** 2. Tasks with Linear

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-3](../nappy-cat/APP-3.md), [CORE-3](../cat-core/CORE-3.md), [LINEAR-4](../linear/LINEAR-4.md)

## Goal

Bring the connected task workflow together so users can manage selected Linear work through commands or simple prompts before the full TUI or GUI exists.

## Acceptance criteria

- [ ] Users can connect/disconnect Linear, choose a destination, browse/link issues, and explicitly create or update linked work.
- [ ] Local and remote state, pending work, errors, and conflicts are understandable; retry and conflict decisions are available through commands or simple prompts.
- [ ] Cached/local tasks remain usable during outages and the Pomodoro timer remains responsive during sync.
- [ ] An end-to-end create/update and reconnect workflow succeeds without publishing unrelated personal tasks or using Google auth.

## Documentation and learning

- [clap documentation](https://docs.rs/clap/latest/clap/)
- [Linear GraphQL API](https://linear.app/developers/graphql)

## Design question

How can the interface make it obvious which action changes Linear and which changes only local work?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
