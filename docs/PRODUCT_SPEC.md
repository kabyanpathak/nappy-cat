# Nappy Cat product specification

Nappy Cat is one Rust productivity system with shared functionality exposed first through commands, then a TUI, and later a native GUI. It combines focus tools, Linear tasks, YouTube Music and Spotify, calendar, habits, and later tracking and a cat companion. This document records product behavior and shared constraints; the [task index](../tasks/README.md) defines independently completable assignments, and the [learning resources](LEARNING_RESOURCES.md) provide research starting points.

Features are planned unless implementation evidence shows otherwise. The current app is a console placeholder and the existing Google OAuth code is unfinished. This specification establishes no passing build, supported provider capability, or measured performance result.

## Ownership and task style

The project is intended to develop the owner's engineering skills. The owner chooses the implementation, investigates tradeoffs, and solves emerging problems. Tasks supply context, outcomes, dependencies, acceptance criteria, and references. They should stay at the level of a demanding assignment or Jira issue rather than prescribe data structures, function signatures, or implementation steps.

Planning, explanation, documentation research, and review are welcome. Application code or full implementation snippets require an explicit request from the developer. A backlog update does not authorize completing the implementation work it describes.

## Product constraints

- **One product, two frontends:** 100% Rust application code. Minimal command access comes first, followed by full TUI integration and a later GUI using the existing egui/eframe direction. Share behavior and data. No embedded web frontend or persistent local web server; the system browser is used only for authentication.
- **Local first:** local tasks, sessions, calendar events, habits, settings, totals, and unlocks are usable offline and signed out. Provider failures do not stop the timer or erase local work.
- **Small and quiet:** finalized runtime memory targets less than 100 MB. This interpretation is inherited from earlier planning and remains unverified. Measure package size and CPU separately, including idle/background behavior.
- **Optional accounts:** Google and Linear have independent account lifecycles. Neither account is required for the local productivity features.
- **Owner-directed scope:** unsupported integrations are visible blockers. Changing the constraints or bypassing a blocked stage requires an explicit project-owner decision.

The old Drive suite is retired: no file vault, Drive storage backend, warehouse, Git hosting, quota system, or sharing-key project. Gmail and unspecified integrations are outside the planned release.

## Delivery sequence

| Stage | Usable outcome |
| --- | --- |
| Foundation | Build baseline, minimal persistence, command entry point, and clear runtime ownership |
| 1. Pomodoro | Reliable presets and focus/break behavior, commands, and a small terminal status display |
| 2. Tasks with Linear | Local tasks plus explicit linking, editing, and recoverable sync through minimal commands/prompts |
| 3. Optional simple to-do | A bounded local-only convenience if existing task commands need it |
| 4. YouTube Music | First accepted supported music integration, usable through minimal controls |
| 5a. Calendar | Local events, agenda, and scheduled task blocks after YouTube Music |
| 5b. Habits | Recurring habits and occurrence completion directly in the calendar |
| 6. Spotify | Second accepted music integration and provider coexistence |
| 7. Integrated TUI | Full terminal workflows for delivered features, then an installable developer-facing release |
| 8. Tracking and GUI | Daily history/analytics, rewards, and a native GUI for broader audiences; keep the TUI supported |
| 9. GUI polish and release | Companion/window modes, cosmetics, accessibility, and later GUI packaging |

The first useful Pomodoro and Linear workflows use minimal commands/prompts and a timer status line or bar. They do not depend on full TUI pages, a graphical shell, tracking, or companion work. Full TUI implementation/integration is scheduled after the Spotify outcome, then packaged in TUI-4. Tracking and the GUI follow, with terminal access retained as new shared features arrive.

YouTube Music is first; Spotify is second. Calendar follows YouTube Music, and habits follow calendar. Apple Music is explicitly excluded. Google authentication is completed only for relevant feature needs. Reliability and footprint work accompanies every stage.

A provider feasibility ticket may complete with an evidence-backed negative result. The implementation remains Blocked until the owner accepts a compatible scope, deferral, or sequence change. Record the same decision on affected downstream tickets.

## Focus sessions

Provide named presets with customizable focus, short-break, long-break durations and cycle length, start/pause/resume/reset, completion feedback, and a clear next action. Define and verify what selecting or editing a preset does to an active session. Early controls are command-based with a minimal terminal status display. Completed and interrupted sessions remain distinguishable. Session behavior must be independent of frame rate, and paused time must not count toward focus.

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

## Calendar and habits

Calendar starts after the accepted YouTube Music outcome. Deliver local event creation/editing/deletion, a date/range agenda, timed and all-day events, and explicit timezone/daylight-saving behavior. Choose a bounded initial recurrence scope. Link task blocks deliberately; rescheduling or deleting a block does not silently alter a Linear issue or award focus completion. External provider synchronization, invitations, and shared calendars remain outside initial scope.

Habits follow calendar and appear in that same agenda. Define a small daily/weekly schedule model, occurrence identity, and complete/undo/skip behavior. Schedule edits and timezone changes must not rewrite historical outcomes unexpectedly or duplicate occurrences. Planned activity, completed habits, completed tasks, and actual focus remain distinguishable. Analytics/streaks/rewards follow in the later tracking stage.

