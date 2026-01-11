from fastapi import FastAPI, HTTPException
from fastapi.responses import StreamingResponse
from pydantic import BaseModel
from .engine import SynthesisEngine
from .routers import youtube

app = FastAPI()

# Include Routers
app.include_router(youtube.router)

class SynthesisRequest(BaseModel):
    text: str
    reference_audio_path: str

@app.get("/health")
def health_check():
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
