# Nappy Cat

> Your all-in-one productivity game: a small native desktop companion, built in Rust.

Nappy Cat combines a Pomodoro timer, daily app-time tracking, a task list, and an animated cat. Focus, finish tasks, and unlock cosmetic rewards such as cat skins. Keep the cat beside your work, open a compact popup, or switch to full screen.

The philosophy stays simple: minimal UI, a small footprint, and quiet background operation. All application code is Rust, with a native GUI, no embedded browser or web frontend, and browser use only for authentication. The finalized app targets **less than 100 MB of runtime memory**; package size and CPU usage will also be measured. This is a target, not a verified result.

## Build the essentials first

- Pomodoro focus and break sessions with pause, resume, and reset.
- Local tasks that work without an account or internet connection.
- Daily time spent running Nappy Cat, with focus time shown separately.
- An animated cat with cosmetic unlocks.
- Always-on-top cat, compact popup, and full-screen modes sharing one app state.
- Optional Google OAuth 2.0 sign-in, retained from the original direction.
- Optional Linear connection: view linked tasks and create or update them from Nappy Cat.

After the timer and Linear tasks work, investigate YouTube Music first, then Spotify. Native playback capabilities and account requirements must be validated before promising either integration; Google sign-in alone does not establish music access. Gmail and other integrations are later possibilities.

Google Drive is no longer the storage backend. The file vault, Drive database, warehouse, Git hosting, quotas, and sharing-key projects are retired from the roadmap. Tasks, sessions, settings, and rewards will be stored locally.

## Current implementation and structure

The repository contains a Cargo workspace, an early Google OAuth scaffold, and an `open-cat` binary that currently prints “Hello, world!”. The productivity features and GUI are planned; authentication still needs completion and verification.

The product name is **Nappy Cat**. Existing package names, task IDs, and directories remain unchanged during this documentation transition.

```text
nappy-cat/
├── Cargo.toml
├── Cargo.lock
├── crates/
│   └── cat-core/          # Existing library; future domain and service modules
├── apps/
│   └── open-cat/          # Existing binary; future Nappy Cat native app
├── tasks/
│   ├── cat-core/          # CORE-1 through CORE-4
│   └── open-cat/          # OPENCAT-1 through OPENCAT-4
├── CAT_ECOSYSTEM_MASTER_SPEC.md
└── README.md
```

Start with modules in these crates, then extract libraries as features grow. Planned boundaries cover Google auth, Linear auth, Linear sync, local tasks, Pomodoro, tracking and rewards, cat animation, GUI, and music. Keep all window modes in one GUI boundary and music providers in one music boundary initially. These are future design boundaries, not new workspace members.

## Roadmap

| Stage | Work | Tasks |
| --- | --- | --- |
| Foundation | Preserve workspace, define storage, build native shell, complete optional Google auth | [CORE-1](tasks/cat-core/CORE-1.md), [OPENCAT-1](tasks/open-cat/OPENCAT-1.md), [CORE-2](tasks/cat-core/CORE-2.md) |
| Local productivity | Tasks, Pomodoro, daily tracking, rewards, and their views | [CORE-3](tasks/cat-core/CORE-3.md), [CORE-4](tasks/cat-core/CORE-4.md), [OPENCAT-2](tasks/open-cat/OPENCAT-2.md) |
| Connected companion | Linear auth/sync, cat animation, and all window modes | [CORE-3](tasks/cat-core/CORE-3.md), [OPENCAT-3](tasks/open-cat/OPENCAT-3.md) |
| Release, then music | Reliability and footprint checks; YouTube Music feasibility before Spotify | [OPENCAT-4](tasks/open-cat/OPENCAT-4.md) |

The [master specification](CAT_ECOSYSTEM_MASTER_SPEC.md) defines behavior, architecture, and release criteria. Existing task IDs now describe the new product; their old Drive-related scope is superseded.

## Development commands

```bash
cargo check --workspace
cargo test --workspace
cargo run --bin open-cat
```

These commands target the existing packages. Running the binary does not yet launch a GUI. The OAuth scaffold is unfinished; this documentation update does not establish a passing build.
