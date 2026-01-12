from fastapi import APIRouter, HTTPException
from fastapi.responses import FileResponse
import os
from typing import List
from ..models.audio_processing import (
    AudioPreviewRequest,
    AudioPreviewResponse,
    AudioProcessRequest,
    AudioProcessResponse,
    AudioFormat,
    QualitySettings
)
from ..models.history import ScriptHistory
from ..services.audio_processing_service import AudioProcessingService
from ..services.history_service import HistoryService
from ..services.project_service import ProjectService
from ..services.settings_service import DATA_DIR, load_settings

router = APIRouter(prefix="/audio")

# Initialize services
def get_project_service():
    settings = load_settings()
    return ProjectService(settings.projects_directory)

audio_service = AudioProcessingService()


@router.post("/preview", response_model=AudioPreviewResponse)
async def generate_preview(request: AudioPreviewRequest):
    """
    Generate a preview of audio with effects applied (first N seconds)
    """
    try:
        # Get the active project
        settings = load_settings()
        if not settings.active_project_id:
            raise HTTPException(status_code=400, detail="No active project")
        
        project_service = get_project_service()
        active_project = project_service.get_project(settings.active_project_id)
        if not active_project:
            raise HTTPException(status_code=404, detail="Active project not found")
        
        history_service = HistoryService(active_project.base_path)
        history_entry = history_service.get_history_entry(request.history_id)
        if not history_entry:
            raise HTTPException(status_code=404, detail="History entry not found")
        
        # Get the audio file path
        audio_path = os.path.join(active_project.base_path, history_entry.audio_path)
        
        if not os.path.exists(audio_path):
            raise HTTPException(status_code=404, detail="Audio file not found")
        
        # Generate preview
        preview_path = audio_service.generate_preview(
            audio_path=audio_path,
            pitch_shift=request.pitch_shift,
            speed_factor=request.speed_factor,
            duration=request.preview_duration
        )
        
        # Return preview URL (relative to preview directory)
        preview_filename = os.path.basename(preview_path)
        preview_url = f"/audio/file/preview/{preview_filename}"
        
        return AudioPreviewResponse(
            preview_url=preview_url,
            duration=request.preview_duration
        )
    
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        print(f"Preview generation failed: {e}")
        raise HTTPException(status_code=500, detail=f"Failed to generate preview: {str(e)}")


@router.post("/process", response_model=AudioProcessResponse)
async def process_audio(request: AudioProcessRequest):
    """
    Process full audio file with effects and save to project directory
    """
    try:
        # Get the active project
        settings = load_settings()
        if not settings.active_project_id:
            raise HTTPException(status_code=400, detail="No active project")
        
        project_service = get_project_service()
        active_project = project_service.get_project(settings.active_project_id)
        if not active_project:
            raise HTTPException(status_code=404, detail="Active project not found")
        
        history_service = HistoryService(active_project.base_path)
        history_entry = history_service.get_history_entry(request.history_id)
        if not history_entry:
            raise HTTPException(status_code=404, detail="History entry not found")
        
        # Get the audio file path
        audio_path = os.path.join(active_project.base_path, history_entry.audio_path)
        
        if not os.path.exists(audio_path):
            raise HTTPException(status_code=404, detail="Audio file not found")
        
        # Generate output filename
        original_name = os.path.basename(history_entry.audio_path)
        output_filename = audio_service.generate_output_filename(
            original_name=original_name,
            pitch=request.pitch_shift,
            speed=request.speed_factor,
            format=request.output_format.value
        )
        
        # Output path in project directory
        os.makedirs(os.path.join(active_project.base_path, "outputs"), exist_ok=True)
        output_path = os.path.join(active_project.base_path, "outputs", output_filename)
        
        # Process the audio file
        metadata = audio_service.process_audio_file(
            audio_path=audio_path,
            output_path=output_path,
            pitch_shift=request.pitch_shift,
            speed_factor=request.speed_factor,
            output_format=request.output_format,
            quality_settings=request.quality_settings
        )
        
        # Create new history entry for processed file
        processed_name = f"{history_entry.name} (processed)"
        new_history = ScriptHistory.create(
            name=processed_name,
            script_text=history_entry.script_text,
            audio_path=os.path.join("outputs", output_filename),
            character_mappings=history_entry.character_mappings
        )
        new_history.processed_from = request.history_id
        
        # Save to history
        history_service.save_history_entry(new_history)
        
        return AudioProcessResponse(
            processed_file_path=os.path.join("outputs", output_filename),
            duration=metadata["duration"],
            format=metadata["format"],
            file_size=metadata["file_size"],
            history_id=new_history.id
        )
    
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        print(f"Audio processing failed: {e}")
        raise HTTPException(status_code=500, detail=f"Failed to process audio: {str(e)}")


@router.get("/formats")
async def get_supported_formats() -> List[dict]:
    """
    Get list of supported audio formats
    """
    return [
        {"value": "wav", "label": "WAV (Lossless)", "description": "Uncompressed audio, best quality"},
        {"value": "mp3", "label": "MP3 (Lossy)", "description": "Compressed audio, good quality, small size"},
        {"value": "ogg", "label": "OGG (Lossy)", "description": "Compressed audio, open format"},
        {"value": "flac", "label": "FLAC (Lossless)", "description": "Compressed lossless audio"}
    ]


@router.get("/file/preview/{filename}")
async def serve_preview_file(filename: str):
    """
    Serve preview audio files
    """
    preview_path = os.path.join(audio_service.preview_dir, filename)
    
    if not os.path.exists(preview_path):
        raise HTTPException(status_code=404, detail="Preview file not found")
    
    return FileResponse(
        preview_path,
        media_type="audio/wav",
        headers={"Cache-Control": "no-cache"}
    )


@router.get("/file/{project_id}/{file_path:path}")
async def serve_audio_file(project_id: str, file_path: str):
    """
    Serve processed audio files from project directory
    """
    full_path = os.path.join(DATA_DIR, "projects", project_id, file_path)
    
    if not os.path.exists(full_path):
        raise HTTPException(status_code=404, detail="Audio file not found")
    
    # Determine media type based on extension
    ext = os.path.splitext(file_path)[1].lower()
    media_types = {
        ".wav": "audio/wav",
        ".mp3": "audio/mpeg",
        ".ogg": "audio/ogg",
        ".flac": "audio/flac"
    }
    media_type = media_types.get(ext, "audio/wav")
    
    return FileResponse(full_path, media_type=media_type)
