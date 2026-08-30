# CORE-2: OAuth 2.0 PKCE & Ephemeral Loopback Listener

**Epic**: `CAT-CORE-EPIC` (The Shared Foundation)  
**Component**: `crates/cat-core` (Authentication Module)  
**Priority**: Blocker  
**Story Points**: 5 SP  

---

## 🎯 What This Task Accomplishes
Implements a secure, standalone **OAuth 2.0 Authorization Code flow with PKCE (Proof Key for Code Exchange)** for desktop applications without relying on persistent background servers or open ports.

The engine uses the isolated Google Drive scope `https://www.googleapis.com/auth/drive.file`. To capture Google's redirect, it temporarily binds an **ephemeral TCP listener on port 0**, asks the OS for a dynamic port, launches the system browser, catches the single authorization code callback, sends back a clean confirmation HTML page, and **immediately terminates the listener**. It then exchanges the code for OAuth tokens and caches them in `token.json` with automated token refresh.

---

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] **1. Create Module**: Create `crates/cat-core/src/auth.rs` and declare `pub mod auth;` in `crates/cat-core/src/lib.rs`.
*   [ ] **2. Data Modeling**: Define `pub struct TokenStore` with fields `access_token: String`, `refresh_token: String`, and `expires_at: u64`. Add `#[derive(Serialize, Deserialize, Clone, Debug)]`.
*   [ ] **3. PKCE Generator**: Write `pub fn generate_pkce() -> (String, String)`:
    *   Generate a high-entropy random string (43–128 chars) as `code_verifier`.
    *   Compute its SHA-256 hash using the `sha2` crate.
    *   Encode the hash as `Base64URL` without padding using the `base64` crate as `code_challenge`.
*   [ ] **4. Dynamic Port Ephemeral Listener**:
    *   Bind a TCP socket: `let listener = std::net::TcpListener::bind("127.0.0.1:0")?;`.
    *   Extract the dynamic port assigned by the OS: `let port = listener.local_addr()?.port();`.
*   [ ] **5. Browser Launch & URL Construction**:
    *   Construct Google OAuth URL with `client_id`, `redirect_uri=http://127.0.0.1:<PORT>`, `response_type=code`, `code_challenge`, `code_challenge_method=S256`, and `scope=https://www.googleapis.com/auth/drive.file`.
    *   Launch the system browser using `open::that(&url)`.
*   [ ] **6. Capture Callback & Drop Listener**:
    *   Accept the incoming connection: `let (mut stream, _) = listener.accept()?;`.
    *   Parse the `GET /?code=...` query parameter from the HTTP request line.
    *   Write an HTTP 200 response: `HTTP/1.1 200 OK
Content-Type: text/html

<h1>Authentication Successful! You can close this tab.</h1>`.
    *   Explicitly drop `listener` to close the port immediately.
*   [ ] **7. Token Exchange & Local Cache**:
    *   Send a `POST` request to `https://oauth2.googleapis.com/token` with the authorization code and `code_verifier`.
    *   Calculate expiration timestamp: `current_time + expires_in`.
    *   Write `TokenStore` to `token.json` using `std::fs::write` and `serde_json::to_string_pretty`.
*   [ ] **8. Auto-Refresh Logic**: Write `pub async fn get_valid_token(&mut self) -> Result<String, AuthError>` that checks `expires_at` and automatically uses `refresh_token` to fetch a new `access_token` if expired.

---

## 🦀 Rust Implementation Guide & Architectural Notes

### 1. The Ephemeral Port 0 Trick
Binding to `127.0.0.1:0` tells the OS kernel to assign any currently unallocated port in the dynamic range (typically 49152–65535). Calling `listener.local_addr()?.port()` retrieves this port so you can build the dynamic redirect URI on the fly.

### 2. Base64URL Without Padding
Google OAuth strictly requires RFC 7636 Base64URL encoding (`-` instead of `+`, `_` instead of `/`, and no trailing `=` padding). Use `base64::engine::general_purpose::URL_SAFE_NO_PAD`.

---

## 📚 Documentation & Reference Links
*   **Google OAuth 2.0 for Desktop/Native Apps**: [https://developers.google.com/identity/protocols/oauth2/native-app](https://developers.google.com/identity/protocols/oauth2/native-app)
*   **RFC 7636 (PKCE Standard)**: [https://datatracker.ietf.org/doc/html/rfc7636](https://datatracker.ietf.org/doc/html/rfc7636)
*   **Rust `std::net::TcpListener`**: [https://doc.rust-lang.org/std/net/struct.TcpListener.html](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
*   **`sha2` Crate Reference**: [https://docs.rs/sha2/latest/sha2/](https://docs.rs/sha2/latest/sha2/)
*   **`base64` Crate Reference**: [https://docs.rs/base64/latest/base64/](https://docs.rs/base64/latest/base64/)