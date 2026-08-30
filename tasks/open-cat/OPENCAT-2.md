# OPENCAT-2: Vault File Grid, 5GB Visual Gauge & Media Launcher

**Epic**: `OPENCAT-EPIC` (The Shared File Vault GUI)  
**Component**: `apps/open-cat` (UI Views & Actions)  
**Priority**: High  
**Story Points**: 5 SP  

---

## 🎯 What This Task Accomplishes
Builds the core interactive file explorer inside `open-cat`:
* Visual **5GB Storage Gauge** with dynamic color coding (Green / Amber / Red).
* Interactive **File Grid & Table View** displaying file names, formatted sizes (KB/MB/GB), MIME badges, and upload dates.
* Native OS **Download Dialog** (`rfd` crate).
* **One-Click Media Launcher**: Calls `cat-core`'s temp-buffering downloader and launches VLC/QuickTime pointing to the buffered file.

---

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] **1. 5GB Visual Storage Meter**:
    *   In `update()`, render an `egui::ProgressBar` using `quota.used_bytes as f32 / quota.max_bytes as f32`.
    *   Apply conditional color accents: Green (`< 70%`), Amber (`70% - 90%`), Red (`> 90%`).
*   [ ] **2. File Grid / Table View**:
    *   Use `egui_extras::TableBuilder` to render a responsive multi-column table (Icon, Name, Size, Date, Actions).
    *   Format file sizes cleanly with helper: `fn format_bytes(bytes: u64) -> String`.
*   [ ] **3. Download Action**:
    *   Add a "Download" button per row.
    *   When clicked, invoke `rfd::FileDialog::new().set_file_name(&file.name).save_file()`.
    *   If a destination path is chosen, dispatch `self.cmd_tx.send(AppCommand::DownloadFile(file.id.clone(), target_path))`.
*   [ ] **4. Native Media Playback Action**:
    *   If `file.mime_type.starts_with("video/")` or `audio/`, display a "Play Video" button.
    *   When clicked, dispatch `self.cmd_tx.send(AppCommand::PlayMedia(file.clone()))`.
    *   The Tokio worker calls `cat_core::buffer_to_temp()`, emits buffering percentage events, and upon completion executes `open::that(&temp_path)`.
*   [ ] **5. Refresh & Status Actions**:
    *   Add a "Refresh" button triggering `AppCommand::RefreshFiles`.

---

## 🦀 Rust Implementation Guide & Architectural Notes

### 1. Launching Native Applications Detached
Using `open::that(&path)` asks the operating system (macOS Finder / Windows Shell) to launch the user's default handler for that file extension (e.g., VLC for `.mp4`). This spawns as an independent OS process without blocking `open-cat`.

### 2. Formatting File Sizes
Use clean integer arithmetic: `< 1024` -> B, `< 1024*1024` -> KB, `< 1024*1024*1024` -> MB, else GB.

---

## 📚 Documentation & Reference Links
*   **`egui_extras::TableBuilder`**: [https://docs.rs/egui_extras/latest/egui_extras/struct.TableBuilder.html](https://docs.rs/egui_extras/latest/egui_extras/struct.TableBuilder.html)
*   **`rfd` (Rust File Dialog) Crate**: [https://docs.rs/rfd/latest/rfd/](https://docs.rs/rfd/latest/rfd/)
*   **`open` Crate Reference**: [https://docs.rs/open/latest/open/](https://docs.rs/open/latest/open/)