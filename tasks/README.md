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

There are **36 task files**. Only the workspace setup is known to exist; Google auth is an unfinished scaffold. No productivity, GUI, music, or progression feature is claimed complete. Existing build issues belong to CORE-1, not to this planning update.

## Delivery order and stage gates

**Pomodoro → tasks with Linear → optional simple to-do → YouTube Music → first extras → Spotify → remaining extras.**

| Stage | Tasks | Completion point |
| --- | --- | --- |
| Foundation | CORE-1, CORE-5, APP-1, APP-2; start QUALITY-1 | A reproducible baseline, durable local state, and responsive native lifecycle |
| 1. Pomodoro | CORE-4, APP-3 | A useful signed-out/offline timer with session recovery |
| 2. Tasks with Linear | CORE-3, CORE-6, LINEAR-1–4, APP-4 | Explicit native issue reading/linking/creation/updates and reliable recovery |
| 3. Optional simple to-do | APP-5 | A small local-only view, or an explicit decision to skip it |
| 4. YouTube Music | MUSIC-1, then conditional MUSIC-2; CORE-2 if needed | A verified supported experience, or an explicit owner decision about the blocked stage |
| 5. First extras | PROGRESS-1–3, APP-7–9 | Initial tracking, rewards, animated cat, and companion/popup modes are presentable |
| 6. Spotify | MUSIC-3–5 | The accepted second-provider scope and provider coexistence, or an explicit revised scope |
| 7. Remaining extras | APP-10–12; finish QUALITY-2–5 | Full-screen behavior, selected cosmetics, accessibility, recovery, footprint, and packaging |
| Alongside feature work | CORE-2, CORE-7, APP-6, QUALITY-1–4 | Optional Google auth, justified crate extraction, settings, and ongoing verification |

Google sign-in is retained but is not a prerequisite for Pomodoro or Linear. You do not need every future crate, skin, mode, or release package before music.

### Music decisions are explicit gates

YouTube Music and Spotify capabilities must be researched separately. Neither OAuth nor a metadata API proves native playback support. MUSIC-1 and MUSIC-3 can be completed by documenting an incompatibility; their implementation tickets remain **Blocked** if no accepted compatible path exists.

Before entering first extras, complete MUSIC-2 **or record the owner’s explicit acceptance of a deferral or changed music scope**. Before remaining extras, complete MUSIC-3–5 **or record the owner’s explicit revised stage plan**. The same exception must be recorded on affected downstream tickets. This is a future product decision, not an approval being requested for this documentation update.

Do not quietly move music to the end, treat research as a shipped player, or substitute a browser player, webview, scraping flow, or external-player dependency. If a constraint must change, explain the evidence and let the owner decide. Packaging a scoped release also requires the accepted provider/distribution constraints to be clear.

## Tickets

### Shared foundation and domain behavior

| Ticket | Stage | Initial status |
| --- | --- | --- |
| [CORE-1: Establish the development baseline](cat-core/CORE-1.md) | Foundation | Partially established — workspace exists; baseline fixes and decisions pending |
| [CORE-2: Complete optional Google sign-in](cat-core/CORE-2.md) | Alongside relevant features | In progress — scaffold exists; working flow unverified |
| [CORE-3: Provide durable local task behavior](cat-core/CORE-3.md) | 2. Tasks with Linear | Planned |
| [CORE-4: Build reliable Pomodoro sessions](cat-core/CORE-4.md) | 1. Pomodoro | Planned |
| [CORE-5: Persist local work and recover safely](cat-core/CORE-5.md) | Foundation | Planned |
| [CORE-6: Manage credentials and account lifecycle](cat-core/CORE-6.md) | Before provider connections | Planned |
| [CORE-7: Evolve the crate boundaries as features grow](cat-core/CORE-7.md) | As needed after working features | Planned |

### Native application

| Ticket | Stage | Initial status |
| --- | --- | --- |
| [APP-1: Create a responsive native shell](nappy-cat/APP-1.md) | Foundation | Planned |
| [APP-2: Define startup, background use, and shutdown](nappy-cat/APP-2.md) | Foundation | Planned |
| [APP-3: Deliver the first usable Pomodoro interface](nappy-cat/APP-3.md) | 1. Pomodoro | Planned |
| [APP-4: Use Linear tasks from the native app](nappy-cat/APP-4.md) | 2. Tasks with Linear | Planned |
| [APP-5: Decide whether a separate simple to-do view is useful](nappy-cat/APP-5.md) | 3. Optional simple to-do | Planned |
| [APP-6: Make preferences and connection state understandable](nappy-cat/APP-6.md) | Incrementally with each feature | Planned |
| [APP-7: Show daily progress and earned rewards](nappy-cat/APP-7.md) | 5. First extras | Planned |
| [APP-8: Bring the cat companion to life](nappy-cat/APP-8.md) | 5. First extras | Planned |
| [APP-9: Add companion and compact popup modes](nappy-cat/APP-9.md) | 5. First extras | Planned |
| [APP-10: Finish full-screen and window recovery behavior](nappy-cat/APP-10.md) | 7. Remaining extras | Planned |
| [APP-11: Expand selected cosmetics and companion polish](nappy-cat/APP-11.md) | 7. Remaining extras | Planned |
| [APP-12: Complete accessibility and interaction polish](nappy-cat/APP-12.md) | 7. Remaining extras | Planned |

