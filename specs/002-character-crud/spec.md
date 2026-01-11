# Epic 1.1: Character Management - Specification

## 1. Header
- **Title:** Character Management
- **Roadmap anchor:** 1.1
- **Priority:** P1
- **Type:** Feature
- **Target area:** Frontend (Rust/Dioxus) + Local Persistence
- **Target Acceptance Criteria:** AC-1, AC-2

## 2. Problem Statement
Users currently have no way to define or organize the "voices" they want to use. We need a system to create and manage characters, where each character has a name and a reference audio file (the "voice").

## 3. Goals and Non-Goals
**Goals:**
- Implement the "Library" vs "Editor" 2-panel layout.
- Allow users to Create, Read, Update, and Delete characters.
- Persist character data to disk (JSON) so it survives restarting the app.
- Allow users to associate a local audio file (.wav/.mp3) with a character.

**Non-Goals:**
- Audio playback (previewing the voice).
- Youtube extraction (downloading the voice).
- Synthesis (generating new audio).
- Cloud sync.

## 4. User Stories
- **US-1:** As a user, I want to see a list of my characters on the left side of the app.
- **US-2:** As a user, I want to click a "+" button to add a new character.
- **US-3:** As a user, I want to edit a character's name and select a voice reference file on the right side of the app.
- **US-4:** As a user, I want my characters to be saved automatically so I don't lose my work.

## 5. Scope
### In-Scope
- **Data Model:** `Character` struct (ID, Name, Description, VoiceReferencePath).
- **Persistence:** Local JSON file (`characters.json`).
- **UI Layout:** Split pane (Sidebar for list, Main area for details).
- **UI Components:** Character List Item, Input Forms, File Picker Integration.

### Out-of-Scope
- Audio player controls.
- Drag-and-drop file import (will start with File Picker dialog for simplicity, DnD as stretch).
- Input validation on audio file format (beyond simple extension check).

## 6. Requirements
### Functional Requirements
- **FR-1:** App must load `characters.json` on startup.
- **FR-2:** App must save `characters.json` on any change (Create/Update/Delete).
- **FR-3:** User must be able to select a file from the filesystem. The app stores the **absolute path**.
- **FR-4:** UI must display a list of characters. Selecting one populates the editor.

### Non-Functional Requirements
- **NFR-1 (Usability):** Changes should feel instant.
- **NFR-2 (Persistence):** File I/O should not block the UI thread (async).

## 7. Acceptance Criteria

### AC-1: 2-Panel Layout
- **Description:** User sees a sidebar (Library) and an editor panel.
- **Verification approach:** Visual check. Widths are roughly 1/3 sidebar, 2/3 editor.

### AC-2: Character CRUD
- **Description:** Create a character "Gandalf", assign a .wav file, restart app, verify "Gandalf" and the path still exist.
- **Verification approach:**
  1. Click "+".
  2. Rename "New Character" to "Gandalf".
  3. Select a dummy .wav file.
  4. Close App.
  5. Open App.
  6. Verify "Gandalf" is selected and path is correct.

## 8. Dependencies
- `serde`, `serde_json` for serialization.
- `rfd` or similar for native file dialogs (or Dioxus file input).
- `dirs` crated (optional) to find config directory.

## 9. Risks and Mitigations
- **Risk:** File path links break if user moves files.
  - **Mitigation:** Accept this for MVP (local absolute paths). Future: Copy files to managed library folder.
