# OPENCAT-2: Vault File Grid, 5GB Visual Gauge & Media Launcher

**Epic**: `OPENCAT-EPIC` (The Shared File Vault GUI)  
**Component**: `apps/open-cat` (UI Views & Actions)  
**Priority**: High  
**Story Points**: 5 SP  

---

## 🎯 High-Level Goal & System Behavior
Build the primary user-facing file explorer for the vault:
* A visual **5GB Storage Meter** reflecting remote quota usage with dynamic warning colors.
* A structured **File Table/Grid** displaying vault contents with formatted file sizes, MIME badges, and action buttons.
* Native OS **File Save Dialog** (`rfd`) for downloads.
* **Native Media Launcher**: Triggers `cat-core`'s temp-buffer pipeline and executes the OS default media player (VLC/QuickTime) against the local file.

---

## 🧭 Architectural Milestones
*   [ ] **1. Visual Quota Gauge**: Render an `egui::ProgressBar` displaying used vs. available quota, styled with conditional threshold colors (Green $ightarrow$ Amber $ightarrow$ Red).
*   [ ] **2. Responsive File Table**: Build a multi-column table (`egui_extras::TableBuilder`) to display file names, formatted byte sizes, dates, and action controls.
*   [ ] **3. Native Download Integration**: Hook table download actions to native OS save dialogs (`rfd`) and dispatch background download streams.
*   [ ] **4. Native Media Playback Flow**: Connect media actions to `cat-core`'s temp-buffering downloader, display a buffering indicator in the UI, and launch the media player detached via the `open` crate.

---

## 📚 Documentation & Reference
*   **`egui_extras::TableBuilder`**: [https://docs.rs/egui_extras/latest/egui_extras/struct.TableBuilder.html](https://docs.rs/egui_extras/latest/egui_extras/struct.TableBuilder.html)
*   **`rfd` (Rust File Dialog)**: [https://docs.rs/rfd/latest/rfd/](https://docs.rs/rfd/latest/rfd/)
*   **`open` Crate**: [https://docs.rs/open/latest/open/](https://docs.rs/open/latest/open/)