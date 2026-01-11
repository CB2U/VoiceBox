# Epic 1.1: Verification Walkthrough

## Prerequisites
- Ensure you are in `frontend/` directory.
- Run `cargo run`.

## Verification Steps

### 1. Visual Layout Check (AC-1)
- [ ] Verify the application window is split into two panels:
    - **Sidebar (Left)**: Contains the character list and "Add Character" button.
    - **Editor (Right)**: Contains input fields for Name, Description, and Voice Reference.
- [ ] Verify resizing the window maintains the layout.

### 2. Character CRUD & Persistence (AC-2)
- [ ] **Create**:
    - Click **"+ Add Character"**.
    - Verify a new entry "New Character" appears in the sidebar.
    - Verify the Editor panel populates with the details of the new character.
- [ ] **Update**:
    - Build the name field and change it to **"Gandalf"**.
    - Verify the name in the sidebar updates immediately as you type.
    - Add a description (e.g., "The Grey Wizard").
- [ ] **File Selection**:
    - Click **"Select File"** in the Voice Reference section.
    - Choose any `.wav` or `.mp3` file from your system.
    - Verify the absolute path appears in the text field.
- [ ] **Persistence**:
    - Close the application.
    - Run `cargo run` again to restart.
    - Verify **"Gandalf"** is still in the list.
    - Select it and verify the Description and Voice Reference path are preserved.
- [ ] **Delete**:
    - Click **"Delete Character"**.
    - Verify the character is removed from the list.
    - Restart the app and verify it remains deleted.
