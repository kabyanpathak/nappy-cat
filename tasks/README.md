# Nappy Cat development backlog

These are learning assignments for the project owner, written as Jira-style tasks. You own the design, implementation, debugging, and tradeoffs. Assistance should provide context, references, questions, or review; application code and full implementation recipes require a separate request from you.

Each ticket describes an outcome and observable acceptance criteria rather than prescribing structs, functions, algorithms, or a file-by-file solution. Library links are reading material, not mandates to add dependencies. Use documentation matching your selected dependency versions. Optional GitHub examples and provider caveats are collected in [learning resources](../docs/LEARNING_RESOURCES.md).

## How to use the backlog

- Start with the foundation and first usable Pomodoro. Take one bounded ticket at a time; numeric IDs are stable references, not a universal execution order.
- Dependencies identify work needed to finish a ticket. Exploration can happen earlier, but feature delivery follows the stages below. Tasks marked “throughout” grow with the product; their final acceptance does not delay the first timer.
- Use Planned, In progress, Blocked, Done, or Skipped (optional work only). Record evidence when changing status: a short demonstration/test result and any significant design decision are enough.
- A ticket is Done when its outcomes are demonstrated and relevant checks pass. Having a scaffold, reading documentation, or compiling alone does not complete a feature. Research tickets may finish with a documented negative result.
- Keep discoveries inside the ticket or create a focused follow-up. Estimates/story points are deliberately unset until you understand the work; do not treat the backlog as fixed effort estimates.
- Keep the implementation yours. The design question is a prompt to investigate, not an extra essay assignment. Ask for code help separately whenever useful.

There are **45 task files**. Existing workspace/auth scaffold status is preserved; no new feature is claimed implemented by this planning change.

## Delivery order and stage gates

**Pomodoro → Linear → optional small to-do → YouTube Music → calendar → habits → Spotify → integrated TUI release → tracking and GUI → GUI polish/release.**

Apple Music is excluded. Early functionality uses commands/prompts and only a small Pomodoro status line/bar. CORE-4 and APP-3 deliver that first timer; CORE-3 and LINEAR-1–4 plus APP-4 deliver working tasks before full TUI work. Basic habit occurrence records precede the later tracking/analytics system.

| Stage | Tasks | Completion point |
| --- | --- | --- |
| Foundation | CORE-1, initial CORE-5, APP-1–2; start QUALITY-1 | Usable command entry point, durable local state, runtime ownership |
| 1. Pomodoro | CORE-4, APP-3 | Presets and reliable timer with minimal status output |
| 2. Linear | CORE-3, CORE-6, LINEAR-1–4, APP-4 | End-to-end task/Linear behavior through commands or simple prompts |
| 3. Optional to-do | APP-5 | Small convenience or explicit skip |
| 4. YouTube Music | MUSIC-1–2; CORE-2 if needed | First accepted supported music capability |
| 5a. Calendar | CAL-1–2 | Local agenda, events, scheduled task blocks |
| 5b. Habits | HABIT-1–2 | Scheduled habits and completion in the calendar |
| 6. Spotify | MUSIC-3–5 | Second accepted provider and coexistence |
| 7. Integrated TUI | TUI-1–4 | Full terminal workflows and packaged developer-facing release |
| 8. Tracking and GUI | PROGRESS-1–3, APP-13, APP-7–9 | Shared analytics, GUI access to existing features, initial companion modes |
| 9. GUI polish/release | APP-10–12, final QUALITY-2–5 | Broader-audience GUI delivery while maintaining the TUI |
| Throughout | CORE-7, APP-6, QUALITY-1–4 | Boundaries, settings, reliability, diagnostics, footprint |

Full TUI integration is scheduled after Spotify; it also explicitly depends on working Pomodoro and Linear. Tracking and GUI start after TUI-4. TUI-4 has its own release-scope checks and does not depend on finishing GUI-only tasks.

### Provider gates

YouTube Music and Spotify need separate current feasibility decisions. MUSIC-1/MUSIC-3 may close with a negative research result, but that does not deliver MUSIC-2/MUSIC-4. A blocked music stage needs an explicit owner-accepted scope change or deferral before its downstream stage proceeds. Record that decision on affected tickets rather than marking unsupported functionality complete. Calendar follows the accepted YouTube Music outcome; TUI integration follows the accepted Spotify/coexistence outcome.

