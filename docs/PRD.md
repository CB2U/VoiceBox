# Product Requirements Document (PRD)

## 1. Summary

**Voice Box** is a desktop application designed to streamline the creation of audio recaps for Dungeons & Dragons campaigns. It leverages AI voice cloning (via `Chatterbox`) to synthesize speech for various characters. Users manage a library of NPCs/Players, associate reference audio clips (uploaded or extracted from YouTube), and script dialogues which are then synthesized into a cohesive audio file.

## 2. Problem Statement

DMs often want high-quality audio recaps for their sessions but lack the tools to easily "act out" multiple voices. Existing voice cloning tools are often cloud-based (expensive/privacy concerns) or command-line only (hard to use). There is no dedicated studio tool that combines script management, local voice cloning, and audio extraction into a single workflow.

## 3. Target Users

* **Primary:** Dungeon Masters (D&D, Pathfinder, etc.) who want immersive audio recaps.
* **Secondary:** Content creators making machinim or audio dramas requiring multiple distinct voices on a budget.

## 4. User Stories

* **US-1:** As a DM, I want to create a character profile (Name, Avatar) so I can organize my voices.
* **US-2:** As a DM, I want to drag-and-drop an audio file (WAV/MP3) onto a character to set their voice reference.
* **US-3:** As a DM, I want to paste a YouTube link with start/end timestamps to extract a voice sample without downloading the whole video manually.
* **US-4:** As a DM, I want to write a script using a simple format (e.g., `[Gandalf]: Hello`) to assign lines to characters.
* **US-5:** As a DM, I want to click "Synthesize" to generate audio for the script using the local backend.
* **US-6:** As a DM, I want to export the final combined audio file to play during my game.

## 5. Requirements

### Functional Requirements (FR)

* **FR-1 (Character Mgmt):** Create, Read, Update, Delete (CRUD) characters.
* **FR-2 (Audio Import):** Support drag-and-drop import of WAV/MP3 files < 10MB.
* **FR-3 (YouTube Extract):** Input YT URL + Start/End time -> Download audio -> Trim -> Convert to WAV -> Save to Character.
* **FR-4 (Scripting):** Text editor supporting parsing of `[Character Name]: Text` syntax.
* **FR-5 (Synthesis):** Send Text + Reference Audio Path to Python backend via HTTP; receive synthesized audio blob/path back.
* **FR-6 (Playback):** Audio player controls (Play/Pause/Seek) for synthesized results.
* **FR-7 (Export):** Save combined audio output to disk (WAV/MP3).
* **FR-8 (Backend):** Local Python server (FastAPI) wrapping `Chatterbox` functionality.

### Non-Functional Requirements (NFR)

* **NFR-1 (Platform):** Primary: Linux (Pop!_OS). Secondary: Cross-platform ready (Windows/Mac).
* **NFR-2 (Performance):** UI must not freeze during synthesis (Async operations).
* **NFR-3 (Privacy):** All processing must happen locally; no audio sent to cloud APIs.
* **NFR-4 (Tech Stack):** Frontend: Dioxus (Rust). Backend: Python (Chatterbox).

## 6. Acceptance Criteria (AC)

* **AC-1:** User can launch the Dioxus app and see a 2-panel layout (Library vs. Editor). (Maps to FR-1, FR-4)
* **AC-2:** User can create a character "Gandalf" and assign a local `.wav` file to it. (Maps to FR-1, FR-2)
* **AC-3:** User can input a YouTube URL with timestamps (e.g., 0:10-0:15), and the app saves a 5-second WAV clip to the character. (Maps to FR-3)
* **AC-4:** Backend server receives a POST request with text and reference audio path, returning a valid audio file. (Maps to FR-5, FR-8)
* **AC-5:** Script parser correctly identifies lines starting with `[Name]:` and assigns the corresponding character's voice. (Maps to FR-4)
* **AC-6:** The "Synthesize" button is disabled if the backend is unreachable. (Maps to NFR-2)

## 7. MVP Scope

### In-Scope

* Dioxus Desktop App (Linux target).
* Local Python Backend (API wrapper for Chatterbox).
* Manual start/stop of Python backend (for now).
* YouTube audio extraction (yt-dlp integration).
* Script format: `[Name]: Text`.
* Single-track export.

### Out-of-Scope

* Multi-track audio editing (background music, sound effects).
* Cloud sync.
* Rich text editor (bold/italics).
* Automatic backend installation/management (user must have Python/Chatterbox installed).

## 8. Risks and Mitigations

* **Risk:** `Chatterbox` dependency versioning issues.
  * *Mitigation:* Use a Python virtual environment (`venv`) and a `requirements.txt`.
* **Risk:** YouTube download blocking/throttling.
  * *Mitigation:* Use `yt-dlp` (regularly updated) and handle errors gracefully in UI.
* **Risk:** Synthesis latency.
  * *Mitigation:* Implement loading states/spinners in Dioxus so the user knows work is happening.

## 9. Open Questions

* [NEEDS CLARIFICATION: exact python-rust IPC security token requirements? (None for MVP local-only?)]
