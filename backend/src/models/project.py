from pydantic import BaseModel
from typing import Optional
from datetime import datetime, timezone
import uuid

class Project(BaseModel):
    id: str
    name: str
    base_path: str
    created_at: datetime

    @classmethod
    def create(cls, name: str, base_path: str) -> "Project":
        return cls(
            id=str(uuid.uuid4()),
            name=name,
            base_path=base_path,
            created_at=datetime.now(timezone.utc)
        )
