# CORE-2: OAuth 2.0 PKCE & Ephemeral Listener

**Epic**: `CAT-CORE-EPIC`
**Component**: `cat-core` (Auth)
**Priority**: High

## Description
Build the OAuth 2.0 authentication engine entirely inside `cat-core`. It should use an ephemeral TCP listener (port 0) and handle the entire Google token exchange, returning a reusable Token struct.

## 🧗‍♀️ Step-by-Step Developer Checklist
*   [ ] 1. In `crates/cat-core/src/auth.rs`, define `pub struct TokenStore { access_token, refresh_token, expires_at }`.
*   [ ] 2. Write `pub fn generate_pkce()` using the `sha2` crate to hash a random string.
*   [ ] 3. Write an async function `pub async fn authenticate() -> Result<TokenStore, AuthError>`.
*   [ ] 4. Inside `authenticate()`, bind `std::net::TcpListener::bind("127.0.0.1:0")` to get a dynamic port.
*   [ ] 5. Use the `open` crate to launch the browser.
*   [ ] 6. Catch the callback, exchange the code via `reqwest`, save `token.json`, and return the `TokenStore` struct.