from fastapi import FastAPI, HTTPException
from fastapi.responses import StreamingResponse
from fastapi.middleware.cors import CORSMiddleware
from pydantic import BaseModel
from .engine import SynthesisEngine
from .routers import youtube, settings, files

app = FastAPI()

# CORS middleware for audio file serving and frontend access
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],  # In production, specify exact origins
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include Routers
app.include_router(youtube.router, tags=["youtube"])
app.include_router(settings.router, tags=["settings"])
app.include_router(files.router, tags=["files"])

class SynthesisRequest(BaseModel):
    text: str
    reference_audio_path: str

@app.get("/health")
def health():
    return {
        "status": "ok",
        "version": "0.1.0"
    }

@app.post("/synthesize")
def synthesize(request: SynthesisRequest):
    if not request.text or not request.text.strip():
        raise HTTPException(status_code=400, detail="Text cannot be empty")
        
    try:
        engine = SynthesisEngine.get_instance()
        audio_buffer = engine.generate(request.text, request.reference_audio_path)
        return StreamingResponse(audio_buffer, media_type="audio/wav")
    except FileNotFoundError as e:
        raise HTTPException(status_code=400, detail=str(e))
    except Exception as e:
        print(f"Synthesis failed: {e}")
        raise HTTPException(status_code=500, detail=str(e))
