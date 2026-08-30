# The Cat Ecosystem: Master Project Specification & Blueprint
> **A 100% Rust, Low-Memory Personal Cloud & Productivity Suite Powered by Google Drive (5TB)**

---

## 0. AI Operating Rules & Collaboration Guidelines

> [!IMPORTANT]
> ### 🛡️ ROLE OF THE AI ASSISTANT (Senior Architect & Engineering Manager)
> When this specification is provided to Antigravity (or any AI assistant on your development machine), the AI must strictly adhere to the following rules:
>
> 1. **Architectural & Planning Guide**: Break down features into clear, bite-sized tasks, architectural diagrams, data flows, and checklists that the developer can read and implement.
> 2. **Documentation & Reference Source**: Explain complex protocols, Google Drive API behaviors, `egui` lifecycle patterns, and Rust borrow checker concepts in plain English.
> 3. **NO UNSOLICITED CODE**: 
>    * **The AI MUST NOT write the project code.**
>    * **The AI MUST NOT output full implementation code or copy-paste code snippets unless the user EXPLICITLY asks** (e.g., *"Show me the code for this function"* or *"Write this struct for me"*).
> 4. **Senior Tech Lead Persona**: Act as a technical mentor / engineering manager. Review the developer's code when asked, explain compiler errors, suggest design improvements, and point out edge cases.

---

## 1. Executive Summary & Philosophy

The **Cat Ecosystem** is a modular suite of lightweight personal desktop applications and microservices written in **100% Rust**, backed by a 5TB Google Drive storage pool.

### Core Architectural Principles:
1. **True Standalone Architecture**: No persistent local web servers or background daemons. Apps manage their own async runtimes, ephemeral OS listeners for auth, and native OS integrations.
2. **Featherlight Footprint**: No Garbage Collection, no Electron, no Chromium overhead. GUI runs at **< 35 MB RAM**.
3. **Simple, Idiomatic Rust**: Standard data structs (`serde`), `tokio` async networking, MPSC channels, and native `egui` immediate-mode GUIs.
4. **Google Drive as Personal Cloud**: Uses the isolated `https://www.googleapis.com/auth/drive.file` scope. The apps only see files they create.
5. **Single-User Coherence**: The apps seamlessly cross-integrate: `fidget-cat` stores streak metadata in `cat-db`, `git-cat` archives packfiles in `ware-cat`, and `open-cat` shares files with friends under strict 5GB quotas.

---

## 2. The 5 Microservices + Core Engine

```mermaid
flowchart TB
    subgraph Storage["Google Drive 5TB Storage Pool"]
        OpenCatFolder["/open-cat/ (Shared Vault & 5GB Quota)"]
        CatDbFolder["/cat-db/ (Document DB Collections)"]
        WareCatFolder["/ware-cat/ (Data Warehouse & Git Blobs)"]
    end

    subgraph CoreEngine["crates/cat-core (Shared Library)"]
        Auth["OAuth 2.0 (Ephemeral Listener)"]
        DriveREST["Drive REST API Client"]
        Quota["Software Quota Guard"]
    end

    subgraph Apps["The Desktop Application Suite (Standalone)"]
        App1["🐱 open-cat\n(Dropbox / Media Vault with Quota)"]
        App2["🗄️ cat-db\n(Document Database Manager)"]
        App3["🐾 fidget-cat\n(2D Pomodoro & Pet Widget)"]
        App4["📦 ware-cat\n(Data Warehouse / Cold Storage)"]
        App5["🐙 git-cat\n(Git on Google Drive & Remote Backup)"]
    end

    CoreEngine --> Apps
    App1 <--> OpenCatFolder
    App2 <--> CatDbFolder
    App3 -->|"Logs stats & sessions"| App2
    App4 <--> WareCatFolder
    App5 -->|"Stores Git Bundles & Packfiles"| App4
    App5 -->|"Stores Repo Metadata"| App2
```

---

### App 1: `open-cat` (The Shared File Vault)
* **Purpose**: A personal Dropbox/photo vault with drag-and-drop upload and video playback.
* **Features**:
  * Standalone `egui` GUI with a clean dropzone.
  * **5GB Quota Guard**: Hard software-enforced 5GB cap per shared folder via `_meta/quota.json`.
  * **Friend Sharing**: Distributable configuration key scoped strictly to the `open-cat` folder.
  * **Media Playback**: Downloads media to OS temp directory and securely launches native players (VLC/mpv).

