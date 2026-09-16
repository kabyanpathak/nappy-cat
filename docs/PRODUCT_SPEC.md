# Nappy Cat product specification

Nappy Cat is one native Rust desktop app that combines focus tools, tasks, supported music integrations, and a small cat companion. This document records product behavior and shared constraints; the [task index](../tasks/README.md) defines independently completable assignments, and the [learning resources](LEARNING_RESOURCES.md) provide research starting points.

Features are planned unless implementation evidence shows otherwise. The current app is a console placeholder and the existing Google OAuth code is unfinished. This specification establishes no passing build, supported provider capability, or measured performance result.

## Ownership and task style

The project is intended to develop the owner's engineering skills. The owner chooses the implementation, investigates tradeoffs, and solves emerging problems. Tasks supply context, outcomes, dependencies, acceptance criteria, and references. They should stay at the level of a demanding assignment or Jira issue rather than prescribe data structures, function signatures, or implementation steps.

Planning, explanation, documentation research, and review are welcome. Application code or full implementation snippets require an explicit request from the developer. A backlog update does not authorize completing the implementation work it describes.

## Product constraints

- **One native app:** 100% Rust application code, following the existing egui/eframe direction. No embedded web frontend or persistent local web server. The system browser is used only for authentication; normal task editing, music controls, and settings stay native.
- **Local first:** local tasks, sessions, settings, totals, and unlocks are usable offline and signed out. Provider failures do not stop the timer or erase local work.
- **Small and quiet:** finalized runtime memory targets less than 100 MB. This interpretation is inherited from earlier planning and remains unverified. Measure package size and CPU separately, including idle/background behavior.
- **Optional accounts:** Google and Linear have independent account lifecycles. Neither account is required for the local productivity features.
- **Owner-directed scope:** unsupported integrations are visible blockers. Changing the constraints or bypassing a blocked stage requires an explicit project-owner decision.

The old Drive suite is retired: no file vault, Drive storage backend, warehouse, Git hosting, quota system, or sharing-key project. Gmail and unspecified integrations are outside the planned release.

## Delivery sequence

| Stage | Required outcome |
| --- | --- |
| Foundation | Development baseline, minimal persistence, and responsive native shell |
| Pomodoro | Usable focus/break timer and native controls |
| Linear tasks | Local task model, account connection, linked issues, and recoverable sync |
| Optional simple to-do | A small local-only view only if it adds value |
| YouTube Music | A verified, supported native music experience |
| First extras | Tracking, initial rewards, animated cat, companion, and compact controls |
| Spotify | A verified second provider after the first extras are usable |
| Remaining extras and release | Full-screen mode, remaining polish, and release acceptance |

Pomodoro does not depend on tasks or login. Linear does not depend on Google authentication or a polished standalone to-do app. YouTube Music does not wait for rewards, animation, every window mode, or release packaging. Spotify follows the first substantial companion pass, not the completion of every extra. Google auth is completed alongside the relevant feature needs. Reliability and footprint checks run throughout.

A provider feasibility task can finish with an evidence-backed negative result. Its unsupported implementation task remains **Blocked**; it is not completed or silently replaced. The owner then decides whether to revise the constraints, defer the provider, or authorize a different sequence.

## Focus sessions

Provide configurable focus and break durations, start/pause/resume/reset, completion feedback, and a clear next action. Completed and interrupted sessions remain distinguishable. Session behavior must be independent of frame rate, and paused time must not count toward focus.

Sleep, restart, and clock changes must not silently create earned focus time. Recovery should make the resulting session state understandable. Notifications and sounds are optional, configurable, and tolerant of denied system permissions.

## Tasks and Linear

Local task behavior supports creation, editing, completion/reopening, and deletion with stable identity and persistence. The initial model supports the linked workflow; a separate local-only to-do view remains optional and small. Any adapted project must be reviewed for license, dependencies, maintenance, and fit with native UI and footprint goals.

Users select the destination workspace/team and explicitly choose which tasks to link or publish. Personal tasks are never automatically sent to a provider. Initial Linear scope covers reading linked issues, creating issues from chosen tasks, and updating supported fields such as title, description, and status. Local completion must have a deliberate mapping to provider statuses.

Make pending, synced, failed, and conflicting changes visible. Support manual refresh and modest background refresh while connected, without requiring a public webhook server. Recover from outages and rate limits without unbounded retries or duplicate issues. Ambiguous remote-create outcomes require reconciliation before another create is attempted. Concurrent edits must not be silently overwritten.

Local deletion unlinks by default; remote deletion is outside the initial scope. Disconnect stops provider activity and preserves local work. Changing accounts or destinations must not send queued work to an unintended account/team.

## Accounts and credentials

Verify current official native-app authorization requirements before implementation; use the [provider references](LEARNING_RESOURCES.md). Distributed clients must not depend on keeping an embedded confidential secret private. Credentials stay separate from productivity data and logs, with an appropriate secure storage strategy.

