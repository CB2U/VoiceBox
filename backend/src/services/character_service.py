import os
import json
import logging
from typing import List, Optional
from pydantic import BaseModel
from .settings_service import load_settings, save_settings, PROJECT_ROOT

logger = logging.getLogger(__name__)

class Character(BaseModel):
    id: str
    name: str
    description: Optional[str] = ""
    voice_reference_path: Optional[str] = ""

def get_characters_file(project_id: Optional[str] = None) -> str:
    if not project_id:
        # Fallback to legacy location for safety during transition
        return os.path.join(PROJECT_ROOT, "frontend", "data", "characters.json")
    
    return os.path.join(PROJECT_ROOT, "frontend", "data", "projects", project_id, "characters.json")

def load_characters() -> List[Character]:
    settings = load_settings()
    filepath = get_characters_file(settings.active_project_id)
    
    if os.path.exists(filepath):
        try:
            with open(filepath, 'r') as f:
                data = json.load(f)
                return [Character(**c) for c in data]
        except Exception as e:
            logger.error(f"Failed to load characters from {filepath}: {e}")
            return []
    return []

def save_characters(characters: List[Character]) -> None:
    settings = load_settings()
    filepath = get_characters_file(settings.active_project_id)
    
    os.makedirs(os.path.dirname(filepath), exist_ok=True)
    try:
        with open(filepath, 'w') as f:
            json.dump([c.dict() for c in characters], f, indent=2)
    except Exception as e:
        logger.error(f"Failed to save characters to {filepath}: {e}")
        raise
