# QUALITY-5: Package and document a usable release

**Type:** Task

**Stage:** 9. GUI polish and release

**Priority:** High

**Status:** Planned

**Dependencies:** [APP-12](../nappy-cat/APP-12.md), [QUALITY-2](../quality/QUALITY-2.md), [QUALITY-3](../quality/QUALITY-3.md), [QUALITY-4](../quality/QUALITY-4.md)

## Goal

Deliver the later GUI release alongside the supported TUI so someone can install and use either on the supported platforms without needing the source checkout.

## Acceptance criteria

- [ ] Packages launch on a clean supported machine, include required assets, and present the Nappy Cat name consistently.
- [ ] Installation, upgrade/data migration, and uninstall behavior are documented and tested without unexpectedly removing personal data; platform signing/distribution requirements are addressed.
- [ ] User documentation covers first launch, local use, accounts, music capability limits, recovery, and known platform limitations.
- [ ] Dependency and asset licenses are recorded, release scope and provider distribution eligibility are checked, and release evidence includes recovery and footprint results.

## Scope and sequencing

TUI-4 owns the earlier terminal release and does not depend on this GUI release ticket.

A release must distinguish implemented capabilities from owner-approved deferrals; a provider blocker is not a completed feature.

## Documentation and learning

- [Cargo build profiles](https://doc.rust-lang.org/cargo/reference/profiles.html)
- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html)
- [Spotify quota modes](https://developer.spotify.com/documentation/web-api/concepts/quota-modes)

## Design question

What assumptions from your development machine disappear when someone installs a release?

[Backlog and working rules](../README.md) · [Learning resources](../../docs/LEARNING_RESOURCES.md)
