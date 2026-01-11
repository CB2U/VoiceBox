# Epic 3.1: Progress Indicators - Specification

## 1. Header
- **Title:** Progress Indicators
- **Roadmap anchor:** 3.1
- **Priority:** P2
- **Type:** Enhancement
- **Target area:** Frontend (UI) + Backend (Async/Status reporting)
- **Target Acceptance Criteria:** AC-10, AC-11 (New)

## 2. Problem Statement
Users currently initiate long-running tasks like "YouTube Extract Audio" and "Synthesize Script" without knowing how long they will take or if they are progressing. This leads to a poor user experience and uncertainty about whether the app has hung.

## 3. Goals and Non-Goals
**Goals:**
- Add real-time progress bars for the YouTube extraction process.
- Add real-time progress bars for the script synthesis process.
- Ensure the progress is displayed in the relevant UI sections (Youtube Extractor component and Script Editor).

**Non-Goals:**
- System-wide progress notification tray.
- Estimated Time of Arrival (ETA) calculations (unless provided easily by `yt-dlp`).
- Progress for local file imports (usually too fast to matter).

## 4. User Stories
- **US-1:** As a DM, I want to see a progress bar while the YouTube video is being downloaded and processed so I know it hasn't crashed.
- **US-2:** As a DM, I want to see a progress bar (e.g., "Line 4 of 10") during script synthesis so I can gauge when the preview will be ready.

## 5. Scope
### In-Scope
- **YouTube Extraction:** Real-time percentage update based on `yt-dlp` stdout.
- **Script Synthesis:** Line-by-line percentage update (progress = completed_lines / total_lines).
- **UI Components:** Progress bar component (Dioxus).

### Out-of-Scope
- Multi-step progress (e.g., Download -> Trim -> Convert). Just one aggregate bar is enough for MVP.

## 6. Requirements
### Functional Requirements
- **FR-1:** The backend must provide a mechanism to stream or poll progress updates for long-running tasks.
- **FR-2:** The frontend must update the UI reactively when progress data is received.
- **FR-3:** Progress bars must reach 100% (or disappear) when the task is complete.
- **FR-4:** Errors during processing must stop the progress bar and show an error state.

### Non-Functional Requirements
- **NFR-1 (Responsiveness):** Progress updates should be frequent enough to feel "live" (e.g., at least once per second).
- **NFR-2 (Non-blocking):** Reporting progress must not introduce significant overhead to the processing tasks.

## 7. Acceptance Criteria
### AC-10: YouTube Download Progress
- **Given** a valid YouTube URL and timestamps.
- **When** I click "Extract".
- **Then** a progress bar appears showing the download percentage.
- **And** it reaches 100% before the file is added to the character.

### AC-11: Synthesis Progress
- **Given** a 10-line script.
- **When** I click "Synthesize".
- **Then** I see a progress indicator showing the current line being processed (e.g., "40% complete").

## 8. Technical Considerations
- **Communication:**
  - **Option A (WebSockets):** Robust for real-time, but might be overkill.
  - **Option B (Server-Sent Events - SSE):** Perfect for one-way status updates from Backend to Frontend.
  - **Option C (Polling):** Simple to implement but less efficient.
  - *Recommendation:* Use **Server-Sent Events (SSE)** for YouTube extraction. For Synthesis (which is initiated by the frontend in a loop), the frontend can update its own progress bar since it controls the loop.

- **YouTube Service:** Needs to parse `yt-dlp` output in real-time.
- **Frontend:** Dioxus state needs to hold `progress` (0.0 to 1.0).

## 9. Risks and Mitigations
- **Risk:** `yt-dlp` output formats change, breaking the parser.
  - **Mitigation:** Use a robust regex for percentage extraction and fallback to an indeterminate spinner if parsing fails.
