# Epic 1.2: The Synthesis Engine

## Goal
Connect the existing Character management system to the Python backend's TTS inference engine (`Chatterbox`). This allows users to generate audio from text using a character's reference voice.

## Requirements

### Functional Requirements
- **FR-1:** The backend MUST expose a `POST /synthesize` endpoint.
- **FR-2:** The endpoint MUST accept a JSON payload containing:
    - `text`: The text to synthesize.
    - `reference_audio_path`: The absolute path to the reference WAV file on the local system.
- **FR-3:** The backend MUST validate that `reference_audio_path` exists.
- **FR-4:** The backend MUST invoke the `Chatterbox` library to generate audio.
- **FR-5:** The backend MUST return the generated audio data (WAV format) in the response body (streaming or blob).
- **FR-6:** The backend MUST handle errors (e.g., file not found, inference failure) and return appropriate HTTP status codes (400, 500).

### Non-Functional Requirements
- **NFR-1:** Inference should be run in a way that doesn't permanently block the server (basic async support if possible, though Python GIL might be a factor).
- **NFR-2:** Latency should be minimized where possible.

## User Interface (Frontend)
*Note: This epic focuses on the **Backend** capability. Frontend integration is basic verification.*
- The frontend will need a way to trigger this endpoint (handled in generic API client or specific service).

## Acceptance Criteria
- **AC-1:** `POST /synthesize` exists and is reachable.
- **AC-2:** Sending valid text and a valid wav path returns a 200 OK and a WAV file body.
- **AC-3:** Sending an invalid wav path returns 400 Bad Request.
- **AC-4:** Sending an empty text returns 400 Bad Request.
- **AC-5:** The returned audio is a valid WAV file (can be played).

## Open Questions
- **Q1:** What is the exact package name for `Chatterbox`? (Assumed `chatterbox` for now).
- **Q2:** support for GPU acceleration? (MVP: CPU is fine).
