# LINEAR-3: Create and update Linear issues deliberately

**Type:** Task

**Stage:** 2. Tasks with Linear

**Priority:** High

**Status:** Planned

**Dependencies:** [LINEAR-2](../linear/LINEAR-2.md)

## Goal

Allow selected local work to create or update a Linear issue with predictable ownership and field mapping.

## Acceptance criteria

- [ ] Remote creation requires an explicit user choice of task and destination; unrelated local tasks remain private.
- [ ] Supported title, description, and completion/status changes round-trip with clear mappings and visible result.
- [ ] Remote identity survives restart, and ambiguous create outcomes do not cause blind retries that produce duplicate issues.
- [ ] Local deletion unlinks by default; remote deletion is outside the initial scope. Unsupported fields and permissions are explained.

## Documentation and learning

- [Linear GraphQL API](https://linear.app/developers/graphql)
- [Linear rate limits](https://linear.app/developers/rate-limiting)

## Design question

How will you distinguish a request that failed from a request whose successful response was lost?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
