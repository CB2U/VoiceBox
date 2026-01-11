import os
from pydantic import BaseModel
from typing import Optional

class Settings(BaseModel):
    """Application settings for directory configuration."""
    output_directory: str
    voice_files_directory: str
    active_project_id: Optional[str] = None
    
    @staticmethod
    def get_default_settings(project_root: str) -> "Settings":
        """Get default settings based on project structure."""
        return Settings(
            output_directory=os.path.join(project_root, "frontend", "data", "output"),
            voice_files_directory=os.path.join(project_root, "frontend", "data", "voices"),
            active_project_id=None
        )
