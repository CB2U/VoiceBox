# Epic 2.0: YouTube Integration

## 1. Summary
This epic implements the "YouTube Extraction" feature (FR-3), allowing users to extract specific audio segments from YouTube videos to use as voice reference samples. This integrates `yt-dlp` for downloading and `ffmpeg` for processing.

## 2. Requirements

### Functional Requirements
*   **FR-3.1 (Input):** User provides a YouTube URL, Start Time, and End Time.
*   **FR-3.2 (Download):** Backend downloads the audio stream using `yt-dlp`.
*   **FR-3.3 (Process):** Backend trims the audio to the specified timestamps and converts it to WAV format using `ffmpeg`.
*   **FR-3.4 (Save):** The resulting WAV file is saved to the projects directory and associated with the selected character.
*   **FR-3.5 (Error Handling):** Graceful handling of invalid URLs, download failures (e.g., throttling/blocking), and processing errors.

### Non-Functional Requirements
*   **NFR-2 (Performance):** The UI must remain responsive during the download process (Async/Non-blocking).
*   **NFR-3 (Privacy):** Processing happens locally.
*   **Dependencies:** `yt-dlp` (Python package), `ffmpeg` (System binary).

## 3. Acceptance Criteria

### AC-3
**Goal:** User can input a YouTube URL with timestamps and save a WAV clip.
*   [ ] User can enter a valid YouTube URL.
*   [ ] User can specify start and end timestamps (e.g., "00:10" to "00:15").
*   [ ] Clicking "Extract" triggers the backend process.
*   [ ] A visual indicator (loading spinner) appears during processing.
*   [ ] Upon success, a new audio file appears in the Character's voice list.
*   [ ] The audio file is exactly the length specified by the timestamps (within reasonable tolerance).
*   [ ] If the download fails (e.g., invalid URL), an error message is displayed to the user.

## 4. Open Questions
*   **Q1:** How do we handle `ffmpeg` dependency?
    *   *Assumption:* User must have `ffmpeg` installed on their system path, or we bundle a static binary (Out of scope for this MVP? Stick to system path for Linux first).
*   **Q2:** Validation for timestamps?
    *   *Decision:* Basic validation > 0 and Start < End. `yt-dlp` handles out-of-bounds generally or errors.

## 5. Risks
*   **YouTube Throttling:** `yt-dlp` cat and mouse game.
    *   *Mitigation:* Use latest `yt-dlp`. If it fails, report clear error to user to try manual upload.
