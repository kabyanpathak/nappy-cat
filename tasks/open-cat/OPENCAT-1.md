# OPENCAT-1: Cargo Workspace & Multi-Crate Architecture Setup

**Epic**: `OPENCAT-EPIC-1`
**Component**: Workspace Root
**Priority**: High
**Story Points**: 2 SP

## Description
Initialize the Cargo workspace structure defined in Section 3 of `CAT_ECOSYSTEM_MASTER_SPEC.md`. Configure workspace-level dependency inheritance so all ecosystem crates share identical versions of `tokio`, `serde`, `reqwest`, and `eframe`.

## Acceptance Criteria
- [ ] Running `cargo check --workspace` compiles all crates cleanly without warnings.
- [ ] `cat-core` is consumable as a path dependency in both `apps/open-cat` and `apps/cat-daemon`.

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
Rust's `[workspace]` feature allows multiple crates (packages) to share the same `Cargo.lock` and output `target/` directory. This is crucial for the Cat Ecosystem. By using workspace inheritance, `open-cat` (the GUI) and `cat-daemon` (the server) will use the exact same versions of heavy libraries like `tokio` or `reqwest`. This prevents "diamond dependency" issues where Cargo accidentally compiles two different versions of the same library, which would bloat your executable size and memory footprint.

### How it Works (Logic Flow)
1. **The Root Manifest**: Your root `Cargo.toml` will act strictly as a workspace manifest, not a crate. It will not have a `[package]` section.
2. **Dependency Dictionary**: You define `[workspace.dependencies]` in the root. 
3. **Crate Manifests**: Inside `crates/cat-core/Cargo.toml`, instead of defining a version for `tokio`, you simply write `tokio = { workspace = true }`.

### Documentation & Resources
*   **Cargo Workspaces**: [https://doc.rust-lang.org/cargo/reference/workspaces.html](https://doc.rust-lang.org/cargo/reference/workspaces.html)
*   **Workspace Inheritance**: [https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table](https://doc.rust-lang.org/cargo/reference/workspaces.html#the-dependencies-table)
