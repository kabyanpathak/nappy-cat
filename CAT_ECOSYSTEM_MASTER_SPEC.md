# Nappy Cat: Product and Architecture Specification

> A small, standalone productivity game with a native Rust GUI.

This specification replaces the Google Drive application suite. Its filename and existing workspace paths remain stable. Features below are planned unless explicitly identified as existing.

## 0. Collaboration guidelines

Help the developer break features into clear tasks, explain design choices, and review implementation when requested. Do not write application code or full implementation snippets unless explicitly asked. This transition updates documentation and tasks only; crate creation, renames, dependency changes, and implementation are outside its scope.

## 1. Product and constraints

Nappy Cat is one desktop app combining focus tools with an animated cat. It should help people manage daily work without opening a productivity website.

- **Small and quiet:** target less than 100 MB runtime memory in the finalized release. This interprets “< 100 MB” as memory; measure package size separately and keep it compact too. Compliance has not been measured.
- **Rust and native UI:** 100% Rust application code, following the existing `egui`/`eframe` direction. No Electron, embedded webview, JavaScript frontend, or persistent local web server. Normal OS windowing and credential services are allowed.
- **Browser only for auth:** task editing, settings, and any music controls stay in the native GUI. HTTPS service API calls are allowed.
- **Local first:** tasks, timers, daily totals, rewards, and settings work without an account. Google Drive is not the storage backend.
- **Optional connections:** retain Google OAuth 2.0; add Linear tasks. Music follows a working timer and Linear workflow.
- **Background operation:** the app may stay unobtrusive as a desktop cat or in the background, without a separate always-running service. Suspend unnecessary animation and network work when hidden or idle.

File vaults, Drive database GUIs, quota guards, sharing keys, warehouses, and Git hosting are retired. Gmail and other providers are future ideas, outside the initial release.

## 2. User experience

### Pomodoro

Provide configurable focus and break lengths, start/pause/resume/reset, completion feedback, and a visible next action. Record completed and interrupted sessions distinctly. Derive time from elapsed-time state, not render frames. Sleep, restarts, and clock changes must not silently award focus time. Notifications and sounds are optional.

### Tasks and Linear

Build tasks with Linear after Pomodoro. The underlying task model and local persistence support linked issues and offline recovery; a separate local-only to-do experience is optional and follows the Linear workflow. That small list may be built directly or adapted from a suitable Rust project after checking its license, dependencies, and footprint. It must not become a large prerequisite for music. Local tasks, when enabled, support creation, editing, completion, and deletion with stable identifiers. Users explicitly choose what to send and its destination workspace/team; personal tasks are not automatically published.

Initial Linear scope: read linked issues, create issues from local tasks, and update supported fields such as title, description, and completion/status. Map provider statuses explicitly. Show pending, synced, failed, and conflict states. Refresh manually and on a modest schedule while connected, without requiring a public webhook server. Local deletion unlinks by default; remote deletion is outside initial sync scope.

### Daily tracking and rewards

Implement full daily tracking and rewards with the first extras after YouTube Music. Pomodoro initially needs only its session timing and records.

Show app-use time and focus time separately. Initially, app-use time means time Nappy Cat is running while the device is awake, including background mode. Do not monitor other applications. Explicitly paused tracking and system sleep do not count; focus time counts only during an active, unpaused focus session.

Persist bounded checkpoints and daily totals in the user's local timezone. Split intervals at midnight, handle timezone/clock changes without double-counting, and prevent multiple windows or processes from duplicating time. Restart recovery must not count the closed interval as usage.

Award cosmetic unlocks such as skins from recorded usage and completed focus sessions. Award each milestone once, preserve unlocks offline, and avoid punitive loss of progress. Thresholds can be tuned later; basic productivity tools are never reward-gated.

### Cat and window modes

| Mode | Intended use |
| --- | --- |
| Always-on-top cat | Small movable companion, timer summary, and access to controls |
| Compact popup | Quick task entry, timer controls, and today's progress |
| Full screen | Expanded tasks, history, rewards, and settings |

All modes share one state. Switching must not reset sessions, duplicate tracking, or lose edits. Provide obvious full-screen exit and recovery of hidden/off-screen windows. Always-on-top is user controlled. Verify platform support for transparency and special window behavior; provide a normal compact-window fallback.

