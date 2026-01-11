import os
import logging
from typing import List
from fastapi import APIRouter, HTTPException, Depends
from ..models.project import Project
from ..services.project_service import ProjectService
from ..services.settings_service import load_settings

logger = logging.getLogger(__name__)
router = APIRouter()

def get_project_service():
    settings = load_settings()
    return ProjectService(settings.projects_directory)

@router.get("/projects", response_model=List[Project])
def list_projects(service: ProjectService = Depends(get_project_service)):
    return service.list_projects()

@router.post("/projects", response_model=Project)
def create_project(name: str, service: ProjectService = Depends(get_project_service)):
    try:
        return service.create_project(name)
    except Exception as e:
        logger.error(f"Failed to create project: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@router.delete("/projects/{project_id}")
def delete_project(project_id: str, service: ProjectService = Depends(get_project_service)):
    try:
        service.delete_project(project_id)
        return {"status": "success"}
    except ValueError as e:
        raise HTTPException(status_code=404, detail=str(e))
    except Exception as e:
        logger.error(f"Failed to delete project: {e}")
        raise HTTPException(status_code=500, detail=str(e))

@router.post("/projects/migrate")
def migrate_data(service: ProjectService = Depends(get_project_service)):
    try:
        service.migrate_legacy_data()
        return {"status": "migration process triggered"}
    except Exception as e:
        logger.error(f"Migration failed: {e}")
        raise HTTPException(status_code=500, detail=str(e))
