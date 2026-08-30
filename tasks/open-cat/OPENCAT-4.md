# OPENCAT-4: Friend Sharing Keys & Polish

**Epic**: `OPENCAT-EPIC`
**Component**: `open-cat` (UI layer)
**Priority**: Medium

## Description
Implement the UI text field to paste a base64 friend key, decoding it, and passing the credentials into `cat-core`'s initialization function.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. Create a modal window in `egui` with a `TextEdit` box for the sharing key.
*   [ ] 2. When submitted, decode the string and pass it to `cat_core::DriveClient::from_share_key()`.
*   [ ] 3. Ensure the UI dynamically switches to view the friend's vault instead of your own.
*   [ ] 4. Run memory profiling to ensure the GUI remains under 35MB of RAM footprint.