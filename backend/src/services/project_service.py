import os
import json
import logging
import shutil
from typing import List, Optional
from ..models.project import Project
from .settings_service import load_settings, save_settings

logger = logging.getLogger(__name__)

class ProjectService:
    def __init__(self, projects_directory: str):
        self.projects_dir = projects_directory
        self.projects_file = os.path.join(projects_directory, "projects.json")
        os.makedirs(self.projects_dir, exist_ok=True)
        self._ensure_registry_exists()

    def _ensure_registry_exists(self):
        if not os.path.exists(self.projects_file):
            with open(self.projects_file, 'w') as f:
                json.dump([], f)

    def list_projects(self) -> List[Project]:
        try:
            with open(self.projects_file, 'r') as f:
                data = json.load(f)
                return [Project(**p) for p in data]
        except Exception as e:
            logger.error(f"Failed to list projects: {e}")
            return []

    def create_project(self, name: str) -> Project:
        project_id = os.path.basename(name).replace(' ', '_').lower() # Simple safe name
        # Actually use UUID for uniqueness as per model, but directory can be descriptive
        p = Project.create(name=name, base_path="") # Temporary
        
        # Override id for cleaner directory names if possible, or just keep UUID
        # Let's stick to UUID for ID but name the folder based on it or name
        project_dir = os.path.join(self.projects_dir, p.id)
        os.makedirs(project_dir, exist_ok=True)
        os.makedirs(os.path.join(project_dir, "voices"), exist_ok=True)
        
        p.base_path = project_dir
        
        projects = self.list_projects()
        projects.append(p)
        self._save_projects(projects)
        
        # Initialize project-specific character file
        char_file = os.path.join(project_dir, "characters.json")
        if not os.path.exists(char_file):
            with open(char_file, 'w') as f:
                json.dump([], f)
        
        # Initialize project-specific history file
        history_file = os.path.join(project_dir, "history.json")
        if not os.path.exists(history_file):
            with open(history_file, 'w') as f:
                json.dump([], f)
                
        return p

    def delete_project(self, project_id: str):
        projects = self.list_projects()
        project_to_delete = next((p for p in projects if p.id == project_id), None)
        if not project_to_delete:
            raise ValueError(f"Project {project_id} not found")

        # Optional: Delete directory? For now, just remove from registry
        # Better to keep it or move to trash to prevent accidental data loss
        
        new_projects = [p for p in projects if p.id != project_id]
        self._save_projects(new_projects)

    def _save_projects(self, projects: List[Project]):
        try:
            with open(self.projects_file, 'w') as f:
                json.dump([p.dict() for p in projects], f, indent=2, default=str)
        except Exception as e:
            logger.error(f"Failed to save projects: {e}")
            raise

    def get_project(self, project_id: str) -> Optional[Project]:
        projects = self.list_projects()
        return next((p for p in projects if p.id == project_id), None)

    def migrate_legacy_data(self):
        """Migrate old characters.json and voices to a default project."""
        # Legacy data would be in the parent directory of projects_dir
        data_dir = os.path.dirname(self.projects_dir)
        legacy_char_file = os.path.join(data_dir, "characters.json")
        legacy_voices_dir = os.path.join(data_dir, "voices")
        
        if os.path.exists(legacy_char_file) or os.path.exists(legacy_voices_dir):
            logger.info("Legacy data found. Starting migration...")
            
            # Check if default project already exists
            projects = self.list_projects()
            if any(p.name == "Default Project" for p in projects):
                logger.info("Default project already exists. Skipping migration.")
                return

            default_project = self.create_project("Default Project")
            
            # Migrate characters
            if os.path.exists(legacy_char_file):
                target_char_file = os.path.join(default_project.base_path, "characters.json")
                shutil.move(legacy_char_file, target_char_file)
                logger.info(f"Moved {legacy_char_file} to {target_char_file}")

            # Migrate voices
            if os.path.exists(legacy_voices_dir):
                target_voices_dir = os.path.join(default_project.base_path, "voices")
                # Move contents if target exists, or move entire dir
                if os.path.exists(target_voices_dir):
                    for item in os.listdir(legacy_voices_dir):
                        s = os.path.join(legacy_voices_dir, item)
                        d = os.path.join(target_voices_dir, item)
                        shutil.move(s, d)
                    os.rmdir(legacy_voices_dir)
                else:
                    shutil.move(legacy_voices_dir, target_voices_dir)
                logger.info(f"Moved voice files to {target_voices_dir}")
            
            logger.info("Migration completed successfully.")
            
            # Set default project as active if none is active
            settings = load_settings()
            if not settings.active_project_id:
                settings.active_project_id = default_project.id
                save_settings(settings)
                logger.info(f"Set {default_project.id} as active project")

    def ensure_active_project(self):
        """Ensure at least one project exists and is set as active."""
        projects = self.list_projects()
        settings = load_settings()
        
        if not projects:
            p = self.create_project("Default Project")
            settings.active_project_id = p.id
            save_settings(settings)
            logger.info("Created and set Default Project as active")
        elif not settings.active_project_id:
            settings.active_project_id = projects[0].id
            save_settings(settings)
            logger.info(f"Set existing project {projects[0].id} as active")
