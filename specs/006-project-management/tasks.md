# Epic 3.0: Project Management - Tasks

## Phase 1: Data Model & Backend
- [ ] Create `Project` pydantic model in backend.
- [ ] Implement `ProjectRegistry` service to manage `projects.json`.
- [ ] Create `ProjectRouter` with CRUD endpoints.
- [ ] Implement migration script to move existing data to `projects/default/`.
- [ ] Update `SettingsService` to be project-aware.

## Phase 2: Frontend Infrastructure
- [ ] Add `Project` struct to Rust models.
- [ ] Update `AppState` to track `active_project`.
- [ ] Add `fetch_projects` and `select_project` methods to API client.

## Phase 3: UI Implementation
- [ ] Create `ProjectSelector` component in the header.
- [ ] Create `ProjectManager` dialog for adding/deleting projects.
- [ ] Wire up project switching to trigger global state refreshes.

## Phase 4: Refinement & Testing
- [ ] Verify file system isolation between projects.
- [ ] Test migration from old version.
- [ ] Add unit tests for project registry.
