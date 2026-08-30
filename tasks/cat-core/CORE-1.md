# CORE-1: Workspace Initialization & Core Scaffolding

**Epic**: `CAT-CORE-EPIC`
**Component**: `cat-core`
**Priority**: Blocker

## Description
Set up the Cargo workspace and scaffold the `cat-core` library. This library will act as the "SDK" for the entire Cat Ecosystem, containing all the shared structs, constants, and foundational tools.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. Create a `crates/cat-core` folder and an `apps/open-cat` folder.
*   [ ] 2. Inside `cat-core`, create a `src` folder and an empty `lib.rs`.
*   [ ] 3. Create the root `Cargo.toml`. Add `[workspace]` and `members = ["crates/cat-core", "apps/open-cat"]`.
*   [ ] 4. In the root `Cargo.toml`, add `[workspace.dependencies]` defining `tokio`, `serde`, and `reqwest`.
*   [ ] 5. In `cat-core/Cargo.toml`, add `tokio = { workspace = true }`, etc.
*   [ ] 6. Run `cargo check --workspace`.