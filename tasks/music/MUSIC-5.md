# MUSIC-5: Make music providers coexist with productivity

**Type:** Task

**Stage:** 6. Spotify

**Priority:** High

**Status:** Planned

**Dependencies:** [MUSIC-4](../music/MUSIC-4.md), [MUSIC-2](../music/MUSIC-2.md)

## Goal

Finish provider selection and cross-feature behavior before integrating and packaging the full TUI.

## Acceptance criteria

- [ ] Users can select between implemented providers; unavailable or explicitly deferred providers are clearly labelled.
- [ ] Switching, reconnecting, changing views, and quitting do not create conflicting playback sessions or stale account state.
- [ ] Focus feedback, music, task sync, calendar, and habit work remain usable together, including during outages.
- [ ] Provider-specific limits and measured resource use are documented; unsupported integration work remains visibly blocked rather than counted as complete.

## Scope and sequencing

MUSIC-2 must also be complete if YouTube Music is implemented. A deferred provider requires an explicit owner decision recorded in the stage plan.

## Documentation and learning

- [Tokio tutorial](https://tokio.rs/tokio/tutorial)
- [Spotify rate limits](https://developer.spotify.com/documentation/web-api/concepts/rate-limits)

## Design question

What should happen to one provider’s session when the user selects another?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
