# Tasks: Script Editor Retention and UI Fixes

## Setup
- [x] T1: Initialize spec status in `SPECS.md` and `SPEC.md`

## Implementation

### Script Editor State Retention
- [x] T2: Lift `script_text` and `parsed_lines` signals to `main.rs` and pass them to `ScriptEditor`
- [x] T3: Update `ScriptEditor` to use injected signals instead of local signals

### Script Editor Clear Button
- [x] T4: Add "CLEAR" button to `ScriptEditor` UI with confirmation or simple reset logic
- [x] T5: Ensure "CLEAR" button is disabled during synthesis

### Window Configuration
- [x] T6: Modify `main.rs` to use `launch_cfg` with "always on top" disabled and custom/no menu

## Verification
- [ ] T7: Manually verify state persistence across tab changes
- [ ] T8: Manually verify "CLEAR" button functionality and synthesis locking
- [ ] T9: Manually verify window "Float on Top" is disabled

## Tracking
- [ ] T10: Update `SPECS.md` status to Completed
- [ ] T11: Final audit of spec and evidence consolidation
