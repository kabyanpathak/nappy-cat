# CORE-2: Retain and Complete Google OAuth 2.0

**Component:** Google auth module in `crates/cat-core`; future `cat-google-auth` boundary

**Priority:** High, independent of local productivity

**Status:** Early scaffold exists; working authentication not verified

**Dependencies:** CORE-1 credential/storage contracts

## Goal

Keep optional Google sign-in while removing Google Drive from the product requirements. Local tasks, timers, and rewards must work signed out. Existing scaffold code is retained during this documentation transition.

## Milestones

- [ ] Verify current official Google native-app OAuth requirements and choose minimal scopes for the actual login feature; no Drive scope requirement.
- [ ] Complete authorization-code flow with PKCE and state validation; use provider endpoints and registered client configuration.
- [ ] Open the system browser for authentication and receive the callback through a temporary loopback listener.
- [ ] Handle malformed callbacks, denial, cancellation, timeout, and concurrent login attempts; close the listener on every terminal path.
- [ ] Store credentials securely, preferably in OS credential storage; support expiry, refresh, revoked access, and disconnect.
- [ ] Expose account state and actionable errors to the GUI without logging tokens or blocking rendering.

## Acceptance

Validate successful login, rejected state, cancellation, timeout, refresh failure, and restart/disconnect behavior. No persistent listener or embedded browser remains after login. A distributed app must not rely on keeping an embedded client secret confidential.

Google authentication does not imply YouTube Music or Gmail access. Validate music-specific permissions and capabilities in OPENCAT-4 A, before the first cat/extras stage. Linear uses its own auth boundary under CORE-3.
