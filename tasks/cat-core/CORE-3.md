# CORE-3: Google Drive v3 REST API SDK & Domain Models

**Epic**: `CAT-CORE-EPIC` (The Shared Foundation)  
**Component**: `crates/cat-core` (Network SDK & Data Layer)  
**Priority**: High  
**Story Points**: 5 SP  

---

## 🎯 High-Level Goal & System Behavior
Encapsulate all Google Drive API v3 interactions into a strongly typed, asynchronous SDK client (`DriveClient`). 

This SDK acts as the data layer for the entire ecosystem, isolating raw HTTP headers, JSON parsing, query formatting, and multipart/chunked protocols from the UI layer. It provides methods for vault folder initialization, paginated file exploration, chunked uploads with progress tracking, and zero-copy media buffering to temporary storage.

---

## 🧭 Architectural Milestones
*   [ ] **1. Client & Error Architecture**: Design a `DriveClient` struct holding shared HTTP state (`reqwest::Client`) and an idiomatic error enum (`thiserror`) covering network, serialization, auth, and API errors.
*   [ ] **2. Vault Scaffolding (Idempotent Bootstrap)**: Implement discovery and auto-creation of the `/open-cat/` root folder and hidden `_meta/` configuration directory.
*   [ ] **3. Paginated File Listing**: Fetch and parse Drive file metadata into clean domain structs (`DriveFile`), handling parent folder query filtering and pagination tokens.
*   [ ] **4. Resumable Chunked Upload Engine**: Implement Google Drive's resumable byte upload protocol for large files, supporting thread-safe progress callbacks without loading whole files into memory.
*   [ ] **5. Temp-Buffer Media Downloader**: Stream remote media files directly into the OS temporary directory (`std::env::temp_dir()`) using zero-copy byte streams to enable native media player playback.

---

## 🔒 Systems & Memory Invariants
*   **Zero-Copy Streaming**: Large uploads and downloads must stream in byte chunks; never buffer multi-gigabyte files into RAM.
*   **Connection Reuse**: Share the `reqwest::Client` connection pool across async calls.

---

## 📚 Documentation & Reference
*   **Google Drive API v3 Reference**: [https://developers.google.com/drive/api/reference/rest/v3](https://developers.google.com/drive/api/reference/rest/v3)
*   **Drive Resumable Uploads**: [https://developers.google.com/drive/api/guides/manage-uploads#resumable](https://developers.google.com/drive/api/guides/manage-uploads#resumable)
*   **`thiserror` Crate**: [https://docs.rs/thiserror/latest/thiserror/](https://docs.rs/thiserror/latest/thiserror/)
*   **`reqwest` Crate**: [https://docs.rs/reqwest/latest/reqwest/](https://docs.rs/reqwest/latest/reqwest/)