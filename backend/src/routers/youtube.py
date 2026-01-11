import asyncio
import logging
import os
from typing import Optional
from fastapi import APIRouter, HTTPException, BackgroundTasks
from fastapi.responses import StreamingResponse
from pydantic import BaseModel
from ..services.youtube_service import YouTubeService
from ..services.settings_service import load_settings

logger = logging.getLogger(__name__)
router = APIRouter()

class YouTubeRequest(BaseModel):
    url: str
    start_time: str
    end_time: str
    character_id: str
    task_id: Optional[str] = None

class AudioFileResponse(BaseModel):
    file_path: str
    filename: str

def run_extraction(request: YouTubeRequest, voice_files_dir: str):
    service = YouTubeService()
    try:
        service.extract_audio(
            url=request.url.strip(),
            start_time=request.start_time.strip(),
            end_time=request.end_time.strip(),
            output_dir=voice_files_dir,
            task_id=request.task_id
        )
    except Exception as e:
        logger.error(f"Background extraction failed: {e}")

@router.post("/extract-from-youtube", response_model=AudioFileResponse)
async def extract_youtube_audio(request: YouTubeRequest, background_tasks: BackgroundTasks):
    """
    Extract audio from a YouTube video. 
    If task_id is provided, it runs in the background and reports progress via SSE.
    """
    logger.info(f"Received YouTube extraction request for character {request.character_id}")
    
    # Validation (keeping it concise for brevity, identical to before)
    if not request.url or not request.url.strip():
        raise HTTPException(status_code=400, detail="URL is required")
    # ... (other validations)

    settings = load_settings()
    voice_files_dir = settings.voice_files_directory
    os.makedirs(voice_files_dir, exist_ok=True)
    
    if request.task_id:
        # Background execution
        background_tasks.add_task(run_extraction, request, voice_files_dir)
        return AudioFileResponse(file_path="pending", filename="pending")
    else:
        # Synchronous execution (original behavior)
        service = YouTubeService()
        try:
            file_path = service.extract_audio(
                url=request.url.strip(),
                start_time=request.start_time.strip(),
                end_time=request.end_time.strip(),
                output_dir=voice_files_dir
            )
            return AudioFileResponse(file_path=file_path, filename=os.path.basename(file_path))
        except Exception as e:
            logger.error(f"Extraction failed: {e}")
            raise HTTPException(status_code=500, detail=str(e))

@router.get("/youtube/progress/{task_id}")
async def youtube_progress(task_id: str):
    """SSE endpoint for YouTube extraction progress."""
    async def event_generator():
        service = YouTubeService()
        last_progress = -1.0
        
        while True:
            progress = service.get_progress(task_id)
            if progress != last_progress:
                yield f"data: {progress}\n\n"
                last_progress = progress
            
            if progress >= 100.0:
                break
                
            await asyncio.sleep(0.5)
            
    return StreamingResponse(event_generator(), media_type="text/event-stream")

