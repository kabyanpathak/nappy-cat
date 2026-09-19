# Nappy Cat

> A small Rust productivity app: core functionality first, a terminal interface, then a native GUI.

Nappy Cat brings Pomodoro, Linear tasks, YouTube Music and Spotify, a calendar, and habits into one shared productivity system. Early work targets developers through simple commands and a small timer status display. Once core features are useful, package them into a complete TUI. Later, add tracking and a native GUI for broader audiences, including the cat companion and cosmetic progression.

This is also a project for growing as an engineer. The developer owns the design, implementation, debugging, and tradeoffs. The backlog supplies clear problems, expected behavior, and documentation to investigate; it leaves room to discover the solution.

## Project goals

- **Useful focus tools:** a dependable Pomodoro timer, a shared Linear workflow, and music where provider capabilities permit it.
- **A companion with progression:** a small animated cat, daily progress, and cosmetic unlocks that encourage use without restricting the productivity tools.
- **A small, quiet app:** restrained background work and a finalized runtime-memory target of **less than 100 MB**. This interpretation is inherited from earlier planning and is **unverified**. Package size and CPU usage are measured separately.
- **Rust with two interfaces:** 100% Rust application code. Commands first, a full terminal UI after the core features, then a native GUI following the existing egui/eframe direction. Both interfaces use the same domain behavior and data. The system browser is used only for authentication; no embedded web frontend or persistent local web server.
- **Local ownership:** local tasks, focus sessions, calendar events, habits, settings, totals, and rewards remain available without an account. Connections add capabilities without making local work depend on a provider.
- **Learning through implementation:** tasks describe outcomes and constraints, while the developer chooses the code and solves the problems that arise.

## Intended experience

### Core functionality first

Choose a named Pomodoro preset, customize focus/short-break/long-break durations and cycle length, then start, pause, resume, or reset. Keep completed and interrupted sessions distinct and define sleep/restart behavior. Start with commands and a small terminal status line or bar; full-screen TUI pages are not a prerequisite.

Next, make Linear work end to end: local tasks, account connection, issue browsing/linking, explicit publication and updates, and recoverable synchronization. Simple commands or prompts are enough to exercise these capabilities. A separate local-only to-do convenience remains optional and must not delay music.

### YouTube Music, calendar, habits, then Spotify

YouTube Music is the first music provider; Spotify is second. Apple Music is outside this roadmap. Research each provider's current supported capability before implementation, distinguishing direct playback from remote control. Google sign-in alone does not establish YouTube Music playback. Keep unsupported work visible and obtain a product decision before changing provider scope or constraints; research alone is not delivery.

Calendar follows the accepted YouTube Music outcome. Begin with local events, a date/range agenda, and task time blocks. External calendar providers, invitations, and shared calendars are not initial requirements. Habits then build directly on this calendar: schedule occurrences and mark them complete, skipped, or pending. These basic records come before the later analytics and streak views.

### A complete TUI, then a GUI

After the Spotify stage, bring the delivered functionality together in a keyboard-driven TUI and package a developer-facing terminal release. This milestone includes timer, Linear tasks, music, calendar, habits, and settings. It does not wait for daily analytics, rewards, or graphical windows.

The later GUI and TUI are two ways to use the same app. The owner's Codex/Antigravity comparison expresses this paired-interface direction, not a request for AI-agent or IDE features. Focusd is a reference for presets and a restrained panel-based look. Terminal-inspired typography, borders, and colors may carry into the GUI while controls remain discoverable for a broader audience.

### Tracking and the companion

After the integrated TUI release, build daily history/analytics and rewards alongside the GUI. Retain minimal session and habit records earlier so this work has reliable input. App-use time and focus time remain separate; app use means this app running while the device is awake, not monitoring other applications. Multiple frontends must never multiply counted time.

Keep the cat, compact controls, and expanded views in the later graphical stage. They share existing records and runtime ownership with the terminal interface. Basic productivity tools never depend on cosmetic rewards. Keyboard access, readable states, reduced motion, window recovery, and restrained background work apply as relevant features arrive.

## Build order

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

