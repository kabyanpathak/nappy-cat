# CORE-4: Pomodoro, Daily Tracking, and Rewards

**Component:** Pomodoro and progress modules in `crates/cat-core`

**Priority:** Pomodoro first; full tracking and rewards after YouTube Music

**Status:** Planned

**Dependencies:** CORE-1 local storage and instance-ownership contracts

## Goal

Build focus sessions and local progression for the productivity game. This replaces the vault quota engine.

## Milestone A: Pomodoro first

- [ ] Implement configurable focus/break durations and start, pause, resume, reset, and completion transitions using elapsed time, independent of rendering.
- [ ] Record completed and interrupted sessions distinctly and publish events for UI, cat state, and optional notifications.
## Milestone B: Tracking and initial rewards, after OPENCAT-4 A

- [ ] Track app-use time while Nappy Cat runs on an awake device, including background mode; exclude sleep and explicit tracking pauses. Do not monitor other apps.
- [ ] Track focus time separately, excluding breaks and paused sessions.
- [ ] Persist bounded checkpoints and local-date totals; split at midnight and handle clock/timezone changes.
- [ ] Define sleep/wake and restart recovery so closed or suspended intervals do not silently become focus time or earned usage.
- [ ] Prevent duplicate accounting across windows/processes.
- [ ] Define tunable usage/session milestones, persist cosmetic unlocks once, and expose skin selection without gating productivity tools.

## Acceptance

Accept milestone A independently with controllable-time tests for timer transitions, pause/resume, sleep, and restart. It does not depend on daily tracking, rewards, or tasks. For milestone B, also verify midnight, clock changes, and duplicate completion events. Daily totals and rewards survive restart without double counting. Core behavior works offline with no provider account.

Keep Pomodoro separate from progress rules; combine tracking and rewards in the future `cat-progress` boundary. Rendering and animation belong to OPENCAT-3.
