# Nappy Cat

> A small native desktop companion for focused work, built in Rust.

Nappy Cat brings a Pomodoro timer, Linear tasks, music, and an animated cat into one quiet desktop app. Finish focus sessions, see your daily progress, and unlock cosmetic rewards. Keep the cat beside your work, open compact controls, or use an expanded full-screen view.

This is also a project for growing as an engineer. The developer owns the design, implementation, debugging, and tradeoffs. The backlog supplies clear problems, expected behavior, and documentation to investigate; it leaves room to discover the solution.

## Project goals

- **Useful focus tools:** a dependable Pomodoro timer, a native Linear workflow, and music where provider capabilities permit it.
- **A companion with progression:** a small animated cat, daily progress, and cosmetic unlocks that encourage use without restricting the productivity tools.
- **A small, quiet app:** restrained background work and a finalized runtime-memory target of **less than 100 MB**. This interpretation is inherited from earlier planning and is **unverified**. Package size and CPU usage are measured separately.
- **Rust and native UI:** 100% Rust application code, following the existing egui/eframe direction. Ordinary use stays in the native app; the system browser is used only for authentication. No embedded web frontend or persistent local web server.
- **Local ownership:** local tasks, focus sessions, settings, totals, and rewards remain available without an account. Connections add capabilities without making local work depend on a provider.
- **Learning through implementation:** tasks describe outcomes and constraints, while the developer chooses the code and solves the problems that arise.

## Intended experience

### Focus first

Configure focus and break durations, then start, pause, resume, or reset a session. See the current state and next action clearly. Completed and interrupted sessions remain distinguishable, and sleep or restart must not silently award focus time. Completion notifications and sounds are optional.

### Tasks with Linear

After Pomodoro, bring linked Linear issues into the app. Choose a destination workspace/team, explicitly link or publish a task, and edit supported fields through native controls. Show sync progress, failures, and conflicts so that remote changes are understandable. Cached and local work remain accessible during outages.

A separate simple local-only to-do view is optional. Add it only if the main task workflow does not already meet that need, and keep it small enough to preserve the music priority.

### Music, then the first companion features

YouTube Music is the first music priority after the core workflow. A substantial first pass of tracking, rewards, animation, and companion windows follows it. Spotify comes after that first pass; the remaining extras follow Spotify.

Each music provider needs a feasibility decision based on its current official capabilities, account requirements, and compatibility with the native-app constraints. Direct playback and remote control are different outcomes. Google sign-in alone does not establish YouTube Music access. An unsupported implementation remains blocked until the project owner decides how to proceed; research alone does not count as delivering music. See the [provider references](docs/LEARNING_RESOURCES.md).

### Daily progress and a desktop cat

Show app-use time and focus time separately. Initially, app-use time means time Nappy Cat runs while the device is awake, including background use; it does not mean monitoring other applications. Exclude system sleep and explicit tracking pauses.

Earn cosmetic unlocks from recorded usage and completed focus sessions. Keep rewards offline, award milestones once, and let users select unlocked skins. The timer and tasks never depend on earning a reward.

The intended window modes share one app state:

| Mode | Purpose |
| --- | --- |
| Companion | A small movable, optionally always-on-top cat with quick access to controls |
| Compact popup | Timer, task, music, and progress controls close to the current work |
| Full screen | Expanded tasks, history, rewards, and settings |

Switching views must preserve sessions, edits, playback state, and accounting. Reduced motion, keyboard use, readable controls, and recovery of hidden or off-screen windows are part of the intended experience.

## Build order

| Stage | Usable outcome |
| --- | --- |
| Foundation | A verified development baseline, minimal local persistence, and a responsive native shell |
| 1. Pomodoro | A reliable timer with native controls and session recovery |
| 2. Tasks with Linear | Local task data plus explicit linking, editing, and recoverable sync |
| 3. Optional simple to-do | A small local-only view, if the existing task controls need it |
| 4. YouTube Music | The verified supported native integration, subject to feasibility |
| 5. First extras | Daily tracking, initial rewards, animated cat, companion, and compact controls |
| 6. Spotify | The verified second-provider integration and provider selection |
| 7. Remaining extras and release | Remaining window modes, cosmetics, accessibility, packaging, and final quality checks |

