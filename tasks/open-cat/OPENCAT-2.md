# OPENCAT-2: Pomodoro First, Then Task Views

**Component:** Native productivity views in `apps/open-cat`

**Priority:** Timer first, then tasks with Linear; simple to-do optional

**Status:** Planned

**Dependencies:** A needs OPENCAT-1 and CORE-4 A; B needs A and CORE-3 A/B; C follows B if useful

## Goal

Deliver each usable feature in order without waiting for later game features: Pomodoro, tasks with Linear, then an optional small local-only to-do experience.

## Milestone A: Pomodoro controls

- [ ] Display focus/break state, remaining time, duration settings, and start/pause/resume/reset controls.
- [ ] Add session feedback and optional notifications/sounds; respect denied permissions.
- [ ] Verify offline and signed-out use, keyboard access, and session recovery.

This milestone does not depend on tasks, daily totals, rewards, or cat animation.

## Milestone B: Tasks with Linear

- [ ] Provide native task creation, editing, completion/reopening, and local deletion/unlinking with clear empty/error states.
- [ ] Add Linear connect/disconnect, destination selection, task linking/creation, refresh, sync status, retry, and conflict controls.
- [ ] Show cached/local task state during outages without blocking the timer.
- [ ] Keep Google account state independent of Linear and local productivity.

## Milestone C: Optional simple local to-do

- [ ] Following CORE-3 C, offer a small local-only list if the task UI does not already meet that need.
- [ ] Reuse the shared task model and native controls, including when adapting another Rust project's code.
- [ ] Skip this milestone if redundant; avoid expanding it into a prerequisite for music.

## Later progress views

After YouTube Music, alongside CORE-4 B and OPENCAT-3 A, show daily app-use and focus totals separately, plus initial rewards. Finish cosmetic presentation with the remaining extras after Spotify.

## Acceptance

Accept the timer independently, then validate creating/updating a Linear issue from the GUI with visible feedback and no silent publication of unrelated tasks. If the optional local-only list is added, verify offline use and persistence. Normal interactions remain native; browser launch is limited to authentication.
