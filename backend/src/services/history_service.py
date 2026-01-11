import os
import json
import logging
from typing import List, Optional, Dict
from ..models.history import ScriptHistory
from .settings_service import load_settings

logger = logging.getLogger(__name__)

class HistoryService:
    def __init__(self, project_base_path: str):
        self.project_path = project_base_path
        self.history_file = os.path.join(project_base_path, "history.json")
        self._ensure_history_file_exists()

    def _ensure_history_file_exists(self):
        """Create history.json if it doesn't exist."""
        if not os.path.exists(self.history_file):
            os.makedirs(os.path.dirname(self.history_file), exist_ok=True)
            with open(self.history_file, 'w') as f:
                json.dump([], f)

    def load_history(self) -> List[ScriptHistory]:
        """Load all history entries for this project."""
        try:
            with open(self.history_file, 'r') as f:
                data = json.load(f)
                return [ScriptHistory(**entry) for entry in data]
        except Exception as e:
            logger.error(f"Failed to load history: {e}")
            return []

    def save_history_entry(self, entry: ScriptHistory) -> ScriptHistory:
        """Add a new history entry."""
        history = self.load_history()
        history.append(entry)
        self._save_history(history)
        logger.info(f"Saved history entry: {entry.id}")
        return entry

    def update_history_entry(self, entry_id: str, updates: Dict) -> Optional[ScriptHistory]:
        """Update an existing history entry (e.g., rename)."""
        history = self.load_history()
        for entry in history:
            if entry.id == entry_id:
                # Update allowed fields
                if 'name' in updates:
                    entry.name = updates['name']
                self._save_history(history)
                logger.info(f"Updated history entry: {entry_id}")
                return entry
        logger.warning(f"History entry not found: {entry_id}")
        return None

    def delete_history_entry(self, entry_id: str) -> bool:
        """Delete a history entry and its associated audio file."""
        history = self.load_history()
        entry_to_delete = None
        
        for entry in history:
            if entry.id == entry_id:
                entry_to_delete = entry
                break
        
        if not entry_to_delete:
            logger.warning(f"History entry not found: {entry_id}")
            return False

        # Delete the audio file
        audio_full_path = os.path.join(self.project_path, entry_to_delete.audio_path)
        if os.path.exists(audio_full_path):
            try:
                os.remove(audio_full_path)
                logger.info(f"Deleted audio file: {audio_full_path}")
            except Exception as e:
                logger.error(f"Failed to delete audio file: {e}")

        # Remove from history
        history = [e for e in history if e.id != entry_id]
        self._save_history(history)
        logger.info(f"Deleted history entry: {entry_id}")
        return True

    def _save_history(self, history: List[ScriptHistory]):
        """Save history list to file."""
        try:
            with open(self.history_file, 'w') as f:
                json.dump([entry.dict() for entry in history], f, indent=2, default=str)
        except Exception as e:
            logger.error(f"Failed to save history: {e}")
            raise


def get_history_service() -> HistoryService:
    """Get history service for the active project."""
    from .project_service import ProjectService
    
    settings = load_settings()
    if not settings.active_project_id:
        raise ValueError("No active project")
    
    project_service = ProjectService(settings.projects_directory)
    project = project_service.get_project(settings.active_project_id)
    
    if not project:
        raise ValueError(f"Active project not found: {settings.active_project_id}")
    
    return HistoryService(project.base_path)
