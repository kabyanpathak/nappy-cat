# OPENCAT-4: Friend Sharing Keys, Release Tuning & Polish

**Epic**: `OPENCAT-EPIC` (The Shared File Vault GUI)  
**Component**: Full App Integration & Optimization  
**Priority**: Medium  
**Story Points**: 3 SP  

---

## 🎯 What This Task Accomplishes
Finalizes the `open-cat` standalone application:
* **Friend Sharing Key System**: Export and import Base64-encoded configuration keys scoped strictly to the `/open-cat/` vault folder.
* **Cargo Release Profile Optimization**: Strips symbols, enables Link-Time Optimization (LTO), and optimizes for binary size to keep the footprint strictly `< 35MB RAM`.
* **End-to-End Edge Case Verification**: Validates offline behavior, network drop recovery, and quota boundary limits.

---

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] **1. Sharing Key Serialization**:
    *   Define `pub struct ShareKey { pub folder_id: String, pub access_token: String, pub refresh_token: String }` with `#[derive(Serialize, Deserialize)]`.
    *   Implement export: Serialize `ShareKey` to JSON string and encode with `base64::engine::general_purpose::URL_SAFE_NO_PAD`.
    *   Implement import: Decode Base64 string, deserialize `ShareKey`, and write to local `token.json`.
*   [ ] **2. UI Share Key Modal**:
    *   Add a "Share Vault" button rendering an `egui::Window` modal displaying the exportable key with a "Copy to Clipboard" action.
    *   Add an "Import Key" modal allowing friends to paste a key and immediately switch vaults.
*   [ ] **3. Release Profile Configuration**:
    *   In the root `Cargo.toml`, add aggressive optimization profiles:
        ```toml
        [profile.release]
        opt-level = "z"     # Optimize for size
        lto = true          # Link Time Optimization across all crates
        codegen-units = 1   # Single code generation unit for maximum optimization
        panic = "abort"     # Remove stack unwinding landing pads
        strip = true        # Strip symbols and debuginfo
        ```
*   [ ] **4. Build & Verify Release Binary**:
    *   Run `cargo build --release`.
    *   Check output binary size in `target/release/open-cat`.
*   [ ] **5. Memory & Stability Audit**:
    *   Launch the release build and monitor RAM usage in Activity Monitor / Task Manager (confirm `< 35MB RAM`).
    *   Test edge cases: uploading an oversized file, network disconnect mid-upload, and invalid friend keys.

---

## 🦀 Rust Implementation Guide & Architectural Notes

### 1. Binary Size Optimization (`opt-level = "z"`, `lto = true`)
`lto = true` enables Link-Time Optimization across all crates in the workspace, allowing the compiler to perform dead-code elimination and inline across crate boundaries. `strip = true` automatically removes debug symbols.

---

## 📚 Documentation & Reference Links
*   **Cargo Profiles Guide**: [https://doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html)
*   **Minimizing Rust Binary Size**: [https://github.com/johnthagen/min-sized-rust](https://github.com/johnthagen/min-sized-rust)
*   **`base64` Crate Reference**: [https://docs.rs/base64/latest/base64/](https://docs.rs/base64/latest/base64/)