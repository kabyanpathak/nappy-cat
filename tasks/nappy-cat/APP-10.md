# APP-10: Finish full-screen and window recovery behavior

**Type:** Task

**Stage:** 7. Remaining extras

**Priority:** Normal

**Status:** Planned

**Dependencies:** [APP-9](../nappy-cat/APP-9.md), [MUSIC-5](../music/MUSIC-5.md)

## Goal

Add the expanded full-screen experience and make all window modes reliable on the declared platforms.

## Acceptance criteria

- [ ] Full screen provides the expanded task, session/progress, music, and settings views with an obvious exit.
- [ ] Users can move between all modes during focus, edits, syncing, and playback without lost work or duplicate accounting.
- [ ] Hidden and off-screen windows, resolution changes, and unsupported transparency/always-on-top behavior have usable recovery paths.
- [ ] Placement and mode preferences survive restart where appropriate, and platform limitations are documented.

## Scope and sequencing

If Spotify is blocked, the owner must explicitly accept a revised stage plan before bypassing this dependency.

## Documentation and learning

- [eframe documentation](https://docs.rs/eframe/latest/eframe/)
- [egui documentation](https://docs.rs/egui/latest/egui/)

## Design question

Which parts of state should follow the user between modes, and which are local to a view?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
