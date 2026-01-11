# Implementation Plan - Epic 2.0: YouTube Integration

## Technical Approach

### 1. Backend (Python/FastAPI)
*   **New Dependency:** `yt-dlp`. Add to `backend/requirements.txt`.
*   **New Endpoint:** `POST /extract-from-youtube`.
    *   **Input:** `YouTubeRequest(url: str, start_time: str, end_time: str, character_id: str)`
    *   **Process:**
        1.  Validate URL and timestamps.
        2.  Construct `yt-dlp` command to download best audio to a temporary file.
        3.  Use `ffmpeg` (via `subprocess` or `ffmpeg-python` wrapper if simple enough, but raw subprocess likely sufficient) to trim and convert to WAV.
        4.  Move final WAV to character's audio folder.
        5.  Return `AudioFile(path: str, filename: str)`.
    *   **Error Handling:** Catch subprocess errors (download fail, ffmpeg fail) and return 400/500 with meaningful messages.

### 2. Frontend (Dioxus/Rust)
*   **UI Components:**
    *   Add "Import from YouTube" section to the Character details view (or a modal).
    *   Input field for URL.
    *   Input fields for Start/End (text or duration picker).
    *   "Extract" button.
*   **State Management:**
    *   `is_extracting` boolean for loading state.
    *   Error message display.
*   **Integration:**
    *   Call connection `extract_youtube(url, start, end, char_id)`.
    *   On success, refresh the character's audio list.

## Data Contracts
### API Request
```json
{
  "url": "https://www.youtube.com/watch?v=...",
  "start_time": "00:10",
  "end_time": "00:15",
  "character_id": "char_123"
}
```

### API Response
```json
{
  "file_path": "/path/to/voicebox/characters/char_123/audio/clip_123.wav",
  "filename": "clip_123.wav"
}
```

## Testing Strategy
### Automated Tests
*   **Backend:** Unit test the extraction logic using a dummy/mock `yt-dlp` (mocking the subprocess call to avoid network reliance in CI/basic tests).
*   **Integration:** Test the actual `yt-dlp` call with a known short public domain video (if possible and reliable).

### Manual Verification
*   Test with a real YouTube video.
*   Test with invalid URL.
*   Test with invalid timestamps (Start > End).
*   Verify audio playback after extraction.
