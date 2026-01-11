from pydantic import BaseModel
from typing import Optional, Dict
from datetime import datetime, timezone
import uuid

class ScriptHistory(BaseModel):
    id: str
    name: str
    script_text: str
    audio_path: str  # Relative to project directory
    created_at: datetime
    character_mappings: Dict[str, str]  # character_name -> character_id

    @classmethod
    def create(
        cls,
        name: str,
        script_text: str,
        audio_path: str,
        character_mappings: Dict[str, str]
    ) -> "ScriptHistory":
        return cls(
            id=str(uuid.uuid4()),
            name=name,
            script_text=script_text,
            audio_path=audio_path,
            created_at=datetime.now(timezone.utc),
            character_mappings=character_mappings
        )
