# OPENCAT-1: Native Nappy Cat Shell and Async Bridge

**Component:** `apps/open-cat`

**Priority:** Foundation

**Status:** Planned; existing binary is a console placeholder

**Dependencies:** CORE-1 module contracts

## Goal

Establish the native Rust app using the existing egui/eframe direction. Keep the package name `open-cat` while presenting Nappy Cat to users.

## Milestones

- [ ] Create a minimal native shell for Pomodoro first; add task, connection, and music controls in roadmap order, then progress and companion views.
- [ ] Define typed commands/events between GUI and domain/service modules.
- [ ] Move network and storage work off the UI thread using an app-owned runtime and bounded channels.
- [ ] Wake the UI for events and timer/animation deadlines; avoid continuous high-frame-rate rendering while idle.
- [ ] Surface loading, account, offline, and error states without interrupting local tools.
- [ ] Define worker cancellation, persistence flush, and clean shutdown.
- [ ] Keep state independent of the initial window so OPENCAT-3 can add modes later; do not implement every mode before music.

## Acceptance

Slow I/O and failed connections do not freeze rendering or input. Start signed out without opening a browser. No persistent local server or webview is introduced. Verify worker shutdown and bounded event handling.

GUI code dispatches domain operations rather than implementing provider APIs or timer/reward rules.
