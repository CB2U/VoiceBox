# Plan: Script Editor Retention and UI Fixes

## Architecture Overview

The solution involves lifting the script-specific state from the `ScriptEditor` component to the `app` component in `main.rs`. This ensures that even when the `ScriptEditor` is unmounted during tab switching, the data remains held in the parent's state.

We will also adjust the Dioxus Desktop configuration to disable the "Always on Top" behavior.

## Proposed Changes

### 1. State Lifting in `main.rs`
- Add `script_text` and `parsed_lines` signals to the `app` function.
- Initialize them as empty.
- Pass them as props to the `ScriptEditor` component.

### 2. Update `ScriptEditor` Component
- Modify `ScriptEditor` signature to accept `Signal<String>` for script text and `Signal<Vec<ScriptLine>>` for parsed lines.
- Remove the local `use_signal` calls for these two variables.
- Add a "CLEAR" button to the header that resets both signals.

### 3. Window Configuration in `main.rs`
- Replace `dioxus::launch(app)` with a more explicit configuration:
  ```rust
  use dioxus_desktop::{Config, WindowConfig};
  
  fn main() {
      let cfg = Config::new()
          .with_window(
              WindowConfig::new()
                  .with_always_on_top(false)
                  .with_title("VoiceBox")
          )
          // To disable the default menu which might contain "Float on Top"
          .with_menu(None); 
      
      dioxus::desktop::launch_cfg(app, cfg);
  }
  ```
  *(Note: Exact API for Dioxus 0.6.0 will be verified during implementation, but the principle is to use `launch_cfg` or `LaunchBuilder`)*.

## Testing Plan

### Manual Verification
1. **State Persistence**:
   - Navigate to "Script Editor".
   - Type "[Gandalf]: Test".
   - Switch to "Characters".
   - Switch back to "Script Editor".
   - Verify text remains.
2. **Clear Button**:
   - Click "CLEAR".
   - Verify textarea is empty and "Parsed Lines" section disappears/clears.
3. **Synthesis Lock**:
   - Click "Synthesize".
   - Verify "CLEAR" button is disabled during the process.
4. **Float on Top**:
   - Open another application (e.g., text editor).
   - Click back and forth.
   - Verify VoiceBox doesn't stay on top of other apps unless explicitly focused.
   - Check the menu bar to see if "FLOAT ON TOP" is gone.

## Risks and Mitigations

- **Dioxus 0.6.0 API changes**: If `Config::with_menu(None)` doesn't work as expected, I'll look into `WindowBuilder` or custom menu construction.
- **State Over-lifting**: If too many signals are lifted to `main.rs`, it could become cluttered. However, `script_text` is essential for this requirement.
