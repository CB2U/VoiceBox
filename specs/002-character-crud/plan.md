# Epic 1.1: Character Management - Implementation Plan

## 1. Architecture Review

### Data Layer
We will introduce a `Character` struct and a `CharacterService`.
- **Struct:**
  ```rust
  #[derive(Serialize, Deserialize, Clone, PartialEq)]
  pub struct Character {
      pub id: String, // UUID
      pub name: String,
      pub description: String,
      pub voice_path: Option<String>,
  }
  ```
- **Service:**
  - `load_characters() -> Vec<Character>`
  - `save_characters(chars: &[Character]) -> Result<()>`
  - The service will encapsulate the file I/O logic (`std::fs`).
  - Path: `std::env::current_dir()/data/characters.json` or `dirs::data_dir()/voicebox/characters.json`. For MVP, a `data/` folder in the project root is simplest for debugging.

### State Management (Frontend)
Using Dioxus Context / Signals.
- Global Context: `Signal<Vec<Character>>`
- Selection State: `Signal<Option<String>>` (Selected Character ID)

### UI Components
- **`App`**: Main layout container. Flex-row.
  - **`Sidebar`**: Left panel.
    - **`CharacterList`**: Renders list of buttons.
    - **`AddButton`**: Appends new default character.
  - **`Editor`**: Right panel.
    - **`NameField`**: Input.
    - **`VoiceSelector`**: Button that triggers file dialog + Display current path.
    - **`DeleteButton`**: Removes character.

## 2. Proposed Changes

### Frontend (`/frontend`)

#### [NEW] `src/models/character.rs`
- Define `Character` struct.
- Define `CharacterManager` (or helper functions) for CRUD logic if complex, otherwise inline in components or service.

#### [NEW] `src/services/persistence.rs`
- Functions to read/write JSON to disk.

#### [MODIFY] `src/main.rs`
- Initialize `Character` state.
- Split UI into `Sidebar` and `Main` components.

#### [NEW] `src/components/sidebar.rs`
- Render the list of characters.

#### [NEW] `src/components/editor.rs`
- Render the form for the selected character.

## 3. Verification Plan

### Automated Tests
- **Unit Tests:**
  - Test `Character` serialization/deserialization.
  - Test persistence (saving/loading from a temp file).

### Manual Verification
- **Scenario 1: Persistence**
  - Add character "Test".
  - Close app.
  - Re-open app.
  - "Test" exists.
- **Scenario 2: Data Integrity**
  - Assign a voice path.
  - Verify path survives restart.
