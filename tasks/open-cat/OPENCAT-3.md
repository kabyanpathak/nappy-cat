# OPENCAT-3: Dropzone & Upload Progress UI

**Epic**: `OPENCAT-EPIC`
**Component**: `open-cat` (UI layer)
**Priority**: High

## Description
A UI wrapper for the drag-and-drop feature. It detects drops and calls `cat-core`'s robust upload functions.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. In `update()`, check `ctx.input(|i| i.raw.dropped_files)`.
*   [ ] 2. Extract the file paths and send them to the worker via `AppCommand::Upload(path)`.
*   [ ] 3. The worker calls `cat_core::validate_upload()` first. If it fails, return an `AppEvent::Error` to the UI.
*   [ ] 4. If valid, the worker calls `cat_core::upload_file()` and sends progress percentage updates back to the UI to draw a loading bar.