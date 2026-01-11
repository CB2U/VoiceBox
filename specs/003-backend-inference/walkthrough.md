
# Walkthrough - Epic 1.2: Synthesis Engine

## Overview
Implemented the backend synthesis engine using the `chatterbox-tts` library. The system now exposes a `POST /synthesize` endpoint that generates audio from text using a reference audio file for voice cloning/style transfer.

## Changes

### Backend
- **Requirements**: Added `chatterbox-tts` (which pulls in `torch` and `soundfile`).
- **Engine**: Created `backend/src/engine.py` with `SynthesisEngine` class.
    - Implements singleton pattern.
    - Loads `ChatterboxTTS` model on first use (GPU enabled if available).
    - Generates audio and returns `BytesIO` WAV buffer.
- **API**: Updated `backend/src/main.py` with `POST /synthesize`.
    - Accepts `text` and `reference_audio_path`.
    - Validates inputs (non-empty text, existing file).
    - Returns `audio/wav` streaming response.

## Verification Results

### Automated Integration Test
Ran `verify_synthesis.py` which performed the following checks against the running backend:

1.  **Health Check**: Verified server is running (200 OK).
2.  **Success Case**:
    - Generates audio with valid text and dummy reference WAV.
    - Verified HTTP 200, `Content-Type: audio/wav`.
    - Verified response body is a valid WAV file (Header validation).
3.  **Invalid Reference File**:
    - Sending a non-existent path returns 400 Bad Request.
4.  **Empty Text**:
    - Sending empty text returns 400 Bad Request.

**Test Output Log:**
```text
Health check passed.
Created dummy ref: /mnt/Storage/Documents/Projects/VoiceBox/backend/test_ref.wav

Testing Success Case...
Status: 200
Content-Type: audio/wav
Body size: 157484 bytes
SUCCESS: Valid WAV received.
SUCCESS: WAV Header detected.

Testing Invalid File Case...
SUCCESS: Got 400 as expected. Msg: {"detail":"Reference audio not found: /non/existent/path.wav"}

Testing Empty Text Case...
SUCCESS: Got 400 as expected. Msg: {"detail":"Text cannot be empty"}
```

## Setup Notes
- The `chatterbox-tts` model (~2-3GB) is downloaded automatically on the first run (or via the engine initialization).
- GPU acceleration is enabled automatically if available (verified usage of `cuda`).