### Linear connection

| Ticket | Stage | Initial status |
| --- | --- | --- |
| [LINEAR-1: Connect a Linear account](linear/LINEAR-1.md) | 2. Tasks with Linear | Planned |
| [LINEAR-2: Browse and link Linear work](linear/LINEAR-2.md) | 2. Tasks with Linear | Planned |
| [LINEAR-3: Create and update Linear issues deliberately](linear/LINEAR-3.md) | 2. Tasks with Linear | Planned |
| [LINEAR-4: Recover sync after outages and competing edits](linear/LINEAR-4.md) | 2. Tasks with Linear | Planned |

### Music

| Ticket | Stage | Initial status |
| --- | --- | --- |
| [MUSIC-1: Resolve YouTube Music feasibility](music/MUSIC-1.md) | 4. YouTube Music | Planned |
| [MUSIC-2: Deliver the accepted YouTube Music experience](music/MUSIC-2.md) | 4. YouTube Music | Conditional — awaiting MUSIC-1 and owner scope acceptance |
| [MUSIC-3: Resolve Spotify feasibility for the presentable app](music/MUSIC-3.md) | 6. Spotify | Planned |
| [MUSIC-4: Add the accepted Spotify experience](music/MUSIC-4.md) | 6. Spotify | Conditional — awaiting MUSIC-3 and owner scope acceptance |
| [MUSIC-5: Make music providers coexist with productivity](music/MUSIC-5.md) | 6. Spotify | Planned |

### Progression

| Ticket | Stage | Initial status |
| --- | --- | --- |
| [PROGRESS-1: Track app-use time separately from focus](progress/PROGRESS-1.md) | 5. First extras | Planned |
| [PROGRESS-2: Produce trustworthy daily history](progress/PROGRESS-2.md) | 5. First extras | Planned |
| [PROGRESS-3: Award persistent cosmetic progression](progress/PROGRESS-3.md) | 5. First extras | Planned |

### Quality and delivery

| Ticket | Stage | Initial status |
| --- | --- | --- |
| [QUALITY-1: Establish repeatable development checks](quality/QUALITY-1.md) | Throughout, starting with foundation | Planned |
| [QUALITY-2: Verify recovery across the complete app](quality/QUALITY-2.md) | Throughout, final pass after remaining extras | Planned |
| [QUALITY-3: Meet the runtime footprint target](quality/QUALITY-3.md) | Throughout, final pass before release | Planned |
| [QUALITY-4: Make diagnostics useful without exposing private data](quality/QUALITY-4.md) | Throughout provider and storage work | Planned |
| [QUALITY-5: Package and document a usable release](quality/QUALITY-5.md) | 7. Remaining extras and release | Planned |

## Migration from earlier tasks

Earlier tickets bundled entire features across several roadmap stages. The original four CORE IDs remain familiar but now have narrower scope. OPENCAT IDs are retired in favor of APP IDs under `tasks/nappy-cat/`; do not translate an old OPENCAT number directly into the same APP number. The old files remain available in Git history.

| Previous ticket | Current assignments |
| --- | --- |
| CORE-1 — workspace/local foundation | CORE-1 baseline, CORE-5 persistence, CORE-7 crate evolution, APP-2 lifecycle |
| CORE-2 — Google OAuth/token handling | CORE-2 Google login and CORE-6 credential lifecycle |
| CORE-3 — tasks, Linear, optional to-do | CORE-3 task behavior, LINEAR-1–4 integration, APP-4 connected UI, APP-5 optional view |
| CORE-4 — timer, tracking, rewards | CORE-4 Pomodoro, PROGRESS-1–3, APP-3 timer UI, APP-7 progress UI |
| OPENCAT-1 — native shell | APP-1 shell and APP-2 lifecycle |
| OPENCAT-2 — timer/tasks/progress views | APP-3–7, delivered in their respective stages |
| OPENCAT-3 — companion and window modes | APP-8–12, with PROGRESS-1–3 for the first extras |
| OPENCAT-4 — both music providers and release | MUSIC-1–5 and QUALITY-1–5 |

The original workspace achievement remains acknowledged. Old completion assumptions do not mark new scope Done. The former Drive SDK, vault, quota, database, warehouse, sharing-key, and Git-hosting tasks are retired, not missing from this backlog.

## Future ideas, not release requirements

Gmail, additional task providers, more game interactions, and additional music providers remain ideas. First establish user value, supported APIs, and compatibility with the footprint/native constraints, then write a bounded task. They are not prerequisites for the planned app and are not included as speculative implementation tickets.

[Product overview](../README.md) · [Product specification](../docs/PRODUCT_SPEC.md) · [Learning resources](../docs/LEARNING_RESOURCES.md)
