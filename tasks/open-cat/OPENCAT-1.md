# OPENCAT-1: GUI Application State & Async Tokio Bridge

**Epic**: `OPENCAT-EPIC` (The Shared File Vault GUI)  
**Component**: `apps/open-cat` (UI Architecture)  
**Priority**: Blocker  
**Story Points**: 5 SP  

---

## 🎯 High-Level Goal & System Behavior
Establish the desktop application window using `eframe`/`egui` and decouple the 60 FPS immediate-mode UI rendering loop from asynchronous network I/O.

Using an Actor-style message passing pattern (MPSC channels), the UI thread stays silky-smooth and responsive while a dedicated background Tokio runtime handles heavy `cat-core` operations (authentication, file downloads, chunked uploads).

---

## 🧭 Architectural Milestones
*   [ ] **1. Message Protocol Definition**: Design strongly typed `AppCommand` (UI $ightarrow$ Worker) and `AppEvent` (Worker $ightarrow$ UI) enums to establish a clean boundary between UI rendering and network I/O.
*   [ ] **2. Tokio Worker Runtime Isolation**: Spawn a background thread initializing a dedicated multi-threaded Tokio runtime that processes `cat-core` operations asynchronously.
*   [ ] **3. Immediate-Mode UI Lifecycle (`eframe::App`)**: Implement the `eframe::App` state loop to non-blockingly drain event channels (`try_recv`), render UI panels, and signal immediate repaints on incoming events.
*   [ ] **4. Error & Notification Drawer**: Build a toast/banner notification layer in the UI to display async errors and network statuses cleanly.

---

## 🔒 UI & Concurrency Invariants
*   **60 FPS Guarantee**: Never perform blocking I/O, file reading, or `.await` calls inside `eframe::App::update()`.
*   **Decoupled State**: The GUI layer must never talk to Google Drive directly; it only dispatches commands and reacts to events.

---

## 📚 Documentation & Reference
*   **`egui` Framework**: [https://docs.rs/egui/latest/egui/](https://docs.rs/egui/latest/egui/)
*   **`eframe` Desktop Runner**: [https://docs.rs/eframe/latest/eframe/](https://docs.rs/eframe/latest/eframe/)
*   **Tokio MPSC Channels**: [https://tokio.rs/tokio/tutorial/channels](https://tokio.rs/tokio/tutorial/channels)