# Voice Box Roadmap

## MVP Decisions (Locked)

* **Architecture:** Dioxus (Rust) Frontend <-> HTTP <-> FastAPI (Python) Backend.
* **Scripting:** Text-based parsing (`[Name]: Text`).
* **Voice Source:** Local Files + YouTube (via `yt-dlp`).
* **Storage:** Local file system (JSON for metadata, folder for audio clips).

## Non-Negotiables Summary

* Local-only processing (Privacy).
* Async UI (Non-blocking).
* Linux First.

## Breakpoint Map

* **BP0 (Foundation):** Repo setup, Hello World Dioxus, Hello World FastAPI, IPC established.
* **BP1 (Core Loop):** Character creation, Audio Import (Local), Text-to-Speech (TTS) connection.
* **BP2 (Advanced Input):** YouTube Extraction integration.
* **BP3 (Polish):** Script Editor parsing, Audio Export, UI Polish.
* **BP4 (Scalability):** Project Workspace isolation, Real-time Progress feedback.

## Epics

### 1.0 Foundation & Architecture

* **Goal:** Establish the communication bridge between Rust and Python.
* **Scope:** Project structure, basic HTTP client in Rust, Ping endpoint in Python.
* **Exit Criteria:** Dioxus app displays "Backend Online" status from FastAPI.
* **Target ACs:** AC-1, AC-6

### 1.1 Character Management

* **Goal:** Allow users to define "Voices."
* **Scope:** UI for Character list, Add/Remove Character, Assign local WAV file.
* **Dependencies:** 1.0
* **Exit Criteria:** User can save a character with a name and image path to local JSON.
* **Target ACs:** AC-1, AC-2

### 1.2 The Synthesis Engine

* **Goal:** Connect Character audio to Chatterbox inference.
* **Scope:** Python wrapper for Chatterbox, POST endpoint to receive Text + Audio Path.
* **Dependencies:** 1.1
* **Exit Criteria:** Sending text + wav path returns a synthesized wav file.
* **Target ACs:** AC-4

### 2.0 YouTube Integration

* **Goal:** Streamline sample acquisition.
* **Scope:** UI for URL input + Timestamps. Backend logic to run `yt-dlp` and ffmpeg trim.
* **Dependencies:** 1.2
* **Exit Criteria:** YouTube link -> Saved WAV file associated with Character.
* **Target ACs:** AC-3

### 2.1 Script Editor & Parsing

* **Goal:** The main workspace for the DM.
* **Scope:** Text area, Regex parser for `[Name]:`, sequential synthesis logic.
* **Dependencies:** 1.2
* **Exit Criteria:** Parsing a 3-line script triggers 3 API calls and sequences the audio.
* **Target ACs:** AC-5, AC-7

### 3.0 Project Management

* **Goal:** Organize characters and settings by campaign.
* **Scope:** Project CRUD, automatic migration, file system isolation.
* **Dependencies:** 1.1, 1.2
* **Exit Criteria:** User can switch between two projects with distinct character lists.
* **Target ACs:** AC-9

### 3.1 Progress Indicators

* **Goal:** Provide real-time feedback for long-running tasks.
* **Scope:** YouTube download progress (SSE), Synthesis progress (frontend loop).
* **Dependencies:** 2.0, 2.1
* **Exit Criteria:** Progress bars reach 100% in sync with task completion.
* **Target ACs:** AC-10, AC-11

## Post-MVP

* Waveform visualization.
* Visual "Block" editor for scripts.
* Background music mixing.
