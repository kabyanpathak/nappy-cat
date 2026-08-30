# CORE-2: OAuth 2.0 PKCE & Ephemeral Loopback Listener

**Epic**: `CAT-CORE-EPIC` (The Shared Foundation)  
**Component**: `crates/cat-core` (Authentication Engine)  
**Priority**: Blocker  
**Story Points**: 5 SP  

---

## 🎯 High-Level Goal & System Behavior
Implement a standalone **OAuth 2.0 PKCE authentication flow** for desktop applications that requires **zero persistent background servers or open ports**. 

The app must request the isolated Google Drive scope (`https://www.googleapis.com/auth/drive.file`). To capture the authorization redirect without hardcoding ports or leaving security holes, the engine temporarily requests a dynamic port from the OS, launches the default browser, intercepts the OAuth callback, delivers a clean HTTP confirmation page, and **immediately closes the socket**. Tokens are persisted to disk and automatically refreshed when expired.

---

## 🧭 Architectural Milestones
*   [ ] **1. PKCE Cryptographic Pipeline**: Generate a cryptographically secure random verifier and compute its SHA-256 Base64URL-encoded challenge to protect the authorization exchange against interception.
*   [ ] **2. Ephemeral Socket Handshake**: Bind a dynamic listener on port 0 (`127.0.0.1:0`), launch the browser to Google's sign-in portal with the dynamic redirect URI, accept the single incoming HTTP redirect, extract the `code` query parameter, return an HTTP 200 response, and drop the socket.
*   [ ] **3. Credential Model & Disk Persistence**: Design a serializable token data model (`serde`) to store access tokens, refresh tokens, and expiration timestamps in a local `token.json` file.
*   [ ] **4. Automated Token Lifecycle Engine**: Implement an asynchronous method ensuring callers always receive a valid, non-expired access token, automatically triggering background token refreshes via Google's token endpoint when needed.

---

## 🔒 Security & Protocol Invariants
*   **Scope Isolation**: Strictly request `https://www.googleapis.com/auth/drive.file` so the app cannot read unrelated user Drive files.
*   **Zero Port Leakage**: The TCP listener must be dropped immediately after receiving the authorization code.
*   **PKCE Specification**: The code challenge must be Base64URL without padding (`URL_SAFE_NO_PAD`).

---

## 📚 Documentation & Reference
*   **Google OAuth 2.0 for Desktop Apps**: [https://developers.google.com/identity/protocols/oauth2/native-app](https://developers.google.com/identity/protocols/oauth2/native-app)
*   **RFC 7636 (PKCE Standard)**: [https://datatracker.ietf.org/doc/html/rfc7636](https://datatracker.ietf.org/doc/html/rfc7636)
*   **Rust `std::net::TcpListener`**: [https://doc.rust-lang.org/std/net/struct.TcpListener.html](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
*   **`sha2` Crate**: [https://docs.rs/sha2/latest/sha2/](https://docs.rs/sha2/latest/sha2/)
*   **`base64` Crate**: [https://docs.rs/base64/latest/base64/](https://docs.rs/base64/latest/base64/)