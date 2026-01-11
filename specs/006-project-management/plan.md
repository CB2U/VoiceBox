# Epic 3.0: Project Management - Implementation Plan

## Technical Overview
The project management system will introduce a hierarchical storage structure. We will move away from a single `characters.json` in the root settings directory and instead use a `projects/` subfolder.

### Directory Structure
```text
VoiceBox/
├── settings.json (Global settings, including last_active_project_id)
├── projects.json (Registry of projects)
└── projects/
    ├── {project_id_1}/
    │   ├── character_config.json
    │   └── voice_files/
    │       ├── ref_1.wav
    │       └── ref_2.wav
    └── {project_id_2}/
        ├── character_config.json
        └── voice_files/
```

## Proposed Changes

### Backend (Python/FastAPI)
- **Settings Service:** Update to handle global settings and project-specific settings.
- **Project Router:** Implement CRUD endpoints for projects.
- **Migration Logic:** A startup utility that checks if the old structure exists and migrates it to a "Default Project".
- **Path Resolution:** Update all file-related logic (youtube extraction, synthesis) to use the path of the *currently selected project*.

### Frontend (Rust/Dioxus)
- **Models:** Add `Project` struct.
- **API Client:** Add methods for project endpoints.
- **AppState:** Update to include `active_project`.
- **UI Components:**
  - `ProjectSelector`: A dropdown or sidebar item to switch projects.
  - `ProjectManager`: A modal or view to create/delete projects.
- **Logic:** Ensure that switching a project triggers a reload of the character list.

## Data Contracts

### Project Object
```json
{
  "id": "uuid-v4",
  "name": "My Epic Campaign",
  "created_at": "ISO8601-Timestamp",
  "base_path": "/absolute/path/to/project/dir"
}
```

## Verification Plan

### Automated Tests
- **Unit Tests (Python):** 
  - Test project creation and directory initialization.
  - Test registry updates in `projects.json`.
  - Test migration logic with mock old data.
- **Logic Tests (Rust):**
  - Verify `AppState` updates correctly on project selection.

### Manual Verification
1.  **Fresh Install:** Open app, verify a "Default" project is created.
2.  **Creation:** Create a new project "Campaign 2".
3.  **Isolation:** Add a character to "Campaign 2". Switch back to "Default". Verify "Campaign 2" character is NOT visible in "Default".
4.  **Filesystem Check:** Verify files are physically created in separate subfolders of the library directory.
5.  **Restart:** Restart app and verify the last active project is automatically re-selected.
