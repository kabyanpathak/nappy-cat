# TUI-4: Package the first developer-facing terminal release

**Type:** Task

**Stage:** 7. Integrated TUI

**Priority:** Normal

**Status:** Planned

**Dependencies:** [TUI-3](../tui/TUI-3.md), [QUALITY-1](../quality/QUALITY-1.md), [QUALITY-4](../quality/QUALITY-4.md)

## Goal

Ship the integrated terminal experience after Spotify, without waiting for tracking, a GUI, or companion cosmetics.

## Acceptance criteria

- [ ] A release build installs and runs outside the source checkout on the initially supported platform; terminal cleanup and lifecycle behavior are verified.
- [ ] Document commands, TUI controls, setup, accounts, supported music scope, data location, upgrade/recovery, and known limitations.
- [ ] Verify representative timer/Linear/music/calendar/habit workflows, interrupted operations, and migrations with release-scope recovery and footprint evidence.
- [ ] Record dependency notices and provider distribution eligibility. GUI-only dependencies/assets are not required to run the TUI; any provider deferral is explicitly recorded.

## Documentation and learning

- [Ratatui documentation](https://ratatui.rs/)
- [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)

## Design question

What must be true for another developer to use this daily without your source checkout?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
