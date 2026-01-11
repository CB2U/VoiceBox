# Epic 1.1: Character Management - Tasks

## 1. Core Data & Persistence
- [x] **Define Character Model** <!-- id: 0 -->
  - File: `frontend/src/models.rs` (or similar)
  - Fields: `id`, `name`, `desc`, `voice_path`.
  - Add `serde` derives.
- [x] **Implement Persistence Layer** <!-- id: 1 -->
  - File: `frontend/src/persistence.rs`
  - Implement `load_from_disk()`.
  - Implement `save_to_disk()`.
  - Ensure it creates the file if missing (defaulting to empty list).

## 2. State & Basic Logic
- [x] **Initialize Global State** <!-- id: 2 -->
  - File: `frontend/src/main.rs`
  - Load characters on startup.
  - Create `Signal<Vec<Character>>`.
  - Create `Signal<Option<String>>` for selection.

## 3. UI Implementation
- [x] **Create Sidebar Component** <!-- id: 3 -->
  - Display list of names.
  - Implement "Active" styling for selected character.
- [x] **Implement "Add Character" Button** <!-- id: 4 -->
  - Adds a "New Character" entry to the list.
  - Selects the new character.
  - Autosaves.
- [x] **Create Editor Component** <!-- id: 5 -->
  - Bind Name input to state.
  - Bind Description input to state.
  - Autosave on change (or on blur).
- [x] **Implement Delete Function** <!-- id: 6 -->
  - Add Delete button in Editor.
  - Remove from list, clear selection, save.

## 4. File Picking (Voice Reference)
- [x] **Add File Picker Dependency** <!-- id: 7 -->
  - Add `rfd` (Rust File Dialog) to `Cargo.toml`.
- [x] **Implement "Select Voice" Button** <!-- id: 8 -->
  - Open native dialog.
  - Capture path.
  - Update state & save.

## 5. Verification
- [x] **Unit Tests for Persistence** <!-- id: 9 -->
- [ ] **Manual Verification (Walkthrough)** <!-- id: 10 -->
  - Verify AC-1 (Layout).
  - Verify AC-2 (CRUD + Persistence).
