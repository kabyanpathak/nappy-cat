# CORE-3: Drive REST API SDK & Domain Models

**Epic**: `CAT-CORE-EPIC`
**Component**: `cat-core` (Network SDK)
**Priority**: High

## Description
Encapsulate all Google Drive HTTP requests into a reusable `DriveClient` struct inside `cat-core`. This includes file listing, uploading, and the temp-buffering download logic.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. In `client.rs`, define `pub struct DriveClient { http: reqwest::Client, auth: TokenStore }`.
*   [ ] 2. Define data models like `pub struct DriveFile { id, name, size, mime_type }` mapped via `serde`.
*   [ ] 3. Implement `pub async fn list_files(&self, folder_id: &str) -> Vec<DriveFile>`.
*   [ ] 4. Implement `pub async fn upload_file(&self, path: &PathBuf) -> Result<(), Error>`.
*   [ ] 5. Implement `pub async fn buffer_to_temp(&self, file: &DriveFile) -> PathBuf` that downloads the file to `std::env::temp_dir()`.