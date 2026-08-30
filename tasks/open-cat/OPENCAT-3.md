# OPENCAT-3: Google Drive v3 REST Client & Folder Scaffolding

**Epic**: `OPENCAT-EPIC-1`
**Component**: `crates/cat-core` (Drive REST Client)
**Priority**: High
**Story Points**: 5 SP

## Description
Implement an asynchronous HTTP client wrapping Google Drive API v3 using `reqwest`. The client must handle folder hierarchy bootstrapping for `/open-cat/` and support file listing, chunked uploading, and metadata retrieval.

## Acceptance Criteria
- [ ] Client automatically discovers or creates `/open-cat/` and `_meta/` on startup.
- [ ] Successfully lists all files contained within `/open-cat/`.
- [ ] Uploads large files (>50MB) via chunked transfer with accurate progress reporting.

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
You need a central `DriveClient` struct in `cat-core`. This struct should hold a `reqwest::Client`. `reqwest::Client` internally uses an `Arc` (Atomic Reference Counted pointer) to manage connection pools, so it is cheap to `.clone()` and share across your application threads. 

### How it Works (Logic Flow)
1. **Bearer Authentication**: Every method in `DriveClient` will inject the `Authorization: Bearer <token>` header.
2. **Domain Modeling**: Create structs that mirror Google Drive's JSON responses. For example: `DriveFileList { files: Vec<DriveFile> }`. Use `serde` to automatically map JSON fields to Rust fields.
3. **Resumable Uploads**: For large files, Google Drive requires a two-step process:
   - Make a `POST` request to initiate a resumable upload (returns a session URI).
   - Read the local file in chunks (e.g., 256KB or 1MB chunks) using `tokio::io::AsyncReadExt`.
   - `PUT` these chunks to the session URI using `reqwest`.
4. **Error Handling**: Use the `thiserror` crate to define a custom `enum DriveError` containing variants like `Network(reqwest::Error)`, `AuthExpired`, or `FileNotFound`. 

### Documentation & Resources
*   **Google Drive v3 REST API**: [https://developers.google.com/drive/api/reference/rest/v3](https://developers.google.com/drive/api/reference/rest/v3)
*   **Google Drive Resumable Uploads**: [https://developers.google.com/drive/api/guides/manage-uploads#resumable](https://developers.google.com/drive/api/guides/manage-uploads#resumable)
*   **ThisError Crate (Idiomatic Errors)**: [https://docs.rs/thiserror/latest/thiserror/](https://docs.rs/thiserror/latest/thiserror/)