Google sign-in retains the authorization-code, PKCE, and request-validation requirements from the existing direction. Access is limited to the actual feature need; Drive permissions are not required by this product. A temporary loopback callback is permitted during authentication and must end on success, denial, cancellation, timeout, or error.

Account state should make expiry, refresh failures, revoked access, and disconnect understandable. Disconnect clears local credentials and stops provider requests. Google authentication does not imply permission or a supported playback mechanism for YouTube Music.

## Music

YouTube Music and Spotify each need a documented feasibility decision using current official capabilities, permitted access, authorization, subscription/account requirements, and compatibility with native Rust and the resource goals. The task must distinguish direct in-app playback from remote control of another player.

Deliver only the capability supported by that decision, with native controls and accurate provider/account state. Track information, play/pause, volume, and other controls appear only where supported. Handle sign-out, outages, account restrictions, and playback failures while keeping local tools usable. Provider switching must not accidentally leave conflicting playback sessions.

A browser player, webview, scraping integration, or external-player dependency is not an automatic fallback. Such a change needs an explicit owner decision. Keep the feasibility evidence and any blocker visible in the task record.

## Daily tracking and rewards

Full daily tracking and rewards begin with the first extras after YouTube Music. Initial Pomodoro work needs only session behavior and records.

App-use time means time Nappy Cat is running while the device is awake, including background mode. Do not monitor other applications. Exclude system sleep and explicit tracking pauses. Focus time is separate and includes only an active, unpaused focus session.

Daily totals use the user's local date. Midnight, timezone changes, clock changes, restarts, duplicate windows, and duplicate processes must not lose or double-count usage. Time while the app was closed must not be recovered as app-use time.

Cosmetic milestones are tunable. Award each once, preserve unlocks offline, and avoid punitive loss of progress. Basic productivity features remain available regardless of rewards. Persist the selected skin and show earned progress clearly.

## Cat, windows, and accessibility

Cat states include idle, focus, break, and celebration. The first extras deliver an initial animated cat, a movable companion with user-controlled always-on-top behavior, and compact access to existing controls. Full-screen mode and expanded cosmetics follow Spotify.

All modes share one app state. Switching modes must preserve sessions, task edits, music state, totals, and rewards. Provide an obvious way out of full screen and a way to recover hidden or off-screen windows. Verify transparency and special-window support for each supported platform; use a normal compact-window fallback where needed.

Readable text, keyboard operation, understandable status/errors, and reduced motion apply throughout. Keep asset and animation costs bounded, and suspend unnecessary repainting or background work when hidden or idle. A network or storage delay must not freeze input and rendering.

## Local data and lifecycle

Choose a compact local store in an appropriate OS application-data location. Data includes sessions, tasks, settings, totals, unlocks, provider mappings, and pending sync work as each feature arrives. Define durability, versioning, migration, interrupted-write recovery, and understandable failure behavior without making future schemas prerequisites for the first timer.

One source of ownership must prevent duplicate processes or windows from duplicating sessions, accounting, and writes. Shutdown cancels unnecessary work and preserves completed local changes. Background operation remains part of the app, without a separate always-running service.

## Architecture goal

Keep the application entry point at `apps/nappy-cat` and preserve `crates/cat-core`. The [README's future structure](../README.md#future-structure-goal) describes possible boundaries for Pomodoro, tasks, Google auth, Linear auth/sync, music, progress, pet, and GUI.

These boundaries can remain modules until the developer decides separate crates are justified. Do not scaffold speculative workspace members. Keep domain behavior independent of rendering; group tracking with rewards, providers within music, and all window modes within GUI initially. This remains one in-process desktop application.

## Release acceptance

- Local tools work signed out and offline, retain data across restart, and recover from interrupted operations without silently losing or duplicating work.
- Focus behavior is verified across pause/resume, sleep/wake, restart, and clock changes. Progress additionally covers midnight, timezone changes, duplicate events, and multiple windows/processes.
- Linear supports the agreed native workflow with explicit publication, visible synchronization, account separation, and recovery from outages, revoked credentials, ambiguous remote writes, and conflicts.
- Every shipped music provider has verified capabilities and working native behavior. Blocked provider work stays visible until the owner records a scope or sequencing decision.
- Window modes share state, recover reliably, and support keyboard access, readable controls, reduced motion, and optional notifications.
- Measure release-build memory on each supported OS with representative data in idle companion, compact, full-screen, focus, sync, and playback scenarios. Record the method, steady state, peaks, package size, and CPU separately. Include playback workers; do not claim the less-than-100-MB target is met without evidence.
- Verify native startup, clean shutdown, packaging, required asset/dependency notices, and user-facing setup and recovery documentation on the chosen platforms.

Supported platforms, storage technology, reward thresholds, and final module/crate boundaries remain developer decisions. Tasks should make the relevant decisions visible without prescribing their implementation.
