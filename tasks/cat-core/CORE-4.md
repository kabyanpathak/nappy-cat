# CORE-4: Ecosystem Rules & Quota Guard Engine

**Epic**: `CAT-CORE-EPIC`
**Component**: `cat-core` (Business Logic)
**Priority**: High

## Description
Move all validation logic, file size restrictions, and quota rules into `cat-core`. The microservices should just pass data to `cat-core`, and `cat-core` decides if it's allowed.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. In `quota.rs`, define `pub struct QuotaInfo { max_bytes, used_bytes }`.
*   [ ] 2. Write `pub fn validate_upload(quota: &QuotaInfo, file_size: u64) -> Result<(), QuotaError>`.
*   [ ] 3. Write helper functions to serialize the quota JSON and push it to the remote `_meta` folder.
*   [ ] 4. (Functional Pattern) Write a function `calculate_total_used(files: &[DriveFile]) -> u64` that uses iterators: `files.iter().map(|f| f.size).sum()`.