Google authentication is a separate workstream and is completed when relevant; it is not a prerequisite for Pomodoro or Linear. Reliability and resource measurements accompany feature work. Music does not wait for a finished game or final packaging. Changing product constraints or bypassing a blocked provider stage requires an explicit project-owner decision.

Start with the [task index](tasks/README.md), which contains the assignments, dependencies, and migration from the previous broad task files. The [product specification](docs/PRODUCT_SPEC.md) records shared behavior and constraints. The [learning resources](docs/LEARNING_RESOURCES.md) collect documentation and selected projects to study.

## How to use the tasks

Choose a task whose prerequisites are met. Read its goal and acceptance criteria, explore the linked documentation, and decide how to approach the problem. Break it down further if that helps your own workflow. Keep useful design decisions and verification evidence with the task, and mark it complete when the observable outcome is demonstrated.

The assignments intentionally leave implementation choices open. They do not prescribe structs, function signatures, exact algorithms, or a sequence of code edits. References are starting points for research, not code to copy automatically. Check compatibility and licensing before adapting another project.

AI assistance should support planning, explanation, documentation discovery, and review. Application implementation or full solution snippets are provided only when the developer explicitly asks. Updating this plan does not authorize implementing its tickets.

## Future structure goal

The goal is one desktop application with clear responsibilities. The following is a possible destination as the project grows, **not a scaffolding checklist** or a claim that these crates already exist. Start with useful boundaries; decide when a module deserves its own crate as implementation reveals the tradeoffs.

```text
nappy-cat/
├── apps/
│   └── nappy-cat/          # Native application entry point and composition
├── crates/
│   ├── cat-core/          # Shared foundation; retain this name
│   ├── cat-pomodoro/      # Focus/break behavior and session records
│   ├── cat-tasks/         # Local task behavior and data
│   ├── cat-linear-auth/   # Linear account lifecycle
│   ├── cat-linear/        # Linked issues and synchronization
│   ├── cat-google-auth/   # Optional Google account lifecycle
│   ├── cat-music/         # Shared music behavior and provider integrations
│   ├── cat-progress/      # Daily tracking and cosmetic rewards
│   ├── cat-pet/           # Cat states, skins, and animation
│   └── cat-gui/           # Native views and all window modes
├── docs/                  # Product decisions and learning references
└── tasks/                 # Assignments and roadmap
```

Keep music providers together initially, tracking and rewards together, and all window modes within one GUI boundary. These are in-process responsibilities, not a suite of separate products. Keep domain behavior independent of rendering and avoid dependencies that create cycles. The eventual module-versus-crate decisions belong to the developer.

## Development status and commands

The implementation is at scaffold stage: a Cargo workspace, the `cat-core` library with an unfinished Google OAuth experiment, and a `nappy-cat` binary whose source currently prints “Hello, world!”. The native GUI and product features remain planned. Working authentication, provider feasibility, and performance remain unverified.

With a Rust toolchain that supports the workspace's Rust 2024 edition and declared dependencies:

```bash
cargo check --workspace
cargo test --workspace
cargo run --bin nappy-cat
```

Cargo metadata validates the renamed package and workspace. An offline workspace check on September 16, 2026 reached the existing OAuth scaffold and failed with four error-type mismatches in `crates/cat-core/src/auth.rs`; resolving these remains developer work in CORE-1/CORE-2. No Rust source was changed during planning.

These commands target the renamed app package. Existing scaffold problems need to be resolved before they succeed; running the app does not yet launch the planned GUI. `cat-core` keeps its existing package name.

## Scope boundaries

The former Google Drive suite is retired. File vaults, Drive-backed databases, warehouses, Git hosting, quotas, and sharing-key projects are outside this roadmap. Google Drive is not the storage backend. Gmail and additional integrations remain future ideas, and should receive an explicit priority decision before new implementation tasks are added.
