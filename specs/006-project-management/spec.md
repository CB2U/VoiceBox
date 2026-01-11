# Epic 3.0: Project Management - Specification

## 1. Header
- **Title:** Project Management
- **Roadmap anchor:** 3.0
- **Priority:** P1
- **Type:** Feature
- **Target area:** Frontend (Rust/Dioxus) + Backend (FastAPI) + Persistence
- **Target Acceptance Criteria:** AC-9 (New)

## 2. Problem Statement
Currently, VoiceBox stores all characters and settings in a global scope. Users working on multiple D&D campaigns find it difficult to keep NPCs and voice settings organized. There is no way to separate assets (audio clips, character profiles) by campaign/project.

## 3. Goals and Non-Goals
**Goals:**
- Introduce a "Project" entity that encapsulates Characters and project-specific settings.
- Allow users to Create, Switch, Rename, and Delete projects.
- Store project data in a dedicated directory structure (e.g., `projects/[project_id]/`).
- Migrate existing global data to a "Default" project on first launch after update.

**Non-Goals:**
- Cloud synchronization or sharing projects between users.
- Project archiving/compression (.zip export).
- Multi-user collaboration.

## 4. User Stories
- **US-1:** As a DM, I want to create a new project called "The Icewind Dale" so I can keep my winter-themed NPCs separate from my main campaign.
- **US-2:** As a DM, I want to switch between projects easily from the main UI.
- **US-3:** As a DM, I want each project to have its own folder for audio clips so I don't mix up voice samples.

## 5. Scope
### In-Scope
- **Data Model:** `Project` (UUID, Name, Path, CreatedAt).
- **Persistence:** A master `projects.json` mapping project IDs to names/paths. Each project folder contains its own `characters.json`.
- **UI:** A Project Manager view or a header-based project selector.
- **Migration:** Logic to move `characters.json` and associated audio files into the first project folder.

### Out-of-Scope
- Advanced project metadata (tags, notes).
- Project templates.

## 6. Requirements
### Functional Requirements
- **FR-1:** The app must allow creating a new project with a unique name.
- **FR-2:** The app must allow selecting a project to load its characters.
- **FR-3:** Characters created within a project must be saved in that project's directory.
- **FR-4:** Deleting a project should (optionally) delete its assets or just remove it from the list.

### Non-Functional Requirements
- **NFR-1 (Isolation):** Switching projects should completely reload the character library without requiring an app restart.
- **NFR-2 (Robustness):** Handle missing project directories gracefully.

## 7. Acceptance Criteria
### AC-9: Project Workspace
- **Given** I am in the "General" project.
- **When** I create a new project "Campaign B" and switch to it.
- **Then** the character list should be empty.
- **And** adding a character in "Campaign B" should not affect the "General" project.

## 8. Technical Considerations
- **State Management:** The frontend `AppState` needs to track the `active_project`.
- **Backend API:** New endpoints for project CRUD:
  - `GET /projects`
  - `POST /projects`
  - `PUT /projects/{id}`
  - `DELETE /projects/{id}`
  - `POST /projects/{id}/select`
- **File System:** Projects should live in a specific user-defined or default directory (e.g., `~/Documents/VoiceBox/Projects/`).

## 9. Risks and Mitigations
- **Risk:** Data loss during migration.
  - **Mitigation:** Backup original files before moving them into the project structure.
- **Risk:** Path hell (relative vs absolute paths).
  - **Mitigation:** Use projects as the root for asset resolution where possible.
