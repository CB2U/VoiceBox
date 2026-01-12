# Implementation Plan: Audio Post-Processing

## Goal
Implement a post-processing tab that allows users to adjust pitch, speed, and convert audio formats for all saved history files. This feature enables users to refine their synthesized audio without re-running the entire synthesis process.

## User Review Required

> [!IMPORTANT]
> **Audio Processing Library Choice**
> We will use `librosa` for pitch/speed effects and `pydub` for format conversion. This combination provides high-quality audio processing with a simple API. This will add ~200MB to the backend dependencies due to audio processing libraries.

> [!IMPORTANT]
> **Processing Strategy**
> All audio processing will be done server-side to ensure consistent quality and avoid large file transfers. Previews will be limited to the first 10 seconds to improve responsiveness.

> [!NOTE]
> **File Storage**
> Processed files will be saved to the project directory and added to the synthesis history with metadata linking them to the original file. This maintains consistency with the existing history system.

## Proposed Changes

### Backend - Audio Processing Service

#### [NEW] [audio_processing.py](file:///mnt/Storage/Documents/Projects/VoiceBox/backend/src/models/audio_processing.py)
Create Pydantic models for audio processing requests and responses:
- `AudioProcessRequest`: pitch_shift (float), speed_factor (float), output_format (str), quality_settings (dict)
- `AudioProcessResponse`: processed_file_path (str), duration (float), format (str), file_size (int)
- `AudioPreviewRequest`: Same as process request but for preview generation
- `AudioPreviewResponse`: preview_file_path (str), duration (float)

#### [NEW] [audio_processing_service.py](file:///mnt/Storage/Documents/Projects/VoiceBox/backend/src/services/audio_processing_service.py)
Implement audio processing service with the following methods:
- `apply_pitch_shift(audio_path: str, semitones: float) -> np.ndarray`: Use librosa's pitch_shift
- `apply_speed_change(audio_path: str, speed_factor: float) -> np.ndarray`: Use librosa's time_stretch
- `convert_format(audio_data: np.ndarray, output_format: str, quality: dict) -> bytes`: Use pydub for conversion
- `generate_preview(audio_path: str, effects: dict, duration: float = 10.0) -> str`: Generate preview file
- `process_audio_file(audio_path: str, effects: dict, output_path: str) -> dict`: Apply all effects and save

#### [NEW] [audio.py](file:///mnt/Storage/Documents/Projects/VoiceBox/backend/src/routers/audio.py)
Create FastAPI router with endpoints:
- `POST /audio/preview`: Generate preview with effects applied
- `POST /audio/process`: Process full audio file with effects
- `GET /audio/formats`: List supported output formats
- `GET /audio/file/{file_path}`: Serve processed audio files

#### [MODIFY] [requirements.txt](file:///mnt/Storage/Documents/Projects/VoiceBox/backend/requirements.txt)
Add audio processing dependencies:
```
librosa>=0.10.0
soundfile>=0.12.0
pydub>=0.25.0
numpy>=1.24.0
```

#### [MODIFY] [main.py](file:///mnt/Storage/Documents/Projects/VoiceBox/backend/src/main.py)
Include audio processing router in the FastAPI app.

---

### Frontend - Post-Processing UI

#### [NEW] [audio_post_processing.rs](file:///mnt/Storage/Documents/Projects/VoiceBox/frontend/src/components/audio_post_processing.rs)
Create the main post-processing component with:
- **History File Selector**: Left panel showing all history files with metadata
- **Audio Player**: Playback controls for original and processed audio
- **Effect Controls**:
  - Pitch slider: -12 to +12 semitones with numeric display
  - Speed slider: 0.5x to 2.0x with percentage display
  - Format dropdown: WAV, MP3, OGG, FLAC
  - Quality settings (bitrate for MP3, quality for OGG)
- **Action Buttons**:
  - Preview (10s): Generate and play preview
  - Apply & Save: Process full file and save to history
  - Reset: Clear all adjustments
- **Progress Indicator**: Show processing status and progress

#### [NEW] [audio_processing.rs](file:///mnt/Storage/Documents/Projects/VoiceBox/frontend/src/models/audio_processing.rs)
Define Rust models matching backend schemas:
- `AudioProcessRequest`
- `AudioProcessResponse`
- `AudioPreviewRequest`
- `AudioPreviewResponse`
- `AudioFormat` enum

#### [NEW] [audio_api.rs](file:///mnt/Storage/Documents/Projects/VoiceBox/frontend/src/services/audio_api.rs)
Create API service for audio processing:
- `generate_preview(history_id: String, effects: AudioProcessRequest) -> Result<AudioPreviewResponse>`
- `process_audio(history_id: String, effects: AudioProcessRequest) -> Result<AudioProcessResponse>`
- `get_supported_formats() -> Result<Vec<AudioFormat>>`

