# OPENCAT-4: 5GB Software Quota Guard Engine

**Epic**: `OPENCAT-EPIC-1`
**Component**: `crates/cat-core` (Quota Guard)
**Priority**: High
**Story Points**: 3 SP

## Description
Implement the software-enforced 5GB quota guard system. The guard tracks cumulative folder usage in `_meta/quota.json` on Google Drive and blocks uploads before byte transfer if the 5GB cap is exceeded.

## Acceptance Criteria
- [ ] Upload is rejected instantly with an informative error if file size exceeds remaining quota.
- [ ] Successful upload atomically updates `_meta/quota.json` on Google Drive.
- [ ] Deleting a file decrements quota usage accordingly.

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
Since Google Drive itself tracks quota per-user (which is 15GB+), but `open-cat` needs to enforce a strict **5GB** limit for the shared folder pool, this logic must run entirely in software. It relies on fetching a state file (`quota.json`) from the Google Drive `_meta/` folder before any upload starts.

### How it Works (Logic Flow)
1. **Pre-flight Check Phase**: When the user drags a file, your async upload function should first fetch `_meta/quota.json`.
2. **Rust's `Result` for Early Returns**: 
   - Check `current_used_bytes + new_file_size_bytes <= 5_368_709_120`.
   - If false, immediately return `Err(DriveError::QuotaExceeded)`. The `?` operator ensures that the upload code gracefully short-circuits, preventing bandwidth waste.
3. **Atomic Updates (Software Level)**: If the upload succeeds, update the local Rust `QuotaInfo` struct (`used_bytes += new_size`), serialize it back to JSON string with `serde_json::to_string`, and overwrite the remote `_meta/quota.json` using the Drive API.
4. **State Recovery**: If `_meta/quota.json` is deleted or corrupted, the system should iterate over all files in `/open-cat/` to recalculate the sum of bytes, and write a fresh `quota.json`.

### Documentation & Resources
*   **Rust Result & Error Propagation**: [https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html](https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html)
*   **Google Drive File Update**: [https://developers.google.com/drive/api/reference/rest/v3/files/update](https://developers.google.com/drive/api/reference/rest/v3/files/update)
