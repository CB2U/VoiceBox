# Tasks: Epic 4.0 - Audio Post-Processing

## Phase 1: Backend - Dependencies and Models

### Task 1.1: Add Audio Processing Dependencies
- [ ] Update `requirements.txt` with librosa, soundfile, pydub, numpy
- [ ] Test dependency installation in virtual environment
- [ ] Verify FFmpeg is available (required by pydub)

### Task 1.2: Create Audio Processing Models
- [ ] Create `backend/src/models/audio_processing.py`
- [ ] Define `AudioProcessRequest` model
- [ ] Define `AudioProcessResponse` model
- [ ] Define `AudioPreviewRequest` model
- [ ] Define `AudioPreviewResponse` model
- [ ] Define `AudioFormat` enum (WAV, MP3, OGG, FLAC)

## Phase 2: Backend - Audio Processing Service

### Task 2.1: Implement Core Audio Processing
- [ ] Create `backend/src/services/audio_processing_service.py`
- [ ] Implement `load_audio_file(path: str) -> Tuple[np.ndarray, int]`
- [ ] Implement `apply_pitch_shift(audio: np.ndarray, sr: int, semitones: float) -> np.ndarray`
- [ ] Implement `apply_speed_change(audio: np.ndarray, sr: int, speed_factor: float) -> np.ndarray`
- [ ] Add error handling for invalid audio files

### Task 2.2: Implement Format Conversion
- [ ] Implement `convert_to_wav(audio: np.ndarray, sr: int) -> AudioSegment`
- [ ] Implement `convert_to_mp3(audio_segment: AudioSegment, bitrate: str) -> bytes`
- [ ] Implement `convert_to_ogg(audio_segment: AudioSegment, quality: int) -> bytes`
- [ ] Implement `convert_to_flac(audio_segment: AudioSegment) -> bytes`
- [ ] Add quality settings validation

### Task 2.3: Implement Preview Generation
- [ ] Implement `generate_preview(audio_path: str, effects: dict, duration: float) -> str`
- [ ] Create temporary preview directory
- [ ] Implement preview file cleanup (delete after 5 minutes)
- [ ] Add preview caching to avoid regeneration

### Task 2.4: Implement Full File Processing
- [ ] Implement `process_audio_file(audio_path: str, effects: dict, output_path: str) -> dict`
- [ ] Generate descriptive filename based on effects
- [ ] Save processed file to project directory
- [ ] Return metadata (duration, file size, format)

### Task 2.5: Integrate with History Service
- [ ] Add method to create history entry for processed file
- [ ] Link processed file to original file in metadata
- [ ] Update history service to support "processed_from" field

## Phase 3: Backend - API Endpoints

### Task 3.1: Create Audio Router
- [ ] Create `backend/src/routers/audio.py`
- [ ] Set up FastAPI router with `/audio` prefix
- [ ] Add dependency injection for AudioProcessingService

### Task 3.2: Implement Preview Endpoint
- [ ] Implement `POST /audio/preview`
- [ ] Validate request parameters
- [ ] Call preview generation service
- [ ] Return preview file URL
- [ ] Add error handling and logging

### Task 3.3: Implement Process Endpoint
- [ ] Implement `POST /audio/process`
- [ ] Validate request parameters
- [ ] Call full file processing service
- [ ] Create history entry for processed file
- [ ] Return processed file metadata
- [ ] Add error handling and logging

### Task 3.4: Implement Utility Endpoints
- [ ] Implement `GET /audio/formats` to list supported formats
- [ ] Implement `GET /audio/file/{file_path}` to serve audio files
- [ ] Add CORS headers for audio file serving

### Task 3.5: Register Router
- [ ] Update `backend/src/main.py` to include audio router
- [ ] Test all endpoints with curl or Postman
- [ ] Verify error responses

## Phase 4: Frontend - Models and Services

### Task 4.1: Create Audio Processing Models
- [ ] Create `frontend/src/models/audio_processing.rs`
- [ ] Define `AudioProcessRequest` struct
- [ ] Define `AudioProcessResponse` struct
- [ ] Define `AudioPreviewRequest` struct
- [ ] Define `AudioPreviewResponse` struct
- [ ] Define `AudioFormat` enum with Display trait

### Task 4.2: Create Audio API Service
- [ ] Create `frontend/src/services/audio_api.rs`
- [ ] Implement `generate_preview()` function
- [ ] Implement `process_audio()` function
- [ ] Implement `get_supported_formats()` function
- [ ] Add error handling and type conversions

### Task 4.3: Update Service Module
- [ ] Update `frontend/src/services/mod.rs` to export audio_api
- [ ] Update `frontend/src/models/mod.rs` to export audio_processing

## Phase 5: Frontend - Post-Processing UI

