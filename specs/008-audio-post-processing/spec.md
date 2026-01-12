# Epic 4.0: Audio Post-Processing

## Overview
Add a dedicated post-processing tab that allows users to apply audio effects (pitch shifting, speed adjustment) and convert audio formats for all saved history files.

## User Story
As a VoiceBox user, I want to edit and enhance my previously synthesized audio files so that I can adjust pitch, speed, and format without re-synthesizing the entire script.

## Requirements

### Functional Requirements

#### FR-1: History File Access
- Users can view and select any audio file from the project's synthesis history
- Display file metadata (name, duration, creation date, format)
- Provide audio preview/playback for selected files

#### FR-2: Pitch Shifting
- Allow pitch adjustment from -12 to +12 semitones
- Preserve audio quality during pitch shifting
- Display current pitch adjustment in semitones
- Provide reset button to return to original pitch

#### FR-3: Speed/Tempo Adjustment
- Allow speed adjustment from 0.5x (half speed) to 2.0x (double speed)
- Maintain pitch when adjusting speed (time-stretching)
- Display current speed as percentage or multiplier
- Provide reset button to return to original speed

#### FR-4: Format Conversion
- Support conversion to multiple audio formats:
  - WAV (lossless)
  - MP3 (lossy, configurable bitrate)
  - OGG (lossy, configurable quality)
  - FLAC (lossless)
- Preserve metadata during conversion
- Allow quality/bitrate selection for lossy formats

#### FR-5: Preview and Apply
- Generate real-time preview of effects (first 10 seconds)
- Apply effects and save as new file (preserving original)
- Show processing progress during effect application
- Auto-add processed files to history with descriptive names

#### FR-6: Batch Processing (Future Enhancement)
- Select multiple history files for batch processing
- Apply same effects to all selected files
- Show batch progress indicator

### Non-Functional Requirements

#### NFR-1: Performance
- Preview generation should complete within 2 seconds
- Full file processing should show progress updates every 500ms
- Support files up to 10 minutes in length

#### NFR-2: Quality
- Pitch shifting should use high-quality algorithms (phase vocoder or similar)
- Speed adjustment should minimize artifacts
- Format conversion should use industry-standard encoders

#### NFR-3: Usability
- Intuitive slider controls with visual feedback
- Clear labeling of all parameters
- Undo/reset functionality for all adjustments
- Error messages for unsupported operations

## Acceptance Criteria

### AC-1: Post-Processing Tab
- [ ] New "Post-Processing" tab appears in main navigation
- [ ] Tab displays list of all history audio files
- [ ] Selected file shows waveform or metadata preview
- [ ] Audio player allows playback of original file

### AC-2: Pitch Control
- [ ] Slider adjusts pitch from -12 to +12 semitones
- [ ] Current pitch value displays numerically
- [ ] Preview button generates 10-second preview
- [ ] Apply button processes entire file with pitch shift
- [ ] Processed file maintains audio quality

### AC-3: Speed Control
- [ ] Slider adjusts speed from 0.5x to 2.0x
- [ ] Current speed displays as percentage (50% to 200%)
- [ ] Speed adjustment preserves pitch (time-stretching)
- [ ] Preview and apply buttons work correctly
- [ ] Processed file maintains audio quality

### AC-4: Format Conversion
- [ ] Dropdown lists all supported formats (WAV, MP3, OGG, FLAC)
- [ ] Quality/bitrate options appear for lossy formats
- [ ] Conversion preserves audio content
- [ ] Converted files are playable in standard media players

### AC-5: Combined Effects
- [ ] Multiple effects can be applied simultaneously
- [ ] Preview shows combined effect of all adjustments
- [ ] Processing applies all effects in optimal order
- [ ] Filename indicates applied effects (e.g., "script_001_pitch+3_speed1.5x.mp3")

### AC-6: Error Handling
- [ ] Graceful handling of corrupted audio files
- [ ] Clear error messages for unsupported operations
- [ ] Processing cancellation works correctly
- [ ] Failed operations don't corrupt original files

## Open Questions

### Q1: Audio Processing Library
**Question:** Which Python library should we use for audio processing?
**Options:**
- `pydub` - Simple, high-level API, uses FFmpeg
- `librosa` - Advanced audio analysis, excellent for pitch/tempo
- `soundfile` - Low-level, fast, good for format conversion
- Combination of libraries?

**Recommendation:** Use `librosa` for pitch/speed effects (high quality) and `pydub` for format conversion (simple API, FFmpeg backend).

### Q2: Preview Strategy
**Question:** Should previews be generated server-side or client-side?
**Options:**
- Server-side: Process on backend, stream to frontend
- Client-side: Download file, process in browser with Web Audio API

**Recommendation:** Server-side for consistency and to avoid large file downloads. Generate preview of first 10 seconds only.

### Q3: File Naming Convention
**Question:** How should processed files be named?
**Options:**
- Auto-generate descriptive names (e.g., "script_001_pitch+3_speed1.5x.mp3")
- Prompt user for custom name
- Use original name with suffix

**Recommendation:** Auto-generate with option to rename after processing. Format: `{original_name}_p{pitch}_s{speed}.{format}`

### Q4: History Integration
**Question:** Should processed files be added to synthesis history?
**Options:**
- Add as new history entries with "processed" tag
- Keep separate "processed files" list
- Link to original history entry as "variant"

**Recommendation:** Add as new history entries with metadata linking to original file. This maintains consistency with existing history system.

## Dependencies
- Epic 2.1 (Script Editor) - Uses history system
- Epic 3.0 (Project Management) - File storage location

## Out of Scope
- Real-time audio effects during synthesis
- Advanced audio editing (cutting, splicing, mixing)
- Noise reduction or audio restoration
- Batch processing (deferred to future epic)
- Custom effect chains or plugins

## Technical Notes
- Audio processing will be CPU-intensive; consider async processing
- Large files may require streaming or chunked processing
- Format conversion quality settings should have sensible defaults
- Consider caching processed previews to improve responsiveness
