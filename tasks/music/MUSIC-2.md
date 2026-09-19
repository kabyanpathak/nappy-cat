# MUSIC-2: Deliver the accepted YouTube Music experience

**Type:** Task

**Stage:** 4. YouTube Music

**Priority:** High

**Status:** Conditional — awaiting MUSIC-1 and owner scope acceptance

**Dependencies:** [MUSIC-1](../music/MUSIC-1.md), [APP-4](../nappy-cat/APP-4.md), [CORE-6](../cat-core/CORE-6.md)

## Goal

Implement only the supported music scope accepted after feasibility research, through minimal commands first; the full TUI and GUI reuse this capability later.

## Acceptance criteria

- [ ] The owner-approved capability works end to end and is accurately described as playback, remote control, or another explicitly accepted scope.
- [ ] Required authentication uses CORE-2 when applicable; available track information and playback controls reflect real provider capabilities.
- [ ] Network loss, unavailable content, unsupported accounts, sign-out, and playback errors leave the timer and tasks responsive.
- [ ] Memory/CPU cost and buffering or worker limits are measured; unsupported behavior is neither hidden nor reported as completed.

## Scope and sequencing

No automatic webview, browser player, scraping, or external-player substitution. Add CORE-2 as a dependency if the accepted integration requires Google auth. If no compliant path exists, record Blocked and request a product decision.

## Documentation and learning

- [YouTube Data API overview](https://developers.google.com/youtube/v3)
- [YouTube developer policies](https://developers.google.com/youtube/terms/developer-policies)
- [Tokio tutorial](https://tokio.rs/tokio/tutorial)

## Design question

What is the smallest supported music experience that would be useful during a focus session?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