#### [MODIFY] [main.rs](file:///mnt/Storage/Documents/Projects/VoiceBox/frontend/src/main.rs)
Add `PostProcessing` variant to `Tab` enum and render `AudioPostProcessing` component when selected.

#### [MODIFY] [mod.rs](file:///mnt/Storage/Documents/Projects/VoiceBox/frontend/src/components/mod.rs)
Export the new `audio_post_processing` module.

---

## Data Contracts

### Audio Process Request
```json
{
  "history_id": "uuid-string",
  "pitch_shift": 3.0,
  "speed_factor": 1.2,
  "output_format": "mp3",
  "quality_settings": {
    "bitrate": "192k"
  }
}
```

### Audio Process Response
```json
{
  "processed_file_path": "outputs/script_001_p3_s1.2.mp3",
  "duration": 45.5,
  "format": "mp3",
  "file_size": 1048576,
  "history_id": "new-uuid-string"
}
```

### Audio Preview Request
```json
{
  "history_id": "uuid-string",
  "pitch_shift": 3.0,
  "speed_factor": 1.2,
  "preview_duration": 10.0
}
```

### Audio Preview Response
```json
{
  "preview_url": "/audio/file/previews/temp_preview_xyz.wav",
  "duration": 10.0
}
```

## Verification Plan

### Automated Tests

#### Backend Tests
```bash
# Test audio processing service
pytest backend/tests/test_audio_processing_service.py -v

# Test endpoints
pytest backend/tests/test_audio_router.py -v
```

Test cases:
- Pitch shift: -12, 0, +12 semitones
- Speed: 0.5x, 1.0x, 2.0x
- Format conversion: WAV→MP3, WAV→OGG, WAV→FLAC
- Combined effects: pitch+speed, pitch+format, all three
- Error handling: invalid file, unsupported format, out-of-range values

#### Frontend Tests
Manual testing via UI:
- Load history file and play original
- Adjust pitch slider and generate preview
- Adjust speed slider and generate preview
- Combine effects and generate preview
- Apply effects and verify saved file
- Test all format conversions

### Manual Verification

1. **Pitch Shifting**:
   - Select a history file
   - Adjust pitch to +5 semitones
   - Click "Preview" and verify pitch is higher
   - Click "Apply & Save"
   - Verify new file appears in history
   - Play new file and confirm pitch shift

2. **Speed Adjustment**:
   - Select a history file
   - Adjust speed to 1.5x
   - Click "Preview" and verify faster playback
   - Verify pitch remains unchanged
   - Click "Apply & Save"
   - Verify new file is faster but same pitch

3. **Format Conversion**:
   - Select a WAV file from history
   - Choose MP3 format with 192kbps
   - Click "Apply & Save"
   - Verify MP3 file is created and playable
   - Check file size is smaller than WAV

4. **Combined Effects**:
   - Select a history file
   - Set pitch to -3 semitones
   - Set speed to 0.8x
   - Choose OGG format
   - Click "Preview" and verify both effects
   - Click "Apply & Save"
   - Verify final file has all effects applied

5. **Error Handling**:
   - Test with corrupted audio file
   - Test with extreme values (pitch +20)
   - Test canceling during processing
   - Verify error messages are clear

## Implementation Notes

### Audio Processing Pipeline
1. Load audio file using `librosa.load()`
2. Apply pitch shift if requested: `librosa.effects.pitch_shift()`
3. Apply speed change if requested: `librosa.effects.time_stretch()`
4. Convert to target format using `pydub.AudioSegment`
5. Save to output path
6. Create history entry linking to original

### Performance Considerations
- Use async processing for long files
- Stream progress updates via WebSocket (future enhancement)
- Cache preview files for 5 minutes to avoid regeneration
- Limit preview to first 10 seconds to reduce processing time
- Consider using multiprocessing for batch operations (future)

### File Naming Convention
Processed files will be named: `{original_name}_p{pitch}_s{speed}.{format}`
- Example: `script_001_p3_s1.2.mp3` (pitch +3, speed 1.2x, MP3 format)
- If no effect applied, omit that part: `script_001_s1.5.wav` (only speed)

### Quality Settings Defaults
- MP3: 192 kbps (good quality, reasonable size)
- OGG: Quality 5 (equivalent to ~160 kbps)
- FLAC: Compression level 5 (balanced)
- WAV: 16-bit PCM, 44.1kHz (standard)
