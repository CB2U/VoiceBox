import os
import json
import logging
from fastapi import APIRouter, HTTPException
from ..models.settings import Settings

logger = logging.getLogger(__name__)
router = APIRouter()

from ..services.settings_service import load_settings, save_settings, DATA_DIR

# Using load_settings and save_settings from SettingsService

@router.get("/settings", response_model=Settings)
def get_settings():
    """Get current application settings."""
    logger.info("Retrieving settings")
    return load_settings()

@router.post("/settings", response_model=Settings)
def update_settings(settings: Settings):
    """Update application settings."""
    logger.info(f"Updating settings: {settings}")
    
    # Validate directories exist or can be created
    try:
        os.makedirs(settings.output_directory, exist_ok=True)
        os.makedirs(settings.voice_files_directory, exist_ok=True)
    except Exception as e:
        logger.error(f"Failed to create directories: {e}")
        raise HTTPException(
            status_code=400, 
            detail=f"Invalid directory path: {str(e)}"
        )
    
    save_settings(settings)
    return settings
