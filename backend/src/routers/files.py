import os
import logging
from datetime import datetime
from typing import List
from fastapi import APIRouter, HTTPException
from fastapi.responses import FileResponse
from pydantic import BaseModel
from .settings import load_settings

logger = logging.getLogger(__name__)
router = APIRouter()

class VoiceFile(BaseModel):
    """Voice reference file metadata."""
    filename: str
    path: str
    size: int
    created_at: str

class RenameRequest(BaseModel):
    """Request to rename a file."""
    old_path: str
    new_name: str

@router.get("/files/voice-references", response_model=List[VoiceFile])
def list_voice_files():
    """List all voice reference files in the configured directory."""
    settings = load_settings()
    voice_dir = settings.voice_files_directory
    
    logger.info(f"Listing voice files from {voice_dir}")
    
    if not os.path.exists(voice_dir):
        logger.warning(f"Voice directory does not exist: {voice_dir}")
        return []
    
    files = []
    try:
        for filename in os.listdir(voice_dir):
            filepath = os.path.join(voice_dir, filename)
            
            # Only include audio files
            if os.path.isfile(filepath) and filename.lower().endswith(('.wav', '.mp3', '.ogg', '.flac')):
                stat = os.stat(filepath)
                files.append(VoiceFile(
                    filename=filename,
                    path=filepath,
                    size=stat.st_size,
                    created_at=datetime.fromtimestamp(stat.st_ctime).isoformat()
                ))
        
        logger.info(f"Found {len(files)} voice files")
        return files
    except Exception as e:
        logger.error(f"Failed to list voice files: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@router.post("/files/rename")
def rename_file(request: RenameRequest):
    """Rename a voice reference file."""
    logger.info(f"Renaming {request.old_path} to {request.new_name}")
    
    if not os.path.exists(request.old_path):
        raise HTTPException(status_code=404, detail="File not found")
    
    # Get directory and construct new path
    directory = os.path.dirname(request.old_path)
    new_path = os.path.join(directory, request.new_name)
    
    # Check if new name already exists
    if os.path.exists(new_path):
        raise HTTPException(status_code=400, detail="A file with that name already exists")
    
    try:
        os.rename(request.old_path, new_path)
        logger.info(f"Successfully renamed to {new_path}")
        return {"success": True, "new_path": new_path}
    except Exception as e:
        logger.error(f"Failed to rename file: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@router.delete("/files/voice-reference")
def delete_file(path: str):
    """Delete a voice reference file."""
    logger.info(f"Deleting file: {path}")
    
    if not os.path.exists(path):
        raise HTTPException(status_code=404, detail="File not found")
    
    try:
        os.remove(path)
        logger.info(f"Successfully deleted {path}")
        return {"success": True}
    except Exception as e:
        logger.error(f"Failed to delete file: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@router.get("/files/audio/{path:path}")
def serve_audio(path: str):
    """Serve an audio file for playback."""
    logger.info(f"Serving audio file: {path}")
    
    # Decode URL-encoded path
    import urllib.parse
    decoded_path = urllib.parse.unquote(path)
    
    if not os.path.exists(decoded_path):
        raise HTTPException(status_code=404, detail="Audio file not found")
    
    # Determine MIME type
    ext = os.path.splitext(decoded_path)[1].lower()
    mime_types = {
        '.wav': 'audio/wav',
        '.mp3': 'audio/mpeg',
        '.ogg': 'audio/ogg',
        '.flac': 'audio/flac'
    }
    media_type = mime_types.get(ext, 'application/octet-stream')
    
    return FileResponse(
        decoded_path,
        media_type=media_type,
        headers={
            "Accept-Ranges": "bytes",
            "Access-Control-Allow-Origin": "*"
        }
    )
