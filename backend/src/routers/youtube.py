import os
import logging
from fastapi import APIRouter, HTTPException
from pydantic import BaseModel
from ..services.youtube_service import YouTubeService
from .settings import load_settings

# Configure logging
logger = logging.getLogger(__name__)

router = APIRouter()

class YouTubeRequest(BaseModel):
    url: str
    start_time: str
    end_time: str
    character_id: str

class AudioFileResponse(BaseModel):
    file_path: str
    filename: str

@router.post("/extract-from-youtube", response_model=AudioFileResponse)
def extract_youtube_audio(request: YouTubeRequest):
    """
    Extract audio from a YouTube video within specified time range.
    
    Args:
        request: YouTubeRequest containing URL, time range, and character ID
        
    Returns:
        AudioFileResponse with file path and filename
        
    Raises:
        HTTPException: 400 for validation errors, 500 for server errors
    """
    logger.info(f"Received YouTube extraction request for character {request.character_id}")
    logger.info(f"URL: {request.url}, Time range: {request.start_time} - {request.end_time}")
    
    # Validate request fields
    if not request.url or not request.url.strip():
        logger.warning("Empty URL provided")
        raise HTTPException(status_code=400, detail="URL is required")
    
    if not request.start_time or not request.start_time.strip():
        logger.warning("Empty start time provided")
        raise HTTPException(status_code=400, detail="Start time is required")
    
    if not request.end_time or not request.end_time.strip():
        logger.warning("Empty end time provided")
        raise HTTPException(status_code=400, detail="End time is required")
    
    if not request.character_id or not request.character_id.strip():
        logger.warning("Empty character ID provided")
        raise HTTPException(status_code=400, detail="Character ID is required")
    
    # Load settings to get voice files directory
    settings = load_settings()
    voice_files_dir = settings.voice_files_directory
    logger.info(f"Using voice files directory from settings: {voice_files_dir}")
    
    # Create directory if it doesn't exist
    os.makedirs(voice_files_dir, exist_ok=True)
    
    service = YouTubeService()
    
    try:
        file_path = service.extract_audio(
            url=request.url.strip(),
            start_time=request.start_time.strip(),
            end_time=request.end_time.strip(),
            output_dir=voice_files_dir
        )
        
        logger.info(f"Successfully extracted audio to {file_path}")
        
        return AudioFileResponse(
            file_path=file_path,
            filename=os.path.basename(file_path)
        )
        
    except ValueError as e:
        # Validation errors (invalid URL, time format, etc.)
        logger.warning(f"Validation error: {e}")
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        # Server errors (download failures, file system errors, etc.)
        logger.error(f"Server error during YouTube extraction: {e}", exc_info=True)
        raise HTTPException(status_code=500, detail=str(e))

