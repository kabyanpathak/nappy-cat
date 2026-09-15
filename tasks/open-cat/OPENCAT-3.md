# OPENCAT-3: Animated Cat, Skins, and Window Modes

**Component:** Pet and shared GUI modules in `apps/open-cat`

**Priority:** High

**Status:** Planned

**Dependencies:** OPENCAT-1, OPENCAT-2, CORE-4

## Goal

Make the productivity game a quiet desktop companion. This replaces the drag-and-drop upload task.

## Milestones

- [ ] Render a small animated cat with idle, focus, break, and celebration states driven by domain events.
- [ ] Display unlocked skins and persist the selected skin; keep reward calculation in the core.
- [ ] Add a movable always-on-top cat mode with timer summary and access to controls.
- [ ] Add a compact popup for quick tasks/timer actions and a full-screen mode for expanded work.
- [ ] Switch modes without resetting sessions, duplicating time, or losing edits.
- [ ] Provide user-controlled always-on-top, obvious full-screen exit, and recovery of off-screen/hidden windows.
- [ ] Verify window behavior on supported platforms and provide a normal compact-window fallback where special behavior is unavailable.
- [ ] Limit asset memory and animation frequency; pause unnecessary rendering when hidden and support reduced motion.

## Acceptance

Exercise all mode transitions during focus and task editing. Verify single accounting of time/rewards, persistence of placement and skin, keyboard access, reduced motion, and access to controls from the tiny cat mode.

Keep pet and GUI responsibilities separate as modules, but share one GUI boundary across all three modes. No new crates are required by this task.
