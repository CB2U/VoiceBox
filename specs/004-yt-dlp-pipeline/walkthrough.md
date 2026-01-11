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

## Error Handling Improvements (2026-01-11)

### Changes Made

**Backend Enhancements:**
- Added URL validation with regex patterns for YouTube URLs
- Added time format validation (supports HH:MM:SS, MM:SS, or seconds)
- Added time range validation (start < end)
- Implemented detailed logging throughout extraction process
- Enhanced error messages with specific guidance for common failures
- Added 2-minute timeout for downloads
- Return HTTP 400 for validation errors, HTTP 500 for server errors

**Frontend Enhancements:**
- Extract and display error details from HTTP response body
- Added client-side URL and field validation
- Improved error message styling with red background
- Clear errors when user starts typing
- Ensure loading state always clears on error
- Better network error messages

### Verification

All error scenarios tested successfully:
- ✅ Valid YouTube URL extracts correctly
- ✅ Invalid video ID returns clear error message
- ✅ Non-YouTube URL rejected with validation error
- ✅ Invalid time range (start > end) caught and reported
- ✅ Malformed URLs caught by client-side validation
- ✅ Empty fields validated before submission
- ✅ Backend offline shows connection error

**User Impact:** Users now receive immediate, actionable feedback instead of UI getting stuck in "Extracting..." state.

