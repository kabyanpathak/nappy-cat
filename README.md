# 🐾 The Cat Ecosystem

> **A 100% Rust, Low-Memory Personal Cloud & Productivity Suite Powered by Google Drive (5TB)**

Welcome to the **Cat Ecosystem**! This repository houses a modular suite of lightweight, native desktop applications and microservices. Instead of relying on expensive servers or heavy Electron apps, this ecosystem uses **Google Drive** as a personal cloud backend, enforcing strict software quotas, and running entirely on featherlight Rust binaries (with background daemons utilizing under 30MB of RAM).

---

## 🏗️ The Ecosystem Architecture

The suite is designed as a unified Cargo workspace containing shared core libraries and independent, highly cohesive desktop micro-apps.

### ⚙️ Core Engine & Background Services
* **`cat-core`** (`crates/cat-core`): The shared engine powering the entire ecosystem. Handles OAuth 2.0 PKCE authentication with the strictly isolated `drive.file` scope, Google Drive REST API interactions, and software-enforced quota guards.
* **`cat-daemon`** (`apps/cat-daemon`): A lightweight local Axum background server (`127.0.0.1:4040`). It manages OAuth loopbacks and acts as a zero-copy HTTP `Range` proxy for streaming media directly from Google Drive into native media players (like VLC).

---

### 🐈 The Microservice Suite

#### 1. `open-cat` (The Shared File Vault)
A personal Dropbox and media vault featuring a clean, responsive `egui` drag-and-drop interface.
* **Quota Guard:** Hard software-enforced 5GB cap per shared folder.
* **Media Streaming:** Streams video directly to VLC without downloading the entire file.
* **Social Sharing:** Distributable configuration keys for securely sharing vaults with friends.

#### 2. `cat-db` (The Document Database Manager)
A visual NoSQL Document Store GUI that transforms a standard Google Drive folder into a database.
* **Performance:** Fast in-memory manifest caching for sub-millisecond filtering.
* **Role:** Acts as the central metadata registry and JSON data store for the rest of the apps in the ecosystem.

#### 3. `fidget-cat` (The Productivity Desk Pet)
A tiny, always-on-top 2D desktop widget designed for focus and fun.
* **Features:** An animated sleeping cat you can interact with, combined with a customizable Pomodoro work/break timer.
* **Ecosystem Hook:** Automatically syncs your focus session logs, productivity streaks, and pet happiness levels seamlessly into `cat-db`.

#### 4. `ware-cat` (The Data Warehouse & Archive)
Cold storage, large immutable blob archiving, and an analytical data lake on Google Drive.
* **Features:** Manages append-only JSONL / Parquet event logs, provides batch upload, and executes SHA-256 integrity verification and deduplication.

#### 5. `git-cat` (Git Remote on Google Drive)
Host private Git repositories directly on your Google Drive without relying on third-party hosting services.
* **How it works:** Utilizes Git's native bundle mechanism (`git bundle`). It stores immutable packfiles inside `ware-cat`, while branches, tags, and commit metadata are indexed beautifully in `cat-db`.

---

## 🛠️ Technology Stack
* **Language:** 100% Rust 🦀
* **GUI:** `eframe` / `egui` (Immediate Mode, Native OS integration)
* **Async Runtime:** `tokio`
* **Networking:** `reqwest` (Client) & `axum` (Local Proxy Daemon)
* **Backend:** Google Drive API v3 (REST)

## 🚀 Workspace Layout

```text
nappy-cat/
├── crates/
│   └── cat-core/       # Shared Drive API, OAuth, Quotas
├── apps/
│   ├── open-cat/       # App 1: Vault GUI
│   ├── cat-db/         # App 2: Document DB GUI
│   ├── fidget-cat/     # App 3: 2D Pomodoro & Pet
│   ├── ware-cat/       # App 4: Data Warehouse & Blobs
│   ├── git-cat/        # App 5: Git Remote & Manager
│   └── cat-daemon/     # Background local Axum server
└── tasks/              # Jira-style architecture tracking
```

## 💻 Getting Started

This project uses standard Cargo workspace commands. Assuming you have Rust installed:

```bash
# Verify the entire workspace builds cleanly
cargo check --workspace

# Run the background daemon (Proxy & OAuth handler)
cargo run --bin cat-daemon

# Run the Open-Cat GUI
cargo run --bin open-cat
```
