# OPENCAT-3: Drag-and-Drop Dropzone & Upload Progress UI

**Epic**: `OPENCAT-EPIC` (The Shared File Vault GUI)  
**Component**: `apps/open-cat` (Upload Subsystem)  
**Priority**: High  
**Story Points**: 5 SP  

---

## 🎯 High-Level Goal & System Behavior
Implement an intuitive drag-and-drop upload dropzone in the `open-cat` GUI. 

The interface intercepts OS-level file drops from Finder or Explorer, provides visual feedback during hover, pre-validates files against `cat-core` quota rules, and renders multi-file transfer progress drawers with real-time transfer percentages.

---

## 🧭 Architectural Milestones
*   [ ] **1. Dropzone Event Interceptor**: Listen for OS-level hovered and dropped file events, applying dynamic styling (glowing borders / background tint) when files hover over the window.
*   [ ] **2. File Picker Fallback**: Provide a "Browse Files..." action using `rfd::FileDialog` as an alternative to drag-and-drop.
*   [ ] **3. Pre-Flight Quota Validation**: Dispatch dropped files to the worker to execute `cat_core::validate_upload()`, surfacing quota violation errors immediately if the file exceeds the 5GB cap.
*   [ ] **4. Live Progress Drawer**: Render a real-time progress bar displaying uploaded percentage and file name as chunks stream to Google Drive.

---

## 📚 Documentation & Reference
*   **`egui::InputState`**: [https://docs.rs/egui/latest/egui/struct.InputState.html](https://docs.rs/egui/latest/egui/struct.InputState.html)
*   **`egui::DroppedFile`**: [https://docs.rs/egui/latest/egui/struct.DroppedFile.html](https://docs.rs/egui/latest/egui/struct.DroppedFile.html)