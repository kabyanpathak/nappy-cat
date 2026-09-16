# LINEAR-2: Browse and link Linear work

**Type:** Task

**Stage:** 2. Tasks with Linear

**Priority:** High

**Status:** Planned

**Dependencies:** [LINEAR-1](../linear/LINEAR-1.md), [CORE-3](../cat-core/CORE-3.md)

## Goal

Let users select a workspace/team and bring relevant Linear issues into their local workflow.

## Acceptance criteria

- [ ] The connected account can discover permitted destinations and relevant issues without assuming a single page of results.
- [ ] A user can explicitly link an issue to a local task with stable association and understandable status mapping.
- [ ] Refresh updates visible/cached work and handles inaccessible, moved, or removed issues without losing local content.
- [ ] Loading failures and rate limits are bounded and visible; this ticket does not publish new personal tasks remotely.

## Documentation and learning

- [Linear GraphQL API](https://linear.app/developers/graphql)
- [Linear pagination](https://linear.app/developers/pagination)

## Design question

What should remain useful locally when a linked issue is no longer accessible?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
