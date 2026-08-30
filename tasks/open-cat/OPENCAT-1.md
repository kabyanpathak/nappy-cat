# OPENCAT-1: GUI Application State & Tokio Bridge

**Epic**: `OPENCAT-EPIC`
**Component**: `open-cat` (UI layer)
**Priority**: Blocker

## Description
Create the thin GUI layer for `open-cat` using `eframe`. All heavy lifting is offloaded to `cat-core`. Setup the Tokio MPSC channel to communicate between the UI thread and the background worker.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. In `open-cat/Cargo.toml`, add `cat-core = { path = "../crates/cat-core" }` as a dependency.
*   [ ] 2. Define `AppCommand` and `AppEvent` enums to pass messages (e.g., `AppCommand::FetchFiles`).
*   [ ] 3. Spawn a background thread running a Tokio runtime.
*   [ ] 4. In the background thread, instantiate `let client = cat_core::DriveClient::new().await;`.
*   [ ] 5. Implement `eframe::App`. In `update()`, check the MPSC receiver for events from the background worker.