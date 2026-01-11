import os
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from ..services.youtube_service import YouTubeService

router = APIRouter()

class YouTubeRequest(BaseModel):
    url: str
    start_time: str
    end_time: str
    character_id: str

class AudioFileResponse(BaseModel):
    file_path: str
    filename: str

# Determine project root and data directory
# Current file: backend/src/routers/youtube.py
# Root: backend/src/routers/../../../..
# But simpler to assume we run from 'VoiceBox' root or 'VoiceBox/backend'
# Let's rely on an environment variable or a relative path from execution context if possible.
# Ideally, we should not hardcode paths, but for this task:
# We'll calculate it relative to this file.

CURRENT_FILE = os.path.abspath(__file__)
SRC_DIR = os.path.dirname(os.path.dirname(CURRENT_FILE)) # backend/src
BACKEND_DIR = os.path.dirname(SRC_DIR) # backend
PROJECT_ROOT = os.path.dirname(BACKEND_DIR) # VoiceBox

# We assume frontend/data is where we want to store things
DATA_DIR = os.getenv("VOICEBOX_DATA_DIR", os.path.join(PROJECT_ROOT, "frontend", "data"))

@router.post("/extract-from-youtube", response_model=AudioFileResponse)
def extract_youtube_audio(request: YouTubeRequest):
    service = YouTubeService()
    
    # Construct character's audio directory
    # Structure: frontend/data/characters/{character_id}/audio
    character_dir = os.path.join(DATA_DIR, "characters", request.character_id)
    audio_dir = os.path.join(character_dir, "audio")
    
    try:
        file_path = service.extract_audio(
            url=request.url,
            start_time=request.start_time,
            end_time=request.end_time,
            output_dir=audio_dir
        )
        
        return AudioFileResponse(
            file_path=file_path,
            filename=os.path.basename(file_path)
        )
        
    except Exception as e:
        print(f"Error processing YouTube request: {e}")
        raise HTTPException(status_code=500, detail=str(e))
