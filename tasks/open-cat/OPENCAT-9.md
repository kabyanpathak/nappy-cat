# OPENCAT-9: End-to-End Integration, Friend Sharing Keys & Verification

**Epic**: `OPENCAT-EPIC-1`
**Component**: Full Open-Cat App
**Priority**: Medium
**Story Points**: 3 SP

## Description
Finalize end-to-end integration across `crates/cat-core`, `apps/cat-daemon`, and `apps/open-cat`. Implement friend folder sharing key export/import and verify system stability, memory footprint, and edge cases.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. Define `struct ShareKey { folder_id: String, token: String }` and derive `Serialize, Deserialize`.
*   [ ] 2. Use `serde_json::to_string` to convert it to a string, then use the `base64` crate to encode it so it looks like a clean, shareable string.
*   [ ] 3. Create a text box in your UI where friends can paste this base64 string. Decode it and save it to their local system.
*   [ ] 4. Open the root `Cargo.toml` and add a `[profile.release]` section setting `opt-level = "z"` and `lto = true` to aggressively shrink memory.
*   [ ] 5. Run `cargo build --release`.
*   [ ] 6. Do a final test: Check your Activity Monitor/Task Manager. Is `open-cat` using under 35MB of RAM? Does the quota block uploads over 5GB?

## Acceptance Criteria
- [ ] Friend configuration key loads remote folder and enforces 5GB quota seamlessly.
- [ ] Total memory footprint stays strictly within low-memory ecosystem targets (< 35MB).
- [ ] End-to-end user workflow functions smoothly (Auth -> Dropzone -> Quota -> Upload -> Stream).

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
To share a specific `/open-cat/` vault with friends without giving them full access to your Google Drive, you can generate a specific "Sharing Key". Furthermore, Rust's memory performance shines when compiled properly in release mode; doing a final pass over binary size and memory guarantees the "low-footprint" principle of the Cat Ecosystem.

### How it Works (Logic Flow)
1. **Friend Sharing Keys**:
   - Create a struct: `struct ShareKey { folder_id: String, access_token: String, refresh_token: String }`.
   - Serialize it to JSON, then Base64 encode the string using the `base64` crate.
   - When a friend pastes this key into `open-cat`, decode it, save to `token.json`, and set the target Drive ID. 
2. **Memory & Binary Optimization**:
   - Add this to the root `Cargo.toml` to heavily optimize memory and binary size for the production build:
     ```toml
     [profile.release]
     opt-level = "z"     # Optimize for size
     lto = true          # Link Time Optimization
     codegen-units = 1   # Maximum optimization context
     panic = "abort"     # Remove panic unwinding tables
     ```
3. **Edge Case Verification**:
   - Manually verify that deleting files accurately decrements the quota limit logic.
   - Kill your internet connection mid-upload to ensure `reqwest` timeouts are handled by `thiserror` without crashing the GUI.

### Documentation & Resources
*   **Base64 Crate**: [https://docs.rs/base64/latest/base64/](https://docs.rs/base64/latest/base64/)
*   **Cargo Profiles for Release**: [https://doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html)
