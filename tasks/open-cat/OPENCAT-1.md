# OPENCAT-1: GUI Application State & Async Tokio Bridge

**Epic**: `OPENCAT-EPIC` (The Shared File Vault GUI)  
**Component**: `apps/open-cat` (UI Architecture)  
**Priority**: Blocker  
**Story Points**: 5 SP  

---

## 🎯 What This Task Accomplishes
Scaffolds the native desktop GUI window using `eframe` and `egui`. 

It establishes an asynchronous messaging bridge using `tokio::sync::mpsc` channels. Because `egui` runs in an immediate-mode 60 FPS loop, performing blocking network operations inside the UI thread would freeze the window. This bridge decouples UI rendering from Tokio background workers that execute `cat-core` functions.

---

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] **1. Dependencies**: Add `cat-core = { path = "../../crates/cat-core" }`, `eframe = { workspace = true }`, and `egui = { workspace = true }` to `apps/open-cat/Cargo.toml`.
*   [ ] **2. Command & Event Message Enums**:
    *   Define `pub enum AppCommand` (messages sent from GUI to worker): `Authenticate`, `RefreshFiles`, `UploadFile(PathBuf)`, `DownloadFile(String, PathBuf)`, `PlayMedia(DriveFile)`.
    *   Define `pub enum AppEvent` (messages sent from worker to GUI): `Authenticated`, `FilesLoaded(Vec<DriveFile>)`, `QuotaUpdated(QuotaInfo)`, `UploadProgress(f32)`, `BufferingProgress(f32)`, `Error(String)`.
*   [ ] **3. Spawn Dedicated Tokio Worker Thread**:
    *   In `main.rs`, create unbounded MPSC channels: `let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::unbounded_channel();` and `let (evt_tx, evt_rx) = tokio::sync::mpsc::unbounded_channel();`.
    *   Spawn a background OS thread (`std::thread::spawn`) that initializes a multi-threaded Tokio runtime:
        ```rust
        tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap().block_on(async move { ... });
        ```
*   [ ] **4. Worker Command Loop**:
    *   Inside the worker, loop over `cmd_rx.recv().await`.
    *   Match on commands and dispatch calls to `cat_core::DriveClient`, sending `AppEvent` updates back to the UI.
*   [ ] **5. Implement `eframe::App`**:
    *   Define `pub struct OpenCatApp` holding `cmd_tx`, `evt_rx`, `files: Vec<DriveFile>`, `quota: Option<QuotaInfo>`, `upload_progress: Option<f32>`.
    *   In `eframe::App::update(&mut self, ctx, _frame)`:
        *   Drain `self.evt_rx.try_recv()` to update UI state without blocking.
        *   Render top bar, main canvas, and status footer.
*   [ ] **6. Repaint Signals**: Whenever the worker transmits an `AppEvent`, invoke `ctx.request_repaint()` to trigger an immediate frame redraw.

---

## 🦀 Rust Implementation Guide & Architectural Notes

### 1. The Immediate-Mode UI Lifecycle
In `egui`, the `update()` function runs 60 times per second. You cannot `.await` anything inside `update()`. The non-blocking `try_recv()` pattern allows the UI to poll for background updates instantly each frame.

### 2. Decoupled Actor Architecture
The UI thread knows nothing about HTTP sockets or Google Drive REST endpoints. It only knows how to send `AppCommand` and display `AppEvent`. All networking lives safely inside the Tokio thread pool.

---

## 📚 Documentation & Reference Links
*   **`egui` Documentation**: [https://docs.rs/egui/latest/egui/](https://docs.rs/egui/latest/egui/)
*   **`eframe` Desktop Framework**: [https://docs.rs/eframe/latest/eframe/](https://docs.rs/eframe/latest/eframe/)
*   **Tokio MPSC Channels Tutorial**: [https://tokio.rs/tokio/tutorial/channels](https://tokio.rs/tokio/tutorial/channels)
*   **Tokio Multi-Thread Runtime Builder**: [https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html](https://docs.rs/tokio/latest/tokio/runtime/struct.Builder.html)