# Plan: Epic 1.2 - The Synthesis Engine

## Goal Description
Implement the `POST /synthesize` endpoint in the Python backend to enable text-to-speech generation using the `Chatterbox` library. This involves setting up the backend dependency, creating a wrapper for the inference engine, and exposing the functionality via FastAPI.

## User Review Required
> [!IMPORTANT]
> **Dependency Confirmation**: We are assuming the library is named `chatterbox`. Integrating a specific TTS model might require downloading model weights (~100MB+). This plan assumes automatic downloading or that weights are present.

## Proposed Changes

### Backend
#### [MODIFY] [requirements.txt](file:///mnt/Storage/Documents/Projects/VoiceBox/backend/requirements.txt)
- Add `chatterbox` (and implementation-specific dependencies like `torch`, `numpy` if not included).

#### [NEW] [backend/src/engine.py](file:///mnt/Storage/Documents/Projects/VoiceBox/backend/src/engine.py)
- Create `SynthesisEngine` class.
- Method `generate(text: str, reference_audio_path: str) -> BytesIO`.
- Handle model loading (singleton pattern logic if expensive).

#### [MODIFY] [backend/src/main.py](file:///mnt/Storage/Documents/Projects/VoiceBox/backend/src/main.py)
- Import `SynthesisEngine`.
- Define Pydantic model `SynthesisRequest`.
- Add `@app.post("/synthesize")` endpoint.
- Return `StreamingResponse` or `FileResponse` with media type `audio/wav`.

## Verification Plan

### Automated Tests
- **Unit Test**: Test `SynthesisEngine` with a dummy mock if pure unit test.
- **Integration Test**:
    - Start backend.
    - Use `curl` to send a request with a sample text and a sample WAV file.
    - Check response code is 200.
    - Check response content-type is audio/wav.
    - Verify output file size > 0.

### Manual Verification
- Play the generated audio file to ensure it sounds like speech.
