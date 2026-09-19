# MUSIC-4: Add the accepted Spotify experience

**Type:** Task

**Stage:** 6. Spotify

**Priority:** High

**Status:** Conditional — awaiting MUSIC-3 and owner scope acceptance

**Dependencies:** [MUSIC-3](../music/MUSIC-3.md), [CORE-6](../cat-core/CORE-6.md)

## Goal

Add Spotify within the shared music responsibility while preserving the accepted first-provider experience.

## Acceptance criteria

- [ ] The accepted Spotify capability works with command-accessible account state and controls matching the provider’s actual support.
- [ ] Connection, refresh, disconnect, invalid accounts/devices, and revoked access have clear behavior independent of Google and Linear.
- [ ] Failures and rate limits stay bounded without blocking focus or task work.
- [ ] The app labels unsupported actions accurately and records resource costs for the supported playback/control path.

## Documentation and learning

- [Spotify PKCE flow](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow)
- [Spotify rate limits](https://developer.spotify.com/documentation/web-api/concepts/rate-limits)
- [Spotify quota modes](https://developer.spotify.com/documentation/web-api/concepts/quota-modes)

## Design question

What can be shared between music providers without pretending their capabilities are identical?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
