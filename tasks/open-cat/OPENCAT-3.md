# OPENCAT-3: Drag-and-Drop Dropzone & Upload Progress UI

**Epic**: `OPENCAT-EPIC` (The Shared File Vault GUI)  
**Component**: `apps/open-cat` (Upload Subsystem)  
**Priority**: High  
**Story Points**: 5 SP  

---

## 🎯 What This Task Accomplishes
Implements an intuitive drag-and-drop dropzone in the `open-cat` desktop interface. 

It intercepts OS-level file hover and drop events, highlights the UI dropzone with glowing visual cues, validates file size restrictions via `cat_core::validate_upload()`, and renders multi-file upload progress bars with transfer statistics.

---

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] **1. Detect File Hover**:
    *   In `update()`, check `let is_hovering = !ctx.input(|i| i.raw.hovered_files.is_empty());`.
    *   If hovering, style the central panel with a distinctive border and background tint to signify an active dropzone.
*   [ ] **2. Capture File Drops**:
    *   Check `let dropped_files = ctx.input(|i| i.raw.dropped_files.clone());`.
    *   Extract paths from `dropped_files` and send `AppCommand::UploadFile(path)` down the channel.
*   [ ] **3. Fallback File Browser**:
    *   Add a "Browse Files..." button that triggers `rfd::FileDialog::new().pick_files()`.
*   [ ] **4. Worker Quota Validation**:
    *   When the background worker receives `AppCommand::UploadFile(path)`:
        *   Read file size with `tokio::fs::metadata(&path).await?.len()`.
        *   Call `cat_core::validate_upload(&current_quota, file_size)`.
        *   If quota is exceeded, dispatch `AppEvent::Error("5GB Quota Exceeded! Cannot upload file.".into())`.
*   [ ] **5. Resumable Upload & Progress Drawer**:
    *   If valid, call `cat_core::DriveClient::upload_file` passing a progress closure that sends `AppEvent::UploadProgress(pct)`.
    *   Render an animated upload drawer at the bottom of the UI displaying current file name and live progress bar.

---

## 🦀 Rust Implementation Guide & Architectural Notes

### 1. `egui::InputState` Raw Events
`ctx.input(|i| i.raw.dropped_files)` gives access to files dropped from macOS Finder or Windows Explorer during that specific frame.

### 2. Thread-Safe Progress Callbacks
The progress callback passed to `cat-core` must implement `Fn(f32) + Send + 'static` so Tokio can safely invoke it from background worker threads.

---

## 📚 Documentation & Reference Links
*   **`egui::InputState` Reference**: [https://docs.rs/egui/latest/egui/struct.InputState.html](https://docs.rs/egui/latest/egui/struct.InputState.html)
*   **`egui::DroppedFile` Reference**: [https://docs.rs/egui/latest/egui/struct.DroppedFile.html](https://docs.rs/egui/latest/egui/struct.DroppedFile.html)
*   **Tokio Async Filesystem**: [https://docs.rs/tokio/latest/tokio/fs/index.html](https://docs.rs/tokio/latest/tokio/fs/index.html)