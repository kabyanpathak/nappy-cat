# CORE-3: Google Drive v3 REST API SDK & Domain Models

**Epic**: `CAT-CORE-EPIC` (The Shared Foundation)  
**Component**: `crates/cat-core` (Network SDK)  
**Priority**: High  
**Story Points**: 5 SP  

---

## 🎯 What This Task Accomplishes
Encapsulates all Google Drive API v3 interactions into a reusable, asynchronous `DriveClient` struct inside `cat-core`. 

Instead of writing raw HTTP calls in UI code, `DriveClient` provides high-level domain methods:
* Bootstrapping the `/open-cat/` root vault and `_meta/` directory.
* Listing files with pagination and metadata decoding.
* Resumable chunked uploading for large files.
* Streaming file downloads into temporary storage for media playback.

---

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] **1. Create Module**: Create `crates/cat-core/src/client.rs` and declare `pub mod client;` in `lib.rs`.
*   [ ] **2. Error Domain**: Create `crates/cat-core/src/error.rs` using the `thiserror` crate to define `pub enum CoreError` (variants: `Network(reqwest::Error)`, `Auth(AuthError)`, `QuotaExceeded`, `Io(std::io::Error)`, `Api(String)`).
*   [ ] **3. Domain Models**:
    *   Define `pub struct DriveFile { pub id: String, pub name: String, pub size_bytes: u64, pub mime_type: String, pub modified_time: String }` with `#[derive(Serialize, Deserialize, Clone, Debug)]`.
    *   Define raw Google Drive response structs matching Drive v3 JSON schema.
*   [ ] **4. Client Struct**:
    *   Define `pub struct DriveClient { http: reqwest::Client, auth: TokenStore }`.
    *   Implement `pub fn new(auth: TokenStore) -> Self` initializing `reqwest::Client`.
*   [ ] **5. Folder Discovery & Scaffolding**:
    *   Implement `pub async fn get_or_create_folder(&self, folder_name: &str, parent_id: Option<&str>) -> Result<String, CoreError>`.
    *   Query `GET https://www.googleapis.com/drive/v3/files?q=name='open-cat' and mimeType='application/vnd.google-apps.folder' and trashed=false`.
    *   If not found, send `POST https://www.googleapis.com/drive/v3/files` to create it.
*   [ ] **6. File Listing**:
    *   Implement `pub async fn list_files(&self, folder_id: &str) -> Result<Vec<DriveFile>, CoreError>`.
    *   Handle Google Drive query parameters: `'<folder_id>' in parents and trashed=false`.
*   [ ] **7. Resumable Upload Engine**:
    *   Implement `pub async fn upload_file<F>(&self, file_path: &Path, folder_id: &str, progress_callback: F) -> Result<DriveFile, CoreError>` where `F: Fn(f32) + Send + 'static`.
    *   Initiate upload session (`POST https://www.googleapis.com/upload/drive/v3/files?uploadType=resumable`).
    *   Stream chunks (multiples of 256 KiB) via `PUT` to the session URI while emitting progress.
*   [ ] **8. Temp-Buffer Media Downloader**:
    *   Implement `pub async fn buffer_to_temp<F>(&self, file_id: &str, file_name: &str, progress_callback: F) -> Result<PathBuf, CoreError>`.
    *   Stream `GET https://www.googleapis.com/drive/v3/files/{file_id}?alt=media` directly into `std::env::temp_dir().join(file_name)`.

---

## 🦀 Rust Implementation Guide & Architectural Notes

### 1. `reqwest::Client` Internal Architecture
`reqwest::Client` uses an internal `Arc` (Atomic Reference Counted pointer) to manage HTTP/2 connection pooling. It is cheap to `.clone()` and safe to share across Tokio tasks.

### 2. Zero-Copy Byte Streaming
When downloading media to temp storage, avoid `.await`ing the entire file into a `Vec<u8>` in RAM. Use `tokio::io::copy` or iterate over `response.bytes_stream()` writing chunks directly to `tokio::fs::File`.

---

## 📚 Documentation & Reference Links
*   **Google Drive API v3 Reference**: [https://developers.google.com/drive/api/reference/rest/v3](https://developers.google.com/drive/api/reference/rest/v3)
*   **Drive Resumable Upload Guide**: [https://developers.google.com/drive/api/guides/manage-uploads#resumable](https://developers.google.com/drive/api/guides/manage-uploads#resumable)
*   **Drive Search Query Syntax**: [https://developers.google.com/drive/api/guides/search-files](https://developers.google.com/drive/api/guides/search-files)
*   **`thiserror` Crate Reference**: [https://docs.rs/thiserror/latest/thiserror/](https://docs.rs/thiserror/latest/thiserror/)
*   **`reqwest` Crate Reference**: [https://docs.rs/reqwest/latest/reqwest/](https://docs.rs/reqwest/latest/reqwest/)