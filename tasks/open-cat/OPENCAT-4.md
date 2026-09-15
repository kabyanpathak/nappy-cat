# OPENCAT-4: Core Release Quality, Then Music

**Component:** App integration, packaging, and deferred music boundary

**Priority:** Core release high; music later

**Status:** Planned

**Dependencies:** CORE-1 through CORE-4 and OPENCAT-1 through OPENCAT-3 for release; working timer and Linear integration before music

## Goal

Ship a compact, reliable productivity game, then evaluate music within the same native-app constraints. This replaces sharing keys and the former 35 MB memory target.

## Milestone A: Core release

- [ ] Verify offline startup, local persistence/recovery, auth cancellation/revocation, Linear sync recovery/conflicts, and clean shutdown.
- [ ] Exercise timer/tracking behavior across sleep, restart, midnight, clock changes, and multiple-instance attempts.
- [ ] Verify all window modes, keyboard access, reduced motion, readable controls, and optional notifications.
- [ ] Tune release settings, dependency features, assets, and idle repaint/network behavior based on measurements.
- [ ] Measure release-build memory on each supported OS in idle-cat, popup, full-screen, focus, and Linear-sync scenarios with representative data. Record method, platform, dataset, steady-state use, and peaks.
- [ ] Verify the finalized runtime target of less than 100 MB; record over-budget scenarios and resolve them before claiming compliance. Record package size and idle CPU separately.
- [ ] Package and verify native startup with no web frontend, persistent server, or required browser use outside auth.

Compiler size flags alone do not establish low runtime memory. The original smaller-memory promise is superseded; no performance result has been measured by this documentation update.

## Milestone B: Music, after the core workflow

- [ ] Investigate YouTube Music first using current official documentation: native playback/control capabilities, permitted access, auth scopes, account requirements, and Rust compatibility.
- [ ] Document whether supported behavior is direct in-app playback or remote control; Google login alone is not evidence of playback access.
- [ ] If feasible within the product constraints, implement bounded playback state and native controls such as track information, play/pause, and volume where supported.
- [ ] Investigate Spotify next and add a separate adapter behind the same music boundary if feasible.
- [ ] Verify sign-out, network loss, unsupported accounts, buffering/resource limits, and memory/CPU impact including playback workers.
- [ ] If a provider cannot meet native-only requirements, record the evidence and defer it. Do not silently substitute a browser player, webview, scraping, or external player.

Keep both providers in one future `cat-music` boundary initially. Music feasibility or deferral must be reported separately from core-release completion. Gmail and other integrations remain outside this task.

## Acceptance

Core release has recorded reliability and footprint evidence. Music is either validated and implemented within constraints or explicitly deferred with a documented capability gap. No speculative provider support is advertised as shipped.
