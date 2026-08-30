# CORE-1: Workspace Initialization & Core Scaffolding

**Epic**: `CAT-CORE-EPIC` (The Shared Foundation)  
**Component**: Workspace Root & `crates/cat-core`  
**Priority**: Blocker  
**Story Points**: 2 SP  
**Status**: ✅ DONE  

---

## 🎯 High-Level Goal & System Behavior
Transform the project into a **Cargo Workspace (Virtual Manifest Monorepo)** to establish a unified dependency graph and build cache. `cat-core` acts as the shared systems engine (SDK), while `open-cat` and future services act as thin client consumers.

---

## 🧭 Architectural Milestones
*   [x] **1. Monorepo Scaffolding**: Establish the multi-crate workspace structure separating libraries (`crates/`) from applications (`apps/`).
*   [x] **2. Virtual Manifest & Dependency Centralization**: Configure the root `Cargo.toml` to manage dependency versions centrally, avoiding duplicate compilation units.
*   [x] **3. Member Package Configuration**: Configure `cat-core` (library crate) and `open-cat` (binary crate) with workspace dependency inheritance and path linkage.
*   [x] **4. Build & Cache Verification**: Ensure `cargo check --workspace` passes cleanly and generates a single shared `Cargo.lock`.

---

## 📚 Documentation & Reference
*   **Cargo Workspaces**: [https://doc.rust-lang.org/cargo/reference/workspaces.html](https://doc.rust-lang.org/cargo/reference/workspaces.html)
*   **Dependency Inheritance**: [https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table)