Both features are first usable through commands, then integrated into TUI-3, then APP-13. Minimal occurrence records are required before analytics; a second parallel calendar is not.

## Daily tracking and rewards

Full daily tracking, analytical history, and rewards begin after the integrated TUI release, alongside the broader-audience GUI. Initial Pomodoro needs session behavior and records; early habits need calendar occurrence records and basic completion only. Tracking reuses those records and remains inspectable in both frontends.

App-use time means time Nappy Cat is running while the device is awake, including background mode. Do not monitor other applications. Exclude system sleep and explicit tracking pauses. Focus time is separate and includes only an active, unpaused focus session.

Daily totals use the user's local date. Midnight, timezone changes, clock changes, restarts, duplicate windows, and duplicate processes must not lose or double-count usage. Time while the app was closed must not be recovered as app-use time.

Cosmetic milestones are tunable. Award each once, preserve unlocks offline, and avoid punitive loss of progress. Basic productivity features remain available regardless of rewards. Persist the selected skin and show earned progress clearly.

## Cat, windows, and accessibility

Cat states include idle, focus, break, and celebration. These belong to the later GUI stage after TUI-4 and APP-13, including a movable companion with user-controlled always-on-top behavior and compact controls. None is a dependency of music or the terminal release.

All modes share one app state. Switching modes must preserve sessions, task edits, music state, totals, and rewards. Provide an obvious way out of full screen and a way to recover hidden or off-screen windows. Verify transparency and special-window support for each supported platform; use a normal compact-window fallback where needed.

Readable text, keyboard operation, understandable status/errors, and reduced motion apply throughout. Keep asset and animation costs bounded, and suspend unnecessary repainting or background work when hidden or idle. A network or storage delay must not freeze input and rendering.

## Local data and lifecycle

Choose a compact local store in an appropriate OS application-data location. Data includes sessions, tasks, calendar events, habit schedules/occurrences, settings, totals, unlocks, provider mappings, and pending sync work as each feature arrives. Define durability, versioning, migration, interrupted-write recovery, and understandable failure behavior without making future schemas prerequisites for the first timer.

One source of ownership must prevent duplicate processes or windows from duplicating sessions, accounting, and writes. Shutdown cancels unnecessary work and preserves completed local changes. Background operation has an explicit owner and exit behavior. APP-2 decides whether in-process ownership is enough or a shared local runtime/IPC is justified. A permanently running service is not a requirement, and no persistent web server is introduced.

## Architecture goal

Keep the current entry point at `apps/nappy-cat` and preserve `crates/cat-core`. The [README structure goal](../README.md#future-structure-goal) separates shared feature responsibilities from command, TUI, and GUI presentation. Modules are sufficient until a real boundary justifies a crate.

Both frontends use the same timer, task, provider, calendar, habit, and tracking behavior and data. Neither renderer owns the domain rules. Launching the TUI must not require a GUI or graphical session. Define simultaneous frontend access explicitly, preventing duplicate active sessions, conflicting writes, provider work, and counted time. Detaching an interface and quitting the runtime have distinct documented meanings when background operation is enabled.

The owner cited Codex/Antigravity as inspiration for offering terminal and graphical experiences; no AI assistant, code editor, embedded terminal emulator, or IDE integration is implied. Focusd supplies a visual and behavioral reference, not a mandatory architecture.

## Release acceptance

TUI-4 is the first developer-facing release and applies these checks to its delivered features. GUI-only, analytics, rewards, and companion checks are added for QUALITY-5, the later GUI release; they do not block terminal packaging.

- Local tools work signed out and offline, retain data across restart, and recover from interrupted operations without silently losing or duplicating work.
- Focus behavior is verified across pause/resume, sleep/wake, restart, and clock changes. Progress additionally covers midnight, timezone changes, duplicate events, and multiple windows/processes.
- Linear supports the agreed native workflow with explicit publication, visible synchronization, account separation, and recovery from outages, revoked credentials, ambiguous remote writes, and conflicts.
- Every shipped music provider has verified capabilities and working native behavior. Blocked provider work stays visible until the owner records a scope or sequencing decision.
- Calendar and habit records survive restart and recurrence edits, handle timezone/daylight-saving boundaries, and remain consistent in commands, TUI, and GUI.
- Terminal exit restores the terminal; frontend changes never duplicate runtime ownership or accounting.
- Window modes share state, recover reliably, and support keyboard access, readable controls, reduced motion, and optional notifications.
- Measure release-build memory on each supported OS with representative data in TUI-only, GUI-only, permitted combined use, idle companion, compact, full-screen, focus, sync, and playback scenarios. Record the method, steady state, peaks, package size, and CPU separately. Include playback workers; do not claim the less-than-100-MB target is met without evidence.
- Verify native startup, clean shutdown, packaging, required asset/dependency notices, and user-facing setup and recovery documentation on the chosen platforms.

Supported platforms, storage technology, reward thresholds, and final module/crate boundaries remain developer decisions. Tasks should make the relevant decisions visible without prescribing their implementation.
