# Plan: Script Editor & Parsing

## 1. Architecture

### 1.1 Frontend (Rust/Dioxus)
*   **New View:** `ScriptEditor` component.
*   **Parser:** `fn parse_script(raw_text: &str, characters: &[Character]) -> Vec<ScriptLine>`.
*   **State:** Use `use_signal` or `use_ref` to hold the list of `ScriptLine` items.
*   **Coordinator:** An async function `run_synthesis` that:
    1.  Iterates over `ScriptLine`s.
    2.  Skips invalid/unknown characters.
    3.  Calls `api::synthesize` (reusing Epic 1.2 client).
    4.  Updates the UI state (spinner -> checkmark/play button).

### 1.2 Audio Handling (Native Rust)
*   **Playback:** Use `rodio` or standard `web_sys` (if web) / existing audio mechanisms. Since Dioxus Desktop allows native code, we can use `rodio` `Sink` to queue up the generated files for sequential playback.
*   **Export:** Use the `hound` crate to read the generated WAV files and stitch them into a single output WAV file. This keeps the logic client-side and avoids sending heavy audio back and forth to Python just for valid WAV concatenation.

### 1.3 Backend (Python)
*   No changes required to the inference engine (Epic 1.2 already exposes `POST /synthesize`).
*   We rely on the existing API: `POST /synthesize { text: Str, reference_wav: Path } -> Saved Wav Path`.

## 2. Data Structures (Rust)

```rust
struct ScriptLine {
    id: Uuid,
    character_id: Option<Uuid>, // None if unknown
    character_name: String,     // From script
    text: String,
    
    // Status
    status: SynthesisStatus,
    output_path: Option<String>, 
}

enum SynthesisStatus {
    Idle,
    Queued,
    Working,
    Done,
    Error(String),
}
```

## 3. Implementation Phases

### Phase 1: The Editor & Parser
*   Create `ScriptEditor` UI with a TextArea.
*   Implement `parse_script` logic with Regex.
*   Display the parsed "Line Items" in a list below/beside the editor to verify parsing works.

### Phase 2: Sequential Synthesis
*   Implement the "Synthesize" button handler.
*   Loop through valid lines.
*   Call `api::synthesize`.
*   Update status icons.

### Phase 3: Playback & Export
*   Implement "Play All" (Load files sequentially).
*   Implement "Export" (Stitch with `hound` and save to a user-selected path).

## 4. Testing Strategy

### Automated
*   **Parser Tests:** Unit tests in Rust for `parse_script` handling various edge cases (empty lines, no spaces, unknown chars).
*   **Stitcher Tests:** Unit test ensuring 2 valid WAVs can be combined into one valid WAV.

### Manual
*   **Mock Verification:** Use a mock backend (or the real one) to verify the loop doesn't hang the UI.
*   **Audio Check:** Validate the exported file opens in VLC/Audacity and sounds correct (no clicking at seams).
