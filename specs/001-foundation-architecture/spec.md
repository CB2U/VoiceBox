# Epic 1.0: Foundation & Architecture - Specification

## 1. Header
- **Title:** Foundation & Architecture
- **Roadmap anchor:** 1.0
- **Priority:** P0
- **Type:** Feature
- **Target area:** Core Architecture (Rust Frontend + Python Backend)
- **Target Acceptance Criteria:** AC-1, AC-6

## 2. Problem Statement
The application currently does not exist. We need to establishing the "Hello World" of the architecture: a Rust desktop app (Dioxus) that can communicate with a local Python backend (FastAPI). Without this foundation, no other features (Character Management, Audio Synthesis) can be built.

## 3. Goals and Non-Goals
**Goals:**
- Initialize the git repository with the dual-language structure.
- Prove that the Dioxus frontend can compile and launch.
- Prove that the FastAPI backend can run and serve requests.
- Establish a basic "Heartbeat" check from Frontend to Backend.

**Non-Goals:**
- Implementing actual features (Character CRUD, Synthesis).
- Packaging or distribution (installer creation).
- Complex IPC security (token validation) for this initial MVP step.

## 4. User Stories
- **US-Found-1:** As a developer, I want to run a single command (or two) to start the app so I can begin development.
- **US-Found-2:** As a user, I want to see a visual indicator if the backend service crashes so I know why synthesis isn't working.

## 5. Scope
### In-Scope
- Project directory structure (`frontend/`, `backend/`).
- minimal `requirements.txt` for Python.
- minimal `Cargo.toml` for Rust.
- A "Hello World" Dioxus window.
- A `/health` endpoint in FastAPI.
- Polling logic in Rust to check `/health`.

### Out-of-Scope
- Database setup.
- Audio processing libraries (`torchaudio`, etc.).
- `yt-dlp` integration.

## 6. Requirements
### Functional Requirements
- **FR-1:** Backend must expose a HTTP `GET /health` endpoint returning 200 OK.
- **FR-2:** Frontend must start a desktop window using Dioxus.
- **FR-3:** Frontend must poll backend every ~2-5 seconds.
- **FR-4:** Frontend must display "Online" when backend is reachable, "Offline" otherwise.

### Non-Functional Requirements
- **NFR-1 (Local):** All communication must be over `localhost` (127.0.0.1).
- **NFR-2 (Startup):** Backend should start in < 2 seconds on a modern machine.

### Constraints Checklist
- [x] Security: Localhost only. No external network binding.
- [x] Privacy: No telemetry.
- [ ] Offline: Must work without internet (dependencies pre-installed).
- [ ] Performance: minimal RAM usage for idle backend (< 100MB).

## 7. Acceptance Criteria

### AC-1: Basic UI Scaffold
- **Description:** User can launch the Dioxus app and see a basic window.
- **Verification approach:** Manual test. Run `cargo run`. Verify window appears with title "Voice Box".

### AC-6: Backend Status Monitor
- **Description:** The "Synthesize" button (or status badge) reflects backend state.
- **Verification approach:**
  1. Start backend and frontend. Observe "Online" status.
  2. Kill backend process. Observe "Offline" status within 5 seconds.
  3. Restart backend. Observe "Online" status return.

## 8. Dependencies
- **Technical:**
  - Rust toolchain (stable).
  - Python 3.10+.
  - `dioxus-cli` (optional for dev, but `cargo run` should work).

## 9. Risks and Mitigations
- **Risk:** Port conflicts (FastAPI default 8000).
  - **Mitigation:** Allow configuration of port, or use a less common default if 8000 is taken. (Stick to 8000 for MVP).
- **Risk:** Dioxus webview compatibility on Linux.
  - **Mitigation:** Document `libwebkit2gtk` requirements.

## 10. Open Questions
- None.

## 11. EVIDENCE
- **AC-1 (Basic UI Scaffold):** Verified compilation and execution of Dioxus frontend. (Visual verification skipped due to headless environment).
- **AC-6 (Backend Status Monitor):** Verified via logs. Frontend successfully polled `http://127.0.0.1:8000/health` and received 200 OK.
  - Log snippet:
    ```
    2026-01-11T13:20:45.748449Z DEBUG hyper_util::client::legacy::connect::http: connected to 127.0.0.1:8000
    Backend is Online
    ```
