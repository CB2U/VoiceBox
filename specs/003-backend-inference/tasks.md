
# Tasks: Epic 1.2 - The Synthesis Engine

## 1. Preparation
- [x] **Dependency Setup** <!-- id: 0 -->
    - Update `backend/requirements.txt` with `chatterbox-tts` (and any others like `numpy`, `scipy`, `soundfile` if needed).
    - Run `pip install -r backend/requirements.txt` to install.
    - Validate installation by running a small python script that imports `chatterbox`.
- [x] **Model Download** <!-- id: 1 -->
    - Check if `Chatterbox` needs explicit model download steps. (Handled via `from_pretrained`).
    - Validated model load with script.

## 2. Implementation
- [x] **Create Synthesis Engine** <!-- id: 2 -->
    - Create `backend/src/engine.py`.
    - Implement `SynthesisEngine` class as a singleton (or global instance) to hold the model.
    - Implement `generate(text, ref_audio_path)` method.
- [x] **Create API Endpoint** <!-- id: 3 -->
    - Update `backend/src/main.py`.
    - Add `POST /synthesize` endpoint.
    - Integrate `SynthesisEngine`.
    - Handle errors (file not found, model error).

## 3. Verification
- [x] **Manual Verification** <!-- id: 4 -->
    - Create `backend/verify_synthesis.py` (or use curl).
    - Send a request with a sample/dummy wav file.
    - Save the output to `output.wav` (buffer checked in test).
    - Listen to `output.wav` to confirm it worked (Validated via header and size).
- [x] **Cleanup** <!-- id: 5 -->
    - Remove temporary verification scripts.
