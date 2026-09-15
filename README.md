# Nappy Cat

> Your all-in-one productivity game: a small native desktop companion, built in Rust.

Nappy Cat combines a Pomodoro timer, daily app-time tracking, a task list, and an animated cat. Focus, finish tasks, and unlock cosmetic rewards such as cat skins. Keep the cat beside your work, open a compact popup, or switch to full screen.

The philosophy stays simple: minimal UI, a small footprint, and quiet background operation. All application code is Rust, with a native GUI, no embedded browser or web frontend, and browser use only for authentication. The finalized app targets **less than 100 MB of runtime memory**; package size and CPU usage will also be measured. This is a target, not a verified result.

## Product features

- Pomodoro focus and break sessions with pause, resume, and reset.
- Local tasks that work without an account or internet connection.
- Daily time spent running Nappy Cat, with focus time shown separately.
- An animated cat with cosmetic unlocks.
- Always-on-top cat, compact popup, and full-screen modes sharing one app state.
- Optional Google OAuth 2.0 sign-in, retained from the original direction.
- Optional Linear connection: view linked tasks and create or update them from Nappy Cat.

Music is a high-priority productivity feature. Build Pomodoro first, then tasks with Linear, optionally a quick standalone local to-do list, then YouTube Music. Add a substantial first pass of cat features and other extras before Spotify, once the app is more presentable; finish the remaining extras afterward. Native playback capabilities and account requirements must be validated before promising either integration; Google sign-in alone does not establish music access. Gmail and other integrations are later possibilities.

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

## Future file structure goal

The [future workspace tree](CAT_ECOSYSTEM_MASTER_SPEC.md#future-file-structure-goal) shows the intended service crates, shared GUI modes, and music provider modules. Extract these gradually as features grow; the current two-crate structure stays in place for now.

## Roadmap

| Stage | Work | Tasks |
| --- | --- | --- |
| Foundation | Preserve workspace, define minimal storage, build native shell; retain Google auth work alongside relevant features | [CORE-1](tasks/cat-core/CORE-1.md), [OPENCAT-1](tasks/open-cat/OPENCAT-1.md), [CORE-2](tasks/cat-core/CORE-2.md) |
| 1. Pomodoro | Working timer and minimal native controls | [CORE-4 A](tasks/cat-core/CORE-4.md), [OPENCAT-2 A](tasks/open-cat/OPENCAT-2.md) |
| 2. Tasks with Linear | Task model, Linear auth/sync, and native task controls | [CORE-3 A/B](tasks/cat-core/CORE-3.md), [OPENCAT-2 B](tasks/open-cat/OPENCAT-2.md) |
| 3. Optional simple to-do | Small local-only list, built directly or adapted from a suitable Rust project; skip if unnecessary | [CORE-3 C](tasks/cat-core/CORE-3.md), [OPENCAT-2 C](tasks/open-cat/OPENCAT-2.md) |
| 4. YouTube Music | Validate and implement supported native music functionality | [OPENCAT-4 A](tasks/open-cat/OPENCAT-4.md) |
| 5. First extras | Daily tracking, initial rewards, animated cat, and a presentable companion UI | [CORE-4 B](tasks/cat-core/CORE-4.md), [OPENCAT-3 A](tasks/open-cat/OPENCAT-3.md) |
| 6. Spotify | Add the second music provider once the app is more presentable | [OPENCAT-4 B](tasks/open-cat/OPENCAT-4.md) |
| 7. Remaining extras | Finish window modes, skins, polish, and consider further features | [OPENCAT-3 B](tasks/open-cat/OPENCAT-3.md), [OPENCAT-4 C](tasks/open-cat/OPENCAT-4.md) |

Reliability and footprint checks run throughout; a finished game or final release is not a prerequisite for YouTube Music.

The [master specification](CAT_ECOSYSTEM_MASTER_SPEC.md) defines behavior, architecture, and release criteria. Existing task IDs now describe the new product; their old Drive-related scope is superseded.

## Development commands

```bash
cargo check --workspace
cargo test --workspace
cargo run --bin open-cat
```

These commands target the existing packages. Running the binary does not yet launch a GUI. The OAuth scaffold is unfinished; this documentation update does not establish a passing build.
