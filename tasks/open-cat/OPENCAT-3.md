# OPENCAT-3: First Extras, Then Finishing Touches

**Component:** Pet and shared GUI modules in `apps/open-cat`

**Priority:** First extras after YouTube Music; remaining extras after Spotify

**Status:** Planned

**Dependencies:** A follows OPENCAT-4 A, with OPENCAT-1/2 and CORE-4 B for progress; B follows OPENCAT-4 B

## Goal

Add a substantial first pass of companion features after YouTube Music so the app is more presentable for Spotify. Finish the remaining gadgets and polish after Spotify. This task is not a prerequisite for the first music provider.

## Milestone A: First extras and presentable companion

- [ ] Render a small animated cat with idle, focus, break, and celebration states driven by domain events.
- [ ] Add initial daily-progress and reward presentation with CORE-4 B, including a small initial skin selection.
- [ ] Provide a movable always-on-top cat and usable compact popup with task, timer, and music access.
- [ ] Preserve shared sessions, task edits, playback state, and time accounting when switching views.
- [ ] Establish readable styling, keyboard access, reduced motion, and bounded asset/animation costs.
- [ ] Confirm the timer, Linear tasks, and YouTube Music are usable together in the presentable app.

Completion means the initial companion and core controls are coherent and usable; it does not require every skin, animation, or window mode. This is the milestone before Spotify.

## Milestone B: Remaining extras, after Spotify

- [ ] Finish full-screen mode and remaining window behavior, with obvious exit and recovery of hidden/off-screen windows.
- [ ] Expand skins, animations, reward presentation, and other selected gadgets.
- [ ] Verify user-controlled always-on-top and platform behavior; provide compact-window fallbacks where necessary.
- [ ] Finish persistence of placement/skin selection, accessibility, and mode transitions during tasks, focus, and playback.
- [ ] Consider additional features after the planned workflows are stable; prioritize them explicitly instead of expanding the release indefinitely.

## Acceptance

At A, verify a usable animated companion with initial progression and access to the existing productivity/music features. At B, exercise all window transitions without duplicate tracking or lost state. Both stages respect reduced motion and pause unnecessary rendering while hidden.

Keep pet and GUI responsibilities distinct, with all three modes in one future GUI crate. No crate creation is part of this documentation update.