Browser playback, webviews, scraping, and external-player dependencies are not automatic fallbacks under the existing product constraints. Google login is independent of Pomodoro and Linear. External calendar sync is not assumed, and no Apple Music tickets are added.

## Tickets

### Shared foundation and behavior

| Ticket | Stage | Status |
| --- | --- | --- |
| [CORE-1: Establish the development baseline](cat-core/CORE-1.md) | Foundation | Partially established — workspace exists; baseline fixes and decisions pending |
| [CORE-2: Complete optional Google sign-in](cat-core/CORE-2.md) | Alongside relevant features | In progress — scaffold exists; working flow unverified |
| [CORE-3: Provide durable local task behavior](cat-core/CORE-3.md) | 2. Tasks with Linear | Planned |
| [CORE-4: Build reliable Pomodoro sessions](cat-core/CORE-4.md) | 1. Pomodoro | Planned |
| [CORE-5: Persist local work and recover safely](cat-core/CORE-5.md) | Foundation | Planned |
| [CORE-6: Manage credentials and account lifecycle](cat-core/CORE-6.md) | Before provider connections | Planned |
| [CORE-7: Evolve the crate boundaries as features grow](cat-core/CORE-7.md) | As needed after working features | Planned |

### Application entry, minimal controls, and later GUI

| Ticket | Stage | Status |
| --- | --- | --- |
| [APP-1: Establish a minimal command-line application](nappy-cat/APP-1.md) | Foundation | Planned |
| [APP-2: Define runtime ownership and shutdown](nappy-cat/APP-2.md) | Foundation | Planned |
| [APP-3: Expose Pomodoro commands and a minimal status display](nappy-cat/APP-3.md) | 1. Pomodoro | Planned |
| [APP-4: Use Linear tasks through the minimal interface](nappy-cat/APP-4.md) | 2. Tasks with Linear | Planned |
| [APP-5: Decide whether a separate simple to-do view is useful](nappy-cat/APP-5.md) | 3. Optional simple to-do | Planned |
| [APP-6: Make preferences and connection state understandable](nappy-cat/APP-6.md) | Incrementally with each feature | Planned |
| [APP-7: Show daily progress and earned rewards](nappy-cat/APP-7.md) | 8. Tracking and GUI | Planned |
| [APP-8: Bring the cat companion to life](nappy-cat/APP-8.md) | 8. Tracking and GUI | Planned |
| [APP-9: Add companion and compact popup modes](nappy-cat/APP-9.md) | 8. Tracking and GUI | Planned |
| [APP-10: Finish full-screen and window recovery behavior](nappy-cat/APP-10.md) | 9. GUI polish and release | Planned |
| [APP-11: Expand selected cosmetics and companion polish](nappy-cat/APP-11.md) | 9. GUI polish and release | Planned |
| [APP-12: Complete accessibility and interaction polish](nappy-cat/APP-12.md) | 9. GUI polish and release | Planned |
| [APP-13: Add a GUI over the shared productivity features](nappy-cat/APP-13.md) | 8. Tracking and GUI | Planned |

### Linear integration

| Ticket | Stage | Status |
| --- | --- | --- |
| [LINEAR-1: Connect a Linear account](linear/LINEAR-1.md) | 2. Tasks with Linear | Planned |
| [LINEAR-2: Browse and link Linear work](linear/LINEAR-2.md) | 2. Tasks with Linear | Planned |
| [LINEAR-3: Create and update Linear issues deliberately](linear/LINEAR-3.md) | 2. Tasks with Linear | Planned |
| [LINEAR-4: Recover sync after outages and competing edits](linear/LINEAR-4.md) | 2. Tasks with Linear | Planned |

### Music

| Ticket | Stage | Status |
| --- | --- | --- |
| [MUSIC-1: Resolve YouTube Music feasibility](music/MUSIC-1.md) | 4. YouTube Music | Planned |
| [MUSIC-2: Deliver the accepted YouTube Music experience](music/MUSIC-2.md) | 4. YouTube Music | Conditional — awaiting MUSIC-1 and owner scope acceptance |
| [MUSIC-3: Resolve Spotify feasibility before full TUI integration](music/MUSIC-3.md) | 6. Spotify | Planned |
| [MUSIC-4: Add the accepted Spotify experience](music/MUSIC-4.md) | 6. Spotify | Conditional — awaiting MUSIC-3 and owner scope acceptance |
| [MUSIC-5: Make music providers coexist with productivity](music/MUSIC-5.md) | 6. Spotify | Planned |

