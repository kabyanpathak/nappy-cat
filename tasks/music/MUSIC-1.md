# MUSIC-1: Resolve YouTube Music feasibility

**Type:** Research / decision

**Stage:** 4. YouTube Music

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-4](../nappy-cat/APP-4.md)

## Goal

Determine whether the desired YouTube Music experience is achievable within Nappy Cat’s native Rust, browser-only-for-auth, and small-footprint constraints.

## Acceptance criteria

- [ ] Record dated primary sources covering actual music/playback capability, authentication, account requirements, permitted use, and distribution.
- [ ] Distinguish metadata/playlist access, direct in-app audio, and control of another player. Google sign-in or the YouTube Data API alone is not playback support.
- [ ] Assess the documented IFrame/browser and playback-policy restrictions against the product constraints, rather than assuming a Rust client library removes them.
- [ ] Produce an explicit supported scope or a blocker with options for the owner. Research may complete with a negative result; MUSIC-2 stays blocked until a supported path and any constraint changes are accepted.

## Scope and sequencing

APP-5 is optional and may be skipped. This music stage is before tracking, rewards, and animated cats; completing research alone does not mean the player shipped.

## Documentation and learning

- [YouTube Data API overview](https://developers.google.com/youtube/v3)
- [YouTube IFrame Player API](https://developers.google.com/youtube/iframe_api_reference)
- [YouTube developer policies](https://developers.google.com/youtube/terms/developer-policies)

## Design question

Which documented capability actually delivers the music experience you want?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
