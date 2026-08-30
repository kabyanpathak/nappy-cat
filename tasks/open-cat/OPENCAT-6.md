# OPENCAT-6: eframe / egui Core Window Layout & Async MPSC Bridge

**Epic**: `OPENCAT-EPIC-1`
**Component**: `apps/open-cat` (UI Architecture)
**Priority**: High
**Story Points**: 5 SP

## Description
Scaffold the native desktop GUI using `eframe` and `egui`. Establish an asynchronous messaging pipeline using `tokio::sync::mpsc` channels so network requests (listing, downloading, uploading) run on Tokio background threads without blocking the 60 FPS UI loop.

## Acceptance Criteria
- [ ] UI stays perfectly responsive (60 FPS) during long-running network operations.
- [ ] Background worker handles commands asynchronously and updates UI via event queue.
- [ ] Clean error banners render when network or quota errors occur.

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
`egui` (via `eframe`) is an **Immediate Mode GUI**. Its `update()` loop runs at 60 frames per second. You cannot use `.await` inside the `update()` loop, otherwise the entire UI will freeze while waiting for network responses.

To solve the "Two-Runtime Problem" (UI blocking vs Async Network), you must use the Actor pattern with message passing (MPSC: Multi-Producer, Single-Consumer).

### How it Works (Logic Flow)
1. **Message Enums**: Define a `Command` enum (sent from UI to Worker) and an `Event` enum (sent from Worker back to UI).
   - `enum Command { Upload(PathBuf), Delete(String) }`
   - `enum Event { UploadProgress(f32), FilesLoaded(Vec<File>) }`
2. **Channel Creation**: Setup `tokio::sync::mpsc` channels before launching `eframe`.
3. **Tokio Runtime Spawning**: Spawn a standard `std::thread::spawn` which internally builds a `tokio::runtime::Builder::new_multi_thread().enable_all().build()`. This thread loops over the `Command` channel receiver, executing network requests, and sends results back via the `Event` channel sender.
4. **UI Update Loop**: Inside `eframe::App::update(&mut self, ctx)`, you call `self.event_rx.try_recv()`. If there's a new event, you update your UI state (like progress bars or file lists). If empty, just draw the current state.
5. **Context Repaint**: When the worker thread sends an event, it should call `ctx.request_repaint()` so `egui` knows to instantly draw the new data instead of sleeping.

### Documentation & Resources
*   **eframe / egui**: [https://docs.rs/eframe/latest/eframe/](https://docs.rs/eframe/latest/eframe/)
*   **Tokio MPSC Channels**: [https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html](https://docs.rs/tokio/latest/tokio/sync/mpsc/index.html)
