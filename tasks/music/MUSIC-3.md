# MUSIC-3: Resolve Spotify feasibility for the presentable app

**Type:** Research / decision

**Stage:** 6. Spotify

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-9](../nappy-cat/APP-9.md)

## Goal

Check the supported Spotify experience once the first companion features are presentable, before expanding the integration.

## Acceptance criteria

- [ ] Record current official PKCE, playback/control, subscription, endpoint, and distribution/development-account restrictions.
- [ ] Distinguish control of an existing Spotify device from direct native audio playback; assess the browser-based Web Playback SDK against the product constraints.
- [ ] Define a useful compatible scope and the consequences of unsupported accounts or devices.
- [ ] Record the owner’s accepted scope or a blocker. Do not silently adopt a browser player, external-player dependency, or new service architecture.

## Documentation and learning

- [Spotify PKCE flow](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow)
- [Spotify Web Playback SDK](https://developer.spotify.com/documentation/web-playback-sdk)
- [Spotify quota modes](https://developer.spotify.com/documentation/web-api/concepts/quota-modes)

## Design question

Does the supported API provide a player, or only control a player that must already exist?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
