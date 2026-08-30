# CORE-1: Cargo Workspace & Multi-Crate Architecture Setup

**Epic**: `CAT-CORE-EPIC` (The Shared Foundation)  
**Component**: Workspace Root & `crates/cat-core`  
**Priority**: Blocker  
**Story Points**: 2 SP  

---

## 🎯 What This Task Accomplishes
This task converts the repository from a single, isolated package into a **Cargo Workspace** (Virtual Manifest Monorepo). 

In the Cat Ecosystem, `cat-core` is the central "Thick SDK" holding all Google Drive, OAuth, and Quota logic, while apps like `open-cat` are thin GUI clients that consume `cat-core`. By establishing workspace inheritance, all crates share identical versions of `tokio`, `serde`, and `reqwest` without duplicate compilation overhead or version mismatch conflicts.

---

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] **1. Folder Structure**: Create the root layout: `crates/cat-core/src/` and `apps/open-cat/src/`.
*   [ ] **2. Entry Points**:
    *   Create `crates/cat-core/src/lib.rs` (the shared library entry point) with `pub fn init() {}`.
    *   Create `apps/open-cat/src/main.rs` (the binary GUI entry point) with `fn main() {}`.
*   [ ] **3. Root Workspace Manifest**: Replace root `Cargo.toml` with a virtual `[workspace]` declaring members `["crates/cat-core", "apps/open-cat"]` and `resolver = "2"`.
*   [ ] **4. Centralize Dependencies (`[workspace.dependencies]`)**: Define common crates and feature flags in the root `Cargo.toml` (`tokio`, `serde`, `serde_json`, `reqwest`, `thiserror`, `sha2`, `base64`, `open`, `eframe`, `egui`, `rfd`).
*   [ ] **5. Configure Member Manifests**:
    *   Create `crates/cat-core/Cargo.toml` with `name = "cat-core"` and dependencies using `{ workspace = true }`.
    *   Create `apps/open-cat/Cargo.toml` with `name = "open-cat"`, pulling `cat-core = { path = "../../crates/cat-core" }`.
*   [ ] **6. Build Verification**: Run `cargo check --workspace` to ensure Cargo generates a clean `Cargo.lock` and compiles all crates without warnings.

---

## 🦀 Rust Implementation Guide & Architectural Notes

### 1. Virtual Manifest vs Package Manifest
A standard `Cargo.toml` has a `[package]` section. A **virtual workspace root** does **not** have a `[package]` section because it is not a crate itself; it is an orchestrator that unifies compilation units and build artifacts in a single `/target` folder.

### 2. Workspace Inheritance
Instead of repeating `tokio = { version = "1.37", features = ["full"] }` in every crate, you define it once in `[workspace.dependencies]` at the root. In child crates, you simply write `tokio = { workspace = true }`.

---

## 📚 Documentation & Reference Links
*   **Cargo Workspaces Guide**: [https://doc.rust-lang.org/cargo/reference/workspaces.html](https://doc.rust-lang.org/cargo/reference/workspaces.html)
*   **Workspace Dependency Inheritance**: [https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table)
*   **Cargo Dependency Specification**: [https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html)