# OPENCAT-2: Google Drive OAuth 2.0 PKCE Flow & Secure Token Store

**Epic**: `OPENCAT-EPIC-1`
**Component**: `crates/cat-core` (Auth Module)
**Priority**: Blocker
**Story Points**: 5 SP

## Description
Implement the OAuth 2.0 Authorization Code flow with PKCE (Proof Key for Code Exchange) tailored for desktop applications. Use the strictly isolated scope `https://www.googleapis.com/auth/drive.file`.

## Acceptance Criteria
- [ ] Launching auth flow automatically opens the browser to Google sign-in.
- [ ] OAuth callback on `127.0.0.1:4040` extracts authorization code and stores valid token bundle.
- [ ] Token refresh logic recovers seamlessly from expired `access_token` using `refresh_token`.

---

## 🦀 Rust Implementation Guide & Documentation

### Architectural Logic
Google Drive requires OAuth 2.0. Because this is a desktop app (no secure backend to hide a client secret), you must use **PKCE**. 
PKCE involves generating a random string (the `code_verifier`) and hashing it (the `code_challenge`). You send the challenge to Google, the user logs in, Google redirects to your local loopback (`http://127.0.0.1:4040`), and then you exchange the authorization code + your original `code_verifier` for an `access_token`.

### How it Works (Logic Flow)
1. **Cryptography**: Use a crate like `sha2` to generate a SHA-256 hash of a random 64-byte string. Use the `base64` crate to encode it into `Base64URL` format (without padding).
2. **Opening the Browser**: Use the `open` crate to launch the system's default browser pointing to Google's OAuth endpoint.
3. **Data Serialization**: Represent the Google Token response as a Rust struct using `#[derive(Deserialize, Serialize)]`. 
4. **Filesystem IO**: Use `tokio::fs` to save this struct to `token.json` so the user doesn't have to log in on every app restart.
5. **Token Refresh Mechanism**: Before any Google Drive API call, check the `expires_in` timestamp. If expired, send a `POST` request using `reqwest` to refresh the token, update the struct, and overwrite `token.json`.

### Documentation & Resources
*   **Google OAuth Desktop App Flow**: [https://developers.google.com/identity/protocols/oauth2/native-app](https://developers.google.com/identity/protocols/oauth2/native-app)
*   **Serde JSON**: [https://docs.rs/serde_json/latest/serde_json/](https://docs.rs/serde_json/latest/serde_json/)
*   **Reqwest (POST requests)**: [https://docs.rs/reqwest/latest/reqwest/](https://docs.rs/reqwest/latest/reqwest/)
