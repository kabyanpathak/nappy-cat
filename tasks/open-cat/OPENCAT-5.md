# OPENCAT-5: Local Axum Streaming Proxy Daemon (HTTP Range)

**Epic**: `OPENCAT-EPIC-1`
**Component**: `apps/cat-daemon`
**Priority**: Medium
**Story Points**: 5 SP

## Description
Build a lightweight local background HTTP daemon running on `127.0.0.1:4040` using `axum`. It serves two purposes: handling OAuth callbacks and acting as an HTTP `Range`-compliant streaming proxy for media files stored on Google Drive.

## Acceptance Criteria
- [ ] Daemon runs with under 20MB resident memory footprint.
- [ ] Opening `http://127.0.0.1:4040/stream/{file_id}` in VLC starts video playback immediately.
- [ ] Seeking/scrubbing forward and backward in VLC issues HTTP Range requests that stream smoothly.

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
Media players like VLC expect to read a video file natively. To trick VLC into streaming a Google Drive file, `cat-daemon` acts as a middleman proxy. VLC sends a request to `localhost:4040/stream/{id}`. `axum` receives this, attaches the OAuth Bearer token, and forwards the request to Google Drive's API. 

### How it Works (Logic Flow)
1. **Axum Routing & State**: Create an `axum::Router` with routes `/oauth/callback` and `/stream/:file_id`. Use `axum::extract::State` to pass your `DriveClient` or token store into the handlers safely.
2. **Header Extraction**: In the `/stream/:file_id` handler, extract the `HeaderMap`. Look specifically for the `Range` header (e.g., `Range: bytes=1024-2048`).
3. **Proxy Request**: Use `reqwest` to call `GET /drive/v3/files/{file_id}?alt=media`. Attach the exact same `Range` header and the Bearer token.
4. **Streaming the Body (Crucial for Memory)**: Do **NOT** `.await` the full request body into a `Vec<u8>` or memory will explode! Instead, take the `reqwest::Response`, extract its stream (`.bytes_stream()`), wrap it in an `axum::body::Body::from_stream()`, and return that directly to VLC. This creates a zero-copy data pipeline.

### Documentation & Resources
*   **Axum Web Framework**: [https://docs.rs/axum/latest/axum/](https://docs.rs/axum/latest/axum/)
*   **Axum Streaming Bodies**: [https://docs.rs/axum/latest/axum/body/struct.Body.html#method.from_stream](https://docs.rs/axum/latest/axum/body/struct.Body.html#method.from_stream)
*   **Google Drive Media Download**: [https://developers.google.com/drive/api/guides/manage-downloads](https://developers.google.com/drive/api/guides/manage-downloads)
