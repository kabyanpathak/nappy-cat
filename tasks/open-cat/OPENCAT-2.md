# OPENCAT-2: Timer, Task List, and Daily Progress Views

**Component:** Native productivity views in `apps/open-cat`

**Priority:** High

**Status:** Planned

**Dependencies:** OPENCAT-1, CORE-3 A, CORE-4; Linear controls follow CORE-3 B

## Goal

Deliver a compact daily workflow: add a task, start focus, and see today's progress. This replaces the file grid, quota gauge, and media launcher.

## Milestones

- [ ] Display focus/break state, remaining time, duration settings, and start/pause/resume/reset controls.
- [ ] Provide task creation, editing, completion/reopening, and deletion with clear empty/error states and keyboard access.
- [ ] Show daily app-use and focus totals separately, plus earned and upcoming cosmetic rewards.
- [ ] Add optional session notifications/sounds and respect denied notification permissions.
- [ ] Show Google connection state independently from local productivity.
- [ ] Once Linear is available, provide connect/disconnect, destination selection, task linking/creation, refresh, sync status, retry, and conflict controls.
- [ ] Keep all normal interactions in the native GUI; browser launch is limited to authentication.

## Acceptance

Complete a local task/focus workflow while offline and signed out, then restart and recover state. Verify provider errors do not block local controls. For connected tasks, create/update a Linear issue from the GUI with visible feedback and no silent publication of unrelated local tasks.
