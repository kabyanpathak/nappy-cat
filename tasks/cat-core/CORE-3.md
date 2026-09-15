# CORE-3: Local Tasks and Optional Linear Integration

**Component:** Task, Linear auth, and Linear sync modules in `crates/cat-core`

**Priority:** High

**Status:** Planned

**Dependencies:** CORE-1; milestone B follows milestone A, without depending on Google auth

## Goal

Provide durable local tasks first, then let users connect Linear and create/update linked issues from Nappy Cat. This replaces the Drive SDK task.

## Milestone A: Local tasks

- [ ] Support create, read, edit, complete/reopen, and delete with stable local IDs.
- [ ] Persist changes through the local store, recover interrupted writes, and retain tasks across restart.
- [ ] Define task events and errors for the native GUI; keep tasks usable offline and signed out.

## Milestone B: Linear auth and sync

- [ ] Verify official Linear native/public-client auth support, redirects, scopes, token lifecycle, and PKCE capabilities. Record incompatibilities before expanding architecture; do not embed a confidential secret.
- [ ] Keep Linear credential lifecycle separate from Google auth, with secure storage, connect/disconnect, and actionable errors.
- [ ] Let users select a workspace/team and explicitly opt tasks into linking or remote creation.
- [ ] Read linked issues and create/update supported fields, including explicit mapping between local completion and Linear statuses.
- [ ] Persist remote identifiers and a bounded pending-operation queue. Show pending, synced, failed, and conflict states.
- [ ] Support manual refresh and modest polling, rate-limit handling, bounded retries, and offline recovery without a public webhook server.
- [ ] Reconcile ambiguous remote-create outcomes before retrying; prevent duplicate issue creation.
- [ ] Detect concurrent edits and provide conflict resolution. Local deletion unlinks by default; remote deletion is outside initial scope.
- [ ] On disconnect, stop sync and preserve local tasks; handle account changes without sending queued work to the wrong destination.

## Acceptance

Verify local CRUD/restart recovery first. Then validate reading, creating, and updating a linked issue, revoked credentials, outages, rate limits, ambiguous timeouts, and conflicts. Local work remains available during provider failures. No personal task is sent remotely without the user's explicit choice.

Keep future `cat-tasks`, `cat-linear-auth`, and `cat-linear` responsibilities distinct within the current library.
