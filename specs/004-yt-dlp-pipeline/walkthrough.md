# Walkthrough - Epic 2.0: YouTube Integration

## Changes

### Backend
*   **Requirements**: Added `yt-dlp`.
*   **Service**: Created `YouTubeService` in `backend/src/services/youtube_service.py` to handle `yt-dlp` execution.
    *   Includes logic to resolve `yt-dlp` executable path from venv if not in system PATH.
    *   Implements `extract_audio` with `start_time` and `end_time` parameters.
*   **Router**: Created `backend/src/routers/youtube.py` exposing `POST /extract-from-youtube`.
*   **Main**: Registered the new router in `backend/src/main.py`.
*   **Mocking**: Temporarily mocked `chatterbox` dependency in `backend/src/engine.py` to allow backend startup for testing.

### Frontend
*   **Component**: Created `YouTubeImport` in `frontend/src/components/youtube_import.rs`.
    *   UI fields: URL, Start Time, End Time.
    *   Action: "Extract Audio" button triggering the backend API.
*   **Integration**: Added `YouTubeImport` to `Editor` component in `frontend/src/components/editor.rs` alongside the existing file picker.

## Verification Results

### Backend Manual Verification
Verified the API using `curl` against a live YouTube video ("Me at the zoo").

**Command:**
```bash
curl -X POST http://localhost:8000/extract-from-youtube \
-H "Content-Type: application/json" \
-d '{"url": "https://www.youtube.com/watch?v=jNQXAC9IVRw", "start_time": "00:00:00", "end_time": "00:00:05", "character_id": "test_char"}'
```

**Response:**
```json
{
  "file_path": "/mnt/Storage/Documents/Projects/VoiceBox/frontend/data/characters/test_char/audio/yt_eb079b46.wav",
  "filename": "yt_eb079b46.wav"
}
```
*   **Status**: Success (200 OK)
*   **Output**: File created at specified path (checked size ~950KB).

### Frontend Verification
*   **Compilation**: `cargo check` passed successfully.
