# OPENCAT-8: Vault File Grid, 5GB Visual Gauge & VLC Stream Launcher

**Epic**: `OPENCAT-EPIC-1`
**Component**: `apps/open-cat` (File Explorer & Actions)
**Priority**: High
**Story Points**: 5 SP

## Description
Build the interactive file explorer inside `open-cat`, featuring a visual 5GB storage gauge, file cards/table with MIME-type icons, file download dialogs, and one-click VLC video streaming.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. In `update()`, draw a progress bar for the quota: `ui.add(egui::ProgressBar::new(used / max).text("Storage"));`.
*   [ ] 2. Draw your file list using `egui::ScrollArea::vertical().show(ui, |ui| { ... })`. Iterate over your `self.files` vector.
*   [ ] 3. Next to each file in the UI, add an `if ui.button("Download").clicked() { ... }`. Trigger the `rfd` crate to open a native OS "Save As" dialog.
*   [ ] 4. Next to video files, add `if ui.button("Play in VLC").clicked() { ... }`.
*   [ ] 5. For the VLC button, format the local proxy URL: `let url = format!("http://127.0.0.1:4040/stream/{}", file.id);`.
*   [ ] 6. Use `open::that(url)` or `std::process::Command::new("vlc").arg(url).spawn()` to launch the media player detached from your application.

## Acceptance Criteria
- [ ] Storage gauge accurately reflects remote `_meta/quota.json`.
- [ ] Clicking "Download" writes complete file to chosen local path.
- [ ] Clicking "Play in VLC" opens default media player streaming from local daemon without full download.

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
This task combines UI layout logic with system integration. The file list is stored in your UI state (`self.files: Vec<DriveFile>`). You render it dynamically in a `ScrollArea`. When an action button is clicked, you either spawn a system process (for streaming) or dispatch a background task (for downloading).

### How it Works (Logic Flow)
1. **Visual Gauge**: Read your cached `QuotaInfo`. Render an `egui::ProgressBar::new(used_bytes / max_bytes)`. You can style it using `.fill(egui::Color32::GREEN)` transitioning to `RED` if $> 90\%$ full.
2. **Grid / Table Rendering**: Use `egui_extras::TableBuilder` to create clean rows showing File Name, Size, Date, and Actions. 
3. **Downloading Files**: When a user clicks "Download", invoke the `rfd` (Rust File Dialog) crate to prompt the OS native "Save As" window. Once the path is selected, send `Command::Download(file_id, target_path)` to the async worker.
4. **VLC Stream Launcher**: When a user clicks "Play in VLC" for an MP4/MKV file:
   - Construct the local proxy URL: `http://127.0.0.1:4040/stream/{file_id}`.
   - Use the `open` crate: `open::that(url)`. This acts like typing `open url` on macOS or `start url` on Windows, which automatically spawns the user's default media player as a detached background process.

### Documentation & Resources
*   **egui_extras TableBuilder**: [https://docs.rs/egui_extras/latest/egui_extras/struct.TableBuilder.html](https://docs.rs/egui_extras/latest/egui_extras/struct.TableBuilder.html)
*   **RFD (Native File Dialogs)**: [https://docs.rs/rfd/latest/rfd/](https://docs.rs/rfd/latest/rfd/)
*   **Open Crate (Detached Processes)**: [https://docs.rs/open/latest/open/](https://docs.rs/open/latest/open/)