### Task 5.1: Create Audio Post-Processing Component Structure
- [ ] Create `frontend/src/components/audio_post_processing.rs`
- [ ] Set up component state (selected_history, pitch, speed, format, etc.)
- [ ] Create two-panel layout (history list + controls)

### Task 5.2: Implement History File Selector
- [ ] Fetch history entries on component mount
- [ ] Display history list with file names and metadata
- [ ] Implement file selection handler
- [ ] Show selected file details (duration, format, size)

### Task 5.3: Implement Audio Player
- [ ] Add audio player for original file
- [ ] Add audio player for preview file
- [ ] Implement play/pause controls
- [ ] Show playback progress

### Task 5.4: Implement Pitch Control
- [ ] Create pitch slider (-12 to +12 semitones)
- [ ] Add numeric input for precise control
- [ ] Display current pitch value
- [ ] Add reset button

### Task 5.5: Implement Speed Control
- [ ] Create speed slider (0.5x to 2.0x)
- [ ] Add numeric input for precise control
- [ ] Display current speed as percentage
- [ ] Add reset button

### Task 5.6: Implement Format Selector
- [ ] Create format dropdown (WAV, MP3, OGG, FLAC)
- [ ] Show quality settings for lossy formats
- [ ] Add bitrate selector for MP3
- [ ] Add quality selector for OGG

### Task 5.7: Implement Preview Functionality
- [ ] Add "Preview (10s)" button
- [ ] Call preview API endpoint
- [ ] Show loading indicator during generation
- [ ] Play preview when ready
- [ ] Handle errors gracefully

### Task 5.8: Implement Apply & Save Functionality
- [ ] Add "Apply & Save" button
- [ ] Call process API endpoint
- [ ] Show progress indicator
- [ ] Refresh history list when complete
- [ ] Display success message
- [ ] Handle errors gracefully

### Task 5.9: Add Reset Functionality
- [ ] Add "Reset All" button
- [ ] Clear all effect adjustments
- [ ] Reset to default format (WAV)
- [ ] Clear preview

## Phase 6: Frontend - Integration

### Task 6.1: Add Post-Processing Tab
- [ ] Update `Tab` enum in `frontend/src/main.rs`
- [ ] Add `PostProcessing` variant
- [ ] Add tab button in navigation bar
- [ ] Render `AudioPostProcessing` component when selected

### Task 6.2: Update Component Module
- [ ] Update `frontend/src/components/mod.rs`
- [ ] Export `audio_post_processing` module
- [ ] Ensure all imports are correct

### Task 6.3: Style Post-Processing UI
- [ ] Apply consistent styling with other tabs
- [ ] Add hover effects and transitions
- [ ] Ensure responsive layout
- [ ] Add icons for buttons (if available)

## Phase 7: Testing and Verification

### Task 7.1: Backend Unit Tests
- [ ] Write tests for pitch shifting
- [ ] Write tests for speed adjustment
- [ ] Write tests for format conversion
- [ ] Write tests for combined effects
- [ ] Write tests for error handling

### Task 7.2: Backend Integration Tests
- [ ] Test preview endpoint with various parameters
- [ ] Test process endpoint with various parameters
- [ ] Test file serving endpoint
- [ ] Test error responses

### Task 7.3: Frontend Manual Testing
- [ ] Test history file selection
- [ ] Test audio playback
- [ ] Test pitch adjustment and preview
- [ ] Test speed adjustment and preview
- [ ] Test format conversion
- [ ] Test combined effects
- [ ] Test error scenarios (invalid file, network error)

### Task 7.4: End-to-End Testing
- [ ] Test complete workflow: select → adjust → preview → apply
- [ ] Verify processed files appear in history
- [ ] Verify processed files are playable
- [ ] Test with various audio file types
- [ ] Test with long audio files (5+ minutes)

### Task 7.5: Create Walkthrough
- [ ] Document feature in walkthrough.md
- [ ] Take screenshots of UI
- [ ] Record demo video of workflow
- [ ] Document any known limitations

## Phase 8: Documentation and Cleanup

### Task 8.1: Update Project Documentation
- [ ] Update `SPECS.md` with Epic 4.0 status
- [ ] Update `SPEC.md` to point to Epic 4.0
- [ ] Update README.md with post-processing feature

### Task 8.2: Code Cleanup
- [ ] Remove debug logging
- [ ] Add code comments where needed
- [ ] Format code (black for Python, rustfmt for Rust)
- [ ] Remove unused imports

### Task 8.3: Performance Optimization
- [ ] Profile audio processing performance
- [ ] Optimize preview generation
- [ ] Add caching where beneficial
- [ ] Document performance characteristics

## Notes
- FFmpeg must be installed on the system for pydub to work
- Librosa requires numpy and scipy, which may take time to install
- Consider adding progress callbacks for long-running operations (future enhancement)
- Batch processing is out of scope for this epic but should be considered for future work
