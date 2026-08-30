# OPENCAT-2: Vault Grid UI & Native Media Launcher

**Epic**: `OPENCAT-EPIC`
**Component**: `open-cat` (UI layer)
**Priority**: High

## Description
Build the visual file explorer. It consumes the `DriveFile` models from `cat-core` and uses `cat-core`'s temp-buffering method to launch VLC.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. Draw a `cat_core::QuotaInfo` progress bar using `egui::ProgressBar`.
*   [ ] 2. Render the file list using `egui_extras::TableBuilder`. 
*   [ ] 3. When a user clicks "Play", send `AppCommand::PlayMedia` to the worker.
*   [ ] 4. The worker calls `cat_core::buffer_to_temp(file).await`.
*   [ ] 5. When the worker finishes buffering, it uses the `open` crate to launch the native media player.