---

### App 2: `cat-db` (The Document Database Manager)
* **Purpose**: A visual NoSQL Document Store GUI that turns a Google Drive folder into a database.

---

### App 3: `fidget-cat` (The Productivity Desk Pet)
* **Purpose**: A tiny, always-on-top 2D desktop widget for focus and fun.

---

### App 4: `ware-cat` (The Data Warehouse & Archive)
* **Purpose**: Cold storage, large immutable blob archiving, and analytical data lake on Google Drive.

---

### App 5: `git-cat` (Git Remote & Hosting on Google Drive)
* **Purpose**: Host private Git repositories directly on your Google Drive without third-party services.

---

## 3. Cargo Workspace Layout

```text
cat-ecosystem/
├── Cargo.toml
├── crates/
│   └── cat-core/               # Shared Drive API, OAuth, Quotas, Models
├── apps/
│   ├── open-cat/               # App 1: Vault GUI
│   ├── cat-db/                 # App 2: Document DB GUI
│   ├── fidget-cat/             # App 3: 2D Pomodoro & Pet
│   ├── ware-cat/               # App 4: Data Warehouse & Blobs
│   └── git-cat/                # App 5: Git Remote & Manager
└── README.md
```

### `Cargo.toml` (Root Workspace)
```toml
[workspace]
resolver = "2"
members = [
    "crates/cat-core",
    "apps/open-cat",
    "apps/cat-db",
    "apps/fidget-cat",
    "apps/ware-cat",
    "apps/git-cat",
]

[workspace.dependencies]
tokio = { version = "1.37", features = ["full"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
reqwest = { version = "0.12", features = ["json", "stream", "multipart"] }
eframe = "0.27"
egui = "0.27"
open = "5.1"
rfd = "0.14"
git2 = "0.18"
```

---

## 4. Technical Rules (Standalone App Standards)

1. **Authentication (Ephemeral Loopback)**:
   * The app MUST NOT run a persistent background web server.
   * To authenticate, the app binds a `std::net::TcpListener` to `127.0.0.1:0` (OS assigns a random free port).
   * The app extracts the port, generates the Google OAuth URL with `redirect_uri=http://127.0.0.1:<PORT>`, and opens the browser.
   * Upon receiving the HTTP callback, the listener processes the code and is immediately dropped.
2. **Quota Guard Formula**:
   $$\text{used\_bytes} + \text{incoming\_file\_bytes} \le \text{max\_allowed\_bytes}$$
   *If violated, reject immediately before initiating byte transfer.*
3. **Media Playback**:
   * The app MUST NOT proxy media via localhost.
   * Instead, fetch the file via Drive API `alt=media`, buffer it into `std::env::temp_dir()`, and launch the native OS media player against the local temp file.

---

## 5. Development Roadmap & Task Progression

### Phase 1: Shared Core (`crates/cat-core`)
- [ ] Initialize Cargo workspace (without daemon).
- [ ] Implement OAuth 2.0 PKCE flow using Ephemeral Ports (`127.0.0.1:0`).
- [ ] Implement Google Drive v3 REST Client (Folder create/find, list, chunked upload, download).
- [ ] Implement Quota Guard (`_quota.json` read/write/verify).

### Phase 2: The File Vault (`apps/open-cat`)
- [ ] `eframe` window with internal Tokio runtime and MPSC channels.
- [ ] Drag-and-drop dropzone & 5GB visual storage gauge.
- [ ] File grid with download and VLC temp-playback actions.

### Phase 3: Document Database (`apps/cat-db`)
- [ ] Empty folder initializer.
- [ ] Visual collection tree & JSON document inspector.

### Phase 4: Productivity Pet (`apps/fidget-cat`)
- [ ] Lightweight 2D floating window with custom `egui::Painter`.
- [ ] Pomodoro work/break timer state machine syncing to `cat-db`.

### Phase 5: Warehouse & Git (`apps/ware-cat` & `apps/git-cat`)
- [ ] Chunked archive manager.
- [ ] Git repository packfile/bundle exporter (`git2` crate).
