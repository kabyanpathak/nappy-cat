# CORE-4: Ecosystem Rules & 5GB Quota Guard Engine

**Epic**: `CAT-CORE-EPIC` (The Shared Foundation)  
**Component**: `crates/cat-core` (Business Logic & Quota Guard)  
**Priority**: High  
**Story Points**: 3 SP  

---

## 🎯 What This Task Accomplishes
Implements the software-enforced 5GB quota guard system. The guard tracks cumulative storage usage in `_meta/quota.json` on Google Drive and blocks uploads before byte transfer if the 5GB cap is exceeded.

It also provides functional iterator utilities to audit, recalculate, and synchronize quota states across all ecosystem apps.

---

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] **1. Create Module**: Create `crates/cat-core/src/quota.rs` and declare `pub mod quota;` in `lib.rs`.
*   [ ] **2. Quota Schema**:
    *   Define `pub struct QuotaInfo { pub max_bytes: u64, pub used_bytes: u64, pub file_count: u64, pub last_synced_utc: String }` with `#[derive(Serialize, Deserialize, Clone, Debug)]`.
    *   Define constant `pub const FIVE_GIGABYTES: u64 = 5 * 1024 * 1024 * 1024;` (5,368,709,120 bytes).
*   [ ] **3. Pre-Flight Quota Validator**:
    *   Write `pub fn validate_upload(quota: &QuotaInfo, incoming_file_bytes: u64) -> Result<(), CoreError>`.
    *   Formula: If `quota.used_bytes + incoming_file_bytes > quota.max_bytes`, immediately return `Err(CoreError::QuotaExceeded)`.
*   [ ] **4. Remote Quota Synchronizer**:
    *   Implement `pub async fn fetch_quota(&self, meta_folder_id: &str) -> Result<QuotaInfo, CoreError>`.
    *   Implement `pub async fn update_quota(&self, meta_folder_id: &str, new_quota: &QuotaInfo) -> Result<(), CoreError>`.
*   [ ] **5. Functional State Recalculator**:
    *   Implement `pub fn recalculate_usage(files: &[DriveFile]) -> (u64, u64)` using Functional Programming iterators:
        ```rust
        let total_bytes: u64 = files.iter().map(|f| f.size_bytes).sum();
        let total_count = files.len() as u64;
        ```
*   [ ] **6. Post-Upload / Post-Delete Commits**:
    *   Implement helper methods to adjust `used_bytes` upward on upload completion and downward on file deletion.

---

## 🦀 Rust Implementation Guide & Architectural Notes

### 1. Functional Iterators in Rust
Rust's `Iterator` trait provides zero-cost abstractions. Calling `.iter().map().sum()` compiles down into vectorized assembly identical to or faster than an explicit `for` loop, while eliminating index out-of-bounds risks.

### 2. Short-Circuiting with `Result`
By executing `validate_upload` *before* allocating buffers or calling the resumable upload endpoint, zero network bandwidth or RAM is wasted on rejected files.

---

## 📚 Documentation & Reference Links
*   **Rust Functional Features (Iterators & Closures)**: [https://doc.rust-lang.org/book/ch13-00-functional-features.html](https://doc.rust-lang.org/book/ch13-00-functional-features.html)
*   **Rust Monadic Error Handling (`Result`)**: [https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
*   **`serde_json` Crate Reference**: [https://docs.rs/serde_json/latest/serde_json/](https://docs.rs/serde_json/latest/serde_json/)