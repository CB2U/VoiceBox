# Epic 2.1: Script Editor & Parsing

## 1. Goal
Provide a "Script Editor" workspace where the user can write dialogue in a simple format (`[Name]: Text`) and synthesize the entire conversation into a coherent audio sequence. This epic bridges the Character Management (1.1) and Synthesis Engine (1.2) capabilities, enabling the core value proposition of VoiceBox.

## 2. Context
*   **Epic 1.1 (Character Mgmt)** provided the ability to define Characters and assign reference audio.
*   **Epic 1.2 (Synthesis Engine)** provided the backend API to clone a voice from a reference.
*   **Epic 2.1** builds the UI/UX for the DM to use these tools to create a scene.

## 3. Requirements

### 3.1 Script Format & Parsing
*   **Syntax:** The editor must support a line-based syntax:
    ```text
    [Gandalf]: You cannot pass!
    [Frodo]: I wish the ring had never come to me.
    ```
*   **Parser Rules:**
    1.  Lines starting with `[Name]:` (case-insensitive check against Character Library) are treated as dialogue.
    2.  `Name` must match an existing Character Name.
    3.  Text following the colon is the payload for synthesis.
    4.  Lines *not* matching this pattern should be ignored (or treated as direction/silence - *for MVP, we will ignore distinct direction lines but preserve order*).
    5.  Empty lines are ignored.

### 3.2 UI Elements
*   **Script Input:** A multi-line text area (TextArea) for typing the script.
*   **Synthesize Button:** Triggers the parsing and synthesis workflow.
*   **Status Indicator:** Shows progress (e.g., "Synthesizing line 2/5...").
*   **Playback Controls:** Play/Pause/Stop for the resulting audio.
*   **Export Button:** Saves the combined audio to a file.

### 3.3 Synthesis Logic
*   **Sequential Processing:** The application must iterate through parsed lines.
*   **Error Handling:**
    *   If a Character is not found: Show an error to the user (e.g., "Character 'Boromir' not found").
    *   If Reference Audio is missing: Show an error.
    *   If Backend fails: Stop and alert user.
*   **Concurrency:** To prevent freezing, synthesis calls should be async.

### 3.4 Audio Sequencing & Export
*   **Playback:** The app should be able to play the synthesized lines in sequence.
*   **Export:** The app must ideally concatenate the audio files into a single WAV/MP3 for export, OR (for strictly MVP if concatenation is hard in pure Rust/WASM) save individual files to a folder.
    *   *Decision:* Provide a single concatenated file if possible, or a "Play" function that chains them.
    *   *Roadmap Alignment:* BP3 explicitly lists "Audio Export". We should aim for a single export file.

## 4. Acceptance Criteria

### AC-5: Script Parsing
*   **Given** a script with `[Gandalf]: Hello` and `[Frodo]: Hi`.
*   **When** the user clicks Synthesize.
*   **Then** the system identifies two distinct dialogue lines linked to the characters "Gandalf" and "Frodo".

### AC-7: Synthesis & Export
*   **Given** a valid script and available backend.
*   **When** synthesis completes.
*   **Then** the user can play the full conversation in order.
*   **And** the user can click "Export" to save the result as a `.wav` file to their machine.

### AC-8: Error Feedback (Implicit)
*   **Given** a script using `[Unknown]: Hello`.
*   **When** synthesis is attempted.
*   **Then** the UI displays a clear error message identifying "Unknown" as a missing character.

## 5. Technical Considerations
*   **State Management:** We need a way to store the `Script` string and the `SynthesizedSegments` list in the Dioxus state.
*   **Audio Concatenation:** We may need a Rust crate (like `hound` for WAV) to concatenate audio buffers in memory before saving, OR we can lean on the Python backend to do the "Mix" step if we want to offload it (though NFR-3 says "Private/Local", Python is local).
    *   *Proposal:* Let the Rust frontend coordinate the *calls*, but we might need a "Combine" function.
    *   *Simpler Approach:* Just play them sequentially for "Preview", and for "Export", maybe send a list of file paths to the Backend to "stitch" them with ffmpeg?
    *   *Winner:* **Frontend-driven playback** for preview. **Backend-driven stitching** for export? Or `hound` in Rust.
    *   *Plan:* We will try to use `hound` in Rust for pure local WAV stitching to minimize backend complexity.

## 6. Open Questions
*   How do we handle long scripts? (Pagination? Streaming?) -> *MVP: Single text block.*
*   Do we cache synthesized lines? -> *MVP: No, re-synthesize on click.*