Cat states include idle, focus, break, and celebration. Keep assets small, cache only what is needed, support reduced motion, and avoid permanent high-frame-rate repainting.

### Music as a priority feature

After Pomodoro, tasks with Linear, and the optional small to-do list, prioritize YouTube Music before animated cats and other extras. Then build a substantial first pass of those extras to make the app more presentable, add Spotify, and finish the remaining extras. Both providers share one music boundary. A finished game or final release is not a prerequisite for music. Verify official provider capabilities, authorization, playback rights, subscription requirements, and compatibility with native Rust and the memory target before implementation. Distinguish direct in-app playback from remote control of another player.

Google login does not establish permission or a supported mechanism for YouTube Music playback. If a provider cannot meet the constraints, record the limitation and defer it. A webview, browser player, scraping flow, or external-player dependency is not an automatic fallback. Add playback controls only where verified capabilities support them.

## 3. Current workspace and future boundaries

The workspace currently contains `crates/cat-core` and `apps/open-cat`. The library has an early auth scaffold; the binary is a console placeholder. Google auth and productivity features are not complete. Nappy Cat is the product name; `open-cat` remains the executable/package name for now.

Keep the existing `crates/`, `apps/`, and `tasks/` layout. Begin with modules; extract crates only when their implementation or dependencies justify it. These are provisional future names, not new workspace members:

| Boundary / possible crate | Responsibility | Initial home |
| --- | --- | --- |
| `cat-core` | Shared identifiers, errors, storage interfaces, orchestration | Existing library |
| `cat-google-auth` | Google OAuth and account lifecycle | Existing library |
| `cat-linear-auth` | Linear authorization and credentials | Existing library |
| `cat-tasks` | Local task model and operations | Existing library |
| `cat-linear` | Linear API adapter, issue mapping, sync queue | Existing library |
| `cat-pomodoro` | Timer state machine and session events | Existing library |
| `cat-progress` | Daily totals and cosmetic reward rules | Existing library |
| `cat-pet` | Cat states, skins, animation | Existing app |
| `cat-gui` | Shared views and all three window modes | Existing app |
| `cat-music` | Common playback model and separate provider adapters | Service logic in library, controls in app; YouTube Music before extras |

Combine tracking with rewards, all GUI modes together, and music providers together initially. These are in-process libraries, not separate microservices or desktop products.

### Future file structure goal

This is a target layout for gradual extraction, not the current tree or a request to create crates now. Retain `apps/open-cat` as the entry point; Nappy Cat remains the product name. Each extracted crate will have its own `Cargo.toml` and `src/lib.rs`; the app keeps `src/main.rs`.

```text
nappy-cat/
├── Cargo.toml
├── Cargo.lock
├── apps/
│   └── open-cat/                 # Thin native app entry point and composition
├── crates/
│   ├── cat-core/                 # Shared types and local storage interfaces
│   ├── cat-pomodoro/             # Focus/break state and session records
│   ├── cat-tasks/                # Task model; optional simple local to-do
│   ├── cat-linear-auth/          # Linear login and credential lifecycle
│   ├── cat-linear/               # Linear issue adapter and sync
│   ├── cat-google-auth/          # Retained Google OAuth
│   ├── cat-music/
│   │   └── src/
│   │       ├── lib.rs            # Shared playback state and interface
│   │       ├── youtube_music.rs # First provider, subject to feasibility
│   │       └── spotify.rs       # Added after first extras
│   ├── cat-progress/             # Daily tracking and rewards
│   ├── cat-pet/                  # Cat state, skins, and animation
│   └── cat-gui/
│       └── src/
│           ├── lib.rs            # Shared native views and app commands
│           └── modes/
│               ├── mod.rs
│               ├── companion.rs # Always-on-top cat
│               ├── popup.rs     # Compact controls
│               └── fullscreen.rs
├── tasks/
│   ├── cat-core/                 # Retain existing CORE task IDs
│   └── open-cat/                 # Retain existing OPENCAT task IDs
├── CAT_ECOSYSTEM_MASTER_SPEC.md
└── README.md
```

