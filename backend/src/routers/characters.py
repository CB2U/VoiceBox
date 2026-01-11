import logging
from typing import List
from fastapi import APIRouter, HTTPException
from ..services.character_service import Character, load_characters, save_characters

logger = logging.getLogger(__name__)
router = APIRouter()

# Note: Using Character from models.character_service for now. 
# In a cleaner architecture, this would be in models/character.py

@router.get("/characters", response_model=List[Character])
def list_characters():
    return load_characters()

@router.post("/characters", response_model=List[Character])
def update_characters(characters: List[Character]):
    try:
        save_characters(characters)
        return characters
    except Exception as e:
        logger.error(f"Failed to update characters: {e}")
        raise HTTPException(status_code=500, detail=str(e))
