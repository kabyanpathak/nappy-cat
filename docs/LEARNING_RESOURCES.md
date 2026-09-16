# Learning resources

Use these references to investigate the questions in a task and make your own design decisions. They are reading material, not an implementation checklist. Start with the links attached to the task; this index provides more context when needed.

Links and provider constraints were checked on **September 16, 2026**. Provider access rules can change. For crate documentation linked through `latest`, select the version in `Cargo.lock` before applying examples. A newer documentation version is not a reason to upgrade the project automatically. Libraries listed here are options to study, not required new dependencies.

## Rust and project boundaries

| Reference | What to learn |
| --- | --- |
| [The Rust Book](https://doc.rust-lang.org/book/) | Ownership, error handling, modules, traits, and how to express responsibilities clearly. |
| [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html) | Package boundaries, shared dependencies, and workspace commands. |
| [egui documentation](https://docs.rs/egui/latest/egui/) | Immediate-mode UI, application state, layout, input, and repaint behavior. |
| [eframe documentation](https://docs.rs/eframe/latest/eframe/) | Desktop application lifecycle and integration with egui. |
| [Tokio tutorial](https://tokio.rs/tokio/tutorial) | Async work, shared state, channels, and keeping network work from blocking the interface. |

## Time, persistence, and credentials

| Reference | What to learn |
| --- | --- |
| [`Instant`](https://doc.rust-lang.org/std/time/struct.Instant.html), [`SystemTime`](https://doc.rust-lang.org/std/time/struct.SystemTime.html), and [`Duration`](https://doc.rust-lang.org/std/time/struct.Duration.html) | The difference between elapsed time, wall-clock timestamps, and a length of time; examine platform behavior around sleep and clock changes. |
| [Chrono](https://docs.rs/chrono/latest/chrono/) | Calendar dates, local time, offsets, and daily tracking boundaries. |
| [Serde](https://serde.rs/) | Representing application data for storage and transport. |
| [rusqlite](https://docs.rs/rusqlite/latest/rusqlite/) | SQLite access, transactions, and reliable local persistence if SQLite fits the storage decision. |
| [keyring](https://docs.rs/keyring/latest/keyring/) | Platform credential storage and the availability and behavior of its backends. |
| [oauth2 5.0.0](https://docs.rs/oauth2/5.0.0/oauth2/) | Authorization code flow, PKCE, state validation, refresh tokens, and HTTP client security considerations. |

## Linear integration

| Reference | What to learn |
| --- | --- |
| [OAuth authentication](https://linear.app/developers/oauth-2-0-authentication) | Account connection, scopes, PKCE, token refresh, and disconnection. |
| [GraphQL API](https://linear.app/developers/graphql) | The schema and query/mutation behavior available to the app. |
| [Pagination](https://linear.app/developers/pagination) and [filtering](https://linear.app/developers/filtering) | Retrieving the complete relevant issue set without assuming one response contains everything. |
| [Rate limiting](https://linear.app/developers/rate-limiting) | Provider limits and how failures should affect synchronization. |
| [Webhooks](https://linear.app/developers/webhooks) | Delivery requirements and whether push updates fit a desktop application. |

Linear explicitly supports PKCE with an optional client secret during the PKCE token exchange. Its documentation currently specifies 24-hour access tokens and refresh tokens. Webhook receivers require a publicly accessible HTTPS endpoint; a localhost listener alone cannot receive them. Evaluate synchronization within the project's native desktop scope before adding a service dependency. See the [OAuth](https://linear.app/developers/oauth-2-0-authentication) and [webhook](https://linear.app/developers/webhooks) requirements.

## Google and YouTube feasibility

| Reference | What to learn |
| --- | --- |
| [Google OAuth for installed applications](https://developers.google.com/identity/protocols/oauth2/native-app) | Desktop authorization, PKCE, browser consent, loopback callbacks, scopes, and token lifecycle. |
| [YouTube Data API](https://developers.google.com/youtube/v3) | Official search, video metadata, playlist, and account capabilities. |
| [YouTube IFrame Player API](https://developers.google.com/youtube/iframe_api_reference) | The supported browser-based video player and its display requirements. |
| [YouTube developer policies](https://developers.google.com/youtube/terms/developer-policies) | Restrictions that affect an embedded music experience. |

The official Data API documentation covers YouTube data operations; it does not establish support for native YouTube Music streaming. The IFrame API is a JavaScript video player with visible-player requirements, and the policies prohibit separating audio from video or providing a hidden/background player. These sources do not establish a supported direct native Rust music player. [Data API](https://developers.google.com/youtube/v3), [IFrame API](https://developers.google.com/youtube/iframe_api_reference), [policies](https://developers.google.com/youtube/terms/developer-policies).

The project permits a browser only for authentication. Browser playback, a browser handoff, or a web player is therefore not an accepted fallback. The feasibility task should document what is supported, what remains unproven, and whether the intended native playback experience is blocked under the current constraints. A constraint change needs an explicit project decision.

## Spotify feasibility

| Reference | What to learn |
| --- | --- |
| [Authorization code with PKCE](https://developer.spotify.com/documentation/web-api/tutorials/code-pkce-flow) | OAuth for applications that cannot keep a client secret confidential. |
| [Quota modes](https://developer.spotify.com/documentation/web-api/concepts/quota-modes) | Development access, account eligibility, allowed users, and distribution limits. |
| [February 2026 migration guide](https://developer.spotify.com/documentation/web-api/tutorials/february-2026-migration-guide) | Current development-mode restrictions and changed endpoints. |
| [Rate limits](https://developer.spotify.com/documentation/web-api/concepts/rate-limits) | Request limits and failure handling. |
| [Web Playback SDK](https://developer.spotify.com/documentation/web-playback-sdk) and [SDK reference](https://developer.spotify.com/documentation/web-playback-sdk/reference) | The capabilities and account requirements of Spotify's browser playback offering. |

Spotify's development mode currently requires the app owner to have Premium and allows up to five allowlisted authenticated users. Existing apps may retain previously granted client IDs or user counts; recheck the rules for the actual app before relying on access. [Quota modes](https://developer.spotify.com/documentation/web-api/concepts/quota-modes), [migration guide](https://developer.spotify.com/documentation/web-api/tutorials/february-2026-migration-guide).

The Web Playback SDK is a browser JavaScript library and requires a Premium user. It does not establish direct native Rust playback support and does not meet the project's browser-only-for-authentication constraint. Distinguish controlling an existing playback device from playing audio inside this app, and resolve that product and capability question before committing to a playback implementation. [SDK overview](https://developer.spotify.com/documentation/web-playback-sdk), [SDK reference](https://developer.spotify.com/documentation/web-playback-sdk/reference).

## Verification, performance, and delivery

| Reference | What to learn |
| --- | --- |
| [Rust testing](https://doc.rust-lang.org/book/ch11-00-testing.html) | Behavioral tests, integration boundaries, and useful assertions. |
| [Tokio testing](https://tokio.rs/tokio/topics/testing) | Testing asynchronous and time-dependent behavior without slow real-time waits. |
| [tracing](https://docs.rs/tracing/latest/tracing/) | Diagnostics that explain application behavior and asynchronous work. |
| [Cargo profiles](https://doc.rust-lang.org/cargo/reference/profiles.html) | Debug/release differences and the tradeoffs of performance settings. |
| [Building and testing Rust with GitHub Actions](https://docs.github.com/en/actions/tutorials/build-and-test-code/rust) | Reproducible checks and build automation across supported environments. |

## Optional projects to read

These are examples to examine when a task raises a relevant question. Reading them does not commit the project to their dependencies, structure, or code.

- [emilk/eframe_template](https://github.com/emilk/eframe_template): inspect a small egui application lifecycle and its build setup.
- [emilk/egui](https://github.com/emilk/egui): explore demo widgets and trace how an interaction produces its visible behavior.
- [tokio-rs/mini-redis](https://github.com/tokio-rs/mini-redis): study an explicitly educational async application, particularly how it separates concurrent responsibilities.
