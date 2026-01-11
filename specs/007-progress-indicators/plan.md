# Epic 3.1: Progress Indicators - Implementation Plan

## Technical Overview
We will implement real-time progress reporting for two distinct workflows.

### 1. YouTube Extraction (Backend-to-Frontend)
YouTube extraction happens on the backend. Since Dioxus communicates with FastAPI via HTTP, the simplest way to get real-time progress is via **Server-Sent Events (SSE)**.

- **Backend:**
  - `YouTubeService.extract_audio` will be modified to accept a callback or use a generator to yield status lines.
  - A new endpoint `GET /youtube/progress/{task_id}` will stream event data.
  - We'll use `yt-dlp`'s `--newline` and `--progress` flags to parse the percentage.
- **Frontend:**
  - Use `EventSource` (via JS interop or a Rust crate) to listen to the progress stream.
  - Update a local `Signal<f32>` to drive the UI progress bar.

### 2. Script Synthesis (Frontend-driven)
Script synthesis is already a loop in the Dioxus frontend (it calls `/synthesize` for each line).
- **Frontend:**
  - Calculate `progress = current_line_index / total_lines`.
  - Display this in a `ProgressBar` component within the `ScriptEditor`.

## Proposed Changes

### [Component] Backend - YouTube Service
- Update `extract_audio` to run in a non-blocking process and capture `stdout`.
- Add a regex-based parser for `[download]  xx.x% of ...`.

### [Component] Backend - Routes
- Add `GET /youtube/status/{request_id}` (SSE).

### [Component] Frontend - UI
- Create a `ProgressBar` reusable component.
- Add progress signals to `YoutubeExtractor` and `ScriptEditor`.

## Verification Plan

### Automated Tests
- **Backend (Python):** Mock `subprocess.run` to simulate `yt-dlp` output and verify the parser correctly extracts the percentage.

### Manual Verification
1.  **YouTube:** Paste a URL, click Extract. Observe steady movement of the progress bar.
2.  **Synthesis:** Load a script with 10 lines, click Synthesize. Observe the progress bar stepping up by 10% for each line synthesized.
3.  **Error Case:** Disconnect internet during YouTube download; verify the progress bar stops and an error message is shown.
