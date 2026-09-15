# OPENCAT-4: YouTube Music, Spotify, and Ongoing Release Quality

**Component:** Music integration, app integration, and packaging

**Priority:** YouTube Music immediately after core productivity; Spotify after first extras

**Status:** Planned

**Dependencies:** A follows CORE-4 A, CORE-3 A/B, and OPENCAT-2 A/B; optional to-do (CORE-3 C / OPENCAT-2 C) may be skipped. B follows A and OPENCAT-3 A. Final C checks follow OPENCAT-3 B.

## Goal

Make music a high-priority productivity feature. The order is Pomodoro → tasks with Linear → optional simple to-do → YouTube Music → first extras → Spotify → remaining extras. Neither a finished game nor final release is required before YouTube Music.

## Milestone A: YouTube Music before extras

- [ ] Verify current official native playback/control capabilities, permitted access, authorization, account requirements, and Rust compatibility.
- [ ] Retain and complete Google OAuth through CORE-2 as required by the verified integration; Google sign-in alone does not prove playback access.
- [ ] Distinguish supported direct in-app playback from remote control and document the actual capability.
- [ ] Implement the supported native scope with shared playback state and controls such as track information, play/pause, and volume where available.
- [ ] Verify playback alongside Pomodoro and Linear, plus sign-out, network loss, account errors, and buffering/resource limits.

This stage does not wait for daily tracking, rewards, animated cats, all window modes, or release packaging.

## Milestone B: Spotify after the first extras

- [ ] After OPENCAT-3 A makes the app more presentable, verify Spotify's current official auth, playback, subscription, and native-client requirements.
- [ ] Add a Spotify adapter and account lifecycle behind the same music boundary where supported.
- [ ] Provide provider selection and consistent native controls without conflicting playback sessions.
- [ ] Verify switching providers, disconnecting, outages, unsupported accounts, and memory/CPU cost.

Do not wait for every gadget or skin before this stage. Remaining companion work resumes in OPENCAT-3 B.

## Milestone C: Quality throughout, final checks after remaining extras

- [ ] At each feature stage, verify relevant persistence/recovery, auth failures, responsive UI, and clean shutdown.
- [ ] Exercise timer/tracking edge cases as those features land; verify sync recovery and conflicts with Linear.
- [ ] Measure release-build memory and idle CPU after each music provider and extras stage, including playback workers and representative data.
- [ ] At final acceptance, cover all supported OSes and modes, active focus, Linear sync, and playback. Record method, dataset, steady-state memory, peaks, package size, and CPU.
- [ ] Meet the finalized runtime target of less than 100 MB and resolve over-budget cases before claiming compliance.
- [ ] Complete accessibility, packaging, and native startup checks with no persistent local server, web frontend, or required browser use outside auth.

Compiler size flags alone do not establish low runtime memory. Quality checks accompany development rather than delaying music until a separate core release.

## Provider constraints and acceptance

Keep YouTube Music and Spotify in one future `cat-music` crate, with separate provider modules. If official capabilities cannot meet the native-only constraints, record evidence and flag the blocked music milestone for a product decision. Do not silently substitute a browser player, webview, scraping, or external player, or mark unsupported music as complete.

Accept YouTube Music and Spotify independently with verified capabilities and working native flows. A provider blocker must be visible rather than quietly demoting music to the end. Gmail and unspecified new integrations remain outside this task.
