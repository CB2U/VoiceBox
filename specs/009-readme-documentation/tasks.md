# Tasks: README Documentation Update

## Setup
- [x] **T1: Archive Current README** <!-- id: 0 -->
  - **Goal:** Preserve existing SDD template documentation
  - **Steps:** 
    1. Copy current `README.md` to `docs/SDD_TEMPLATE.md`
    2. Add a note at the top explaining it's archived content
  - **Done when:** `docs/SDD_TEMPLATE.md` exists with current README content
  - **Verify:** File exists and content matches current README
  - **Files:** `docs/SDD_TEMPLATE.md`

## Core Implementation
- [x] **T2: Write Project Overview Section** <!-- id: 1 -->
  - **Goal:** Create compelling introduction to VoiceBox
  - **Steps:**
    1. Write project title and one-sentence description
    2. Add "What is VoiceBox?" section explaining the tool
    3. Add "Why VoiceBox?" section explaining the problem it solves
    4. Reference PRD problem statement for accuracy
  - **Done when:** Overview clearly communicates project purpose
  - **Verify:** Read aloud to ensure clarity and impact
  - **Files:** `README.md`

- [x] **T3: Write Use Cases Section** <!-- id: 2 -->
  - **Goal:** Provide concrete examples for target users
  - **Steps:**
    1. List 2-3 use cases for DMs (D&D session recaps, NPC voice library)
    2. List 1-2 use cases for content creators (audio dramas, machinima)
    3. Keep each use case to 1-2 sentences
  - **Done when:** Use cases are specific and relatable
  - **Verify:** Check against PRD user stories (US-1 through US-6)
  - **Files:** `README.md`

- [x] **T4: Write Quick Start Guide** <!-- id: 3 -->
  - **Goal:** Enable users to run the application
  - **Steps:**
    1. List prerequisites (Python 3.10+, Rust stable, system dependencies)
    2. Document backend setup (venv creation, dependency installation)
    3. Document backend startup command
    4. Document frontend setup (cargo check)
    5. Document frontend startup command
    6. Add verification step (health check endpoint)
  - **Done when:** Commands are complete and tested
  - **Verify:** Execute commands on a clean environment or reference existing working setup
  - **Files:** `README.md`

- [x] **T5: Write Architecture Section** <!-- id: 4 -->
  - **Goal:** Document technical stack and design
  - **Steps:**
    1. List tech stack (Dioxus, FastAPI, Chatterbox, yt-dlp, etc.)
    2. Explain architecture pattern (Rust ↔ HTTP ↔ Python)
    3. Add simple component diagram or description
    4. Reference constitution.md for architecture decisions
  - **Done when:** Architecture is clear and accurate
  - **Verify:** Compare against `specs/001-foundation-architecture/plan.md`
  - **Files:** `README.md`

- [x] **T6: Write Project Structure Section** <!-- id: 5 -->
  - **Goal:** Explain directory layout
  - **Steps:**
    1. Create directory tree showing key folders
    2. Add brief description for each major directory (backend/, frontend/, specs/, docs/)
    3. Explain the purpose of SPEC.md, SPECS.md, PRD.md, roadmap.md
  - **Done when:** Structure matches actual project layout
  - **Verify:** Compare against actual directory structure
  - **Files:** `README.md`

- [x] **T7: Add Documentation Links** <!-- id: 6 -->
  - **Goal:** Connect README to detailed documentation
  - **Steps:**
    1. Add links to `docs/PRD.md`
    2. Add links to `docs/roadmap.md`
    3. Add links to `SPECS.md`
    4. Add links to `docs/constitution.md`
    5. Use relative paths for all links
    6. Add brief context for each link
  - **Done when:** All links are present and use relative paths
  - **Verify:** Click links in markdown preview
  - **Files:** `README.md`

## Verification
- [x] **T8: Manual Verification (AC-U1, AC-U2, AC-U3, AC-U4)** <!-- id: 7 -->
  - **Goal:** Verify all acceptance criteria
  - **Steps:**
    1. **AC-U1:** Read project overview and verify clarity
    2. **AC-U2:** Test quick start commands (or verify against running system)
    3. **AC-U3:** Review architecture documentation for completeness
    4. **AC-U4:** Click all documentation links and verify they work
  - **Done when:** All ACs are met
  - **Evidence:** Document verification results in spec.md EVIDENCE section
  - **Files:** N/A

## Tracking
- [x] **T9: Update SPECS Index** <!-- id: 8 -->
  - **Goal:** Keep index current
  - **Steps:** 
    1. Add new row to `SPECS.md` for spec 009
    2. Set status to "Completed"
    3. Add link to spec folder
  - **Done when:** SPECS.md includes this spec
  - **Files:** `SPECS.md`

- [x] **T10: Update SPEC.md Pointer** <!-- id: 9 -->
  - **Goal:** Update current work pointer
  - **Steps:**
    1. Update `SPEC.md` to reference `specs/009-readme-documentation/`
    2. Set status to "Completed"
  - **Done when:** SPEC.md points to this spec
  - **Files:** `SPEC.md`

- [x] **T11: Final Spec Update** <!-- id: 10 -->
  - **Goal:** Document evidence
  - **Steps:** 
    1. Update `spec.md` with verification summary from T8
    2. Fill `## EVIDENCE` section with AC verification results
  - **Done when:** `spec.md` has complete EVIDENCE section
  - **Files:** `specs/009-readme-documentation/spec.md`