### Calendar

| Ticket | Stage | Status |
| --- | --- | --- |
| [CAL-1: Build local calendar behavior](calendar/CAL-1.md) | 5a. Calendar | Planned |
| [CAL-2: Connect planned work to the calendar](calendar/CAL-2.md) | 5a. Calendar | Planned |

### Habits integrated with calendar

| Ticket | Stage | Status |
| --- | --- | --- |
| [HABIT-1: Define recurring habits on the calendar](habits/HABIT-1.md) | 5b. Habits | Planned |
| [HABIT-2: Record habit completion in the calendar](habits/HABIT-2.md) | 5b. Habits | Planned |

### Terminal integration and first release

| Ticket | Stage | Status |
| --- | --- | --- |
| [TUI-1: Build the full terminal shell after core workflows](tui/TUI-1.md) | 7. Integrated TUI | Planned |
| [TUI-2: Bring Pomodoro and Linear into the TUI](tui/TUI-2.md) | 7. Integrated TUI | Planned |
| [TUI-3: Integrate music, calendar, habits, and settings](tui/TUI-3.md) | 7. Integrated TUI | Planned |
| [TUI-4: Package the first developer-facing terminal release](tui/TUI-4.md) | 7. Integrated TUI | Planned |

### Later tracking and rewards

| Ticket | Stage | Status |
| --- | --- | --- |
| [PROGRESS-1: Track app-use time separately from focus](progress/PROGRESS-1.md) | 8. Tracking and GUI | Planned |
| [PROGRESS-2: Produce trustworthy daily history](progress/PROGRESS-2.md) | 8. Tracking and GUI | Planned |
| [PROGRESS-3: Award persistent cosmetic progression](progress/PROGRESS-3.md) | 8. Tracking and GUI | Planned |

### Quality and delivery

| Ticket | Stage | Status |
| --- | --- | --- |
| [QUALITY-1: Establish repeatable development checks](quality/QUALITY-1.md) | Throughout, starting with foundation | Planned |
| [QUALITY-2: Verify recovery across the complete app](quality/QUALITY-2.md) | Throughout, final pass after remaining extras | Planned |
| [QUALITY-3: Meet the runtime footprint target](quality/QUALITY-3.md) | Throughout, final pass before release | Planned |
| [QUALITY-4: Make diagnostics useful without exposing private data](quality/QUALITY-4.md) | Throughout provider and storage work | Planned |
| [QUALITY-5: Package and document a usable release](quality/QUALITY-5.md) | 9. GUI polish and release | Planned |

## Scope revision and stable IDs

Existing IDs remain stable. APP-1 now covers the initial command entry point, APP-2 shared lifecycle, APP-3 minimal timer controls/status, and APP-4 command-accessible Linear workflows. APP-13 adds the later GUI shell and existing-feature access. TUI-1–4 cover full terminal integration and its first release. CAL-1–2 and HABIT-1–2 add calendar and habits.

The previous “first extras before Spotify” gate is retired. MUSIC-3 and MUSIC-5 no longer depend on companion windows; PROGRESS-1 and APP-13 follow TUI-4. QUALITY-5 covers the later GUI release rather than blocking initial terminal packaging. No old completion status proves the new scope complete.

Earlier OPENCAT IDs remain retired in favor of APP IDs; do not equate their numbers. Former Drive/vault/warehouse/Git-hosting tasks remain retired. See Git history for previous scope rather than treating historical roadmaps as current gates.

## Future ideas, not current requirements

External calendar synchronization, invitations, shared calendars, Gmail, other task providers, and more game interactions need an explicit scope decision. Apple Music is excluded. No AI-agent/IDE functionality is implied by the dual-interface inspiration.

[Product overview](../README.md) · [Product specification](../docs/PRODUCT_SPEC.md) · [Learning resources](../docs/LEARNING_RESOURCES.md)
