# OPENCAT-4: Friend Sharing Keys, Release Tuning & Polish

**Epic**: `OPENCAT-EPIC` (The Shared File Vault GUI)  
**Component**: Full App Integration & Optimization  
**Priority**: Medium  
**Story Points**: 3 SP  

---

## 🎯 High-Level Goal & System Behavior
Finalize and package the `open-cat` standalone application:
* **Friend Sharing Key System**: Generate and consume Base64-encoded configuration bundles to grant friends scoped access to a shared `/open-cat/` vault folder.
* **Compiler Optimization Profile**: Configure aggressive Cargo release flags (LTO, size optimization, symbol stripping) to achieve the target `< 35MB RAM` footprint.
* **End-to-End Reliability Audit**: Stress-test edge cases including network dropouts, quota boundary enforcement, and token persistence across restarts.

---

## 🧭 Architectural Milestones
*   [ ] **1. Friend Sharing Key Engine**: Design a Base64URL-encoded configuration format (`ShareKey`) allowing users to export and import vault configurations.
*   [ ] **2. Sharing Modal UI**: Build dialog windows in `egui` for copying the export key to the clipboard and importing friend keys.
*   [ ] **3. Release Profile Configuration**: Configure root `Cargo.toml` with `opt-level = "z"`, `lto = true`, `codegen-units = 1`, and `panic = "abort"` to maximize runtime efficiency and minimize binary size.
*   [ ] **4. Memory & Stability Verification**: Build `--release` and audit memory consumption and recovery behavior under network failure conditions.

---

## 📚 Documentation & Reference
*   **Cargo Profiles Reference**: [https://doc.rust-lang.org/cargo/reference/profiles.html](https://doc.rust-lang.org/cargo/reference/profiles.html)
*   **Minimizing Rust Binary Size**: [https://github.com/johnthagen/min-sized-rust](https://github.com/johnthagen/min-sized-rust)
*   **`base64` Crate**: [https://docs.rs/base64/latest/base64/](https://docs.rs/base64/latest/base64/)