The roadmap separates functional delivery from interface integration. Finish Pomodoro and Linear before full TUI implementation; early timer status output is intentionally small. The chosen full-TUI integration milestone follows Spotify. Tracking and GUI follow that terminal release. Google authentication is independent and added when a feature needs it.

Provider feasibility may expose a blocker. Record an owner-approved deferral or changed scope before bypassing a blocked provider gate; a negative research result is not a shipped integration.

Start with the [task index](tasks/README.md), [product specification](docs/PRODUCT_SPEC.md), and [learning resources](docs/LEARNING_RESOURCES.md).

## How to use the tasks

Choose a task whose prerequisites are met. Read its goal and acceptance criteria, explore the linked documentation, and decide how to approach the problem. Break it down further if that helps your own workflow. Keep useful design decisions and verification evidence with the task, and mark it complete when the observable outcome is demonstrated.

The assignments intentionally leave implementation choices open. They do not prescribe structs, function signatures, exact algorithms, or a sequence of code edits. References are starting points for research, not code to copy automatically. Check compatibility and licensing before adapting another project.

AI assistance should support planning, explanation, documentation discovery, and review. Application implementation or full solution snippets are provided only when the developer explicitly asks. Updating this plan does not authorize implementing its tickets.

## Future structure goal

Keep `apps/nappy-cat` as the current entry point and preserve `crates/cat-core`. Shared feature behavior and data serve command, TUI, and GUI entry paths. The following is a possible destination, not required scaffolding or a claim that these crates exist:

```text
nappy-cat/
├── apps/
│   └── nappy-cat/          # Composition and command/frontend entry paths
├── crates/
│   ├── cat-core/          # Shared foundation; retain this name
│   ├── cat-pomodoro/      # Presets, session behavior, session records
│   ├── cat-tasks/         # Local task behavior and data
│   ├── cat-linear-auth/   # Linear account lifecycle
│   ├── cat-linear/        # Linked issues and synchronization
│   ├── cat-google-auth/   # Optional Google account lifecycle
│   ├── cat-music/         # YouTube Music and Spotify capabilities
│   ├── cat-calendar/      # Events, agenda, task scheduling
│   ├── cat-habits/        # Recurring habits and calendar occurrence outcomes
│   ├── cat-progress/      # Later daily analytics and rewards
│   ├── cat-tui/           # Full terminal presentation
│   ├── cat-gui/           # Later native GUI and window modes
│   └── cat-pet/           # Later cat states, skins, animation
├── docs/
└── tasks/
```

Start with modules and extract crates only when justified. Keep domain behavior independent of either renderer; do not duplicate timer/sync rules or maintain separate databases per frontend. A TUI launch must not require a graphical environment. Choose runtime/process ownership in APP-2 and verify it again when the GUI arrives. A shared local runtime or IPC may be evaluated if needed; the dual-interface goal does not mandate a daemon, web server, or cloud service.

## Development status and commands

The implementation is at scaffold stage: a Cargo workspace, the `cat-core` library with an unfinished Google OAuth experiment, and a `nappy-cat` binary whose source currently prints “Hello, world!”. Commands, the TUI, native GUI, and product features remain planned. Working authentication, provider feasibility, and performance remain unverified.

With a Rust toolchain that supports the workspace's Rust 2024 edition and declared dependencies:

```bash
cargo check --workspace
cargo test --workspace
cargo run --bin nappy-cat
```

Cargo metadata validates the renamed package and workspace. An offline workspace check on September 16, 2026 reached the existing OAuth scaffold and failed with four error-type mismatches in `crates/cat-core/src/auth.rs`; resolving these remains developer work in CORE-1/CORE-2. No Rust source was changed during planning.

These commands target the renamed app package. Existing scaffold problems need to be resolved before they succeed; running the app does not yet launch the planned TUI or GUI. `cat-core` keeps its existing package name.

## Scope boundaries

The former Google Drive suite is retired. File vaults, Drive-backed databases, warehouses, Git hosting, quotas, and sharing-key projects are outside this roadmap. Google Drive is not the storage backend. Apple Music is excluded. External calendar synchronization, Gmail, and additional integrations remain future ideas, and should receive an explicit priority decision before new implementation tasks are added.
