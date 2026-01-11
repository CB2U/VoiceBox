import logging
from typing import List, Dict
from fastapi import APIRouter, HTTPException, Depends
from pydantic import BaseModel
from ..models.history import ScriptHistory
from ..services.history_service import HistoryService, get_history_service

logger = logging.getLogger(__name__)
router = APIRouter()


class CreateHistoryRequest(BaseModel):
    name: str
    script_text: str
    audio_path: str
    character_mappings: Dict[str, str]


@router.get("/history", response_model=List[ScriptHistory])
def list_history(service: HistoryService = Depends(get_history_service)):
    """Get all history entries for the active project."""
    try:
        return service.load_history()
    except Exception as e:
        logger.error(f"Failed to load history: {e}")
        raise HTTPException(status_code=500, detail=str(e))


@router.post("/history", response_model=ScriptHistory)
def create_history_entry(
    request: CreateHistoryRequest,
    service: HistoryService = Depends(get_history_service)
):
    """Create a new history entry."""
    try:
        entry = ScriptHistory.create(
            name=request.name,
            script_text=request.script_text,
            audio_path=request.audio_path,
            character_mappings=request.character_mappings
        )
        return service.save_history_entry(entry)
    except Exception as e:
        logger.error(f"Failed to create history entry: {e}")
        raise HTTPException(status_code=500, detail=str(e))


@router.patch("/history/{entry_id}", response_model=ScriptHistory)
def update_history_entry(
    entry_id: str,
    updates: Dict[str, str],
    service: HistoryService = Depends(get_history_service)
):
    """Update a history entry (e.g., rename)."""
    try:
        entry = service.update_history_entry(entry_id, updates)
        if not entry:
            raise HTTPException(status_code=404, detail="History entry not found")
        return entry
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Failed to update history entry: {e}")
        raise HTTPException(status_code=500, detail=str(e))


@router.delete("/history/{entry_id}")
def delete_history_entry(
    entry_id: str,
    service: HistoryService = Depends(get_history_service)
):
    """Delete a history entry and its associated audio file."""
    try:
        success = service.delete_history_entry(entry_id)
        if not success:
            raise HTTPException(status_code=404, detail="History entry not found")
        return {"status": "success"}
    except HTTPException:
        raise
    except Exception as e:
        logger.error(f"Failed to delete history entry: {e}")
        raise HTTPException(status_code=500, detail=str(e))