Crate extraction follows actual feature work, not tree order. GUI modes remain modules in one crate, music providers remain modules in one crate, and tracking/rewards stay together. Adapted to-do code belongs behind the task boundary rather than becoming another desktop application. Keep shared contracts independent of adapters to avoid cyclic crate dependencies.

The GUI sends typed commands to domain/service modules and receives events through bounded channels. Domain logic is independent of rendering. Network/storage work must not block the UI thread. The app owns its runtime and cancels workers on exit.

## 4. Storage, auth, and network rules

- Choose a compact versioned local store in CORE-1. Persist tasks, sessions, daily totals, unlocks, settings, provider mappings, and pending sync operations in the OS application-data directory. Specify atomic commits, migrations, and interrupted-write recovery.
- Keep credentials separate from productivity data and logs. Prefer OS credential storage; document a secure fallback before using one. Disconnect clears local credentials and stops provider requests while preserving local work.
- Complete Google OAuth 2.0 with PKCE and state validation. A temporary loopback listener may exist only during sign-in; close it on success, cancellation, error, or timeout. Remove Drive access from the planned login requirements. Choose minimal scopes for actual features and request additional access only when needed.
- Treat Linear auth as a separate provider boundary. Confirm its supported native/public-client flow, redirect requirements, and PKCE support before implementation. Do not embed a confidential client secret in a distributed binary. Record any incompatibility before expanding architecture.
- Reuse network clients, bound retries and queues, honor rate limits, and surface errors. Provider outages must not stop local tasks or timers.
- Persist sync operation identity before remote creation. Reconcile ambiguous timeouts before retrying to avoid duplicate issues. Define conflict behavior without silently overwriting concurrent edits.

Implementation tasks must verify current official provider documentation. These are requirements, not claims that every provider supports the desired flow.

## 5. Roadmap

| Stage | Task | Outcome |
| --- | --- | --- |
| Foundation | CORE-1, OPENCAT-1 | Minimal storage contracts and native shell |
| Alongside relevant features | CORE-2 | Retained Google OAuth, ready for music's validated auth needs |
| 1. Pomodoro | CORE-4 A, OPENCAT-2 A | Working timer and native controls |
| 2. Tasks with Linear | CORE-3 A/B, OPENCAT-2 B | Task model, Linear auth/sync, native task workflow |
| 3. Optional simple to-do | CORE-3 C, OPENCAT-2 C | Small local-only list, built or adapted; may be skipped |
| 4. YouTube Music | OPENCAT-4 A | Validate and implement supported native music scope |
| 5. First extras | CORE-4 B, OPENCAT-3 A | Tracking, initial rewards, animated cat, and presentable companion UI |
| 6. Spotify | OPENCAT-4 B | Second provider after the first extras |
| 7. Remaining extras | OPENCAT-3 B, OPENCAT-4 C | Remaining modes, skins, polish, and possible new features |

Pomodoro does not depend on tasks or provider login. Linear needs the task model and persistence, not a separate polished to-do app or Google auth. The optional to-do stage is intentionally small and skippable. YouTube Music follows this core workflow, without waiting for rewards, animation, all window modes, or final release. Spotify follows the first extras, not their completion. Reliability and footprint checks apply throughout. The eight task IDs remain; milestone labels define sequencing rather than numeric task order.

## 6. Release acceptance

- Start without an account; create tasks, run sessions, and retain totals/unlocks after restart.
- Connect/disconnect Google independently; connect Linear and create/update an issue from the native GUI.
- Recover from outages, revoked credentials, ambiguous writes, and concurrent edits without losing local work or duplicating issues.
- Switch all window modes without duplicating state. Verify reduced motion, keyboard controls, readable text, and optional notifications.
- Exercise pause/resume, sleep/wake, midnight, clock/timezone changes, restart, and duplicate-instance handling.
- Measure release-build process memory on every supported OS in idle-cat, popup, full-screen, focus, and Linear-sync scenarios with representative data. Record platform, dataset, measurement method, steady-state values, and peaks. Target less than 100 MB runtime memory and report over-budget cases. Record package size and idle CPU separately; compiler size flags do not prove low memory use.
- Recheck footprint when music is added, including playback workers. Ship no persistent local server or web frontend and require no browser use outside authentication.

Supported platforms and reward thresholds remain implementation decisions. This documentation update certifies no build, provider support, or performance result.
