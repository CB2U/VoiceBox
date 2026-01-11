import os
import json
import logging
from ..models.settings import Settings

logger = logging.getLogger(__name__)

# Determine project root
CURRENT_FILE = os.path.abspath(__file__)
# src/services/settings_service.py -> src/services -> src -> backend -> project_root
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
                settings = Settings(**data)
        except Exception as e:
            logger.error(f"Failed to load settings: {e}")
            settings = Settings.get_default_settings(PROJECT_ROOT)
    else:
        settings = Settings.get_default_settings(PROJECT_ROOT)
    
    # Only set project-specific default if directory is the generic global default or empty
    if settings.active_project_id:
        project_dir = os.path.join(DATA_DIR, "projects", settings.active_project_id)
        default_project_voice_dir = os.path.join(project_dir, "voices")
        
        # If it's the global default or empty, switch to project-specific default
        global_default = os.path.join(PROJECT_ROOT, "frontend", "data", "voices")
        if settings.voice_files_directory == global_default or not settings.voice_files_directory:
            settings.voice_files_directory = default_project_voice_dir
    
    return settings

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
