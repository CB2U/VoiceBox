# VoiceBox

**VoiceBox** is a local-first desktop application designed for Dungeon Masters and content creators to generate high-quality audio recaps and dialogues using AI voice cloning.

## What is VoiceBox?

VoiceBox combines a powerful script editor with a local AI synthesis engine. It allows you to create character profiles, associate them with reference audio (either from local files or extracted directly from YouTube), and synthesize complex scripts into high-quality WAV files—all without your data ever leaving your machine.

Whether you're a DM looking to bring your NPCs to life for a session recap, or a content creator building audio dramas on a budget, VoiceBox provides the tools to manage your "voice library" and produce professional-sounding audio with ease.

## Why VoiceBox?

Creating immersive audio often requires expensive equipment, professional voice actors, or cloud-based AI services that charge high fees and compromise privacy. For DMs and indie creators, these barriers often make high-quality audio unreachable.

VoiceBox exists to bridge this gap by providing:
- **Privacy Core:** All processing happens locally. Your scripts and voice samples are never uploaded to the cloud.
- **Integrated Workflow:** From YouTube audio extraction to script parsing and final synthesis, everything is in one app.
- **Accessibility:** A user-friendly desktop interface for complex CLI tools like Chatterbox and yt-dlp.

## Use Cases

### For Dungeon Masters
- **Session Recaps:** Create immersive audio summaries of your last session featuring the actual voices of key NPCs.
- **NPC Voice Library:** Build a consistent library of character voices so your players always recognize who is speaking.
- **Ambient Dialogue:** Generate background conversations between NPCs to play during exploration or social encounters.

### For Content Creators
- **Audio Dramas:** Produce high-quality voice acting for your stories without needing a full cast of actors.
- **Machinima & Animation:** Quickly generate dialogue for your videos with consistent character voices.
- **Prototyping:** Test out scripts and character interactions with AI voices before committing to final recordings.

## Quick Start

### Prerequisites
- **Python 3.10+**: Required for the backend synthesis engine.
- **Rust (Stable)**: Required for the frontend Dioxus application.
- **System Dependencies**: FFmpeg (for audio processing).

### Backend Setup
1. Navigate to the `backend` directory.
2. Create a virtual environment:
   ```bash
   python -m venv venv
   source venv/bin/activate  # On Windows: venv\Scripts\activate
   ```
3. Install dependencies:
   ```bash
   pip install -r requirements.txt
   ```
4. Start the backend:
   ```bash
   uvicorn src.main:app --reload
   ```
   The backend will be available at `http://localhost:8000`. You can verify it's running by visiting `http://localhost:8000/health`.

### Frontend Setup
1. Navigate to the `frontend` directory.
2. Run the application:
   ```bash
   cargo run
   ```
   The Dioxus desktop window will open once the build is complete.

## Architecture

VoiceBox follows a decoupled architecture, separating the high-performance Rust frontend from the machine-learning-heavy Python backend.

### Tech Stack
- **Frontend**: [Dioxus](https://dioxuslabs.com/) (Rust) - For a native, high-performance desktop UI.
- **Backend**: [FastAPI](https://fastapi.tiangolo.com/) (Python) - Serving as a wrapper for AI synthesis libraries.
- **AI Engine**: [Chatterbox](https://github.com/lucasnewman/chatterbox) - Local AI voice cloning and synthesis.
- **Utility Tools**: [yt-dlp](https://github.com/yt-dlp/yt-dlp) and [FFmpeg](https://ffmpeg.org/) - For audio extraction and processing.

### Communication Flow
1. **User Action**: The User interacts with the Rust UI to manage characters or write scripts.
2. **HTTP Request**: The Frontend sends an async POST request containing text and reference audio paths to the Backend.
3. **Inference**: The Python Backend runs the Chatterbox engine, performing GPU/CPU inference to generate audio.
4. **Response**: The Backend returns the path to the newly synthesized WAV file.
5. **Playback**: The Frontend updates the UI and provides playback controls for the user.

## Project Structure

```text
.
├── backend/            # Python/FastAPI server and AI synthesis logic
├── frontend/           # Rust/Dioxus desktop application UI
├── specs/              # Detailed feature specifications and implementation plans
├── docs/               # Project-wide documentation (PRD, Roadmap, etc.)
├── SPEC.md             # Current work pointer for the SDD workflow
├── SPECS.md            # Index of all project specifications
└── README.md           # This file
```

- **`backend/`**: Contains the FastAPI application and the integration with Chatterbox.
- **`frontend/`**: The core desktop UI built with Rust and Dioxus.
- **`specs/`**: Each subdirectory here represents a specific feature or epic, containing its own `spec.md`, `plan.md`, and `tasks.md`.
- **`docs/`**: Holds high-level documentation like the **PRD** (Product Requirements Document), the **Roadmap**, and the **Constitution**.

## Documentation

For more detailed information about the project's requirements, design, and progress, please refer to the following documents:

- **[PRD (Product Requirements Document)](docs/PRD.md)**: Detailed product goals, user stories, and functional requirements.
- **[Roadmap](docs/roadmap.md)**: High-level overview of project breakpoints and epics.
- **[Project Constitution](docs/constitution.md)**: Core principles, quality bars, and technical non-negotiables.
- **[Spec Index (SPECS.md)](SPECS.md)**: A complete list of all technical specifications for implemented features.
