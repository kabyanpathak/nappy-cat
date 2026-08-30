# OPENCAT-7: Drag-and-Drop Dropzone & Resumable Upload Pipeline

**Epic**: `OPENCAT-EPIC-1`
**Component**: `apps/open-cat` (Upload Subsystem)
**Priority**: High
**Story Points**: 5 SP

## Description
Implement an intuitive drag-and-drop dropzone in the `open-cat` GUI that detects OS-level file drops and triggers the Quota Guard and chunked upload pipeline.

## Acceptance Criteria
- [ ] Dragging files from macOS Finder / Windows Explorer highlights dropzone and captures file paths.
- [ ] Quota check executes before starting upload; rejected files show clear error tooltip.
- [ ] Active uploads display live percentage and progress bar.

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
In an immediate-mode GUI, input events (like dropped files) are polled every frame. Once a file drop is detected, the UI simply captures the file path and hands it off to the background Tokio worker (built in OPENCAT-6) via the `Command` channel.

### How it Works (Logic Flow)
1. **Capturing Drop Events**: Inside `egui`'s `update()` loop, query the input context:
   ```rust
   ctx.input(|i| {
       for file in &i.raw.dropped_files {
           if let Some(path) = &file.path {
               // Send Command::Upload(path) to background worker
           }
       }
   });
   ```
2. **Visual Feedback**: Check `i.raw.hovered_files`. If it's not empty, it means the user is hovering a file over the window. Change the `egui::CentralPanel` background color to create a glowing/dashed "dropzone" effect.
3. **File System Validation**: When the background worker receives the `Command::Upload(path)`, it uses `tokio::fs::metadata(&path)` to get the exact file size.
4. **Triggering Quota Guard**: Before uploading, run the Quota Guard check (OPENCAT-4) using the file size.
5. **Streaming Progress**: As the `reqwest` resumable upload loop pushes chunks to Google Drive, send `Event::UploadProgress(percent)` down the MPSC channel. The UI will catch this and render an `egui::ProgressBar`.

### Documentation & Resources
*   **egui Input Handling**: [https://docs.rs/egui/latest/egui/struct.InputState.html](https://docs.rs/egui/latest/egui/struct.InputState.html)
*   **Tokio FS Metadata**: [https://docs.rs/tokio/latest/tokio/fs/fn.metadata.html](https://docs.rs/tokio/latest/tokio/fs/fn.metadata.html)
