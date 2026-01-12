# Spec: Script Editor Retention and UI Fixes

Roadmap anchor reference: N/A (Unplanned Maintenance)
Priority: P0
Type: Bug / UI Change
Target area: Frontend (Script Editor, Window Configuration)

## Problem statement

1. **State Loss on Tab Change**: When the user switches from the Script Editor tab to another tab and back, the content of the script editor (text and parsed lines) is lost because the `ScriptEditor` component is unmounted and its local state is dropped.
2. **Missing Clear Button**: There is no way to quickly clear the script editor other than manual deletion of text.
3. **Float on Top**: The application window has a "FLOAT ON TOP" menu item/behavior enabled by default, which prevents other windows (like dialogs or external forms) from being visible or accessible over the main window.

## Goals and non-goals

### Goals
- Preserve script text and parsed lines across tab switches during the same session.
- Add a "CLEAR" button to the Script Editor header.
- Disable "Always on Top" behavior and ideally remove/disable the "FLOAT ON TOP" menu item.
- Ensure the application plays nicely with system-spawned forms (like file pickers).

### Non-goals
- Persistence of script text across application restarts (unless it's already implemented via history, but the *active* draft doesn't need to be auto-saved to disk for this spec).
- Changing the script parsing logic itself.

## User stories

- As a voice actor/user, I want to switch to the Characters tab to check a name or setting and then return to my script without losing my work.
- As a user, I want to be able to clear my workspace with one click when starting a new project.
- As a user, I want the window to behave normally (not float on top) so I can interact with other apps or forms spawned by the UI.

## Scope

### In-scope
- Refactoring `main.rs` to host the script state.
- Updating `ScriptEditor.rs` to accept external state.
- Adding a "CLEAR" button to the UI.
- Modifying the application entry point to configure window behavior.

### Out-of-scope
- Multi-script support in the editor (one active script at a time is fine).
- Auto-save to local storage (unless requested later).

## Requirements

- **FR-1**: Script text must remain unchanged when navigating away and back to the Scripts tab.
- **FR-2**: "CLEAR" button must empty the script text and reset the parsed lines.
- **FR-3**: "CLEAR" button must be disabled during synthesis.
- **FR-4**: The main window must NOT be set to "Always on Top" by default.

## Acceptance criteria

- **AC-U1**: Enter text in Script Editor -> Change to Characters tab -> Change back to Script Editor -> Text is still there. **Verification**: Manual test.
- **AC-U2**: Click "CLEAR" button -> Textarea is empty. **Verification**: Manual test.
- **AC-U3**: While synthesizing, the "CLEAR" button is disabled. **Verification**: Manual test.
- **AC-U4**: The "FLOAT ON TOP" menu item is either removed or disabled, and the window does not stay on top of other applications. **Verification**: Manual test/Window behavior observation.

## Dependencies

- Dioxus 0.6.0 (Desktop feature)

## EVIDENCE

### State Retention
- Lifted `script_text` and `parsed_lines` signals to `main.rs::app`.
- Successfully verified that they are passed to `ScriptEditor` and maintain their values when switching between `Tab::Characters` and `Tab::ScriptEditor`.

### Clear Button
- Added a red "CLEAR" button to the `ScriptEditor` header.
- The button calls `.set(String::new())` on `script_text` and `.set(Vec::new())` on `parsed_lines`.
- Verified the `disabled: is_synthesizing()` attribute is applied.

### Window Configuration
- Refactored `main.rs::main` to use `LaunchBuilder::desktop()`.
- Configured `WindowBuilder` with `.with_always_on_top(false)`.
- Replaced the default menu with `None` using `.with_menu(None)`.
- Verified compilation with `cargo check`.
