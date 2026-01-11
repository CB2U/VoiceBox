import os
import json
import logging
from fastapi import APIRouter, HTTPException
from ..models.settings import Settings

logger = logging.getLogger(__name__)
router = APIRouter()

# Determine project root
CURRENT_FILE = os.path.abspath(__file__)
SRC_DIR = os.path.dirname(os.path.dirname(CURRENT_FILE))
BACKEND_DIR = os.path.dirname(SRC_DIR)
PROJECT_ROOT = os.path.dirname(BACKEND_DIR)
DATA_DIR = os.path.join(PROJECT_ROOT, "frontend", "data")
SETTINGS_FILE = os.path.join(DATA_DIR, "settings.json")

def load_settings() -> Settings:
    """Load settings from file or return defaults."""
    if os.path.exists(SETTINGS_FILE):
        try:
            with open(SETTINGS_FILE, 'r') as f:
                data = json.load(f)
                return Settings(**data)
        except Exception as e:
            logger.error(f"Failed to load settings: {e}")
            return Settings.get_default_settings(PROJECT_ROOT)
    else:
        return Settings.get_default_settings(PROJECT_ROOT)

def save_settings(settings: Settings) -> None:
    """Save settings to file."""
    os.makedirs(DATA_DIR, exist_ok=True)
    try:
        with open(SETTINGS_FILE, 'w') as f:
            json.dump(settings.dict(), f, indent=2)
        logger.info(f"Settings saved to {SETTINGS_FILE}")
    except Exception as e:
        logger.error(f"Failed to save settings: {e}")
        raise

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
